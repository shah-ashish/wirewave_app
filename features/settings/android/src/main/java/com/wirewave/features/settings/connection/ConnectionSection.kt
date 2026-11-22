package com.wirewave.features.settings.connection

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Switch
import androidx.compose.material3.SwitchDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.session.ConnectionPrefs
import com.wirewave.features.settings.components.SectionLabel
import com.wirewave.features.settings.components.SurfaceCard

/** Matches CONNECTION card in 7 - Settings - App.png (Auto-connect + Keep alive in background). */
@Composable
fun ConnectionSection(
    prefs: ConnectionPrefs,
    onToggleAutoConnect: (Boolean) -> Unit,
    onToggleKeepAlive: (Boolean) -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(WireWaveTheme.Metrics.spacingSm)
    ) {
        SectionLabel("CONNECTION")
        SurfaceCard {
            ToggleSettingRow(
                title = "Auto-connect",
                subtitle = "Connect to your last used PC automatically",
                checked = prefs.autoConnect,
                onCheckedChange = onToggleAutoConnect,
                showTopDivider = false
            )
            ToggleSettingRow(
                title = "Keep alive in background",
                subtitle = "Keep the connection running when the app is closed",
                checked = prefs.keepAliveInBackground,
                onCheckedChange = onToggleKeepAlive,
                showTopDivider = true
            )
        }
    }
}

@Composable
private fun ToggleSettingRow(
    title: String,
    subtitle: String,
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit,
    showTopDivider: Boolean
) {
    if (showTopDivider) {
        HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)
    }
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(
                horizontal = WireWaveTheme.Metrics.spacingMd,
                vertical = 14.dp
            ),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(14.dp)
    ) {
        Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(3.dp)) {
            Text(
                text = title,
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = WireWaveTheme.Typography.body
            )
            Text(
                text = subtitle,
                color = WireWaveTheme.Colors.textSecondary,
                fontSize = WireWaveTheme.Typography.badge
            )
        }
        Switch(
            checked = checked,
            onCheckedChange = onCheckedChange,
            colors = SwitchDefaults.colors(
                checkedThumbColor = WireWaveTheme.Colors.textPrimary,
                checkedTrackColor = WireWaveTheme.Colors.statusOkFg,
                uncheckedThumbColor = WireWaveTheme.Colors.textPrimary,
                uncheckedTrackColor = WireWaveTheme.Colors.borderSubtle,
                uncheckedBorderColor = WireWaveTheme.Colors.borderSubtle
            )
        )
    }
}
