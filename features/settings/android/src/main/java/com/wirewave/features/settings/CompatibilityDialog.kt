package com.wirewave.features.settings

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import com.wirewave.core.compatibility.CompatibilityReport
import com.wirewave.core.compatibility.CompatibilityStatus
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.features.settings.components.SurfaceCard

/** Non-dismissible compatibility blocker shown if the device is below Android 11. */
@Composable
fun CompatibilityDialog(report: CompatibilityReport) {
    val unsupported = report.status as? CompatibilityStatus.Unsupported ?: return

    Dialog(
        onDismissRequest = {},
        properties = DialogProperties(
            dismissOnBackPress = false,
            dismissOnClickOutside = false
        )
    ) {
        SurfaceCard {
            Column(
                modifier = Modifier.padding(WireWaveTheme.Metrics.spacingLg),
                verticalArrangement = Arrangement.spacedBy(WireWaveTheme.Metrics.spacingSm)
            ) {
                Text(
                    text = "Device Not Compatible",
                    color = WireWaveTheme.Colors.statusErrFg,
                    fontSize = WireWaveTheme.Typography.title,
                    fontWeight = FontWeight.SemiBold
                )
                Text(
                    text = unsupported.reason,
                    color = WireWaveTheme.Colors.textPrimary,
                    fontSize = WireWaveTheme.Typography.body
                )
                Text(
                    text = "Minimum required: ${report.minimumRequired}",
                    color = WireWaveTheme.Colors.textSecondary,
                    fontSize = WireWaveTheme.Typography.badge
                )
            }
        }
    }
}
