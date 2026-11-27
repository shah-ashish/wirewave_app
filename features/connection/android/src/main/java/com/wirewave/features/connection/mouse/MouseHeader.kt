package com.wirewave.features.connection.mouse

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.VolumeUp
import androidx.compose.material.icons.rounded.Mic
import androidx.compose.material.icons.rounded.MicOff
import androidx.compose.material.icons.rounded.Settings
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.ui.CircleIconButton

@Composable
internal fun MouseHeader(
    speakerActive: Boolean,
    micActive: Boolean,
    onToggleSpeaker: () -> Unit,
    onToggleMic: () -> Unit,
    onNavigateSettings: () -> Unit,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier
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
            CircleIconButton(
                icon = Icons.AutoMirrored.Rounded.VolumeUp,
                contentDescription = "Speaker",
                active = speakerActive,
                onClick = onToggleSpeaker
            )
            CircleIconButton(
                icon = if (micActive) Icons.Rounded.Mic else Icons.Rounded.MicOff,
                contentDescription = "Microphone",
                active = micActive,
                onClick = onToggleMic
            )
            CircleIconButton(
                icon = Icons.Rounded.Settings,
                contentDescription = "Settings",
                onClick = onNavigateSettings
            )
        }
    }
}
