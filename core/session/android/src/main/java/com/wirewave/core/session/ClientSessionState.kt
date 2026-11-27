package com.wirewave.core.session

import com.wirewave.core.discovery.DiscoveredHost
import com.wirewave.core.transport.TransportKind

sealed interface ClientSessionState {
    data class Disconnected(val errorMessage: String? = null) : ClientSessionState
    data class Connecting(val host: DiscoveredHost) : ClientSessionState
    data class AwaitingOtp(
        val host: DiscoveredHost,
        val errorMessage: String? = null
    ) : ClientSessionState
    data class Connected(
        val desktopId: String,
        val pcName: String,
        val transport: TransportKind
    ) : ClientSessionState
}
