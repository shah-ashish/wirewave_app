package com.wirewave.core.session

import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.channels.ReceiveChannel
import org.json.JSONObject

internal sealed interface OutboundControlCommand {
    data class Speaker(val active: Boolean) : OutboundControlCommand
    data class Mic(val active: Boolean) : OutboundControlCommand
    data class HeartbeatPong(val sequence: Long) : OutboundControlCommand

    fun encode(): ByteArray {
        val json = when (this) {
            is Speaker -> JSONObject().apply {
                put("type", "speaker_command")
                put("active", active)
            }
            is Mic -> JSONObject().apply {
                put("type", "mic_command")
                put("active", active)
            }
            is HeartbeatPong -> JSONObject().apply {
                put("type", "heartbeat_pong")
                put("seq", sequence)
            }
        }
        return json.toString().toByteArray(Charsets.UTF_8)
    }
}

internal class OutboundControlQueue {
    private val channel = Channel<OutboundControlCommand>(Channel.BUFFERED)
    val commands: ReceiveChannel<OutboundControlCommand> = channel

    fun trySendSpeaker(active: Boolean): Boolean =
        channel.trySend(OutboundControlCommand.Speaker(active)).isSuccess

    fun trySendMic(active: Boolean): Boolean =
        channel.trySend(OutboundControlCommand.Mic(active)).isSuccess

    suspend fun sendHeartbeatPong(sequence: Long) {
        channel.send(OutboundControlCommand.HeartbeatPong(sequence))
    }

    fun close() {
        channel.close()
    }
}
