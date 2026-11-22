package com.wirewave.core.compatibility

import android.os.Build
import com.wirewave.core.config.AppConfig

sealed interface CompatibilityStatus {
    data object Supported : CompatibilityStatus
    data class Unsupported(val reason: String) : CompatibilityStatus
}

data class CompatibilityReport(
    val status: CompatibilityStatus,
    val detectedOs: String,
    val minimumRequired: String
) {
    val isSupported: Boolean
        get() = status is CompatibilityStatus.Supported

    val badgeLabel: String
        get() = when (status) {
            CompatibilityStatus.Supported -> "Supported"
            is CompatibilityStatus.Unsupported -> "Unsupported"
        }
}

object CompatibilityChecker {
    fun checkCompatibility(): CompatibilityReport =
        evaluateApiLevel(
            sdkInt = Build.VERSION.SDK_INT,
            releaseLabel = Build.VERSION.RELEASE ?: "Unknown",
            minSdk = AppConfig.minSdkRequired
        )

    /** Pure evaluator for deterministic JVM unit tests. */
    fun evaluateApiLevel(
        sdkInt: Int,
        releaseLabel: String,
        minSdk: Int
    ): CompatibilityReport {
        val detectedOs = "Android $releaseLabel (API $sdkInt)"
        val status = when {
            sdkInt >= minSdk -> CompatibilityStatus.Supported
            else -> CompatibilityStatus.Unsupported(
                "${AppConfig.appName} requires ${AppConfig.minOsLabel} or newer. Detected: $detectedOs."
            )
        }
        return CompatibilityReport(
            status = status,
            detectedOs = detectedOs,
            minimumRequired = AppConfig.minOsLabel
        )
    }
}
