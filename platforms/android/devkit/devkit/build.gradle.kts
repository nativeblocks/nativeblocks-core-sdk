import com.vanniktech.maven.publish.AndroidSingleVariantLibrary
import com.vanniktech.maven.publish.SonatypeHost
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.library")
    id("kotlin-android")
    id("org.jetbrains.kotlin.plugin.compose")
    id("org.jetbrains.kotlin.plugin.serialization")
    id("com.vanniktech.maven.publish")
}

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
            buildConfigField("String", "VERSION", "\"${ModuleInfo.VERSION}\"")
        }
        getByName("debug") {
            buildConfigField("String", "VERSION", "\"${ModuleInfo.VERSION}\"")
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
mavenPublishing {
    publishToMavenCentral(SonatypeHost.CENTRAL_PORTAL)
    coordinates(ModuleInfo.GROUP_ID, ModuleInfo.ARTIFACT_ID, ModuleInfo.VERSION)
//    signAllPublications()
    configure(
        AndroidSingleVariantLibrary(
            variant = "release",
            sourcesJar = true,
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
    implementation("androidx.core:core-ktx:1.16.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.11.0")
    //==========================compose==========================
    implementation(platform("androidx.compose:compose-bom:2026.06.01"))
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.10.0")
    implementation("androidx.activity:activity-compose:1.10.1")
    implementation("androidx.compose.material:material")
    implementation("androidx.compose.material:material-icons-core")
    implementation("androidx.compose.animation:animation")
    implementation("androidx.compose.ui:ui-tooling-preview")
    debugImplementation("androidx.compose.ui:ui-tooling")
    //==========================remote===========================
    implementation("io.ktor:ktor-client-core:2.3.12")
    implementation("io.ktor:ktor-client-okhttp:2.3.12")
    implementation("io.ktor:ktor-client-logging:2.3.12")
    implementation("io.ktor:ktor-serialization-kotlinx-json:2.3.12")
    //==========================di=======================================
    implementation(platform("io.insert-koin:koin-bom:4.1.1"))
    implementation("io.insert-koin:koin-core")
    implementation("io.insert-koin:koin-android")
    implementation("io.insert-koin:koin-androidx-compose")
    implementation("io.insert-koin:koin-android-compat")
    //==========================nativeblocks=============================
    implementation("io.nativeblocks:runtime-android:1.0.0-beta1")
    //==========================service=================================
    implementation("androidx.lifecycle:lifecycle-service:2.10.0")
    //==========================cache===================================
    implementation("androidx.datastore:datastore-preferences:1.2.1")
    //==========================barcode=================================
    implementation("com.google.zxing:core:3.5.4")
    implementation("com.journeyapps:zxing-android-embedded:4.3.0")
}

object ModuleInfo {
    const val GROUP_ID = "io.nativeblocks"
    const val ARTIFACT_ID = "devkit-android"
    const val VERSION = "1.0.0"
    const val DESCRIPTION = "Nativeblocks Devkit for Android"
    const val URL = "https://nativeblocks.io"
}