package io.nativeblocks.gradleplugin

import com.android.build.api.variant.ApplicationAndroidComponentsExtension
import com.android.build.api.variant.LibraryAndroidComponentsExtension
import com.android.build.api.variant.Variant
import com.android.build.gradle.AppExtension
import com.android.build.gradle.LibraryExtension
import com.google.devtools.ksp.gradle.KspExtension
import io.nativeblocks.gradleplugin.tasks.NativeblocksEmbedTask
import io.nativeblocks.gradleplugin.tasks.NativeblocksEmbeddedAssetsTask
import io.nativeblocks.gradleplugin.tasks.NativeblocksSyncTask
import org.gradle.api.GradleException
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.tasks.Delete
import java.io.File

open class NativeblocksGradlePlugin : Plugin<Project> {

    override fun apply(project: Project) {
        project.plugins.withId("com.google.devtools.ksp") {
            project.afterEvaluate {
                configureKsp(project)
            }
        }

        project.androidAppComponent()?.let { app ->
            registerEmbed(project)
            keepEmbeddedOnClean(project)
            app.onVariants { variant ->
                registerTask(project, variant)
                registerEmbeddedAssets(project, variant)
            }
        }
        project.androidLibraryComponent()?.onVariants { variant ->
            registerTask(project, variant)
        }
    }

    private fun Variant.flavor(): String =
        "${flavorName.orEmpty().upperFirst()}${buildType.orEmpty().upperFirst()}"

    private fun registerEmbed(project: Project) {
        project.tasks.register("nativeblocksEmbed", NativeblocksEmbedTask::class.java) {
            it.group = "nativeblocks"
            it.description = "Downloads the embeddable frames and localizations into this app."
            it.appDir.set(project.layout.projectDirectory)
            it.rootDir.set(project.rootProject.layout.projectDirectory)
        }
    }

    private fun keepEmbeddedOnClean(project: Project) {
        val buildDir = project.layout.buildDirectory
        project.tasks.withType(Delete::class.java).matching { it.name == "clean" }.configureEach { clean ->
            clean.setDelete(listOf(project.fileTree(buildDir) { it.exclude("$EMBEDDED_BUILD_DIR/**") }))
            val buildFolder = buildDir.get().asFile
            clean.doLast { deleteEmptyFolders(buildFolder) }
        }
    }

    private fun registerEmbeddedAssets(project: Project, variant: Variant) {
        val task = project.tasks.register(
            "nativeblocksEmbeddedAssets${variant.name.upperFirst()}",
            NativeblocksEmbeddedAssetsTask::class.java
        ) {
            it.embedded.from(project.layout.buildDirectory.dir(EMBEDDED_BUILD_DIR))
        }
        variant.sources.assets?.addGeneratedSourceDirectory(task, NativeblocksEmbeddedAssetsTask::outputDir)
    }

    private fun configureKsp(project: Project) {
        project.extensions.findByType(KspExtension::class.java)?.apply {
            arg("basePackageName", project.namespace().orEmpty())
            arg("moduleName", project.name.upperFirst())
        }
    }

    private fun integrationsDir(project: Project, variant: Variant) =
        project.layout.buildDirectory.dir(project.provider {
            val packagePath = project.namespace().orEmpty().replace(".", "/")
            "generated/ksp/${variant.name}/resources/$packagePath/integration/consumer"
        })

    private fun registerTask(project: Project, variant: Variant) {
        val flavor = variant.flavor()
        val shouldBuild = project.hasProperty("nativeblocks.build") &&
                project.property("nativeblocks.build").toString().toBoolean()

        val assembleTaskName = "assemble$flavor"

        project.tasks.register("nativeblocksSync$flavor", NativeblocksSyncTask::class.java) {
            it.integrationsDir.set(integrationsDir(project, variant))
            it.rootDir.set(project.rootProject.layout.projectDirectory)
        }.also { task ->
            if (shouldBuild) {
                task.configure { it.dependsOn(assembleTaskName) }
            }
        }
    }
}

private const val EMBEDDED_BUILD_DIR = "nativeblocks/embedded"

private fun deleteEmptyFolders(root: File) {
    root.walkBottomUp()
        .filter { it != root && it.isDirectory && it.list().isNullOrEmpty() }
        .forEach { it.delete() }
}

private fun String.upperFirst(): String = replaceFirstChar { it.uppercase() }

private fun Project.androidAppComponent(): ApplicationAndroidComponentsExtension? =
    extensions.findByType(ApplicationAndroidComponentsExtension::class.java)

private fun Project.androidLibraryComponent(): LibraryAndroidComponentsExtension? =
    extensions.findByType(LibraryAndroidComponentsExtension::class.java)

private fun Project.namespace(): String? {
    val androidExtension = project.extensions.findByName("android")
        ?: throw GradleException("Could not retrieve namespace from Android extension")
    return when (androidExtension) {
        is AppExtension -> androidExtension.namespace
        is LibraryExtension -> androidExtension.namespace
        else -> {
            try {
                androidExtension::class.java.getMethod("getNamespace").invoke(androidExtension) as? String
            } catch (e: Exception) {
                throw GradleException("Could not retrieve namespace from Android extension: ${e.message}")
            }
        }
    }
}