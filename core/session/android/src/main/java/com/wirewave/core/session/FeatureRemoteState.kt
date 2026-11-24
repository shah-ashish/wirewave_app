package com.wirewave.core.session

import com.wirewave.core.stream.ChannelId

/**
 * Universal remote state representation for any real-time stream feature
 * negotiated over WireWave's encrypted control plane.
 */
data class FeatureRemoteState(
    val channel: ChannelId = ChannelId.SPEAKER,
    val allowed: Boolean = false,
    val active: Boolean = false,
    val hostAddress: String? = null,
    val udpPort: Int? = null,
    val streamId: Int? = null,
    val udpKey: ByteArray? = null,
    val detail: String? = null
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as FeatureRemoteState

        if (channel != other.channel) return false
        if (allowed != other.allowed) return false
        if (active != other.active) return false
        if (hostAddress != other.hostAddress) return false
        if (udpPort != other.udpPort) return false
        if (streamId != other.streamId) return false
        if (udpKey != null) {
            if (other.udpKey == null) return false
            if (!udpKey.contentEquals(other.udpKey)) return false
        } else if (other.udpKey != null) return false
        if (detail != other.detail) return false

        return true
    }

    override fun hashCode(): Int {
        var result = channel.hashCode()
        result = 31 * result + allowed.hashCode()
        result = 31 * result + active.hashCode()
        result = 31 * result + (hostAddress?.hashCode() ?: 0)
        result = 31 * result + (udpPort ?: 0)
        result = 31 * result + (streamId ?: 0)
        result = 31 * result + (udpKey?.contentHashCode() ?: 0)
        result = 31 * result + (detail?.hashCode() ?: 0)
        return result
    }
}

typealias SpeakerRemoteState = FeatureRemoteState
typealias MicRemoteState = FeatureRemoteState
