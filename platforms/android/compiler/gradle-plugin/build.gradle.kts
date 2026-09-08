import com.vanniktech.maven.publish.GradlePlugin
import com.vanniktech.maven.publish.JavadocJar

plugins {
    alias(libs.plugins.java.library)
    alias(libs.plugins.java.gradle.plugin)
    alias(libs.plugins.maven.publish)
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.apollo)
    alias(libs.plugins.vanniktech.publish)
}

val moduleVersion: String = Regex("""(?ms)^\[versions].*?^\s*android\s*=\s*"([^"]+)"""")
    .find(file("$rootDir/../../../versions.toml").readText())
    ?.groupValues?.get(1)
    ?: error("no [versions].android in versions.toml")

gradlePlugin {
    plugins {
        create(ModuleInfo.ARTIFACT_ID) {
            id = ModuleInfo.GROUP_ID + "." + ModuleInfo.ARTIFACT_ID
            implementationClass = ModuleInfo.IMPLEMENTATION_CLASS
            displayName = ModuleInfo.ARTIFACT_ID
            description = ModuleInfo.DESCRIPTION
            version = moduleVersion
        }
    }
}

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

apollo {
    service("service") {
        packageName.set("io.nativeblocks.network")
    }
}

publishing {
    repositories {
        maven {
            name = "staging"
            url = uri(
                providers.gradleProperty("nbStagingRepo")
                    .getOrElse("$rootDir/../../../deploy/build/maven")
            )
        }
    }
}

mavenPublishing {
    coordinates(ModuleInfo.GROUP_ID, ModuleInfo.ARTIFACT_ID, moduleVersion)
    configure(
        GradlePlugin(
            javadocJar = JavadocJar.None(),
            sourcesJar = false
        )
    )

    pom {
        name.set(ModuleInfo.ARTIFACT_ID)
        description.set(ModuleInfo.DESCRIPTION)
        url.set(ModuleInfo.URL)
        licenses {
            license {
                name = "NATIVEBLOCKS TERMS OF SERVICE"
                url = "https://nativeblocks.io/terms-of-service"
            }
        }
        developers {
            developer {
                name = "Nativeblocks"
                email = "dev@nativeblocks.io"
            }
        }
        scm {
            connection = "scm:git:github.com/nativeblocks/nativeblocks-core-sdk.git"
            developerConnection = "scm:git:ssh://github.com/nativeblocks/nativeblocks-core-sdk.git"
            url = "https://github.com/nativeblocks/nativeblocks-core-sdk"
        }
    }
}

dependencies {
    implementation(gradleApi())
    implementation(libs.kotlinx.serialization.json)
    implementation(libs.kotlinx.coroutines.core)
    //==========================remote===========================
    implementation(libs.apollo.runtime)
    implementation(libs.okhttp)
    compileOnly(libs.android.gradle.plugin)
    //==========================KSP===========================
    compileOnly(libs.ksp.gradle.plugin)
}

object ModuleInfo {
    const val GROUP_ID = "io.nativeblocks"
    const val ARTIFACT_ID = "gradle-plugin-android"
    const val DESCRIPTION = "Nativeblocks gradle plugin for Android"
    const val URL = "https://nativeblocks.io"
    const val IMPLEMENTATION_CLASS = "io.nativeblocks.gradleplugin.NativeblocksGradlePlugin"
}

