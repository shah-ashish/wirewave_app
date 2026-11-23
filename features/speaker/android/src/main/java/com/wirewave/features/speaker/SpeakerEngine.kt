package com.wirewave.features.speaker

import com.wirewave.core.stream.ChannelId
import com.wirewave.core.stream.UdpStreamClient
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import java.io.IOException

data class SpeakerUiState(
    val isOn: Boolean = false,
    val latencyMode: LatencyMode = LatencyMode.BALANCED,
    val errorMessage: String? = null
)

/**
 * Receives encrypted UDP LAUD audio datagrams from Desktop and feeds [JitterBuffer] + [AudioTrackPlayer].
 */
class SpeakerEngine(
    private val scope: CoroutineScope,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) {
    private val jitterBuffer = JitterBuffer()
    private val player = AudioTrackPlayer(jitterBuffer)

    private val _state = MutableStateFlow(SpeakerUiState())
    val state: StateFlow<SpeakerUiState> = _state.asStateFlow()

    private var recvJob: Job? = null
    private var udpClient: UdpStreamClient? = null

    fun setLatencyMode(mode: LatencyMode) {
        jitterBuffer.setLatencyMode(mode)
        _state.update { it.copy(latencyMode = mode) }
    }

    fun start(hostAddress: String, hostUdpPort: Int, udpKey: ByteArray, streamId: Int) {
        stop()
        _state.update { it.copy(isOn = true, errorMessage = null) }
        player.start()

        recvJob = scope.launch(ioDispatcher) {
            val client = try {
                UdpStreamClient(hostAddress, hostUdpPort, udpKey, streamId)
            } catch (e: IOException) {
                _state.update { it.copy(isOn = false, errorMessage = "Unable to open UDP audio socket.") }
                player.stop()
                return@launch
            }
            udpClient = client

            try {
                // Punch through desktop firewall so server locks onto our UDP endpoint.
                repeat(3) { client.sendHello(ChannelId.SPEAKER) }

                while (isActive) {
                    val datagram = client.receiveNext() ?: continue
                    if (datagram.channel != ChannelId.SPEAKER) continue
                    val laud = LaudPacket.decode(datagram.payload) ?: continue
                    jitterBuffer.push(laud)
                }
            } catch (e: CancellationException) {
                throw e
            } catch (_: IOException) {
                _state.update { it.copy(isOn = false, errorMessage = "Audio stream interrupted.") }
            } finally {
                client.close()
                if (udpClient === client) udpClient = null
                player.stop()
                _state.update { it.copy(isOn = false) }
            }
        }
    }

    fun stop() {
        recvJob?.cancel()
        recvJob = null
        udpClient?.close()
        udpClient = null
        player.stop()
        _state.update { it.copy(isOn = false) }
    }
}
