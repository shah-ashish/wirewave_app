package com.wirewave.android

import android.content.Context
import android.util.Log
import com.wirewave.android.service.WireWaveConnectionService
import com.wirewave.core.compatibility.CompatibilityChecker
import com.wirewave.core.compatibility.CompatibilityReport
import com.wirewave.core.config.AppConfig
import com.wirewave.core.crypto.KeypairStore
import com.wirewave.core.discovery.HostScanner
import com.wirewave.core.identity.DeviceIdentity
import com.wirewave.core.identity.DeviceIdentityStore
import com.wirewave.core.session.SessionController
import com.wirewave.core.session.TrustAndPrefsStore
import com.wirewave.core.updater.ApkInstaller
import com.wirewave.core.updater.UpdateDownloader
import com.wirewave.core.updater.UpdateManifestClient
import com.wirewave.features.connection.discovery.DiscoveryController
import com.wirewave.features.settings.about.AboutLogic
import com.wirewave.features.settings.about.AboutSnapshot
import com.wirewave.features.settings.update.UpdateController
import com.wirewave.features.speaker.SpeakerEngine
import com.wirewave.features.mic.MicCaptureEngine
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import okhttp3.OkHttpClient
import java.util.concurrent.TimeUnit

/**
 * Manual DI container owned by [WireWaveApp].
 * Runs Stage 1 bootstrap and wires Stage 2 discovery, crypto, and persistent session services.
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
    val trustStore: TrustAndPrefsStore = TrustAndPrefsStore(appContext)
    private val keypair = KeypairStore(appContext).fetchOrCreate()

    private val baseAboutSnapshot: AboutSnapshot = AboutLogic.loadSnapshot(
        context = appContext,
        identity = deviceIdentity,
        compatibility = compatibilityReport
    )

    val sessionController: SessionController = SessionController(
        scope = appScope,
        identity = deviceIdentity,
        deviceName = baseAboutSnapshot.deviceName,
        keypair = keypair,
        context = appContext,
        store = trustStore,
        onForegroundStateChanged = { active, pcName ->
            when {
                active && trustStore.connectionPrefs.value.keepAliveInBackground ->
                    WireWaveConnectionService.start(appContext, pcName ?: "PC")
                !active -> WireWaveConnectionService.stop(appContext)
            }
        }
    )

    val discoveryController: DiscoveryController = DiscoveryController(
        scope = appScope,
        scanner = HostScanner(appContext),
        session = sessionController
    )

    val updateController: UpdateController = UpdateController(
        scope = appScope,
        manifestClient = UpdateManifestClient(httpClient),
        downloader = UpdateDownloader(appContext, httpClient),
        installer = ApkInstaller(appContext)
    )

    val speakerEngine: SpeakerEngine = SpeakerEngine(appScope)
    val micCaptureEngine: MicCaptureEngine = MicCaptureEngine(appScope)

    fun updateMicrophoneForeground(active: Boolean, pcName: String) {
        if (active || trustStore.connectionPrefs.value.keepAliveInBackground) {
            WireWaveConnectionService.start(appContext, pcName, microphoneActive = active)
        } else {
            WireWaveConnectionService.stop(appContext)
        }
    }

    fun aboutSnapshotFor(connectedPc: String?): AboutSnapshot = AboutLogic.loadSnapshot(
        context = appContext,
        identity = deviceIdentity,
        compatibility = compatibilityReport,
        connectedPc = connectedPc
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
