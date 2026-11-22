package com.wirewave.core.identity

import android.annotation.SuppressLint
import android.content.Context
import android.provider.Settings
import java.security.MessageDigest
import java.util.UUID

data class DeviceIdentity(
    val schemaVersion: Int,
    val deviceId: String
)

/**
 * Fetches or generates a persistent device ID that survives uninstall/reinstall.
 *
 * Uses Android's app-signing-key-scoped `Settings.Secure.ANDROID_ID` (SSAID)
 * hashed with an application-specific salt, so the same release keystore yields
 * the same ID across reinstalls without exposing raw platform identifiers.
 */
class DeviceIdentityStore(context: Context) {
    private val appContext = context.applicationContext
    private val prefs = appContext.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    fun fetchOrCreateDeviceId(): DeviceIdentity {
        loadStoredId()?.let { return it }

        val created = DeviceIdentity(
            schemaVersion = SCHEMA_VERSION,
            deviceId = generateDeviceId()
        )
        saveStoredId(created)
        return created
    }

    private fun loadStoredId(): DeviceIdentity? {
        val stored = prefs.getString(KEY_DEVICE_ID, null)?.trim()
        return stored?.takeIf { it.isNotEmpty() }?.let {
            DeviceIdentity(
                schemaVersion = prefs.getInt(KEY_SCHEMA_VERSION, SCHEMA_VERSION),
                deviceId = it
            )
        }
    }

    @SuppressLint("HardwareIds")
    private fun generateDeviceId(): String {
        val ssaid = Settings.Secure.getString(
            appContext.contentResolver,
            Settings.Secure.ANDROID_ID
        )?.trim()
        val seed = ssaid?.takeIf { it.isNotEmpty() } ?: UUID.randomUUID().toString()
        return deriveSaltedId(seed)
    }

    private fun saveStoredId(identity: DeviceIdentity) {
        prefs.edit()
            .putInt(KEY_SCHEMA_VERSION, identity.schemaVersion)
            .putString(KEY_DEVICE_ID, identity.deviceId)
            .apply()
    }

    companion object {
        private const val PREFS_NAME = "wirewave_identity"
        private const val KEY_DEVICE_ID = "device_id"
        private const val KEY_SCHEMA_VERSION = "schema_version"
        private const val SCHEMA_VERSION = 1
        private const val DEVICE_ID_SALT = "wirewave:device:android:v1:"

        internal fun deriveSaltedId(seed: String): String {
            val digest = MessageDigest.getInstance("SHA-256")
                .digest((DEVICE_ID_SALT + seed.trim()).toByteArray(Charsets.UTF_8))
            val hex = digest.joinToString("") { "%02x".format(it) }
            return "wwd_${hex.take(24)}"
        }
    }
}
