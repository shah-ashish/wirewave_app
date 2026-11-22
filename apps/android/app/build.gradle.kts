import groovy.json.JsonSlurper

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

val repoRoot = rootDir.parentFile.parentFile

@Suppress("UNCHECKED_CAST")
fun readJson(relativePath: String): Map<String, Any> =
    JsonSlurper().parse(file("$repoRoot/$relativePath")) as Map<String, Any>

val versionJson = readJson("version.json")
val globalConfig = readJson("config/wirewave.config.json")

@Suppress("UNCHECKED_CAST")
val androidCfg = globalConfig["android"] as Map<String, Any>
val appName = globalConfig["app_name"] as String

android {
    namespace = "com.wirewave.android"
    compileSdk = 34

    defaultConfig {
        applicationId = androidCfg["app_id"] as String
        minSdk = (androidCfg["min_sdk"] as Number).toInt()
        targetSdk = 34
        versionCode = (versionJson["version_code"] as Number).toInt()
        versionName = versionJson["version_name"] as String
        resValue("string", "app_name", appName)
    }

    signingConfigs {
        create("release") {
            val keystorePath = System.getenv("ANDROID_KEYSTORE_PATH")
            if (!keystorePath.isNullOrBlank() && file(keystorePath).exists()) {
                storeFile = file(keystorePath)
                storePassword = System.getenv("ANDROID_KEYSTORE_PASSWORD")
                keyAlias = System.getenv("ANDROID_KEY_ALIAS")
                keyPassword = System.getenv("ANDROID_KEY_PASSWORD")
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            val releaseSigning = signingConfigs.getByName("release")
            if (releaseSigning.storeFile != null) {
                signingConfig = releaseSigning
            }
        }
        debug {
            isMinifyEnabled = false
        }
    }

    buildFeatures {
        compose = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.8"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
}

dependencies {
    implementation(project(":core:config"))
    implementation(project(":core:identity"))
    implementation(project(":core:compatibility"))
    implementation(project(":core:updater"))
    implementation(project(":core:crypto"))
    implementation(project(":core:transport"))
    implementation(project(":core:discovery"))
    implementation(project(":core:session"))
    implementation(project(":features:settings"))
    implementation(project(":features:connection"))

    implementation("androidx.activity:activity-compose:1.8.2")
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation(platform("androidx.compose:compose-bom:2024.02.01"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.material3:material3")
}
