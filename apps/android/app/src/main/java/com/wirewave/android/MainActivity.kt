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
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.core.content.ContextCompat
import com.wirewave.core.session.ClientSessionState

class MainActivity : ComponentActivity() {
    private var bluetoothPermissionsGranted by mutableStateOf(false)

    private val runtimePermissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { _ ->
        bluetoothPermissionsGranted = hasBluetoothPermissions()
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

        requestNeededPermissions()
    }

    private fun requestNeededPermissions() {
        val needed = mutableListOf<String>()
        if (!bluetoothPermissionsGranted && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            needed.add(Manifest.permission.BLUETOOTH_SCAN)
            needed.add(Manifest.permission.BLUETOOTH_CONNECT)
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED) {
                needed.add(Manifest.permission.POST_NOTIFICATIONS)
            }
        }
        if (needed.isNotEmpty()) {
            runtimePermissionLauncher.launch(needed.toTypedArray())
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

    override fun onStop() {
        super.onStop()
        val container = (application as WireWaveApp).container
        if (!container.trustStore.connectionPrefs.value.keepAliveInBackground) {
            container.speakerEngine.stop()
            container.micCaptureEngine.stop()
            container.mouseEngine.stop()
            if (container.sessionController.state.value is ClientSessionState.Connected) {
                container.sessionController.disconnectForBackground()
            }
        }
    }

    override fun onStart() {
        super.onStart()
        val container = (application as WireWaveApp).container
        if (container.sessionController.state.value is ClientSessionState.Disconnected &&
            container.trustStore.connectionPrefs.value.autoConnect &&
            !container.trustStore.wasExplicitlyDisconnected()
        ) {
            container.discoveryController.scan(allowAutoConnect = true)
        }
    }
}
