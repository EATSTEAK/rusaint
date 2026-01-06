import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.jreleaser.model.Active
import java.io.ByteArrayOutputStream

plugins {
    alias(libs.plugins.jreleaser)
    alias(libs.plugins.rust.android)
    alias(libs.plugins.android.library)
    alias(libs.plugins.jetbrains.kotlin.android)
    `maven-publish`
}

group = "dev.eatsteak"
description = "Easy and Reliable SSU u-saint scraper"
version = "0.13.6"

android {
    namespace = "dev.eatsteak.rusaint"

    buildToolsVersion = "34.0.0"
    ndkVersion = "29.0.14206865"
    compileSdk = 34

    defaultConfig {
        minSdk = 24

        version = project.version

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }

}

cargo {
    module = "../../.."
    libname = "rusaint-ffi"
    targets = listOf("arm", "x86", "arm64", "x86_64")
    profile = "release"
    targetIncludes = arrayOf("librusaint_ffi.so")
}

tasks.whenTaskAdded(closureOf<Task> {
    if (this.name == "releaseSourcesJar") {
        this.dependsOn("generateBindings")
    }
})

tasks.withType<KotlinCompile> {
    dependsOn("generateBindings")
}

tasks.register<Exec>("generateBindings") {
    dependsOn("cargoBuild")
    outputs.dir("src/main/kotlin")

    doFirst {
        mkdir("src/main/kotlin")
    }

    commandLine(
        "cargo", "run", "-p", "uniffi-bindgen", "generate",
        "./build/rustJniLibs/android/arm64-v8a/librusaint_ffi.so",
        "--library",
        "--language",
        "kotlin",
        "--no-format",
        "--out-dir",
        "src/main/kotlin"
    )

    // Add error handling
    errorOutput = ByteArrayOutputStream()
    doLast {
        if (executionResult.get().exitValue != 0) {
            throw GradleException("Failed to generate bindings: $errorOutput")
        }
    }
}

dependencies {
    //noinspection UseTomlInstead
    // See: https://github.com/gradle/gradle/issues/21267
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation(libs.kotlinx.coroutines.android)
    implementation(libs.core.ktx)
    testImplementation(libs.junit)
    androidTestImplementation(libs.ext.junit)
    androidTestImplementation(libs.espresso.core)
}

jreleaser {
    gitRootSearch = true

    project {
        name = "rusaint"
        copyright = "2024-2025 EATSTEAK (Koo Hyomin)"
        author("Koo Hyomin <me@eatsteak.dev>")
    }

    release {
        github {
            repoOwner = "EATSTEAK"
            name = "rusaint"
            skipTag = true
            skipRelease = true
        }
    }

    signing {
        active = Active.ALWAYS
        armored = true
        verify = true
    }

    deploy {
        maven {
            mavenCentral {
                create("sonatype") {
                    active = Active.ALWAYS
                    url = "https://central.sonatype.com/api/v1/publisher"
                    stagingRepository("build/staging-deploy")
                    applyMavenCentralRules = false
                    sign = true
                    checksums = true
                    javadocJar = true
                    retryDelay = 30
                    maxRetries = 240
                }
            }
        }
    }
}

publishing {
    repositories {
        maven {
            url = uri(layout.buildDirectory.dir("staging-deploy"))
        }
    }

    publications {
        register<MavenPublication>("release") {
            groupId = project.group.toString()
            artifactId = "rusaint"
            setVersion(project.version)

            afterEvaluate {
                from(components["release"])
            }

            pom {
                name.set("rusaint")
                description.set(project.description)
                url.set("https://github.com/eatsteak/rusaint")
                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://raw.githubusercontent.com/EATSTEAK/rusaint/refs/heads/main/LICENSE")
                    }
                }
                developers {
                    developer {
                        id.set("eatsteak")
                        name.set("Koo Hyomin")
                        url.set("https://eatsteak.dev")
                        email.set("me@eatsteak.dev")
                    }
                }
                scm {
                    connection = "scm:git:https://github.com/eatsteak/rusaint.git"
                    developerConnection = "scm:git:ssh://github.com/eatsteak/rusaint.git"
                    url = "https://github.com/eatsteak/rusaint"
                }
            }
        }
    }
}
