package com.wirewave.android

import android.content.Context
import android.util.Log
import com.wirewave.core.compatibility.CompatibilityChecker
import com.wirewave.core.compatibility.CompatibilityReport
import com.wirewave.core.config.AppConfig
import com.wirewave.core.identity.DeviceIdentity
import com.wirewave.core.identity.DeviceIdentityStore
import com.wirewave.core.updater.ApkInstaller
import com.wirewave.core.updater.UpdateDownloader
import com.wirewave.core.updater.UpdateManifestClient
import com.wirewave.features.settings.about.AboutLogic
import com.wirewave.features.settings.about.AboutSnapshot
import com.wirewave.features.settings.update.UpdateController
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import okhttp3.OkHttpClient
import java.util.concurrent.TimeUnit

/**
 * Manual DI container owned by [WireWaveApp].
 * Runs Stage 1 background bootstrap (compatibility + persistent device ID) once on startup.
 */
class AppContainer(context: Context) {
    private val appContext = context.applicationContext
    private val appScope = CoroutineScope(SupervisorJob() + Dispatchers.Main.immediate)

    private val httpClient: OkHttpClient = OkHttpClient.Builder()
        .connectTimeout(15, TimeUnit.SECONDS)
        .readTimeout(60, TimeUnit.SECONDS)
        .build()

    val compatibilityReport: CompatibilityReport = CompatibilityChecker.checkCompatibility()
    val deviceIdentity: DeviceIdentity = DeviceIdentityStore(appContext).fetchOrCreateDeviceId()

    val aboutSnapshot: AboutSnapshot = AboutLogic.loadSnapshot(
        context = appContext,
        identity = deviceIdentity,
        compatibility = compatibilityReport
    )

    val updateController: UpdateController = UpdateController(
        scope = appScope,
        manifestClient = UpdateManifestClient(httpClient),
        downloader = UpdateDownloader(appContext, httpClient),
        installer = ApkInstaller(appContext)
    )

    init {
        Log.i(
            TAG,
            "Bootstrap ready | appId=${AppConfig.appId} updateAppId=${AppConfig.updateAppId} " +
                "supported=${compatibilityReport.isSupported}"
        )
    }

    companion object {
        private const val TAG = "WireWaveBootstrap"
    }
}
