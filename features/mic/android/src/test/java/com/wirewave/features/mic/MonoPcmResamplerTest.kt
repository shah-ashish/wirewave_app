package com.wirewave.features.mic

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder

class MonoPcmResamplerTest {
    @Test
    fun `48 khz input remains unchanged`() {
        val input = shortArrayOf(-1000, 0, 1000).toBytes()
        val output = MonoPcmResampler(48_000, 48_000).process(input, input.size)
        assertTrue(input.contentEquals(output))
    }

    @Test
    fun `44 point 1 khz input is converted to 48 khz`() {
        val input = ShortArray(441) { it.toShort() }.toBytes()
        val output = MonoPcmResampler(44_100, 48_000).process(input, input.size)
        val outputFrames = output.size / 2
        assertTrue(outputFrames in 479..481)
    }

    @Test
    fun `streaming chunks preserve resampler phase`() {
        val resampler = MonoPcmResampler(32_000, 48_000)
        val first = ShortArray(160) { 100 }.toBytes()
        val second = ShortArray(160) { 200 }.toBytes()
        val outputFrames = resampler.process(first, first.size).size / 2 +
            resampler.process(second, second.size).size / 2
        assertEquals(479, outputFrames)
    }

    private fun ShortArray.toBytes(): ByteArray =
        ByteBuffer.allocate(size * 2).order(ByteOrder.LITTLE_ENDIAN).apply {
            forEach { putShort(it) }
        }.array()
}
