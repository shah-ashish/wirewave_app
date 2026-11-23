package com.wirewave.core.stream

import com.wirewave.core.config.AppConfig
import org.bouncycastle.crypto.modes.ChaCha20Poly1305
import org.bouncycastle.crypto.params.AEADParameters
import org.bouncycastle.crypto.params.KeyParameter
import java.nio.ByteBuffer
import java.security.MessageDigest

enum class ChannelId(val wireByte: Byte) {
    SPEAKER(1),
    MIC(2),
    MOUSE(3),
    KEYBOARD(4),
    CAMERA(5);

    companion object {
        fun fromByte(raw: Byte): ChannelId? = entries.firstOrNull { it.wireByte == raw }
    }
}

data class DecryptedDatagram(
    val channel: ChannelId,
    val sequence: Long,
    val payload: ByteArray
)

/** 64-packet sliding anti-replay window matching Rust `ReplayWindow`. */
class ReplayWindow {
    private var initialized = false
    private var highestSeq = 0L
    private var bitmap = 0L

    fun checkAndRecord(seq: Long): Boolean {
        if (!initialized) {
            initialized = true
            highestSeq = seq
            bitmap = 1L
            return true
        }
        return when {
            seq > highestSeq -> {
                val shift = (seq - highestSeq).toInt()
                bitmap = if (shift >= 64) 1L else (bitmap shl shift) or 1L
                highestSeq = seq
                true
            }
            seq == highestSeq -> false
            else -> {
                val delta = (highestSeq - seq).toInt()
                if (delta >= 64) return false
                val mask = 1L shl delta
                if ((bitmap and mask) != 0L) return false
                bitmap = bitmap or mask
                true
            }
        }
    }
}

/**
 * Per-datagram ChaCha20-Poly1305 cipher wire-compatible with Rust `wirewave-core-stream`.
 * Header format (17 bytes, authenticated as AAD): `WW2U (4) | channel (1) | stream_id (4 BE) | seq (8 BE)`.
 */
class UdpCipher(
    rawKey: ByteArray,
    private val streamId: Int
) : AutoCloseable {
    private val key = rawKey.copyOf(32)
    private var sendSeq = 1L
    private val replay = ReplayWindow()

    fun encryptDatagram(channel: ChannelId, plaintext: ByteArray): ByteArray {
        val seq = sendSeq++
        val header = ByteBuffer.allocate(HEADER_BYTES)
            .put(MAGIC)
            .put(channel.wireByte)
            .putInt(streamId)
            .putLong(seq)
            .array()

        val nonce = buildNonce(channel.wireByte, seq)
        val cipher = ChaCha20Poly1305()
        cipher.init(true, AEADParameters(KeyParameter(key), 128, nonce, header))
        val out = ByteArray(cipher.getOutputSize(plaintext.size))
        val len = cipher.processBytes(plaintext, 0, plaintext.size, out, 0)
        cipher.doFinal(out, len)

        return ByteBuffer.allocate(HEADER_BYTES + out.size)
            .put(header)
            .put(out)
            .array()
    }

    fun decryptDatagram(datagram: ByteArray, length: Int = datagram.size): DecryptedDatagram? {
        if (length < HEADER_BYTES + 16) return null
        if (!datagram.copyOfRange(0, 4).contentEquals(MAGIC)) return null

        val header = datagram.copyOfRange(0, HEADER_BYTES)
        val buf = ByteBuffer.wrap(header)
        buf.position(4)
        val channel = ChannelId.fromByte(buf.get()) ?: return null
        val sid = buf.int
        if (sid != streamId) return null
        val seq = buf.long

        val nonce = buildNonce(channel.wireByte, seq)
        val cipher = ChaCha20Poly1305()
        cipher.init(false, AEADParameters(KeyParameter(key), 128, nonce, header))
        val ctLen = length - HEADER_BYTES
        val out = ByteArray(cipher.getOutputSize(ctLen))

        val plain = try {
            val len = cipher.processBytes(datagram, HEADER_BYTES, ctLen, out, 0)
            val finalLen = cipher.doFinal(out, len)
            out.copyOf(len + finalLen)
        } catch (_: Exception) {
            return null
        }

        if (!replay.checkAndRecord(seq)) return null
        return DecryptedDatagram(channel, seq, plain)
    }

    override fun close() {
        key.fill(0)
    }

    private fun buildNonce(channelByte: Byte, seq: Long): ByteArray =
        ByteBuffer.allocate(12)
            .put(0).put(0).put(0).put(channelByte)
            .putLong(seq)
            .array()

    companion object {
        private val MAGIC = byteArrayOf('W'.code.toByte(), 'W'.code.toByte(), '2'.code.toByte(), 'U'.code.toByte())
        const val HEADER_BYTES = 17
        const val MAX_DATAGRAM_BYTES = 1400
        val UDP_HELLO_PAYLOAD: ByteArray = "WW2U_HELLO".toByteArray(Charsets.UTF_8)

        fun deriveUdpKey(sessionKey: ByteArray, streamId: Int): ByteArray {
            val digest = MessageDigest.getInstance("SHA-256")
            digest.update(AppConfig.connection.protocolDomain.toByteArray(Charsets.UTF_8))
            digest.update(":udp-stream:".toByteArray(Charsets.UTF_8))
            digest.update(sessionKey)
            digest.update(ByteBuffer.allocate(4).putInt(streamId).array())
            return digest.digest()
        }
    }
}
