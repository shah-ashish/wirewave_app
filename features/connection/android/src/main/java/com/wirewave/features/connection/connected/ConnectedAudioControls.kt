package com.wirewave.features.connection.connected

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Keyboard
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.ui.FeatureCircleTile

@Composable
internal fun ConnectedAudioControls(
    speakerTile: @Composable () -> Unit,
    micTile: @Composable () -> Unit,
    isKeyboardActive: Boolean,
    onToggleKeyboard: () -> Unit
) {
    Column(verticalArrangement = Arrangement.spacedBy(14.dp)) {
        Text(
            text = "AUDIO & INPUT",
            color = WireWaveTheme.Colors.textSection,
            fontSize = WireWaveTheme.Typography.section,
            fontWeight = FontWeight.Medium
        )
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceEvenly
        ) {
            speakerTile()
            micTile()
            FeatureCircleTile(
                title = "Keyboard",
                icon = Icons.Rounded.Keyboard,
                contentDescription = "Keyboard",
                active = isKeyboardActive,
                enabled = true,
                status = if (isKeyboardActive) "Open" else "Tap type",
                statusColor = if (isKeyboardActive) {
                    WireWaveTheme.Colors.accentBlue
                } else {
                    WireWaveTheme.Colors.textSecondary
                },
                onToggle = onToggleKeyboard
            )
        }
    }
}
