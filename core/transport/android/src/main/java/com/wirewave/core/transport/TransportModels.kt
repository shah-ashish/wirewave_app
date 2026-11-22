package com.wirewave.core.transport

import java.io.DataInputStream
import java.io.DataOutputStream
import java.io.IOException

enum class TransportKind(val wireName: String, val label: String) {
    WIFI("wi_fi", "Wi-Fi"),
    BLUETOOTH("bluetooth", "Bluetooth");

    companion object {
        fun fromWire(raw: String): TransportKind = when (raw.lowercase()) {
            "bluetooth" -> BLUETOOTH
            else -> WIFI
        }
    }
}

/** Length-prefixed binary framing (`WW2F` + uint32 BE length + payload) matching Rust `core/transport`. */
object FrameCodec {
    private val MAGIC = byteArrayOf('W'.code.toByte(), 'W'.code.toByte(), '2'.code.toByte(), 'F'.code.toByte())
    private const val MAX_FRAME_BYTES = 64 * 1024

    @Throws(IOException::class)
    fun writeFrame(output: DataOutputStream, payload: ByteArray) {
        if (payload.size > MAX_FRAME_BYTES) {
            throw IOException("Frame exceeds maximum allowed size.")
        }
        output.write(MAGIC)
        output.writeInt(payload.size)
        output.write(payload)
        output.flush()
    }

    @Throws(IOException::class)
    fun readFrame(input: DataInputStream): ByteArray {
        val header = ByteArray(4)
        input.readFully(header)
        if (!header.contentEquals(MAGIC)) {
            throw IOException("Invalid WireWave frame magic.")
        }
        val length = input.readInt()
        if (length < 0 || length > MAX_FRAME_BYTES) {
            throw IOException("Invalid WireWave frame length: $length")
        }
        val body = ByteArray(length)
        input.readFully(body)
        return body
    }
}
