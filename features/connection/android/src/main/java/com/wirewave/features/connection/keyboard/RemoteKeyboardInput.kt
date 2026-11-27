package com.wirewave.features.connection.keyboard

import androidx.compose.foundation.layout.size
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp

/** Captures native keyboard input while the screen owns keyboard visibility and focus. */
@Composable
internal fun RemoteKeyboardInput(
    focusRequester: FocusRequester,
    onSendKeyInput: (text: String?, keyCode: String?) -> Unit,
    modifier: Modifier = Modifier
) {
    val controller = remember { KeyboardInputController() }
    val emptyField = remember {
        TextFieldValue(controller.placeholder, TextRange(controller.placeholder.length))
    }
    var textValue by remember { mutableStateOf(emptyField) }

    BasicTextField(
        value = textValue,
        onValueChange = { updatedValue ->
            if (controller.handleEdit(updatedValue.text, onSendKeyInput)) {
                textValue = emptyField
            }
        },
        keyboardOptions = KeyboardOptions(
            imeAction = ImeAction.Default,
            autoCorrect = true
        ),
        keyboardActions = KeyboardActions(
            onDone = { onSendKeyInput(null, "Enter") },
            onSend = { onSendKeyInput(null, "Enter") },
            onGo = { onSendKeyInput(null, "Enter") }
        ),
        modifier = modifier
            .size(1.dp)
            .alpha(0f)
            .focusRequester(focusRequester)
    )
}
