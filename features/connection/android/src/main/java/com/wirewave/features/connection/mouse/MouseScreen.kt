package com.wirewave.features.connection.mouse

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.ime
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.VolumeUp
import androidx.compose.material.icons.rounded.Add
import androidx.compose.material.icons.rounded.Home
import androidx.compose.material.icons.rounded.Keyboard
import androidx.compose.material.icons.rounded.KeyboardArrowDown
import androidx.compose.material.icons.rounded.KeyboardArrowUp
import androidx.compose.material.icons.rounded.Mic
import androidx.compose.material.icons.rounded.MicOff
import androidx.compose.material.icons.rounded.PhotoCamera
import androidx.compose.material.icons.rounded.Settings
import androidx.compose.material.icons.rounded.SwapVert
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.input.pointer.PointerEventPass
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.session.MousePrefs
import com.wirewave.features.connection.overlay.ScreenOverlay
import com.wirewave.features.connection.overlay.ScreenOverlayHost

@Composable
fun MouseScreen(
    speakerActive: Boolean,
    micActive: Boolean,
    onToggleSpeaker: () -> Unit,
    onToggleMic: () -> Unit,
    overlayMode: ScreenOverlay,
    onOverlayChange: (ScreenOverlay) -> Unit,
    onNavigateHome: () -> Unit,
    onNavigateCamera: () -> Unit,
    onNavigateSettings: () -> Unit = {},
    onSendKeyInput: (text: String?, keyCode: String?) -> Unit = { _, _ -> },
    mouseEngine: MouseEngine? = null,
    mousePrefs: MousePrefs = MousePrefs(),
    modifier: Modifier = Modifier
) {
    val keyboardController = LocalSoftwareKeyboardController.current
    val focusRequester = remember { FocusRequester() }
    val density = LocalDensity.current
    val imeBottom = WindowInsets.ime.getBottom(density)
    val isKeyboardVisible = imeBottom > 0

    val sentinel = "  "
    var textValue by remember {
        mutableStateOf(TextFieldValue(sentinel, TextRange(sentinel.length)))
    }

    // Hidden input field that triggers and captures Android native keyboard input (Gboard, Samsung, etc.)
    BasicTextField(
        value = textValue,
        onValueChange = { newText ->
            val currentStr = newText.text
            if (currentStr.length > sentinel.length) {
                val added = currentStr.substring(sentinel.length)
                if (added.contains("\n")) {
                    val parts = added.split("\n")
                    parts.forEachIndexed { index, part ->
                        if (part.isNotEmpty()) {
                            onSendKeyInput(part, null)
                        }
                        if (index < parts.size - 1) {
                            onSendKeyInput(null, "Enter")
                        }
                    }
                } else {
                    onSendKeyInput(added, null)
                }
                textValue = TextFieldValue(sentinel, TextRange(sentinel.length))
            } else if (currentStr.length < sentinel.length) {
                val count = sentinel.length - currentStr.length
                repeat(count) {
                    onSendKeyInput(null, "Backspace")
                }
                textValue = TextFieldValue(sentinel, TextRange(sentinel.length))
            } else if (currentStr != sentinel) {
                onSendKeyInput(currentStr, null)
                textValue = TextFieldValue(sentinel, TextRange(sentinel.length))
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
        modifier = Modifier
            .size(1.dp)
            .alpha(0f)
            .focusRequester(focusRequester)
    )

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(WireWaveTheme.Colors.bgCanvas)
            .windowInsetsPadding(WindowInsets.safeDrawing)
    ) {
        // --- Top Bar ---
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp, vertical = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(
                text = "Mouse",
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = 24.sp,
                fontWeight = FontWeight.Bold
            )

            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                // Speaker Button
                Box(
                    modifier = Modifier
                        .size(38.dp)
                        .clip(CircleShape)
                        .background(
                            if (speakerActive) WireWaveTheme.Colors.accentBlue
                            else WireWaveTheme.Colors.bgCard
                        )
                        .clickable(role = Role.Button, onClick = onToggleSpeaker),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.AutoMirrored.Rounded.VolumeUp,
                        contentDescription = "Speaker",
                        tint = if (speakerActive) Color.White else WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(20.dp)
                    )
                }

                // Microphone Button
                Box(
                    modifier = Modifier
                        .size(38.dp)
                        .clip(CircleShape)
                        .background(
                            if (micActive) WireWaveTheme.Colors.accentBlue
                            else WireWaveTheme.Colors.bgCard
                        )
                        .clickable(role = Role.Button, onClick = onToggleMic),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = if (micActive) Icons.Rounded.Mic else Icons.Rounded.MicOff,
                        contentDescription = "Microphone",
                        tint = if (micActive) Color.White else WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(20.dp)
                    )
                }

                // Settings Button
                Box(
                    modifier = Modifier
                        .size(38.dp)
                        .clip(CircleShape)
                        .background(WireWaveTheme.Colors.bgCard)
                        .clickable(role = Role.Button, onClick = onNavigateSettings),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.Rounded.Settings,
                        contentDescription = "Settings",
                        tint = WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(20.dp)
                    )
                }
            }
        }

        Spacer(modifier = Modifier.height(4.dp))

        // --- Main Trackpad Area with Multi-touch Gestures ---
        Box(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
                .padding(horizontal = 16.dp)
        ) {
            TrackpadSurface(
                mouseEngine = mouseEngine,
                mousePrefs = mousePrefs
            )
        }

        Spacer(modifier = Modifier.height(10.dp))

        // --- Mouse Click Buttons (Left / Right) ---
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp),
            horizontalArrangement = Arrangement.spacedBy(10.dp)
        ) {
            MouseClickButton(
                leftSelected = true,
                buttonId = MouseEngine.BUTTON_LEFT,
                mouseEngine = mouseEngine,
                modifier = Modifier.weight(1f)
            )
            MouseClickButton(
                leftSelected = false,
                buttonId = MouseEngine.BUTTON_RIGHT,
                mouseEngine = mouseEngine,
                modifier = Modifier.weight(1f)
            )
        }

        Spacer(modifier = Modifier.height(10.dp))

        // --- Bottom Navigation Dock ---
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp),
            contentAlignment = Alignment.Center
        ) {
            BottomDock(
                overlayMode = overlayMode,
                isKeyboardActive = isKeyboardVisible,
                onHomeClick = onNavigateHome,
                onControlsToggle = {
                    if (isKeyboardVisible) {
                        keyboardController?.hide()
                    }
                    onOverlayChange(
                        if (overlayMode == ScreenOverlay.CONTROLS) ScreenOverlay.NONE
                        else ScreenOverlay.CONTROLS
                    )
                },
                onCameraClick = onNavigateCamera,
                onKeyboardToggle = {
                    if (isKeyboardVisible) {
                        keyboardController?.hide()
                    } else {
                        if (overlayMode == ScreenOverlay.CONTROLS) {
                            onOverlayChange(ScreenOverlay.NONE)
                        }
                        focusRequester.requestFocus()
                        keyboardController?.show()
                    }
                }
            )
        }

        // --- Host Overlay (Controls) ---
        ScreenOverlayHost(
            overlay = overlayMode,
            onControlKeyClick = { code ->
                onSendKeyInput(null, code)
            }
        )

        Spacer(modifier = Modifier.height(6.dp))
    }
}

