package com.wirewave.features.connection.discovery

import com.wirewave.core.discovery.DiscoveredHost
import com.wirewave.core.discovery.HostScanner
import com.wirewave.core.session.SessionController
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class DiscoveryUiState(
    val isScanning: Boolean = false,
    val hosts: List<DiscoveredHost> = emptyList()
)

class DiscoveryController(
    private val scope: CoroutineScope,
    private val scanner: HostScanner,
    private val session: SessionController
) {
    private val _state = MutableStateFlow(DiscoveryUiState())
    val state: StateFlow<DiscoveryUiState> = _state.asStateFlow()

    fun scan(allowAutoConnect: Boolean = false) {
        if (_state.value.isScanning) return
        _state.update { it.copy(isScanning = true) }
        scope.launch {
            val found = scanner.scanAll()
            _state.update { DiscoveryUiState(isScanning = false, hosts = found) }

            if (allowAutoConnect && session.store.connectionPrefs.value.autoConnect && !session.store.wasExplicitlyDisconnected()) {
                val lastId = session.store.lastDesktopId()
                val match = found.firstOrNull { it.desktopId == lastId && !it.busy }
                if (match != null) {
                    session.connect(match)
                }
            }
        }
    }
}
