package com.wirewave.features.settings.about

import android.content.Context
import android.os.Build
import android.provider.Settings
import com.wirewave.core.compatibility.CompatibilityReport
import com.wirewave.core.config.AppConfig
import com.wirewave.core.identity.DeviceIdentity

data class AboutSnapshot(
    val appName: String,
    val deviceName: String,
    val connectedPc: String,
    val appId: String,
    val deviceId: String,
    val compatibility: CompatibilityReport
)

object AboutLogic {
    fun loadSnapshot(
        context: Context,
        identity: DeviceIdentity,
        compatibility: CompatibilityReport,
        connectedPc: String? = null
    ): AboutSnapshot = AboutSnapshot(
        appName = AppConfig.appName,
        deviceName = resolveDeviceName(context),
        connectedPc = connectedPc?.takeIf { it.isNotBlank() } ?: "Not connected",
        appId = AppConfig.appId,
        deviceId = identity.deviceId,
        compatibility = compatibility
    )

    private fun resolveDeviceName(context: Context): String {
        val globalName = Settings.Global.getString(
            context.contentResolver,
            Settings.Global.DEVICE_NAME
        )?.trim()
        return globalName?.takeIf { it.isNotEmpty() }
            ?: Build.MODEL?.takeIf { it.isNotBlank() }
            ?: "Android Device"
    }
}
