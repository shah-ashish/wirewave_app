package com.wirewave.features.settings.update

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.AppConfig
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.features.settings.components.SurfaceCard

@Composable
fun UpdateSection(
    phase: UpdatePhase,
    actionSpec: UpdateActionSpec?,
    onAction: (UpdateAction) -> Unit,
    modifier: Modifier = Modifier
) {
    SurfaceCard(modifier = modifier) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .heightIn(min = WireWaveTheme.Metrics.rowHeight + 6.dp)
                .padding(horizontal = WireWaveTheme.Metrics.spacingMd),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(
                text = "Version ${AppConfig.versionName}",
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = WireWaveTheme.Typography.cardTitle,
                fontWeight = FontWeight.SemiBold
            )

            when (actionSpec) {
                null -> Text(
                    text = busyStatusLabel(phase),
                    color = WireWaveTheme.Colors.textSecondary,
                    fontSize = WireWaveTheme.Typography.body
                )
                else -> Text(
                    text = actionSpec.label,
                    color = WireWaveTheme.Colors.accentBlue,
                    fontSize = WireWaveTheme.Typography.body,
                    fontWeight = FontWeight.Medium,
                    modifier = Modifier
                        .clip(RoundedCornerShape(8.dp))
                        .clickable(role = Role.Button) { onAction(actionSpec.action) }
                        .padding(horizontal = 6.dp, vertical = 4.dp)
                )
            }
        }

        when (phase) {
            is UpdatePhase.Available -> DetailFooter(
                text = "Version ${phase.release.versionName} is ready to download.",
                color = WireWaveTheme.Colors.textSecondary
            )
            is UpdatePhase.Downloading -> ProgressFooter(
                versionName = phase.versionName,
                percent = phase.percent
            )
            is UpdatePhase.ReadyToInstall -> DetailFooter(
                text = "Version ${phase.versionName} verified. Select Install to update.",
                color = WireWaveTheme.Colors.statusOkFg
            )
            is UpdatePhase.Failed -> DetailFooter(
                text = phase.message,
                color = WireWaveTheme.Colors.statusErrFg
            )
            UpdatePhase.Idle, UpdatePhase.Checking, UpdatePhase.UpToDate -> Unit
        }
    }
}

private fun busyStatusLabel(phase: UpdatePhase): String = when (phase) {
    UpdatePhase.Checking -> "Checking..."
    is UpdatePhase.Downloading -> "Downloading ${phase.percent}%"
    else -> ""
}

@Composable
private fun DetailFooter(text: String, color: Color) {
    HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)
    Text(
        text = text,
        color = color,
        fontSize = WireWaveTheme.Typography.badge,
        modifier = Modifier.padding(
            horizontal = WireWaveTheme.Metrics.spacingMd,
            vertical = WireWaveTheme.Metrics.spacingSm
        )
    )
}

@Composable
private fun ProgressFooter(versionName: String, percent: Int) {
    HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(
                horizontal = WireWaveTheme.Metrics.spacingMd,
                vertical = WireWaveTheme.Metrics.spacingSm
            ),
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(
                text = "Downloading v$versionName",
                color = WireWaveTheme.Colors.textSecondary,
                fontSize = WireWaveTheme.Typography.badge
            )
            Text(
                text = "$percent%",
                color = WireWaveTheme.Colors.textSecondary,
                fontSize = WireWaveTheme.Typography.badge
            )
        }
        LinearProgressIndicator(
            progress = { (percent / 100f).coerceIn(0f, 1f) },
            modifier = Modifier
                .fillMaxWidth()
                .height(6.dp)
                .clip(RoundedCornerShape(WireWaveTheme.Metrics.radiusPill)),
            color = WireWaveTheme.Colors.accentBlue,
            trackColor = WireWaveTheme.Colors.borderSubtle
        )
    }
}
