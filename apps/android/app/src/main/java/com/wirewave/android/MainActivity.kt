package com.wirewave.android

import android.Manifest
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.SystemBarStyle
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
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
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import androidx.core.content.ContextCompat
import com.wirewave.core.session.ClientSessionState
import com.wirewave.features.connection.connected.ConnectedHomeScreen
import com.wirewave.features.connection.discovery.DiscoveryScreen
import com.wirewave.features.connection.pairing.PairingCodeScreen
import com.wirewave.features.settings.SettingsScreen

class MainActivity : ComponentActivity() {
    private var bluetoothPermissionsGranted by mutableStateOf(false)

    private val bluetoothPermissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { grants ->
        bluetoothPermissionsGranted = grants.values.all { it }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge(
            statusBarStyle = SystemBarStyle.dark(android.graphics.Color.BLACK),
            navigationBarStyle = SystemBarStyle.dark(android.graphics.Color.BLACK)
        )

        val container = (application as WireWaveApp).container
        bluetoothPermissionsGranted = hasBluetoothPermissions()

        setContent {
            WireWaveRoot(
                container = container,
                bluetoothPermissionsGranted = bluetoothPermissionsGranted
            )
        }

        if (!bluetoothPermissionsGranted && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            bluetoothPermissionLauncher.launch(
                arrayOf(
                    Manifest.permission.BLUETOOTH_SCAN,
                    Manifest.permission.BLUETOOTH_CONNECT
                )
            )
        }
    }

    private fun hasBluetoothPermissions(): Boolean {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S) return true
        return ContextCompat.checkSelfPermission(
            this,
            Manifest.permission.BLUETOOTH_SCAN
        ) == PackageManager.PERMISSION_GRANTED && ContextCompat.checkSelfPermission(
            this,
            Manifest.permission.BLUETOOTH_CONNECT
        ) == PackageManager.PERMISSION_GRANTED
    }
}

@Composable
private fun WireWaveRoot(
    container: AppContainer,
    bluetoothPermissionsGranted: Boolean
) {
    var showSettings by rememberSaveable { mutableStateOf(false) }
    val sessionState by container.sessionController.state.collectAsState()
    val discoveryState by container.discoveryController.state.collectAsState()
    val connectionPrefs by container.trustStore.connectionPrefs.collectAsState()

    LaunchedEffect(bluetoothPermissionsGranted) {
        container.discoveryController.scan(allowAutoConnect = true)
    }

    val brandMark: @Composable () -> Unit = {
        Image(
            painter = painterResource(id = R.drawable.app_icon),
            contentDescription = null,
            modifier = Modifier.size(34.dp)
        )
    }

    if (showSettings) {
        val connectedPc = (sessionState as? ClientSessionState.Connected)?.pcName
        SettingsScreen(
            aboutSnapshot = container.aboutSnapshotFor(connectedPc),
            updateController = container.updateController,
            connectionPrefs = connectionPrefs,
            onToggleAutoConnect = container.trustStore::setAutoConnect,
            onToggleKeepAlive = container.trustStore::setKeepAliveInBackground,
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
            errorMessage = "Connecting to ${state.host.pcName}...",
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
        is ClientSessionState.Connected -> ConnectedHomeScreen(
            pcName = state.pcName,
            transport = state.transport,
            onDisconnect = container.sessionController::disconnectExplicitly,
            onOpenSettings = { showSettings = true },
            brandMark = brandMark
        )
    }
}
