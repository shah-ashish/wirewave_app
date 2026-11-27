package com.wirewave.features.mic

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Mic
import androidx.compose.material.icons.rounded.MicOff
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.ui.FeatureCircleTile

@Composable
fun MicCircleTile(
    isOn: Boolean,
    isBlocked: Boolean,
    blockedDetail: String?,
    onToggle: () -> Unit,
    modifier: Modifier = Modifier
) {
    val active = isOn && !isBlocked
    FeatureCircleTile(
        title = "Mic",
        icon = if (isBlocked) Icons.Rounded.MicOff else Icons.Rounded.Mic,
        contentDescription = blockedDetail ?: "Microphone",
        active = active,
        enabled = !isBlocked,
        status = when {
            isBlocked -> "Blocked"
            active -> "On"
            else -> "Off"
        },
        statusColor = if (active) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.textSecondary,
        onToggle = onToggle,
        modifier = modifier.alpha(if (isBlocked) 0.55f else 1f)
    )
}
