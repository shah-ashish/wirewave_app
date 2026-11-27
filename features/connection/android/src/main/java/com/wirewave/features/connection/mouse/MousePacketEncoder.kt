package com.wirewave.features.connection.mouse

import java.nio.ByteBuffer

/** Encodes the desktop mouse protocol: event byte, two big-endian shorts, button byte. */
internal class MousePacketEncoder {
    companion object {
        const val EVENT_MOVE_REL: Byte = 0x01
        const val EVENT_BUTTON_DOWN: Byte = 0x02
        const val EVENT_BUTTON_UP: Byte = 0x03
        const val EVENT_SCROLL: Byte = 0x04
        const val EVENT_CLICK: Byte = 0x05
    }

    fun move(rawDx: Float, rawDy: Float, pointerSpeed: Float): ByteArray? {
        val dx = clampDelta(rawDx * pointerSpeed)
        val dy = clampDelta(rawDy * pointerSpeed)
        if (dx == 0.toShort() && dy == 0.toShort()) return null
        return packet(EVENT_MOVE_REL, dx, dy)
    }

    fun scroll(rawDeltaY: Float, scrollSpeed: Float, naturalScroll: Boolean): ByteArray? {
        // Traditional scrolling reverses the finger's direction; natural scrolling follows it.
        val direction = if (naturalScroll) 1f else -1f
        val delta = clampDelta(rawDeltaY * scrollSpeed * direction)
        if (delta == 0.toShort()) return null
        return packet(EVENT_SCROLL, delta)
    }

    fun click(button: Byte): ByteArray = packet(EVENT_CLICK, button = button)

    fun buttonDown(button: Byte): ByteArray = packet(EVENT_BUTTON_DOWN, button = button)

    fun buttonUp(button: Byte): ByteArray = packet(EVENT_BUTTON_UP, button = button)

    private fun clampDelta(value: Float): Short =
        value.toInt().coerceIn(Short.MIN_VALUE.toInt(), Short.MAX_VALUE.toInt()).toShort()

    private fun packet(
        event: Byte,
        firstDelta: Short = 0,
        secondDelta: Short = 0,
        button: Byte = 0
    ): ByteArray = ByteBuffer.allocate(6).apply {
        put(event)
        putShort(firstDelta)
        putShort(secondDelta)
        put(button)
    }.array()
}
