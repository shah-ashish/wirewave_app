package com.wirewave.core.config

/** Controls whether updates download in-app ([DEV]) or redirect to Google Play ([STORE]). */
enum class BuildChannel {
    DEV,
    STORE;

    companion object {
        fun fromRaw(raw: String): BuildChannel = when (raw.lowercase()) {
            "store" -> STORE
            else -> DEV
        }
    }
}

/** Single compile-time authority synced from `config/wirewave.config.json` and `version.json`. */
object AppConfig {
    val appName: String = BuildConfig.APP_NAME
    val appId: String = BuildConfig.APP_ID
    val updateAppId: String = BuildConfig.UPDATE_APP_ID
    val versionName: String = BuildConfig.VERSION_NAME
    val versionCode: Int = BuildConfig.VERSION_CODE
    val updateEndpoint: String = BuildConfig.UPDATE_ENDPOINT
    val storeUrl: String = BuildConfig.STORE_URL
    val minSdkRequired: Int = BuildConfig.MIN_SDK_REQUIRED
    val minOsLabel: String = BuildConfig.MIN_OS_LABEL
    val buildChannel: BuildChannel = BuildChannel.fromRaw(BuildConfig.BUILD_CHANNEL)
}
