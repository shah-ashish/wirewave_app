package com.wirewave.features.connection.mouse

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme

/** Left or Right mouse click button with custom highlighted mouse icon. */
@Composable
internal fun MouseClickButton(
    leftSelected: Boolean,
    onButtonDown: () -> Unit,
    onButtonUp: () -> Unit,
    modifier: Modifier = Modifier
) {
    val shape = RoundedCornerShape(14.dp)
    var isPressed by remember { mutableStateOf(false) }

    Box(
        modifier = modifier
            .height(52.dp)
            .clip(shape)
            .background(if (isPressed) WireWaveTheme.Colors.bgCardHover else WireWaveTheme.Colors.bgCard)
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, shape)
            .pointerInput(Unit) {
                detectTapGestures(
                    onPress = {
                        isPressed = true
                        onButtonDown()
                        tryAwaitRelease()
                        onButtonUp()
                        isPressed = false
                    }
                )
            },
        contentAlignment = Alignment.Center
    ) {
        MouseHardwareIcon(
            leftActive = leftSelected,
            rightActive = !leftSelected
        )
    }
}

/** Draws an exact 2-button mouse icon with the specified button zone filled in accentBlue. */
@Composable
private fun MouseHardwareIcon(
    leftActive: Boolean,
    rightActive: Boolean,
    modifier: Modifier = Modifier
) {
    val accent = WireWaveTheme.Colors.accentBlue
    val strokeColor = WireWaveTheme.Colors.textSecondary

    Canvas(modifier = modifier.size(width = 18.dp, height = 24.dp)) {
        val w = size.width
        val h = size.height
        val corner = 7.dp.toPx()
        val splitY = h * 0.44f

        // Mouse outer body
        drawRoundRect(
            color = strokeColor,
            topLeft = Offset(0f, 0f),
            size = Size(w, h),
            cornerRadius = CornerRadius(corner, corner),
            style = Stroke(width = 1.5.dp.toPx())
        )

        // Horizontal line separating buttons from palm
        drawLine(
            color = strokeColor,
            start = Offset(0f, splitY),
            end = Offset(w, splitY),
            strokeWidth = 1.5.dp.toPx()
        )

        // Vertical divider separating left and right buttons
        drawLine(
            color = strokeColor,
            start = Offset(w / 2f, 0f),
            end = Offset(w / 2f, splitY),
            strokeWidth = 1.5.dp.toPx()
        )

        // Highlight active click button
        when {
            leftActive -> {
                drawRoundRect(
                    color = accent,
                    topLeft = Offset(1.5.dp.toPx(), 1.5.dp.toPx()),
                    size = Size((w / 2f) - 2.dp.toPx(), splitY - 2.dp.toPx()),
                    cornerRadius = CornerRadius(corner * 0.8f, 2.dp.toPx())
                )
            }
            rightActive -> {
                drawRoundRect(
                    color = accent,
                    topLeft = Offset((w / 2f) + 1.dp.toPx(), 1.5.dp.toPx()),
                    size = Size((w / 2f) - 2.dp.toPx(), splitY - 2.dp.toPx()),
                    cornerRadius = CornerRadius(2.dp.toPx(), corner * 0.8f)
                )
            }
        }
    }
}
