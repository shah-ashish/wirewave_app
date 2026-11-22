import groovy.json.JsonSlurper

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
}

val repoRoot = rootDir.parentFile.parentFile

@Suppress("UNCHECKED_CAST")
fun readJson(relativePath: String): Map<String, Any> =
    JsonSlurper().parse(file("$repoRoot/$relativePath")) as Map<String, Any>

val versionJson = readJson("version.json")
val globalConfig = readJson("config/wirewave.config.json")
val themeJson = readJson("config/theme.json")

@Suppress("UNCHECKED_CAST")
val androidCfg = globalConfig["android"] as Map<String, Any>
@Suppress("UNCHECKED_CAST")
val updateIds = versionJson["update_app_ids"] as Map<String, Any>
@Suppress("UNCHECKED_CAST")
val themeColors = themeJson["colors"] as Map<String, String>
@Suppress("UNCHECKED_CAST")
val themeType = themeJson["typography"] as Map<String, Number>
@Suppress("UNCHECKED_CAST")
val themeMetrics = themeJson["metrics"] as Map<String, Number>

val appId = androidCfg["app_id"] as String
val updateAppId = updateIds["android"] as String
require(appId != updateAppId) { "Android app_id and update_app_id must be distinct." }

val buildChannel = (System.getenv("WIREWAVE_BUILD_CHANNEL") ?: "dev").lowercase()

fun parseArgbHex(hex: String): String {
    val clean = hex.removePrefix("#")
    return when (clean.length) {
        6 -> "0xFF${clean.uppercase()}L"
        8 -> "0x${clean.uppercase()}L"
        else -> error("Unsupported hex color: $hex")
    }
}

android {
    namespace = "com.wirewave.core.config"
    compileSdk = 34

    defaultConfig {
        minSdk = (androidCfg["min_sdk"] as Number).toInt()

        buildConfigField("String", "APP_NAME", "\"${globalConfig["app_name"]}\"")
        buildConfigField("String", "APP_ID", "\"$appId\"")
        buildConfigField("String", "UPDATE_APP_ID", "\"$updateAppId\"")
        buildConfigField("String", "VERSION_NAME", "\"${versionJson["version_name"]}\"")
        buildConfigField("int", "VERSION_CODE", "${(versionJson["version_code"] as Number).toInt()}")
        buildConfigField("String", "UPDATE_ENDPOINT", "\"${versionJson["update_endpoint"]}\"")
        buildConfigField("String", "STORE_URL", "\"${androidCfg["store_url"]}\"")
        buildConfigField("int", "MIN_SDK_REQUIRED", "${(androidCfg["min_sdk"] as Number).toInt()}")
        buildConfigField("String", "MIN_OS_LABEL", "\"${androidCfg["min_os_label"]}\"")
        buildConfigField("String", "BUILD_CHANNEL", "\"$buildChannel\"")

        // Compile-time theme tokens from config/theme.json
        themeColors.forEach { (key, value) ->
            buildConfigField("long", "COLOR_${key.uppercase()}", parseArgbHex(value))
        }
        themeType.forEach { (key, value) ->
            buildConfigField("float", "TYPE_${key.uppercase()}", "${value.toFloat()}f")
        }
        themeMetrics.forEach { (key, value) ->
            buildConfigField("float", "METRIC_${key.uppercase()}", "${value.toFloat()}f")
        }
    }

    buildFeatures {
        buildConfig = true
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
    implementation(platform("androidx.compose:compose-bom:2024.02.01"))
    implementation("androidx.compose.ui:ui")
}
