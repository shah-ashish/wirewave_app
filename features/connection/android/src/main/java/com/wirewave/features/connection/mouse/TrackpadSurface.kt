package com.wirewave.features.connection.mouse

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.PointerEventPass
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.session.MousePrefs

/** Multi-touch Trackpad: 1-finger move/tap, 2-finger scroll/tap right-click, and scroll strip. */
@Composable
internal fun TrackpadSurface(
    mousePrefs: MousePrefs,
    onMove: (Float, Float) -> Unit,
    onScroll: (Float) -> Unit,
    onLeftClick: () -> Unit,
    onRightClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    val trackpadShape = RoundedCornerShape(16.dp)
    var touchPos by remember { mutableStateOf<Offset?>(null) }

    Box(
        modifier = modifier
            .fillMaxSize()
            .clip(trackpadShape)
            .background(WireWaveTheme.Colors.bgCard)
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, trackpadShape)
    ) {
        Row(modifier = Modifier.fillMaxSize()) {
            // --- Left Area: Trackpad Surface ---
            Box(
                modifier = Modifier
                    .weight(1f)
                    .fillMaxHeight()
                    .pointerInput(mousePrefs) {
                        awaitPointerEventScope {
                            while (true) {
                                val downEvent = awaitPointerEvent(PointerEventPass.Main)
                                val downCount = downEvent.changes.count { it.pressed }
                                if (downCount == 0) continue

                                val startTime = System.currentTimeMillis()
                                var totalDragDist = 0f
                                var maxPointers = downCount
                                var lastPos1 = downEvent.changes.firstOrNull { it.pressed }?.position ?: Offset.Zero
                                var lastCenter: Offset? = null
                                var isTwoFingerScrolling = false

                                touchPos = lastPos1

                                while (true) {
                                    val event = awaitPointerEvent(PointerEventPass.Main)
                                    val pressed = event.changes.filter { it.pressed }
                                    if (pressed.size > maxPointers) {
                                        maxPointers = pressed.size
                                    }
                                    if (pressed.isEmpty()) break

                                    if (pressed.size == 1 && maxPointers == 1) {
                                        // 1-finger drag -> Cursor movement
                                        val currentPos = pressed[0].position
                                        touchPos = currentPos
                                        val dx = currentPos.x - lastPos1.x
                                        val dy = currentPos.y - lastPos1.y
                                        totalDragDist += kotlin.math.hypot(dx, dy)
                                        lastPos1 = currentPos
                                        pressed[0].consume()
                                        onMove(dx, dy)
                                    } else if (pressed.size >= 2) {
                                        // 2-finger drag -> Scroll
                                        val p1 = pressed[0].position
                                        val p2 = pressed[1].position
                                        val currentCenter = (p1 + p2) / 2f
                                        touchPos = currentCenter

                                        if (!isTwoFingerScrolling) {
                                            isTwoFingerScrolling = true
                                            lastCenter = currentCenter
                                        } else {
                                            val prev = lastCenter ?: currentCenter
                                            val dy = currentCenter.y - prev.y
                                            totalDragDist += kotlin.math.abs(dy)
                                            lastCenter = currentCenter
                                            onScroll(dy)
                                        }
                                        pressed[0].consume()
                                        pressed[1].consume()
                                    } else {
                                        isTwoFingerScrolling = false
                                        lastCenter = null
                                    }
                                }

                                touchPos = null
                                val elapsed = System.currentTimeMillis() - startTime
                                // Tap detection: released quickly without significant dragging
                                if (elapsed < 320 && totalDragDist < 24f) {
                                    if (maxPointers >= 2) {
                                        // 2-finger tap -> Right Click!
                                        onRightClick()
                                    } else {
                                        // 1-finger tap -> Left Click!
                                        onLeftClick()
                                    }
                                }
                            }
                        }
                    }
            ) {
                // Touch position ripple feedback
                touchPos?.let { pos ->
                    Canvas(modifier = Modifier.fillMaxSize()) {
                        drawCircle(
                            color = Color.White.copy(alpha = 0.08f),
                            radius = 36.dp.toPx(),
                            center = pos
                        )
                    }
                }
            }

            // --- Vertical Divider Line ---
            Box(
                modifier = Modifier
                    .width(1.dp)
                    .fillMaxHeight()
                    .background(WireWaveTheme.Colors.borderSubtle)
            )

            MouseScrollStrip(mousePrefs = mousePrefs, onScroll = onScroll)
        }
    }
}
