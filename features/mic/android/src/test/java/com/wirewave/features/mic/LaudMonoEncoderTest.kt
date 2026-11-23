package com.wirewave.features.mic

import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder

class LaudMonoEncoderTest {
    @Test
    fun `encodes the mono packet contract`() {
        val encoded = LaudMonoEncoder.encode(byteArrayOf(1, 2), 2, 9, 256)
        val buffer = ByteBuffer.wrap(encoded).order(ByteOrder.BIG_ENDIAN)
        assertEquals(22, encoded.size)
        assertEquals(1, buffer.get(6).toInt())
        assertEquals(9, buffer.getInt(8))
        assertEquals(256, buffer.getLong(12))
    }

    @Test
    fun `rejects an odd pcm16 payload`() {
        assertThrows(IllegalArgumentException::class.java) {
            LaudMonoEncoder.encode(byteArrayOf(1), 1, 0, 0)
        }
    }
}
