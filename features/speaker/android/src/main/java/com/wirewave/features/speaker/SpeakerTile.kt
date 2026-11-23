package com.wirewave.features.speaker

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.VolumeUp
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme

/**
 * Interactive Android Speaker circle button matching 3 - Connected.png.
 * Replaces the Stage 2 "Coming soon" placeholder for Speaker.
 */
@Composable
fun SpeakerCircleTile(
    isOn: Boolean,
    onToggle: () -> Unit,
    modifier: Modifier = Modifier
) {
    val circleBg = if (isOn) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.bgCard
    val borderColor = if (isOn) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.borderSubtle
    val iconTint = if (isOn) WireWaveTheme.Colors.textPrimary else WireWaveTheme.Colors.textSecondary
    val statusColor = if (isOn) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.textSecondary

    Column(
        modifier = modifier,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Box(
            modifier = Modifier
                .size(76.dp)
                .clip(CircleShape)
                .background(circleBg)
                .border(1.5.dp, borderColor, CircleShape)
                .clickable(role = Role.Switch, onClick = onToggle),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = Icons.Rounded.VolumeUp,
                contentDescription = "Speaker",
                tint = iconTint,
                modifier = Modifier.size(30.dp)
            )
        }
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Speaker",
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = WireWaveTheme.Typography.badge,
            fontWeight = FontWeight.SemiBold
        )
        Text(
            text = if (isOn) "On" else "Off",
            color = statusColor,
            fontSize = WireWaveTheme.Typography.badge
        )
    }
}
