package com.wirewave.features.connection.keyboard

/** Translates edits to the hidden keyboard field into remote text and key events. */
internal class KeyboardInputController {
    // Leading spaces let the IME report backspace even when no visible text is stored.
    val placeholder = "  "

    /** Returns true when an edit was handled and the field should be reset. */
    fun handleEdit(text: String, sendInput: (String?, String?) -> Unit): Boolean {
        when {
            text.length > placeholder.length -> sendText(text.substring(placeholder.length), sendInput)
            text.length < placeholder.length -> repeat(placeholder.length - text.length) {
                sendInput(null, "Backspace")
            }
            text != placeholder -> sendInput(text, null)
            else -> return false
        }
        return true
    }

    private fun sendText(text: String, sendInput: (String?, String?) -> Unit) {
        val lines = text.split("\n")
        lines.forEachIndexed { index, line ->
            if (line.isNotEmpty()) sendInput(line, null)
            if (index < lines.lastIndex) sendInput(null, "Enter")
        }
    }
}
