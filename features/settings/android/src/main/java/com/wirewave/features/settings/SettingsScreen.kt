package com.wirewave.features.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.ChevronLeft
import androidx.compose.material.icons.rounded.DesktopWindows
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
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.session.ConnectionPrefs
import com.wirewave.core.session.MousePrefs
import com.wirewave.core.session.TrustedDesktop
import com.wirewave.features.settings.about.AboutSection
import com.wirewave.features.settings.about.AboutSnapshot
import com.wirewave.features.settings.components.SectionLabel
import com.wirewave.features.settings.components.SurfaceCard
import com.wirewave.features.settings.connection.ConnectionSection
import com.wirewave.features.settings.mouse.MouseSettingsSection
import com.wirewave.features.settings.update.UpdateController
import com.wirewave.features.settings.update.UpdateSection

@Composable
fun SettingsScreen(
    aboutSnapshot: AboutSnapshot,
    updateController: UpdateController,
    connectionPrefs: ConnectionPrefs,
    mousePrefs: MousePrefs = MousePrefs(),
    trustedDesktops: List<TrustedDesktop> = emptyList(),
    connectedDesktopId: String? = null,
    onToggleAutoConnect: (Boolean) -> Unit,
    onToggleKeepAlive: (Boolean) -> Unit,
    onPointerSpeedChange: (Float) -> Unit = {},
    onScrollSpeedChange: (Float) -> Unit = {},
    onToggleNaturalScroll: (Boolean) -> Unit = {},
    onToggleShowMouseButtons: (Boolean) -> Unit = {},
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
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(10.dp)
        ) {
            Row(
                modifier = Modifier
                    .defaultMinSize(minHeight = 44.dp)
                    .clip(RoundedCornerShape(10.dp))
                    .clickable(role = Role.Button, onClick = onBack)
                    .padding(end = 8.dp),
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
            Text(
                text = "Settings",
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = 22.sp,
                fontWeight = FontWeight.Bold
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

        MouseSettingsSection(
            prefs = mousePrefs,
            onPointerSpeedChange = onPointerSpeedChange,
            onScrollSpeedChange = onScrollSpeedChange,
            onToggleNaturalScroll = onToggleNaturalScroll,
            onToggleShowMouseButtons = onToggleShowMouseButtons
        )

        PairedComputersSection(
            trustedDesktops = trustedDesktops,
            connectedDesktopId = connectedDesktopId,
            onForgetDesktop = onForgetDesktop
        )

        AboutSection(snapshot = aboutSnapshot)

        Spacer(
            modifier = Modifier
                .navigationBarsPadding()
                .height(WireWaveTheme.Metrics.spacingMd)
        )
    }

    CompatibilityDialog(report = aboutSnapshot.compatibility)
}

@Composable
private fun PairedComputersSection(
    trustedDesktops: List<TrustedDesktop>,
    connectedDesktopId: String?,
    onForgetDesktop: (String) -> Unit
) {
    Column(verticalArrangement = Arrangement.spacedBy(WireWaveTheme.Metrics.spacingSm)) {
        SectionLabel("PAIRED COMPUTERS")
        SurfaceCard {
            if (trustedDesktops.isEmpty()) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = 16.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(12.dp)
                ) {
                    DesktopBadge(connected = false)
                    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
                        Text(
                            text = "No paired computers yet",
                            color = WireWaveTheme.Colors.textPrimary,
                            fontSize = WireWaveTheme.Typography.body,
                            fontWeight = FontWeight.Medium
                        )
                        Text(
                            text = "Connect to a PC to manage trusted access here.",
                            color = WireWaveTheme.Colors.textSecondary,
                            fontSize = WireWaveTheme.Typography.badge
                        )
                    }
                }
            } else {
                trustedDesktops.forEachIndexed { index, desktop ->
                    if (index > 0) {
                        HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)
                    }
                    val isConnected = connectedDesktopId != null && desktop.desktopId == connectedDesktopId
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = 14.dp),
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(12.dp)
                    ) {
                        DesktopBadge(connected = isConnected)
                        Column(
                            modifier = Modifier.weight(1f),
                            verticalArrangement = Arrangement.spacedBy(3.dp)
                        ) {
                            Text(
                                text = desktop.pcName,
                                color = WireWaveTheme.Colors.textPrimary,
                                fontSize = WireWaveTheme.Typography.body,
                                fontWeight = FontWeight.SemiBold,
                                maxLines = 1,
                                overflow = TextOverflow.Ellipsis
                            )
                            Row(verticalAlignment = Alignment.CenterVertically) {
                                if (isConnected) {
                                    Box(
                                        modifier = Modifier
                                            .size(6.dp)
                                            .clip(CircleShape)
                                            .background(WireWaveTheme.Colors.statusOkFg)
                                    )
                                    Spacer(modifier = Modifier.width(6.dp))
                                }
                                Text(
                                    text = if (isConnected) "Connected" else "Trusted · Offline",
                                    color = if (isConnected) {
                                        WireWaveTheme.Colors.statusOkFg
                                    } else {
                                        WireWaveTheme.Colors.textSecondary
                                    },
                                    fontSize = WireWaveTheme.Typography.badge
                                )
                            }
                        }
                        Box(
                            modifier = Modifier
                                .defaultMinSize(minHeight = 40.dp)
                                .clip(RoundedCornerShape(8.dp))
                                .background(WireWaveTheme.Colors.bgCardHover)
                                .border(1.dp, WireWaveTheme.Colors.borderSubtle, RoundedCornerShape(8.dp))
                                .clickable(role = Role.Button) { onForgetDesktop(desktop.desktopId) }
                                .padding(horizontal = 14.dp, vertical = 9.dp),
                            contentAlignment = Alignment.Center
                        ) {
                            Text(
                                text = "Forget",
                                color = WireWaveTheme.Colors.statusErrFg,
                                fontSize = WireWaveTheme.Typography.badge,
                                fontWeight = FontWeight.Medium
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun DesktopBadge(connected: Boolean) {
    val tint = if (connected) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.textSecondary
    val shape = RoundedCornerShape(10.dp)
    Box(
        modifier = Modifier
            .size(40.dp)
            .clip(shape)
            .background(WireWaveTheme.Colors.bgCardHover)
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, shape),
        contentAlignment = Alignment.Center
    ) {
        Icon(
            imageVector = Icons.Rounded.DesktopWindows,
            contentDescription = null,
            tint = tint,
            modifier = Modifier.size(18.dp)
        )
    }
}
