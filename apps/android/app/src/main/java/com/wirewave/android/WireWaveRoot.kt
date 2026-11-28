package com.wirewave.android

import android.Manifest
import android.content.pm.PackageManager
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.compose.BackHandler
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import androidx.core.content.ContextCompat
import com.wirewave.core.session.ClientSessionState
import com.wirewave.features.connection.connected.ConnectedHomeScreen
import com.wirewave.features.connection.discovery.DiscoveryScreen
import com.wirewave.features.connection.mouse.MouseScreen
import com.wirewave.features.connection.overlay.ScreenOverlay
import com.wirewave.features.connection.pairing.PairingCodeScreen
import com.wirewave.features.speaker.SpeakerCircleTile
import com.wirewave.features.mic.MicCircleTile

@Composable
internal fun WireWaveRoot(
    container: AppContainer,
    bluetoothPermissionsGranted: Boolean
) {
    val context = LocalContext.current
    var showSettings by rememberSaveable { mutableStateOf(false) }
    var connectedScreen by rememberSaveable { mutableStateOf("home") }
    var activeOverlay by rememberSaveable { mutableStateOf(ScreenOverlay.NONE) }
    val sessionState by container.sessionController.state.collectAsState()
    val discoveryState by container.discoveryController.state.collectAsState()
    val mousePrefs by container.trustStore.mousePrefs.collectAsState()
    val speakerState by container.speakerEngine.state.collectAsState()
    val speakerRemoteState by container.sessionController.speakerState.collectAsState()
    val micCaptureState by container.micCaptureEngine.state.collectAsState()
    val micRemoteState by container.sessionController.micState.collectAsState()
    val mouseRemoteState by container.sessionController.mouseState.collectAsState()
    val micPermissionLauncher = rememberLauncherForActivityResult(
        ActivityResultContracts.RequestPermission()
    ) { granted ->
        if (granted) container.sessionController.requestMicActive(true)
    }

    val onToggleMicrophone: () -> Unit = {
        when {
            micRemoteState.active -> container.sessionController.requestMicActive(false)
            ContextCompat.checkSelfPermission(
                context,
                Manifest.permission.RECORD_AUDIO
            ) == PackageManager.PERMISSION_GRANTED -> container.sessionController.requestMicActive(true)
            else -> micPermissionLauncher.launch(Manifest.permission.RECORD_AUDIO)
        }
    }

    SessionFeatureEffects(
        container = container,
        sessionState = sessionState,
        speakerRemoteState = speakerRemoteState,
        micRemoteState = micRemoteState,
        mouseRemoteState = mouseRemoteState,
        micError = micCaptureState.error
    )

    LaunchedEffect(connectedScreen) {
        if (connectedScreen == "mouse") {
            container.sessionController.requestMouseActive(true)
        } else {
            container.sessionController.requestMouseActive(false)
        }
    }

    LaunchedEffect(bluetoothPermissionsGranted) {
        container.discoveryController.scan(allowAutoConnect = true)
    }

    LaunchedEffect(sessionState) {
        if (sessionState !is ClientSessionState.Connected) {
            connectedScreen = "home"
            activeOverlay = ScreenOverlay.NONE
        }
    }

    BackHandler(enabled = activeOverlay != ScreenOverlay.NONE || connectedScreen != "home") {
        if (activeOverlay != ScreenOverlay.NONE) {
            activeOverlay = ScreenOverlay.NONE
        } else {
            connectedScreen = "home"
        }
    }

    val brandMark: @Composable () -> Unit = {
        Image(
            painter = painterResource(id = R.drawable.app_icon),
            contentDescription = null,
            modifier = Modifier.size(34.dp)
        )
    }

    if (showSettings) {
        AppSettingsRoute(
            container = container,
            sessionState = sessionState,
            microphoneActive = micRemoteState.active,
            onBack = { showSettings = false }
        )
        return
    }

    when (val state = sessionState) {
        is ClientSessionState.Disconnected -> DiscoveryScreen(
            uiState = discoveryState,
            errorMessage = state.errorMessage,
            onScan = { container.discoveryController.scan(allowAutoConnect = false) },
            onConnect = container.sessionController::connect,
            onOpenSettings = { showSettings = true },
            brandMark = brandMark
        )
        is ClientSessionState.Connecting -> DiscoveryScreen(
            uiState = discoveryState.copy(isScanning = true),
            errorMessage = null,
            connectingMessage = "Connecting to ${state.host.pcName}...",
            onScan = {},
            onConnect = {},
            onOpenSettings = { showSettings = true },
            brandMark = brandMark
        )
        is ClientSessionState.AwaitingOtp -> PairingCodeScreen(
            host = state.host,
            errorMessage = state.errorMessage,
            onSubmitCode = container.sessionController::submitOtp,
            onBack = container.sessionController::cancelPairing,
            onOpenSettings = { showSettings = true }
        )
        is ClientSessionState.Connected -> {
            when (connectedScreen) {
                "mouse" -> MouseScreen(
                    speakerActive = speakerRemoteState.allowed && speakerRemoteState.active,
                    micActive = micRemoteState.allowed && micRemoteState.active,
                    onToggleSpeaker = {
                        container.sessionController.requestSpeakerActive(!speakerRemoteState.active)
                    },
                    onToggleMic = onToggleMicrophone,
                    overlayMode = activeOverlay,
                    onOverlayChange = { activeOverlay = it },
                    onNavigateHome = {
                        activeOverlay = ScreenOverlay.NONE
                        connectedScreen = "home"
                    },
                    onNavigateSettings = {
                        showSettings = true
                    },
                    onSendKeyInput = container.sessionController::sendKeyInput,
                    mouseEngine = container.mouseEngine,
                    mousePrefs = mousePrefs
                )
                else -> ConnectedHomeScreen(
                    pcName = state.pcName,
                    transport = state.transport,
                    onDisconnect = {
                        container.speakerEngine.stop()
                        container.micCaptureEngine.stop()
                        container.sessionController.disconnectExplicitly()
                    },
                    onOpenSettings = { showSettings = true },
                    brandMark = brandMark,
                    onNavigateToMouse = {
                        activeOverlay = ScreenOverlay.NONE
                        connectedScreen = "mouse"
                    },
                    onSendKeyInput = container.sessionController::sendKeyInput,
                    activeOverlay = activeOverlay,
                    speakerTile = {
                        SpeakerCircleTile(
                            isOn = speakerState.isOn,
                            isBlocked = !speakerRemoteState.allowed,
                            onToggle = {
                                container.sessionController.requestSpeakerActive(!speakerRemoteState.active)
                            }
                        )
                    },
                    micTile = {
                        MicCircleTile(
                            isOn = micCaptureState.isOn,
                            isBlocked = !micRemoteState.allowed,
                            blockedDetail = micRemoteState.detail,
                            onToggle = onToggleMicrophone
                        )
                    }
                )
            }
        }
    }
}
