package com.wirewave.features.connection.mouse

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertNull
import org.junit.Test

class MousePacketEncoderTest {
    private val encoder = MousePacketEncoder()

    @Test
    fun movementUsesBigEndianSignedDeltasAndPointerSpeed() {
        assertArrayEquals(
            byteArrayOf(1, 0, 3, -1, -4, 0),
            encoder.move(1.5f, -2f, 2f)
        )
    }

    @Test
    fun movementClampsBothSignedBoundaries() {
        assertArrayEquals(
            byteArrayOf(1, 127, -1, -128, 0, 0),
            encoder.move(40000f, -40000f, 1f)
        )
    }

    @Test
    fun movementTruncatesFractionalDeltasTowardZero() {
        assertArrayEquals(byteArrayOf(1, 0, 1, -1, -1, 0), encoder.move(1.9f, -1.9f, 1f))
    }

    @Test
    fun zeroMovementAndScrollDoNotProducePackets() {
        assertNull(encoder.move(0.9f, -0.9f, 1f))
        assertNull(encoder.scroll(0.9f, 1f, false))
        assertNull(encoder.move(10f, 20f, 0f))
    }

    @Test
    fun scrollPreservesDirectionAndSpeed() {
        assertArrayEquals(byteArrayOf(4, 0, 6, 0, 0, 0), encoder.scroll(3f, 2f, true))
        assertArrayEquals(byteArrayOf(4, -1, -6, 0, 0, 0), encoder.scroll(3f, 2f, false))
        assertArrayEquals(byteArrayOf(4, 0, 6, 0, 0, 0), encoder.scroll(-3f, 2f, false))
    }

    @Test
    fun scrollClampsBothSignedBoundaries() {
        assertArrayEquals(byteArrayOf(4, 127, -1, 0, 0, 0), encoder.scroll(40000f, 1f, true))
        assertArrayEquals(byteArrayOf(4, -128, 0, 0, 0, 0), encoder.scroll(40000f, 1f, false))
    }

    @Test
    fun buttonPacketsPreserveEventAndButtonIdentifiers() {
        for (button in listOf<Byte>(1, 2, 3)) {
            assertArrayEquals(byteArrayOf(5, 0, 0, 0, 0, button), encoder.click(button))
            assertArrayEquals(byteArrayOf(2, 0, 0, 0, 0, button), encoder.buttonDown(button))
            assertArrayEquals(byteArrayOf(3, 0, 0, 0, 0, button), encoder.buttonUp(button))
        }
    }
}
