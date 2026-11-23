package com.wirewave.features.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.ChevronLeft
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.session.ConnectionPrefs
import com.wirewave.core.session.TrustedDesktop
import com.wirewave.features.settings.about.AboutSection
import com.wirewave.features.settings.about.AboutSnapshot
import com.wirewave.features.settings.components.SectionLabel
import com.wirewave.features.settings.components.SurfaceCard
import com.wirewave.features.settings.connection.ConnectionSection
import com.wirewave.features.settings.update.UpdateController
import com.wirewave.features.settings.update.UpdateSection

@Composable
fun SettingsScreen(
    aboutSnapshot: AboutSnapshot,
    updateController: UpdateController,
    connectionPrefs: ConnectionPrefs,
    trustedDesktops: List<TrustedDesktop> = emptyList(),
    onToggleAutoConnect: (Boolean) -> Unit,
    onToggleKeepAlive: (Boolean) -> Unit,
    onForgetDesktop: (String) -> Unit = {},
    onBack: () -> Unit,
    modifier: Modifier = Modifier
) {
    val phase by updateController.phase.collectAsState()

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(WireWaveTheme.Colors.bgCanvas)
            .statusBarsPadding()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = WireWaveTheme.Metrics.spacingSm),
        verticalArrangement = Arrangement.spacedBy(WireWaveTheme.Metrics.spacingLg)
    ) {
        Row(
            modifier = Modifier
                .clickable(role = Role.Button, onClick = onBack)
                .padding(vertical = 8.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = Icons.Rounded.ChevronLeft,
                contentDescription = "Back",
                tint = WireWaveTheme.Colors.accentBlue,
                modifier = Modifier.size(22.dp)
            )
            Spacer(modifier = Modifier.width(2.dp))
            Text(
                text = "Back",
                color = WireWaveTheme.Colors.accentBlue,
                fontSize = WireWaveTheme.Typography.title
            )
        }

        UpdateSection(
            phase = phase,
            actionSpec = updateController.primaryActionFor(phase),
            onAction = updateController::dispatch
        )

        ConnectionSection(
            prefs = connectionPrefs,
            onToggleAutoConnect = onToggleAutoConnect,
            onToggleKeepAlive = onToggleKeepAlive
        )

        Column(verticalArrangement = Arrangement.spacedBy(WireWaveTheme.Metrics.spacingSm)) {
            SectionLabel("PAIRED COMPUTERS")
            SurfaceCard {
                if (trustedDesktops.isEmpty()) {
                    Text(
                        text = "No trusted computers yet",
                        color = WireWaveTheme.Colors.textSecondary,
                        fontSize = WireWaveTheme.Typography.badge,
                        modifier = Modifier.padding(WireWaveTheme.Metrics.spacingMd)
                    )
                } else {
                    trustedDesktops.forEachIndexed { index, desktop ->
                        if (index > 0) {
                            HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)
                        }
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = 12.dp),
                            verticalAlignment = Alignment.CenterVertically,
                            horizontalArrangement = Arrangement.SpaceBetween
                        ) {
                            Text(
                                text = desktop.pcName,
                                color = WireWaveTheme.Colors.textPrimary,
                                fontSize = WireWaveTheme.Typography.body,
                                fontWeight = FontWeight.Medium
                            )
                            Text(
                                text = "Forget",
                                color = WireWaveTheme.Colors.statusErrFg,
                                fontSize = WireWaveTheme.Typography.badge,
                                fontWeight = FontWeight.Medium,
                                modifier = Modifier
                                    .clip(RoundedCornerShape(6.dp))
                                    .clickable(role = Role.Button) { onForgetDesktop(desktop.desktopId) }
                                    .padding(horizontal = 8.dp, vertical = 4.dp)
                            )
                        }
                    }
                }
            }
        }

        AboutSection(snapshot = aboutSnapshot)
    }

    CompatibilityDialog(report = aboutSnapshot.compatibility)
}
