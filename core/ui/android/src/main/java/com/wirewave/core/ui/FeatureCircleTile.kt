package com.wirewave.core.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme

/** Stateless feature control. The caller owns status, availability, and actions. */
@Composable
fun FeatureCircleTile(
    title: String,
    icon: ImageVector,
    contentDescription: String,
    active: Boolean,
    enabled: Boolean,
    status: String,
    statusColor: Color,
    onToggle: () -> Unit,
    modifier: Modifier = Modifier
) {
    val background = if (active) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.bgCard
    val border = if (active) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.borderSubtle
    val iconTint = if (active) WireWaveTheme.Colors.textPrimary else WireWaveTheme.Colors.textSecondary

    Column(modifier = modifier, horizontalAlignment = Alignment.CenterHorizontally) {
        Box(
            modifier = Modifier
                .size(76.dp)
                .clip(CircleShape)
                .background(background)
                .border(1.5.dp, border, CircleShape)
                .clickable(enabled = enabled, role = Role.Switch, onClick = onToggle),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = icon,
                contentDescription = contentDescription,
                tint = iconTint,
                modifier = Modifier.size(30.dp)
            )
        }
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = title,
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = WireWaveTheme.Typography.badge,
            fontWeight = FontWeight.SemiBold
        )
        Text(text = status, color = statusColor, fontSize = WireWaveTheme.Typography.badge)
    }
}
