package com.wirewave.features.settings.mouse

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Slider
import androidx.compose.material3.SliderDefaults
import androidx.compose.material3.Switch
import androidx.compose.material3.SwitchDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.session.MousePrefs
import com.wirewave.features.settings.components.SectionLabel
import com.wirewave.features.settings.components.SurfaceCard
import java.util.Locale

@Composable
fun MouseSettingsSection(
    prefs: MousePrefs,
    onPointerSpeedChange: (Float) -> Unit,
    onScrollSpeedChange: (Float) -> Unit,
    onToggleNaturalScroll: (Boolean) -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(WireWaveTheme.Metrics.spacingSm)
    ) {
        SectionLabel("MOUSE & TRACKPAD")
        SurfaceCard {
            // Row 1: Pointer Speed Slider
            SliderSettingRow(
                title = "Pointer speed",
                subtitle = "Adjust cursor movement sensitivity",
                value = prefs.pointerSpeed,
                valueRange = 0.5f..3.0f,
                displayValue = String.format(Locale.US, "%.1fx", prefs.pointerSpeed),
                onValueChange = onPointerSpeedChange
            )

            HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)

            // Row 2: Scroll Speed Slider
            SliderSettingRow(
                title = "Scroll speed",
                subtitle = "Adjust two-finger and scrollbar sensitivity",
                value = prefs.scrollSpeed,
                valueRange = 0.5f..3.0f,
                displayValue = String.format(Locale.US, "%.1fx", prefs.scrollSpeed),
                onValueChange = onScrollSpeedChange
            )

            HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)

            // Row 3: Scroll Behaviour Switch
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = 12.dp),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Column(modifier = Modifier.weight(1f).padding(end = 16.dp)) {
                    Text(
                        text = "Natural scrolling",
                        color = WireWaveTheme.Colors.textPrimary,
                        fontSize = WireWaveTheme.Typography.body,
                        fontWeight = FontWeight.Medium
                    )
                    Spacer(modifier = Modifier.height(2.dp))
                    Text(
                        text = if (prefs.naturalScroll) {
                            "Content moves in the direction of your fingers"
                        } else {
                            "Scrollbar tracks finger direction (traditional)"
                        },
                        color = WireWaveTheme.Colors.textSecondary,
                        fontSize = WireWaveTheme.Typography.badge
                    )
                }

                Switch(
                    checked = prefs.naturalScroll,
                    onCheckedChange = onToggleNaturalScroll,
                    colors = SwitchDefaults.colors(
                        checkedThumbColor = WireWaveTheme.Colors.textPrimary,
                        checkedTrackColor = WireWaveTheme.Colors.accentBlue,
                        uncheckedThumbColor = WireWaveTheme.Colors.textSecondary,
                        uncheckedTrackColor = WireWaveTheme.Colors.bgCardHover
                    )
                )
            }
        }
    }
}

@Composable
private fun SliderSettingRow(
    title: String,
    subtitle: String,
    value: Float,
    valueRange: ClosedFloatingPointRange<Float>,
    displayValue: String,
    onValueChange: (Float) -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = 10.dp)
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(
                text = title,
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = WireWaveTheme.Typography.body,
                fontWeight = FontWeight.Medium
            )
            Text(
                text = displayValue,
                color = WireWaveTheme.Colors.accentBlue,
                fontSize = WireWaveTheme.Typography.body,
                fontWeight = FontWeight.SemiBold
            )
        }
        Spacer(modifier = Modifier.height(2.dp))
        Text(
            text = subtitle,
            color = WireWaveTheme.Colors.textSecondary,
            fontSize = WireWaveTheme.Typography.badge
        )
        Slider(
            value = value,
            onValueChange = onValueChange,
            valueRange = valueRange,
            colors = SliderDefaults.colors(
                thumbColor = WireWaveTheme.Colors.accentBlue,
                activeTrackColor = WireWaveTheme.Colors.accentBlue,
                inactiveTrackColor = WireWaveTheme.Colors.bgCardHover
            ),
            modifier = Modifier.padding(top = 4.dp)
        )
    }
}
