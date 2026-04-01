import groovy.json.JsonSlurper
import java.io.ByteArrayOutputStream
import java.io.File

pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

fun findRustlsPlatformVerifierMavenDir(): File {
    val output = ByteArrayOutputStream()
    exec {
        commandLine(
            "cargo", "metadata",
            "--format-version", "1",
            "--manifest-path", "../../packages/rusaint-ffi/Cargo.toml",
            "--filter-platform", "aarch64-linux-android"
        )
        standardOutput = output
    }

    val metadata = JsonSlurper().parseText(output.toString()) as Map<*, *>
    val packages = metadata["packages"] as List<Map<*, *>>
    val manifestPath = packages.first { pkg ->
        pkg["name"] == "rustls-platform-verifier-android"
    }["manifest_path"] as String

    return File(manifestPath).parentFile.resolve("maven")
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
        maven {
            url = uri(findRustlsPlatformVerifierMavenDir())
            metadataSources {
                artifact()
            }
        }
    }
}

rootProject.name = "rusaint"
include(":lib")
include(":app")
