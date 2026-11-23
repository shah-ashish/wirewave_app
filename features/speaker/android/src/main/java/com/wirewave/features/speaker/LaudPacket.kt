package com.wirewave.features.speaker

import java.nio.ByteBuffer
import java.nio.ByteOrder

data class LaudPacket(
    val flags: Int,
    val channels: Int,
    val sampleFormat: Int,
    val sequence: Long,
    val timestampFrames: Long,
    val pcmBytes: ByteArray
) {
    val isSilence: Boolean get() = (flags and FLAG_SILENCE) != 0

    companion object {
        private val MAGIC = byteArrayOf('L'.code.toByte(), 'A'.code.toByte(), 'U'.code.toByte(), 'D'.code.toByte())
        const val HEADER_BYTES = 20
        const val TARGET_SAMPLE_RATE = 48_000
        const val TARGET_CHANNELS = 2
        const val FRAMES_PER_PACKET = 256
        const val PCM_BYTES_PER_PACKET = FRAMES_PER_PACKET * TARGET_CHANNELS * 2
        const val FLAG_SILENCE = 0x01

        fun decode(raw: ByteArray): LaudPacket? {
            if (raw.size < HEADER_BYTES) return null
            if (!raw.copyOfRange(0, 4).contentEquals(MAGIC)) return null
            if (raw[4].toInt() != 1) return null

            val hdr = ByteBuffer.wrap(raw, 0, HEADER_BYTES).order(ByteOrder.BIG_ENDIAN)
            hdr.position(5)
            val flags = hdr.get().toInt() and 0xFF
            val channels = hdr.get().toInt() and 0xFF
            val format = hdr.get().toInt() and 0xFF
            val seq = hdr.int.toLong() and 0xFFFF_FFFFL
            val ts = hdr.long
            val pcm = raw.copyOfRange(HEADER_BYTES, raw.size)
            return LaudPacket(flags, channels, format, seq, ts, pcm)
        }
    }
}
