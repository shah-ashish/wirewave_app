package com.wirewave.core.stream

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class UdpCipherTest {
    @Test
    fun `udp cipher tolerates out-of-order datagrams and blocks replays`() {
        val key = ByteArray(32) { (it + 3).toByte() }
        val tx = UdpCipher(key, streamId = 77)
        val rx = UdpCipher(key, streamId = 77)

        val d1 = tx.encryptDatagram(ChannelId.SPEAKER, "frame1".toByteArray())
        val d2 = tx.encryptDatagram(ChannelId.SPEAKER, "frame2".toByteArray())

        val r2 = rx.decryptDatagram(d2)
        val r1 = rx.decryptDatagram(d1)
        assertNotNull(r1)
        assertNotNull(r2)
        assertEquals(1L, r1?.sequence)
        assertEquals(2L, r2?.sequence)
        assertArrayEquals("frame1".toByteArray(), r1?.payload)
        assertNull(rx.decryptDatagram(d1))

        tx.close()
        rx.close()
    }
}
