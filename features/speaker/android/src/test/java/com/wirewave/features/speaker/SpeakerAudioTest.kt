package com.wirewave.features.speaker

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder

class SpeakerAudioTest {
    @Test
    fun `laud parser decodes big-endian header and pcm payload`() {
        val raw = ByteBuffer.allocate(LaudPacket.HEADER_BYTES + 8)
            .order(ByteOrder.BIG_ENDIAN)
            .put("LAUD".toByteArray())
            .put(1.toByte())
            .put(0.toByte())
            .put(2.toByte())
            .put(1.toByte())
            .putInt(42)
            .putLong(1024L)
            .put(ByteArray(8) { 7 })
            .array()

        val parsed = LaudPacket.decode(raw)
        assertNotNull(parsed)
        assertEquals(42L, parsed?.sequence)
        assertEquals(1024L, parsed?.timestampFrames)
        assertEquals(8, parsed?.pcmBytes?.size)
    }

    @Test
    fun `jitter buffer reorders out-of-order udp packets`() {
        val jb = JitterBuffer(LatencyMode.LOW)
        fun pkt(seq: Long, marker: Byte) = LaudPacket(
            flags = 0,
            channels = 2,
            sampleFormat = 1,
            sequence = seq,
            timestampFrames = seq * 256,
            pcmBytes = ByteArray(LaudPacket.PCM_BYTES_PER_PACKET) { marker }
        )

        // Push enough packets to clear preroll with seq 2 arriving before seq 1.
        jb.push(pkt(0, 10))
        jb.push(pkt(2, 30))
        jb.push(pkt(1, 20))
        for (i in 3L..10L) jb.push(pkt(i, (i + 10).toByte()))

        assertEquals(10.toByte(), jb.popNext()[0])
        assertEquals(20.toByte(), jb.popNext()[0])
        assertEquals(30.toByte(), jb.popNext()[0])
    }
}
