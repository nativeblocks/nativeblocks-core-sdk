import com.vanniktech.maven.publish.JavaLibrary
import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.SonatypeHost

plugins {
    id("java-library")
    id("org.jetbrains.kotlin.jvm")
    id("org.jetbrains.kotlin.plugin.serialization")
    id("com.vanniktech.maven.publish")
}

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

mavenPublishing {
    publishToMavenCentral(SonatypeHost.CENTRAL_PORTAL)
    coordinates(ModuleInfo.GROUP_ID, ModuleInfo.ARTIFACT_ID, ModuleInfo.VERSION)
//    signAllPublications()
    configure(
        JavaLibrary(
            javadocJar = JavadocJar.Javadoc(),
            sourcesJar = true,
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
    implementation("com.google.devtools.ksp:symbol-processing-api:2.2.0-2.0.2")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.11.0")
    implementation("com.squareup:kotlinpoet:1.18.1")
}

object ModuleInfo {
    const val GROUP_ID = "io.nativeblocks"
    const val ARTIFACT_ID = "compiler-android"
    const val VERSION = "1.0.0-beta1"
    const val DESCRIPTION = "Nativeblocks compiler for Android"
    const val URL = "https://nativeblocks.io"
}