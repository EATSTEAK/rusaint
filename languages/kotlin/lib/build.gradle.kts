import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    alias(libs.plugins.rust.android)
    alias(libs.plugins.android.library)
    alias(libs.plugins.jetbrains.kotlin.android)
}

android {
    namespace = "dev.eatsteak.rusaint"
    ndkVersion = "27.2.12479018"
    compileSdk = 34

    defaultConfig {
        minSdk = 24

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
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

tasks.withType<KotlinCompile> {
    dependsOn("generateBindings")
}

task<Exec>("generateBindings") {
    dependsOn("cargoBuild")
    commandLine("cargo", "run", "-p", "uniffi-bindgen", "generate",
        "./build/rustJniLibs/android/arm64-v8a/librusaint_ffi.so",
        "--library",
        "--language",
        "kotlin",
        "--no-format",
        "--out-dir",
        "src/main/kotlin")
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