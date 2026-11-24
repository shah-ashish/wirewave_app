package com.wirewave.features.connection.overlay

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.expandVertically
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.shrinkVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.Backspace
import androidx.compose.material.icons.automirrored.rounded.KeyboardArrowLeft
import androidx.compose.material.icons.automirrored.rounded.KeyboardArrowRight
import androidx.compose.material.icons.automirrored.rounded.KeyboardReturn
import androidx.compose.material.icons.rounded.ArrowUpward
import androidx.compose.material.icons.rounded.KeyboardArrowDown
import androidx.compose.material.icons.rounded.KeyboardArrowUp
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.WireWaveTheme

enum class ScreenOverlay {
    NONE,
    KEYBOARD,
    CONTROLS
}

/** Host container that displays the active overlay (Keyboard or Controls) with smooth animation. */
@Composable
fun ScreenOverlayHost(
    overlay: ScreenOverlay,
    onControlKeyClick: (String) -> Unit = {},
    modifier: Modifier = Modifier
) {
    AnimatedVisibility(
        visible = overlay == ScreenOverlay.KEYBOARD,
        enter = fadeIn() + expandVertically(),
        exit = fadeOut() + shrinkVertically(),
        modifier = modifier
    ) {
        VirtualKeyboardOverlay()
    }

    AnimatedVisibility(
        visible = overlay == ScreenOverlay.CONTROLS,
        enter = fadeIn() + expandVertically(),
        exit = fadeOut() + shrinkVertically(),
        modifier = modifier
    ) {
        VirtualControlsOverlay(onControlKeyClick = onControlKeyClick)
    }
}

/** Virtual QWERTY Keyboard Overlay. */
@Composable
fun VirtualKeyboardOverlay(
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .fillMaxWidth()
            .background(WireWaveTheme.Colors.bgCanvas)
            .padding(horizontal = 6.dp, vertical = 6.dp),
        verticalArrangement = Arrangement.spacedBy(6.dp)
    ) {
        // Row 1: Q - P
        val row1 = listOf("Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P")
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(4.dp)
        ) {
            row1.forEach { key ->
                KeyButton(label = key, modifier = Modifier.weight(1f))
            }
        }

        // Row 2: A - L
        val row2 = listOf("A", "S", "D", "F", "G", "H", "J", "K", "L")
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 14.dp),
            horizontalArrangement = Arrangement.spacedBy(4.dp)
        ) {
            row2.forEach { key ->
                KeyButton(label = key, modifier = Modifier.weight(1f))
            }
        }

        // Row 3: Shift, Z - M, Backspace
        val row3 = listOf("Z", "X", "C", "V", "B", "N", "M")
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(4.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            KeyIconButton(
                icon = Icons.Rounded.ArrowUpward,
                modifier = Modifier.weight(1.3f)
            )
            row3.forEach { key ->
                KeyButton(label = key, modifier = Modifier.weight(1f))
            }
            KeyIconButton(
                icon = Icons.AutoMirrored.Rounded.Backspace,
                modifier = Modifier.weight(1.3f)
            )
        }

        // Row 4: 123, Spacebar, Enter
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(4.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            KeyButton(label = "123", modifier = Modifier.weight(1.3f))
            KeyButton(label = "", modifier = Modifier.weight(4.5f))
            KeyIconButton(
                icon = Icons.AutoMirrored.Rounded.KeyboardReturn,
                modifier = Modifier.weight(1.3f)
            )
        }
    }
}

