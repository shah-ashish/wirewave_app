package com.wirewave.features.connection.mouse

import com.wirewave.core.stream.ChannelId
import com.wirewave.core.stream.UdpStreamClient
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import java.io.IOException
import java.nio.ByteBuffer

class MouseEngine(
    private val scope: CoroutineScope,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) : AutoCloseable {

    companion object {
        const val EVENT_MOVE_REL: Byte = 0x01
        const val EVENT_BUTTON_DOWN: Byte = 0x02
        const val EVENT_BUTTON_UP: Byte = 0x03
        const val EVENT_SCROLL: Byte = 0x04
        const val EVENT_CLICK: Byte = 0x05

        const val BUTTON_LEFT: Byte = 1
        const val BUTTON_RIGHT: Byte = 2
        const val BUTTON_MIDDLE: Byte = 3
    }

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
        val dx = (rawDx * pointerSpeed).toInt().coerceIn(-32768, 32767).toShort()
        val dy = (rawDy * pointerSpeed).toInt().coerceIn(-32768, 32767).toShort()
        if (dx == 0.toShort() && dy == 0.toShort()) return

        val packet = ByteBuffer.allocate(6).apply {
            put(EVENT_MOVE_REL)
            putShort(dx)
            putShort(dy)
            put(0) // button unused for move
        }.array()

        scope.launch(ioDispatcher) {
            try {
                udp.sendDatagram(ChannelId.MOUSE, packet)
            } catch (_: IOException) {}
        }
    }

    fun sendScroll(rawDeltaY: Float, scrollSpeed: Float, naturalScroll: Boolean) {
        val udp = client ?: return
        // In Traditional mode (naturalScroll = false), dragging down (rawDeltaY > 0)
        // must produce a negative Win32 wheel delta to scroll down.
        // In Natural mode (naturalScroll = true), dragging down produces a positive
        // wheel delta to scroll up (direct manipulation).
        val direction = if (naturalScroll) 1f else -1f
        val delta = (rawDeltaY * scrollSpeed * direction).toInt().coerceIn(-32768, 32767).toShort()
        if (delta == 0.toShort()) return

        val packet = ByteBuffer.allocate(6).apply {
            put(EVENT_SCROLL)
            putShort(delta)
            putShort(0)
            put(0)
        }.array()

        scope.launch(ioDispatcher) {
            try {
                udp.sendDatagram(ChannelId.MOUSE, packet)
            } catch (_: IOException) {}
        }
    }

    fun sendClick(button: Byte) {
        val udp = client ?: return
        val packet = ByteBuffer.allocate(6).apply {
            put(EVENT_CLICK)
            putShort(0)
            putShort(0)
            put(button)
        }.array()

        scope.launch(ioDispatcher) {
            try {
                udp.sendDatagram(ChannelId.MOUSE, packet)
            } catch (_: IOException) {}
        }
    }

    fun sendButtonDown(button: Byte) {
        val udp = client ?: return
        val packet = ByteBuffer.allocate(6).apply {
            put(EVENT_BUTTON_DOWN)
            putShort(0)
            putShort(0)
            put(button)
        }.array()

        scope.launch(ioDispatcher) {
            try {
                udp.sendDatagram(ChannelId.MOUSE, packet)
            } catch (_: IOException) {}
        }
    }

    fun sendButtonUp(button: Byte) {
        val udp = client ?: return
        val packet = ByteBuffer.allocate(6).apply {
            put(EVENT_BUTTON_UP)
            putShort(0)
            putShort(0)
            put(button)
        }.array()

        scope.launch(ioDispatcher) {
            try {
                udp.sendDatagram(ChannelId.MOUSE, packet)
            } catch (_: IOException) {}
        }
    }

    override fun close() {
        stop()
    }
}
