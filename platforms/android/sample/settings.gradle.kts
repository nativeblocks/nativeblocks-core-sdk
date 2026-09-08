pluginManagement {
    repositories {
        maven {
            url = uri("https://maven.nativeblocks.io")
            content { includeGroupByRegex("io\\.nativeblocks.*") }
        }
        mavenLocal()
        gradlePluginPortal()
        google()
        mavenCentral()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.PREFER_SETTINGS)
    repositories {
        maven {
            url = uri("https://maven.nativeblocks.io")
            content { includeGroupByRegex("io\\.nativeblocks.*") }
        }
        mavenLocal()
        google()
        mavenCentral()
    }
}
rootProject.name = "sample"
include(":app")