/** Virtual Controls Overlay. */
@Composable
fun VirtualControlsOverlay(
    onControlKeyClick: (String) -> Unit = {},
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .fillMaxWidth()
            .background(WireWaveTheme.Colors.bgCanvas)
            .padding(horizontal = 12.dp, vertical = 6.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        // Row 1: A, C, V, X (Shortcuts Ctrl+A, Ctrl+C, Ctrl+V, Ctrl+X)
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            listOf(
                "A" to "Ctrl+A",
                "C" to "Ctrl+C",
                "V" to "Ctrl+V",
                "X" to "Ctrl+X"
            ).forEach { (label, code) ->
                ControlKeyButton(
                    label = label,
                    modifier = Modifier.weight(1f),
                    onClick = { onControlKeyClick(code) }
                )
            }
        }

        // Row 2: Ctrl, ^ (Up), Shift
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            ControlKeyButton(
                label = "Ctrl",
                modifier = Modifier.weight(1.4f),
                onClick = { onControlKeyClick("Ctrl") }
            )
            ControlKeyIconButton(
                icon = Icons.Rounded.KeyboardArrowUp,
                modifier = Modifier.weight(1f),
                onClick = { onControlKeyClick("Up") }
            )
            ControlKeyButton(
                label = "Shift",
                modifier = Modifier.weight(1.4f),
                onClick = { onControlKeyClick("Shift") }
            )
        }

        // Row 3: < (Left), v (Down), > (Right)
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 24.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            ControlKeyIconButton(
                icon = Icons.AutoMirrored.Rounded.KeyboardArrowLeft,
                modifier = Modifier.weight(1f),
                onClick = { onControlKeyClick("Left") }
            )
            ControlKeyIconButton(
                icon = Icons.Rounded.KeyboardArrowDown,
                modifier = Modifier.weight(1f),
                onClick = { onControlKeyClick("Down") }
            )
            ControlKeyIconButton(
                icon = Icons.AutoMirrored.Rounded.KeyboardArrowRight,
                modifier = Modifier.weight(1f),
                onClick = { onControlKeyClick("Right") }
            )
        }
    }
}

@Composable
private fun KeyButton(
    label: String,
    modifier: Modifier = Modifier
) {
    val keyShape = RoundedCornerShape(6.dp)
    val interaction = remember { MutableInteractionSource() }
    val isPressed by interaction.collectIsPressedAsState()

    Box(
        modifier = modifier
            .height(42.dp)
            .clip(keyShape)
            .background(if (isPressed) Color(0xFF404044) else Color(0xFF26262A))
            .clickable(interactionSource = interaction, indication = null, onClick = {}),
        contentAlignment = Alignment.Center
    ) {
        Text(
            text = label,
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = 15.sp,
            fontWeight = FontWeight.Medium,
            textAlign = TextAlign.Center
        )
    }
}

@Composable
private fun KeyIconButton(
    icon: ImageVector,
    modifier: Modifier = Modifier
) {
    val keyShape = RoundedCornerShape(6.dp)
    val interaction = remember { MutableInteractionSource() }
    val isPressed by interaction.collectIsPressedAsState()

    Box(
        modifier = modifier
            .height(42.dp)
            .clip(keyShape)
            .background(if (isPressed) Color(0xFF404044) else Color(0xFF1E1E22))
            .clickable(interactionSource = interaction, indication = null, onClick = {}),
        contentAlignment = Alignment.Center
    ) {
        Icon(
            imageVector = icon,
            contentDescription = null,
            tint = WireWaveTheme.Colors.textPrimary,
            modifier = Modifier.size(18.dp)
        )
    }
}

@Composable
private fun ControlKeyButton(
    label: String,
    modifier: Modifier = Modifier,
    onClick: () -> Unit = {}
) {
    val shape = RoundedCornerShape(10.dp)
    val interaction = remember { MutableInteractionSource() }
    val isPressed by interaction.collectIsPressedAsState()

    Box(
        modifier = modifier
            .height(44.dp)
            .clip(shape)
            .background(if (isPressed) Color(0xFF424248) else Color(0xFF26262A))
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, shape)
            .clickable(interactionSource = interaction, indication = null, onClick = onClick),
        contentAlignment = Alignment.Center
    ) {
        Text(
            text = label,
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = 14.sp,
            fontWeight = FontWeight.SemiBold
        )
    }
}

@Composable
private fun ControlKeyIconButton(
    icon: ImageVector,
    modifier: Modifier = Modifier,
    onClick: () -> Unit = {}
) {
    val shape = RoundedCornerShape(10.dp)
    val interaction = remember { MutableInteractionSource() }
    val isPressed by interaction.collectIsPressedAsState()

    Box(
        modifier = modifier
            .height(44.dp)
            .clip(shape)
            .background(if (isPressed) Color(0xFF424248) else Color(0xFF26262A))
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, shape)
            .clickable(interactionSource = interaction, indication = null, onClick = onClick),
        contentAlignment = Alignment.Center
    ) {
        Icon(
            imageVector = icon,
            contentDescription = null,
            tint = WireWaveTheme.Colors.textPrimary,
            modifier = Modifier.size(20.dp)
        )
    }
}
