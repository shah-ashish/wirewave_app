package com.wirewave.features.mic

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
import androidx.compose.material.icons.rounded.Mic
import androidx.compose.material.icons.rounded.MicOff
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme

@Composable
fun MicCircleTile(
    isOn: Boolean,
    isBlocked: Boolean,
    blockedDetail: String?,
    onToggle: () -> Unit,
    modifier: Modifier = Modifier
) {
    val active = isOn && !isBlocked
    val accent = if (active) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.textSecondary
    val status = when {
        isBlocked -> "Blocked"
        active -> "On"
        else -> "Off"
    }

    Column(
        modifier = modifier.alpha(if (isBlocked) 0.55f else 1f),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Box(
            modifier = Modifier
                .size(76.dp)
                .clip(CircleShape)
                .background(if (active) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.bgCard)
                .border(
                    1.5.dp,
                    if (active) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.borderSubtle,
                    CircleShape
                )
                .clickable(enabled = !isBlocked, role = Role.Switch, onClick = onToggle),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = if (isBlocked) Icons.Rounded.MicOff else Icons.Rounded.Mic,
                contentDescription = blockedDetail ?: "Microphone",
                tint = if (active) WireWaveTheme.Colors.textPrimary else accent,
                modifier = Modifier.size(30.dp)
            )
        }
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Mic",
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = WireWaveTheme.Typography.badge,
            fontWeight = FontWeight.SemiBold
        )
        Text(text = status, color = accent, fontSize = WireWaveTheme.Typography.badge)
    }
}
