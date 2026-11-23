package com.wirewave.features.mic

import java.nio.ByteBuffer
import java.nio.ByteOrder

object LaudMonoEncoder {
    private val magic = byteArrayOf('L'.code.toByte(), 'A'.code.toByte(), 'U'.code.toByte(), 'D'.code.toByte())
    const val HEADER_BYTES = 20
    const val TARGET_SAMPLE_RATE = 48_000
    const val PCM_BYTES_PER_PACKET = 512

    fun encode(pcm: ByteArray, length: Int, sequence: Int, timestampFrames: Long): ByteArray {
        require(length in 0..pcm.size && length % 2 == 0) { "PCM16 length must be even and within the source buffer" }
        return ByteBuffer.allocate(HEADER_BYTES + length).order(ByteOrder.BIG_ENDIAN).apply {
            put(magic)
            put(1.toByte())
            put(0.toByte())
            put(1.toByte())
            put(1.toByte())
            putInt(sequence)
            putLong(timestampFrames)
            put(pcm, 0, length)
        }.array()
    }
}
