package com.wirewave.features.connection.connected

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.ime
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Mouse
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.unit.dp
import com.wirewave.core.ui.FeatureNavigationItem
import com.wirewave.core.ui.FeatureNavigationList
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.transport.TransportKind
import com.wirewave.features.connection.keyboard.RemoteKeyboardInput
import com.wirewave.features.connection.overlay.ScreenOverlay
import com.wirewave.features.connection.overlay.ScreenOverlayHost

@Composable
fun ConnectedHomeScreen(
    pcName: String,
    transport: TransportKind,
    onDisconnect: () -> Unit,
    onOpenSettings: () -> Unit,
    brandMark: @Composable () -> Unit,
    speakerTile: @Composable () -> Unit = {},
    micTile: @Composable () -> Unit = {},
    onNavigateToMouse: () -> Unit = {},
    onSendKeyInput: (text: String?, keyCode: String?) -> Unit = { _, _ -> },
    activeOverlay: ScreenOverlay = ScreenOverlay.NONE,
    modifier: Modifier = Modifier
) {
    val keyboardController = LocalSoftwareKeyboardController.current
    val focusRequester = remember { FocusRequester() }
    val density = LocalDensity.current
    val imeBottom = WindowInsets.ime.getBottom(density)
    val isKeyboardVisible = imeBottom > 0

    val rows = listOf(
        FeatureNavigationItem(
            title = "Mouse",
            subtitle = "Move the cursor, scroll, click",
            icon = Icons.Rounded.Mouse,
            iconBackground = WireWaveTheme.Colors.accentBlue,
            onClick = onNavigateToMouse
        )
    )

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
                .imePadding()
                .verticalScroll(rememberScrollState())
                .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = WireWaveTheme.Metrics.spacingSm)
                .padding(bottom = if (activeOverlay != ScreenOverlay.NONE) 280.dp else 16.dp),
            verticalArrangement = Arrangement.spacedBy(22.dp)
        ) {
            ConnectedHomeHeader(
                brandMark = brandMark,
                onOpenSettings = onOpenSettings
            )
            ConnectionSummary(
                pcName = pcName,
                transport = transport,
                onDisconnect = onDisconnect
            )
            ConnectedAudioControls(
                speakerTile = speakerTile,
                micTile = micTile,
                isKeyboardActive = isKeyboardVisible,
                onToggleKeyboard = {
                    if (isKeyboardVisible) {
                        keyboardController?.hide()
                    } else {
                        focusRequester.requestFocus()
                        keyboardController?.show()
                    }
                }
            )

            FeatureNavigationList(title = "USE PHONE AS", rows = rows)
        }

        // Screen Overlay Host (Controls popping up on the current screen)
        ScreenOverlayHost(
            overlay = activeOverlay,
            onControlKeyClick = { code ->
                onSendKeyInput(null, code)
            },
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .navigationBarsPadding()
        )
    }
}
