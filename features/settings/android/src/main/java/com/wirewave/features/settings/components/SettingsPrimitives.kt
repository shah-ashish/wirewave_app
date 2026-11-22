package com.wirewave.features.settings.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Check
import androidx.compose.material.icons.rounded.Close
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme

/** Reusable OLED rounded card surface driven by global [WireWaveTheme]. */
@Composable
fun SurfaceCard(
    modifier: Modifier = Modifier,
    content: @Composable ColumnScope.() -> Unit
) {
    val shape = RoundedCornerShape(WireWaveTheme.Metrics.radiusCard)
    Column(
        modifier = modifier
            .fillMaxWidth()
            .clip(shape)
            .background(WireWaveTheme.Colors.bgCard)
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, shape),
        content = content
    )
}

@Composable
fun SectionLabel(
    text: String,
    modifier: Modifier = Modifier
) {
    Text(
        text = text,
        color = WireWaveTheme.Colors.textSection,
        fontSize = WireWaveTheme.Typography.section,
        fontWeight = FontWeight.Medium,
        modifier = modifier.padding(start = 4.dp)
    )
}

@Composable
fun SettingRow(
    label: String,
    showTopDivider: Boolean,
    modifier: Modifier = Modifier,
    trailing: @Composable () -> Unit
) {
    Column(modifier = modifier.fillMaxWidth()) {
        if (showTopDivider) {
            HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)
        }
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .heightIn(min = WireWaveTheme.Metrics.rowHeight)
                .padding(horizontal = WireWaveTheme.Metrics.spacingMd),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(
                text = label,
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = WireWaveTheme.Typography.body
            )
            trailing()
        }
    }
}

@Composable
fun SettingValueText(
    value: String,
    monospace: Boolean = false
) {
    Text(
        text = value,
        color = WireWaveTheme.Colors.textSecondary,
        fontSize = if (monospace) WireWaveTheme.Typography.mono else WireWaveTheme.Typography.body,
        fontFamily = if (monospace) FontFamily.Monospace else FontFamily.Default
    )
}

@Composable
fun StatusPill(
    isSupported: Boolean,
    label: String
) {
    val bgColor = if (isSupported) WireWaveTheme.Colors.statusOkBg else WireWaveTheme.Colors.statusErrBg
    val fgColor = if (isSupported) WireWaveTheme.Colors.statusOkFg else WireWaveTheme.Colors.statusErrFg
    val icon = if (isSupported) Icons.Rounded.Check else Icons.Rounded.Close

    Row(
        modifier = Modifier
            .clip(RoundedCornerShape(WireWaveTheme.Metrics.radiusPill))
            .background(bgColor)
            .padding(horizontal = 12.dp, vertical = 5.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            imageVector = icon,
            contentDescription = null,
            tint = fgColor,
            modifier = Modifier.size(14.dp)
        )
        Spacer(modifier = Modifier.width(6.dp))
        Text(
            text = label,
            color = fgColor,
            fontSize = WireWaveTheme.Typography.badge,
            fontWeight = FontWeight.Medium
        )
    }
}
