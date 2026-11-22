package com.wirewave.core.updater

import android.content.ActivityNotFoundException
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.provider.Settings
import androidx.core.content.FileProvider
import com.wirewave.core.config.AppConfig
import java.io.File

sealed interface InstallOutcome {
    data object Launched : InstallOutcome
    data object NeedsInstallPermission : InstallOutcome
    data class Failure(val message: String) : InstallOutcome
}

class ApkInstaller(context: Context) {
    private val appContext = context.applicationContext

    /** Launches the local APK installer via FileProvider (used in `BuildChannel.DEV`). */
    fun installApk(apkFile: File): InstallOutcome {
        if (!apkFile.exists()) {
            return InstallOutcome.Failure("Installer file not found. Please download again.")
        }
        if (!appContext.packageManager.canRequestPackageInstalls()) {
            val permIntent = Intent(
                Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES,
                Uri.parse("package:${appContext.packageName}")
            ).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
            appContext.startActivity(permIntent)
            return InstallOutcome.NeedsInstallPermission
        }

        val apkUri = FileProvider.getUriForFile(
            appContext,
            "${appContext.packageName}.fileprovider",
            apkFile
        )
        val installIntent = Intent(Intent.ACTION_VIEW).apply {
            setDataAndType(apkUri, "application/vnd.android.package-archive")
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        }
        return try {
            appContext.startActivity(installIntent)
            InstallOutcome.Launched
        } catch (e: ActivityNotFoundException) {
            InstallOutcome.Failure("No package installer available on this device.")
        }
    }

    /** Redirects the user to Google Play (used in `BuildChannel.STORE`). */
    fun openStoreListing(): InstallOutcome {
        val intent = Intent(Intent.ACTION_VIEW, Uri.parse(AppConfig.storeUrl))
            .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        return try {
            appContext.startActivity(intent)
            InstallOutcome.Launched
        } catch (e: ActivityNotFoundException) {
            InstallOutcome.Failure("Unable to open Google Play Store.")
        }
    }
}
