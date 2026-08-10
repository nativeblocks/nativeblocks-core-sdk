package io.nativeblocks.gradleplugin

import com.android.build.api.variant.ApplicationAndroidComponentsExtension
import com.android.build.api.variant.LibraryAndroidComponentsExtension
import com.android.build.gradle.AppExtension
import com.android.build.gradle.LibraryExtension
import com.google.devtools.ksp.gradle.KspExtension
import io.nativeblocks.gradleplugin.tasks.NativeblocksPrepareSchemaTask
import io.nativeblocks.gradleplugin.tasks.NativeblocksSyncTask
import kotlinx.serialization.json.Json
import org.gradle.api.GradleException
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.configurationcache.extensions.capitalized
import java.io.File

open class NativeblocksGradlePlugin : Plugin<Project> {

    private val json = Json { ignoreUnknownKeys = true; isLenient = true }

    override fun apply(project: Project) {
        project.plugins.withId("com.google.devtools.ksp") {
            project.afterEvaluate {
                configureKsp(project)
            }
        }

        val supportedComponents =
            listOf(project.androidAppComponent(), project.androidLibraryComponent())
        supportedComponents.forEach { component ->
            component?.onVariants { variant ->
                val flavor = "${variant.flavorName?.capitalized()}${variant.buildType?.capitalized()}"
                registerTask(project, flavor)
            }
        }
    }

    private fun readConfig(project: Project): NativeblocksConfig {
        val configFile = File(project.rootDir, "nativeblocks.json")

        if (!configFile.exists()) {
            throw GradleException(
                """
                nativeblocks.json not found in project root at: ${configFile.absolutePath}
                Please create a nativeblocks.json file and get the config from Nativeblocks Studio:
                {
                  "endpoint": "https://the-api-url.com",
                  "authToken": "your-token",
                  "organizationId": "your-org-id"
                }
                """.trimIndent()
            )
        }

        return try {
            json.decodeFromString<NativeblocksConfig>(configFile.readText())
        } catch (e: Exception) {
            throw GradleException("Failed to parse nativeblocks.json: ${e.message}", e)
        }
    }

    private fun configureKsp(project: Project) {
        GlobalState.basePackageName = project.namespace()
        GlobalState.moduleName = project.name

        project.extensions.findByType(KspExtension::class.java)?.apply {
            arg("basePackageName", GlobalState.basePackageName ?: "")
            arg("moduleName", GlobalState.moduleName?.capitalized() ?: "")
        }
    }

    private fun registerTask(project: Project, flavor: String) {
        val shouldBuild = project.hasProperty("nativeblocks.build") &&
                project.property("nativeblocks.build").toString().toBoolean()

        val assembleTaskName = "assemble$flavor"

        project.tasks.register("nativeblocksSync$flavor", NativeblocksSyncTask::class.java) {
            it.config.set(readConfig(project))
            it.flavor.set(flavor)
            it.basePackageName.set(project.namespace())
            it.moduleName.set(project.name)
        }.also { task ->
            if (shouldBuild) {
                task.configure { it.dependsOn(assembleTaskName) }
            }
        }

        project.tasks.register("nativeblocksPrepareSchema$flavor", NativeblocksPrepareSchemaTask::class.java) {
            it.flavor.set(flavor)
            it.basePackageName.set(project.namespace())
            it.moduleName.set(project.name)
        }.also { task ->
            if (shouldBuild) {
                task.configure { it.dependsOn(assembleTaskName) }
            }
        }
    }
}

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