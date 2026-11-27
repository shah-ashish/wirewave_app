package com.wirewave.features.connection.mouse

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.KeyboardArrowDown
import androidx.compose.material.icons.rounded.KeyboardArrowUp
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.PointerEventPass
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.session.MousePrefs

/** Scroll strip that emits raw vertical movement; the caller applies scroll preferences. */
@Composable
internal fun MouseScrollStrip(
    mousePrefs: MousePrefs,
    onScroll: (Float) -> Unit,
    modifier: Modifier = Modifier
) {
    Box(
        modifier = modifier
            .width(60.dp)
            .fillMaxHeight()
            .background(WireWaveTheme.Colors.bgCardHover.copy(alpha = 0.25f))
            .pointerInput(mousePrefs) {
                awaitPointerEventScope {
                    while (true) {
                        // Immediate down detection - zero delay, no hold required!
                        val downEvent = awaitPointerEvent(PointerEventPass.Main)
                        val downPointer = downEvent.changes.firstOrNull { it.pressed } ?: continue
                        var lastY = downPointer.position.y
                        downPointer.consume()

                        while (true) {
                            val event = awaitPointerEvent(PointerEventPass.Main)
                            val pointer = event.changes.firstOrNull { it.id == downPointer.id } ?: break
                            if (!pointer.pressed) break

                            val currentY = pointer.position.y
                            val dy = currentY - lastY
                            lastY = currentY
                            pointer.consume()

                            if (kotlin.math.abs(dy) > 0.5f) {
                                onScroll(dy)
                            }
                        }
                    }
                }
            },
        contentAlignment = Alignment.Center
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(vertical = 16.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.SpaceBetween
        ) {
            Icon(
                imageVector = Icons.Rounded.KeyboardArrowUp,
                contentDescription = "Scroll up",
                tint = WireWaveTheme.Colors.textSecondary.copy(alpha = 0.7f),
                modifier = Modifier.size(18.dp)
            )

            // Scroll Thumb Pill
            Box(
                modifier = Modifier
                    .width(6.dp)
                    .height(56.dp)
                    .clip(CircleShape)
                    .background(Color(0xFF5A5A60))
            )

            Icon(
                imageVector = Icons.Rounded.KeyboardArrowDown,
                contentDescription = "Scroll down",
                tint = WireWaveTheme.Colors.textSecondary.copy(alpha = 0.7f),
                modifier = Modifier.size(18.dp)
            )
        }
    }
}
