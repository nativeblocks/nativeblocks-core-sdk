import com.vanniktech.maven.publish.AndroidSingleVariantLibrary
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.vanniktech.publish)
}

val moduleVersion: String = Regex("""(?ms)^\[versions].*?^\s*android\s*=\s*"([^"]+)"""")
    .find(file("$rootDir/../../../versions.toml").readText())
    ?.groupValues?.get(1)
    ?: error("no [versions].android in versions.toml")

android {
    namespace = "io.nativeblocks.devkit"
    compileSdk = 35

    defaultConfig {
        minSdk = 26
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android.txt"), "proguard-rules.pro")
            buildConfigField("String", "VERSION", "\"${moduleVersion}\"")
        }
        getByName("debug") {
            buildConfigField("String", "VERSION", "\"${moduleVersion}\"")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    buildFeatures {
        buildConfig = true
        compose = true
    }
}

kotlin {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_17)
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
        AndroidSingleVariantLibrary(
            variant = "release",
            sourcesJar = false,
            publishJavadocJar = false,
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
    //==========================kotlin===========================
    implementation(libs.androidx.core.ktx)
    implementation(libs.kotlinx.coroutines.core)
    implementation(libs.kotlinx.serialization.json)
    //==========================compose==========================
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.lifecycle.viewmodel.compose)
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.compose.material)
    implementation(libs.androidx.compose.material.icons.core)
    implementation(libs.androidx.compose.animation)
    implementation(libs.androidx.compose.ui.tooling.preview)
    debugImplementation(libs.androidx.compose.ui.tooling)
    //==========================remote===========================
    implementation(libs.ktor.client.core)
    implementation(libs.ktor.client.okhttp)
    implementation(libs.ktor.client.logging)
    implementation(libs.ktor.serialization.kotlinx.json)
    //==========================di=======================================
    implementation(platform(libs.koin.bom))
    implementation(libs.koin.core)
    implementation(libs.koin.android)
    implementation(libs.koin.androidx.compose)
    implementation(libs.koin.android.compat)
    //==========================nativeblocks=============================
    implementation(libs.nativeblocks.runtime)
    //==========================service=================================
    implementation(libs.androidx.lifecycle.service)
    //==========================cache===================================
    implementation(libs.androidx.datastore.preferences)
    //==========================barcode=================================
    implementation(libs.zxing.core)
    implementation(libs.zxing.android.embedded)
}

object ModuleInfo {
    const val GROUP_ID = "io.nativeblocks"
    const val ARTIFACT_ID = "devkit-android"
    const val DESCRIPTION = "Nativeblocks Devkit for Android"
    const val URL = "https://nativeblocks.io"
}