import com.vanniktech.maven.publish.AndroidSingleVariantLibrary

plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
    alias(libs.plugins.ksp)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.vanniktech.publish)
    alias(libs.plugins.nativeblocks)
}

val moduleVersion: String = Regex("""(?ms)^\[versions].*?^\s*android\s*=\s*"([^"]+)"""")
    .find(file("$rootDir/../../../versions.toml").readText())
    ?.groupValues?.get(1)
    ?: error("no [versions].android in versions.toml")

android {
    testOptions { unitTests.isIncludeAndroidResources = true }
    namespace = "io.nativeblocks.foundation"
    compileSdk = 35

    defaultConfig {
        minSdk = 26
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        getByName("release") {
            buildConfigField("String", "VERSION", "\"${moduleVersion}\"")
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android.txt"), "proguard-rules.pro")
        }
        getByName("debug") {
            buildConfigField("String", "VERSION", "\"${moduleVersion}\"")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = "17"
    }
    buildFeatures {
        compose = true
        buildConfig = true
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
    implementation(libs.androidx.lifecycle.viewmodel.compose)
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.compose.material)
    implementation(libs.androidx.compose.animation)
    implementation(libs.androidx.compose.ui.tooling.preview)
    debugImplementation(libs.androidx.compose.ui.tooling)
    implementation(libs.coil.compose)
    //==========================nativeblocks=======================
    implementation(libs.nativeblocks.runtime)
    ksp(libs.nativeblocks.compiler)
    implementation(libs.nativeblocks.compiler)
    //==========================test==============================
    testImplementation(libs.junit)
    testImplementation(libs.robolectric)
}

object ModuleInfo {
    const val GROUP_ID = "io.nativeblocks"
    const val ARTIFACT_ID = "foundation-android"
    const val DESCRIPTION = "Nativeblocks foundation SDK for Android"
    const val URL = "https://nativeblocks.io"
}