package com.wirewave.features.connection.mouse

import com.wirewave.core.stream.ChannelId
import com.wirewave.core.stream.UdpStreamClient
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import java.io.IOException

class MouseEngine(
    private val scope: CoroutineScope,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) : AutoCloseable {

    companion object {
        const val EVENT_MOVE_REL: Byte = MousePacketEncoder.EVENT_MOVE_REL
        const val EVENT_BUTTON_DOWN: Byte = MousePacketEncoder.EVENT_BUTTON_DOWN
        const val EVENT_BUTTON_UP: Byte = MousePacketEncoder.EVENT_BUTTON_UP
        const val EVENT_SCROLL: Byte = MousePacketEncoder.EVENT_SCROLL
        const val EVENT_CLICK: Byte = MousePacketEncoder.EVENT_CLICK

        const val BUTTON_LEFT: Byte = 1
        const val BUTTON_RIGHT: Byte = 2
        const val BUTTON_MIDDLE: Byte = 3
    }

    private val packetEncoder = MousePacketEncoder()

    @Volatile
    private var client: UdpStreamClient? = null

    fun start(host: String, port: Int, key: ByteArray, streamId: Int) {
        stop()
        scope.launch(ioDispatcher) {
            try {
                val udp = UdpStreamClient(host, port, key, streamId)
                repeat(3) { udp.sendHello(ChannelId.MOUSE) }
                client = udp
            } catch (_: IOException) {
                // Socket creation or network error
            }
        }
    }

    fun stop() {
        val current = client
        client = null
        current?.close()
    }

    fun sendMove(rawDx: Float, rawDy: Float, pointerSpeed: Float) {
        val udp = client ?: return
        val packet = packetEncoder.move(rawDx, rawDy, pointerSpeed) ?: return
        sendPacket(udp, packet)
    }

    fun sendScroll(rawDeltaY: Float, scrollSpeed: Float, naturalScroll: Boolean) {
        val udp = client ?: return
        val packet = packetEncoder.scroll(rawDeltaY, scrollSpeed, naturalScroll) ?: return
        sendPacket(udp, packet)
    }

    fun sendClick(button: Byte) {
        val udp = client ?: return
        sendPacket(udp, packetEncoder.click(button))
    }

    fun sendButtonDown(button: Byte) {
        val udp = client ?: return
        sendPacket(udp, packetEncoder.buttonDown(button))
    }

    fun sendButtonUp(button: Byte) {
        val udp = client ?: return
        sendPacket(udp, packetEncoder.buttonUp(button))
    }

    private fun sendPacket(udp: UdpStreamClient, packet: ByteArray) {
        scope.launch(ioDispatcher) {
            try {
                udp.sendDatagram(ChannelId.MOUSE, packet)
            } catch (_: IOException) {
                // Preserve best-effort delivery when the connection closes or a datagram fails.
            }
        }
    }

    override fun close() {
        stop()
    }
}