/** Multi-touch Trackpad: 1-finger move/tap, 2-finger scroll/tap right-click, and scroll strip. */
@Composable
private fun TrackpadSurface(
    mouseEngine: MouseEngine?,
    mousePrefs: MousePrefs
) {
    val trackpadShape = RoundedCornerShape(16.dp)
    var touchPos by remember { mutableStateOf<Offset?>(null) }

    Box(
        modifier = Modifier
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
                                        mouseEngine?.sendMove(dx, dy, mousePrefs.pointerSpeed)
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
                                            mouseEngine?.sendScroll(dy, mousePrefs.scrollSpeed, mousePrefs.naturalScroll)
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
                                        mouseEngine?.sendClick(MouseEngine.BUTTON_RIGHT)
                                    } else {
                                        // 1-finger tap -> Left Click!
                                        mouseEngine?.sendClick(MouseEngine.BUTTON_LEFT)
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

            // --- Right Area: Side Scroller (60.dp width) with Instant Response ---
            Box(
                modifier = Modifier
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
                                        mouseEngine?.sendScroll(dy, mousePrefs.scrollSpeed, mousePrefs.naturalScroll)
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
    }
}

/** Left or Right mouse click button with custom highlighted mouse icon. */
@Composable
private fun MouseClickButton(
    leftSelected: Boolean,
    buttonId: Byte,
    mouseEngine: MouseEngine?,
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
                        mouseEngine?.sendButtonDown(buttonId)
                        tryAwaitRelease()
                        mouseEngine?.sendButtonUp(buttonId)
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
        if (leftActive) {
            drawRoundRect(
                color = accent,
                topLeft = Offset(1.5.dp.toPx(), 1.5.dp.toPx()),
                size = Size((w / 2f) - 2.dp.toPx(), splitY - 2.dp.toPx()),
                cornerRadius = CornerRadius(corner * 0.8f, 2.dp.toPx())
            )
        } else if (rightActive) {
            drawRoundRect(
                color = accent,
                topLeft = Offset((w / 2f) + 1.dp.toPx(), 1.5.dp.toPx()),
                size = Size((w / 2f) - 2.dp.toPx(), splitY - 2.dp.toPx()),
                cornerRadius = CornerRadius(2.dp.toPx(), corner * 0.8f)
            )
        }
    }
}

