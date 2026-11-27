package com.wirewave.core.session

import com.wirewave.core.crypto.hexToBytesOrNull
import com.wirewave.core.stream.ChannelId
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import org.json.JSONObject

/** Owns one feature's remote state and restores its defaults when a session ends. */
internal class RemoteFeatureStateStore(
    private val channel: ChannelId,
    private val allowedByDefault: Boolean
) {
    private val mutableState = MutableStateFlow(defaultState())
    val state: StateFlow<FeatureRemoteState> = mutableState.asStateFlow()

    fun updateFromMessage(message: JSONObject, hostAddress: String) {
        val active = message.optBoolean("active", false)
        val key = message.optString("udp_key_hex")
            .takeIf { active && it.length == 64 }
            ?.hexToBytesOrNull()
        mutableState.update {
            FeatureRemoteState(
                channel = channel,
                allowed = message.optBoolean("allowed", false),
                active = active,
                hostAddress = hostAddress,
                udpPort = message.optInt("udp_port").takeIf { active && it > 0 },
                streamId = message.optInt("stream_id").takeIf { active },
                udpKey = key,
                detail = message.optString("detail").takeIf { it.isNotBlank() }
            )
        }
    }

    fun reset() {
        mutableState.update { defaultState() }
    }

    private fun defaultState() = FeatureRemoteState(channel = channel, allowed = allowedByDefault)
}
