package com.wirewave.core.stream

import org.junit.Assert.*
import org.junit.Test
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.nio.ByteBuffer

class UdpStreamClientTest {
    @Test
    fun `probe replies preserve token and do not match media`() {
        val token = ByteBuffer.allocate(8).putLong(42).array()
        assertArrayEquals("WW2U_PONG".toByteArray() + token,
            UdpStreamClient.probeReply("WW2U_PING".toByteArray() + token))
        assertNull(UdpStreamClient.probeReply("WW2U_PING".toByteArray()))
        assertNull(UdpStreamClient.probeReply(ByteArray(100)))
    }

    @Test
    fun `send only client answers encrypted probe without consuming media`() {
        DatagramSocket(0).use { server ->
            server.soTimeout = 3000
            val key = ByteArray(32) { 9 }
            UdpStreamClient("127.0.0.1", server.localPort, key, 17).use { client ->
                client.sendHello(ChannelId.MOUSE)
                val buffer = ByteArray(1400)
                val packet = DatagramPacket(buffer, buffer.size)
                server.receive(packet)
                val rx = UdpCipher(key, 17)
                val tx = UdpCipher(key, 17)
                assertNotNull(rx.decryptDatagram(packet.data, packet.length))
                val token = ByteBuffer.allocate(8).putLong(99).array()
                val probe = tx.encryptDatagram(ChannelId.MOUSE, "WW2U_PING".toByteArray() + token)
                server.send(DatagramPacket(probe, probe.size, packet.address, packet.port))
                packet.length = buffer.size
                server.receive(packet)
                assertArrayEquals("WW2U_PONG".toByteArray() + token,
                    rx.decryptDatagram(packet.data, packet.length)!!.payload)
                val media = tx.encryptDatagram(ChannelId.MOUSE, byteArrayOf(1, 2, 3))
                server.send(DatagramPacket(media, media.size, packet.address, packet.port))
                assertArrayEquals(byteArrayOf(1, 2, 3), client.receiveNext()!!.payload)
                tx.close()
                rx.close()
            }
        }
    }
}
