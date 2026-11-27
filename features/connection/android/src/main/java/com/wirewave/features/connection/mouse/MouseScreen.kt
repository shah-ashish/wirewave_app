package com.wirewave.features.connection.mouse

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.ime
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.session.MousePrefs
import com.wirewave.features.connection.keyboard.RemoteKeyboardInput
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

    Box(
        modifier = modifier
            .fillMaxSize()
            .background(WireWaveTheme.Colors.bgCanvas)
    ) {
        RemoteKeyboardInput(
            focusRequester = focusRequester,
            onSendKeyInput = onSendKeyInput,
            modifier = Modifier.align(Alignment.BottomCenter)
        )

        Column(
            modifier = Modifier
                .fillMaxSize()
                .statusBarsPadding()
                .navigationBarsPadding()
                .imePadding()
        ) {
            MouseHeader(
                speakerActive = speakerActive,
                micActive = micActive,
                onToggleSpeaker = onToggleSpeaker,
                onToggleMic = onToggleMic,
                onNavigateSettings = onNavigateSettings
            )

            Spacer(modifier = Modifier.height(4.dp))

            // --- Main Trackpad Area with Multi-touch Gestures ---
            Box(
                modifier = Modifier
                    .weight(1f)
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp)
            ) {
                TrackpadSurface(
                    mousePrefs = mousePrefs,
                    onMove = { dx, dy -> mouseEngine?.sendMove(dx, dy, mousePrefs.pointerSpeed) },
                    onScroll = { delta ->
                        mouseEngine?.sendScroll(delta, mousePrefs.scrollSpeed, mousePrefs.naturalScroll)
                    },
                    onLeftClick = { mouseEngine?.sendClick(MouseEngine.BUTTON_LEFT) },
                    onRightClick = { mouseEngine?.sendClick(MouseEngine.BUTTON_RIGHT) }
                )
            }

            if (mousePrefs.showMouseButtons) {
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
                        onButtonDown = { mouseEngine?.sendButtonDown(MouseEngine.BUTTON_LEFT) },
                        onButtonUp = { mouseEngine?.sendButtonUp(MouseEngine.BUTTON_LEFT) },
                        modifier = Modifier.weight(1f)
                    )
                    MouseClickButton(
                        leftSelected = false,
                        onButtonDown = { mouseEngine?.sendButtonDown(MouseEngine.BUTTON_RIGHT) },
                        onButtonUp = { mouseEngine?.sendButtonUp(MouseEngine.BUTTON_RIGHT) },
                        modifier = Modifier.weight(1f)
                    )
                }
            }

            Spacer(modifier = Modifier.height(10.dp))

            // --- Bottom Navigation Dock ---
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp),
                contentAlignment = Alignment.Center
            ) {
                MouseNavigationDock(
                    controlsActive = overlayMode == ScreenOverlay.CONTROLS,
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
}
