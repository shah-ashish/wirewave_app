package com.wirewave.features.settings.about

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.wirewave.core.config.AppConfig
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.features.settings.components.SectionLabel
import com.wirewave.features.settings.components.SettingRow
import com.wirewave.features.settings.components.SettingValueText
import com.wirewave.features.settings.components.StatusPill
import com.wirewave.features.settings.components.SurfaceCard

private data class AboutRowSpec(
    val label: String,
    val value: String,
    val monospace: Boolean = false
)

@Composable
fun AboutSection(
    snapshot: AboutSnapshot,
    modifier: Modifier = Modifier
) {
    val rows = listOf(
        AboutRowSpec("App name", snapshot.appName),
        AboutRowSpec("Version", AppConfig.versionName, monospace = true),
        AboutRowSpec("Device name", snapshot.deviceName),
        AboutRowSpec("Connected PC", snapshot.connectedPc),
        AboutRowSpec("App ID", snapshot.appId, monospace = true)
    )

    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(WireWaveTheme.Metrics.spacingSm)
    ) {
        SectionLabel("ABOUT")
        SurfaceCard {
            rows.forEachIndexed { index, row ->
                SettingRow(label = row.label, showTopDivider = index > 0) {
                    SettingValueText(value = row.value, monospace = row.monospace)
                }
            }
            SettingRow(label = "Compatibility", showTopDivider = true) {
                StatusPill(
                    isSupported = snapshot.compatibility.isSupported,
                    label = snapshot.compatibility.badgeLabel
                )
            }
        }
    }
}
