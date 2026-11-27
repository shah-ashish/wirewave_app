package com.wirewave.android

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.platform.LocalContext
import com.wirewave.android.service.WireWaveConnectionService
import com.wirewave.core.session.ClientSessionState
import com.wirewave.features.settings.SettingsScreen

/** App-level wiring for Settings; feature UI receives only state and callbacks. */
@Composable
internal fun AppSettingsRoute(
    container: AppContainer,
    sessionState: ClientSessionState,
    microphoneActive: Boolean,
    onBack: () -> Unit
) {
    val context = LocalContext.current
    val connectionPrefs by container.trustStore.connectionPrefs.collectAsState()
    val mousePrefs by container.trustStore.mousePrefs.collectAsState()
    val connectedPc = (sessionState as? ClientSessionState.Connected)?.pcName
    var trustedList by remember { mutableStateOf(container.trustStore.listTrustedDesktops()) }
    SettingsScreen(
        aboutSnapshot = container.aboutSnapshotFor(connectedPc),
        updateController = container.updateController,
        connectionPrefs = connectionPrefs,
        mousePrefs = mousePrefs,
        trustedDesktops = trustedList,
        onToggleAutoConnect = container.trustStore::setAutoConnect,
        onToggleKeepAlive = { enabled ->
            container.trustStore.setKeepAliveInBackground(enabled)
            val state = container.sessionController.state.value
            if (state is ClientSessionState.Connected) {
                if (enabled || microphoneActive) {
                    WireWaveConnectionService.start(
                        context,
                        state.pcName,
                        microphoneActive = microphoneActive
                    )
                } else {
                    WireWaveConnectionService.stop(context)
                }
            }
        },
        onPointerSpeedChange = container.trustStore::setPointerSpeed,
        onScrollSpeedChange = container.trustStore::setScrollSpeed,
        onToggleNaturalScroll = container.trustStore::setNaturalScroll,
        onToggleShowMouseButtons = container.trustStore::setShowMouseButtons,
        onForgetDesktop = { id ->
            container.trustStore.forgetDesktop(id)
            trustedList = container.trustStore.listTrustedDesktops()
            if ((sessionState as? ClientSessionState.Connected)?.desktopId == id) {
                container.sessionController.disconnectExplicitly()
            }
        },
        onBack = onBack
    )
}
