package com.wirewave.core.session

import com.wirewave.core.stream.ChannelId
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.channels.ReceiveChannel
import org.json.JSONObject

internal sealed interface OutboundControlCommand {
    data class FeatureControl(val channel: ChannelId, val active: Boolean) : OutboundControlCommand
    data class HeartbeatPong(val sequence: Long) : OutboundControlCommand
    data class KeyInput(val text: String?, val keyCode: String?) : OutboundControlCommand

    companion object {
        fun Speaker(active: Boolean): FeatureControl = FeatureControl(ChannelId.SPEAKER, active)
        fun Mic(active: Boolean): FeatureControl = FeatureControl(ChannelId.MIC, active)
    }

    fun encode(): ByteArray {
        val json = when (this) {
            is FeatureControl -> {
                val typeName = when (channel) {
                    ChannelId.SPEAKER -> "speaker_command"
                    ChannelId.MIC -> "mic_command"
                    ChannelId.MOUSE -> "mouse_command"
                    ChannelId.KEYBOARD -> "keyboard_command"
                }
                JSONObject().apply {
                    put("type", typeName)
                    put("active", active)
                }
            }
            is HeartbeatPong -> JSONObject().apply {
                put("type", "heartbeat_pong")
                put("seq", sequence)
            }
            is KeyInput -> JSONObject().apply {
                put("type", "key_input")
                if (text != null) put("text", text)
                if (keyCode != null) put("key_code", keyCode)
            }
        }
        return json.toString().toByteArray(Charsets.UTF_8)
    }
}

internal class OutboundControlQueue {
    private val channel = Channel<OutboundControlCommand>(Channel.BUFFERED)
    val commands: ReceiveChannel<OutboundControlCommand> = channel

    fun trySendFeature(targetChannel: ChannelId, active: Boolean): Boolean =
        channel.trySend(OutboundControlCommand.FeatureControl(targetChannel, active)).isSuccess

    fun trySendKeyInput(text: String?, keyCode: String?): Boolean =
        channel.trySend(OutboundControlCommand.KeyInput(text, keyCode)).isSuccess

    fun trySendSpeaker(active: Boolean): Boolean =
        trySendFeature(ChannelId.SPEAKER, active)

    fun trySendMic(active: Boolean): Boolean =
        trySendFeature(ChannelId.MIC, active)

    suspend fun sendHeartbeatPong(sequence: Long) {
        channel.send(OutboundControlCommand.HeartbeatPong(sequence))
    }

    fun close() {
        channel.close()
    }
}