/** Floating bottom navigation dock containing Home, Controls, Camera, and Keyboard. */
@Composable
private fun BottomDock(
    overlayMode: ScreenOverlay,
    isKeyboardActive: Boolean,
    onHomeClick: () -> Unit,
    onControlsToggle: () -> Unit,
    onCameraClick: () -> Unit,
    onKeyboardToggle: () -> Unit
) {
    val dockShape = RoundedCornerShape(26.dp)

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .height(52.dp)
            .clip(dockShape)
            .background(WireWaveTheme.Colors.bgCard)
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, dockShape)
            .padding(horizontal = 10.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        // 1. Home
        DockIconButton(
            icon = Icons.Rounded.Home,
            contentDescription = "Home",
            isActive = false,
            onClick = onHomeClick
        )

        // 2. Controls (+)
        DockIconButton(
            icon = Icons.Rounded.Add,
            contentDescription = "Controls",
            isActive = overlayMode == ScreenOverlay.CONTROLS,
            onClick = onControlsToggle
        )

        // 3. Camera
        DockIconButton(
            icon = Icons.Rounded.PhotoCamera,
            contentDescription = "Camera",
            isActive = false,
            onClick = onCameraClick
        )

        // 4. Keyboard
        DockIconButton(
            icon = Icons.Rounded.Keyboard,
            contentDescription = "Keyboard",
            isActive = isKeyboardActive,
            onClick = onKeyboardToggle
        )
    }
}

@Composable
private fun DockIconButton(
    icon: ImageVector,
    contentDescription: String,
    isActive: Boolean,
    onClick: () -> Unit
) {
    val pillShape = RoundedCornerShape(16.dp)
    Box(
        modifier = Modifier
            .width(56.dp)
            .height(38.dp)
            .clip(pillShape)
            .background(if (isActive) WireWaveTheme.Colors.accentBlue else Color.Transparent)
            .clickable(role = Role.Button, onClick = onClick),
        contentAlignment = Alignment.Center
    ) {
        Icon(
            imageVector = icon,
            contentDescription = contentDescription,
            tint = if (isActive) Color.White else WireWaveTheme.Colors.textSecondary,
            modifier = Modifier.size(22.dp)
        )
    }
}
