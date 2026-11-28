package com.wirewave.core.stream

import java.io.IOException
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.net.SocketTimeoutException
import java.util.concurrent.ArrayBlockingQueue
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.concurrent.thread

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
    private val closed = AtomicBoolean(false)
    private val incoming = ArrayBlockingQueue<DecryptedDatagram>(64)
    private val receiver = thread(name = "wirewave-udp-receiver", isDaemon = true) {
        while (!closed.get()) {
            val packet = DatagramPacket(recvBuf, recvBuf.size)
            try {
                socket.receive(packet)
                if (packet.address != targetAddr || packet.port != hostUdpPort) continue
                val decoded = rxCipher.decryptDatagram(packet.data, packet.length) ?: continue
                val reply = probeReply(decoded.payload)
                if (reply != null) {
                    sendDatagram(decoded.channel, reply)
                } else {
                    // Keep real-time playback bounded if its consumer falls behind.
                    if (!incoming.offer(decoded)) {
                        incoming.poll()
                        incoming.offer(decoded)
                    }
                }
            } catch (_: SocketTimeoutException) {
                // Allows shutdown even while the stream is idle.
            } catch (_: IOException) {
                if (!closed.get()) Thread.sleep(10)
            }
        }
    }

    @Throws(IOException::class)
    fun sendHello(channel: ChannelId) {
        sendDatagram(channel, UdpCipher.UDP_HELLO_PAYLOAD)
    }

    @Throws(IOException::class)
    fun sendDatagram(channel: ChannelId, payload: ByteArray) {
        synchronized(txCipher) {
            if (closed.get()) throw IOException("UDP stream closed")
            val encrypted = txCipher.encryptDatagram(channel, payload)
            val packet = DatagramPacket(encrypted, encrypted.size, targetAddr, hostUdpPort)
            socket.send(packet)
        }
    }

    /** Blocks up to 1s for the next authenticated datagram; returns null on timeout or invalid packet. */
    fun receiveNext(): DecryptedDatagram? {
        return try {
            if (closed.get()) null else incoming.poll(1, TimeUnit.SECONDS)
        } catch (_: InterruptedException) {
            Thread.currentThread().interrupt()
            null
        }
    }

    override fun close() {
        if (!closed.compareAndSet(false, true)) return
        socket.close()
        receiver.join()
        synchronized(txCipher) { txCipher.close() }
        rxCipher.close()
        incoming.clear()
    }

    companion object {
        private val PING = "WW2U_PING".toByteArray(Charsets.US_ASCII)
        private val PONG = "WW2U_PONG".toByteArray(Charsets.US_ASCII)

        internal fun probeReply(payload: ByteArray): ByteArray? {
            if (payload.size != PING.size + 8) return null
            if (!payload.copyOfRange(0, PING.size).contentEquals(PING)) return null
            return PONG + payload.copyOfRange(PING.size, payload.size)
        }
    }
}
