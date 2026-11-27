package com.wirewave.features.speaker

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.VolumeUp
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.ui.FeatureCircleTile

@Composable
fun SpeakerCircleTile(
    isOn: Boolean,
    isBlocked: Boolean,
    onToggle: () -> Unit,
    modifier: Modifier = Modifier
) {
    FeatureCircleTile(
        title = "Speaker",
        icon = Icons.Rounded.VolumeUp,
        contentDescription = "Speaker",
        active = isOn,
        enabled = !isBlocked,
        status = when {
            isBlocked -> "Blocked"
            isOn -> "On"
            else -> "Off"
        },
        statusColor = when {
            isBlocked -> WireWaveTheme.Colors.statusErrFg
            isOn -> WireWaveTheme.Colors.accentBlue
            else -> WireWaveTheme.Colors.textSecondary
        },
        onToggle = onToggle,
        modifier = modifier
    )
}
