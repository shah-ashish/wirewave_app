package com.wirewave.features.connection.keyboard

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class KeyboardInputControllerTest {
    private val controller = KeyboardInputController()
    private val events = mutableListOf<Pair<String?, String?>>()

    private fun edit(text: String): Boolean =
        controller.handleEdit(text) { content, key -> events.add(content to key) }

    @Test
    fun unchangedPlaceholderDoesNotSendInput() {
        assertFalse(edit(controller.placeholder))
        assertTrue(events.isEmpty())
    }

    @Test
    fun appendedTextIsSentWithoutPlaceholder() {
        assertTrue(edit(controller.placeholder + "Hello"))
        assertEquals(listOf("Hello" to null), events)
    }

    @Test
    fun multilineTextPreservesEmptyLinesAndEventOrder() {
        assertTrue(edit(controller.placeholder + "\nHello\n\n"))
        assertEquals(
            listOf(null to "Enter", "Hello" to null, null to "Enter", null to "Enter"),
            events
        )
    }

    @Test
    fun deletingOnePlaceholderCharacterSendsOneBackspace() {
        assertTrue(edit(" "))
        assertEquals(listOf(null to "Backspace"), events)
    }

    @Test
    fun deletingBothPlaceholderCharactersSendsTwoBackspaces() {
        assertTrue(edit(""))
        assertEquals(listOf(null to "Backspace", null to "Backspace"), events)
    }

    @Test
    fun sameLengthReplacementIsSentAsText() {
        assertTrue(edit("ab"))
        assertEquals(listOf("ab" to null), events)
    }
}
