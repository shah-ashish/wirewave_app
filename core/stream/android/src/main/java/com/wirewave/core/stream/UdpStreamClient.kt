package com.wirewave.core.stream

import java.io.IOException
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.net.SocketTimeoutException

/**
 * Encrypted UDP client socket that punches through desktop firewall via signed `UDP_HELLO`
 * and receives authenticated real-time datagrams with zero head-of-line blocking.
 */
class UdpStreamClient(
    private val hostAddress: String,
    private val hostUdpPort: Int,
    udpKey: ByteArray,
    streamId: Int
) : AutoCloseable {
    private val socket = DatagramSocket().apply {
        soTimeout = 1000
        receiveBufferSize = 256 * 1024
    }
    private val txCipher = UdpCipher(udpKey, streamId)
    private val rxCipher = UdpCipher(udpKey, streamId)
    private val targetAddr: InetAddress = InetAddress.getByName(hostAddress)
    private val recvBuf = ByteArray(UdpCipher.MAX_DATAGRAM_BYTES)

    @Throws(IOException::class)
    fun sendHello(channel: ChannelId) {
        val encrypted = txCipher.encryptDatagram(channel, UdpCipher.UDP_HELLO_PAYLOAD)
        val pkt = DatagramPacket(encrypted, encrypted.size, targetAddr, hostUdpPort)
        socket.send(pkt)
    }

    /** Blocks up to 1s for the next authenticated datagram; returns null on timeout or invalid packet. */
    fun receiveNext(): DecryptedDatagram? {
        val pkt = DatagramPacket(recvBuf, recvBuf.size)
        return try {
            socket.receive(pkt)
            rxCipher.decryptDatagram(pkt.data, pkt.length)
        } catch (_: SocketTimeoutException) {
            null
        } catch (_: IOException) {
            null
        }
    }

    override fun close() {
        socket.close()
        txCipher.close()
        rxCipher.close()
    }
}
