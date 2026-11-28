package com.wirewave.features.connection.discovery

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Bluetooth
import androidx.compose.material.icons.rounded.DesktopWindows
import androidx.compose.material.icons.rounded.ErrorOutline
import androidx.compose.material.icons.rounded.Settings
import androidx.compose.material.icons.rounded.Wifi
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.AppConfig
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.discovery.DiscoveredHost
import com.wirewave.core.transport.TransportKind
import com.wirewave.core.ui.CircleIconButton

@Composable
fun DiscoveryScreen(
    uiState: DiscoveryUiState,
    errorMessage: String?,
    onScan: () -> Unit,
    onConnect: (DiscoveredHost) -> Unit,
    onOpenSettings: () -> Unit,
    brandMark: @Composable () -> Unit,
    modifier: Modifier = Modifier,
    connectingMessage: String? = null
) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .background(WireWaveTheme.Colors.bgCanvas)
            .statusBarsPadding()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = WireWaveTheme.Metrics.spacingSm)
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            brandMark()
            CircleIconButton(
                icon = Icons.Rounded.Settings,
                contentDescription = "Settings",
                onClick = onOpenSettings,
                iconTint = WireWaveTheme.Colors.textPrimary
            )
        }

        if (connectingMessage != null) {
            Spacer(modifier = Modifier.height(12.dp))
            DiscoveryStatusBanner(message = connectingMessage, isConnecting = true)
        } else if (errorMessage != null) {
            Spacer(modifier = Modifier.height(12.dp))
            DiscoveryStatusBanner(message = errorMessage, isConnecting = false)
        }

        when {
            uiState.hosts.isEmpty() -> EmptyComputersView(
                isScanning = uiState.isScanning,
                onScan = onScan,
                modifier = Modifier.weight(1f)
            )
            else -> NearbyComputersView(
                hosts = uiState.hosts,
                isScanning = uiState.isScanning,
                onScan = onScan,
                onConnect = onConnect
            )
        }
    }
}

@Composable
private fun DiscoveryStatusBanner(message: String, isConnecting: Boolean) {
    val shape = RoundedCornerShape(12.dp)
    val foreground = if (isConnecting) WireWaveTheme.Colors.statusOkFg else WireWaveTheme.Colors.statusErrFg
    val background = if (isConnecting) WireWaveTheme.Colors.statusOkBg else WireWaveTheme.Colors.statusErrBg
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clip(shape)
            .background(background)
            .border(1.dp, foreground.copy(alpha = 0.45f), shape)
            .padding(horizontal = 14.dp, vertical = 12.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(10.dp)
    ) {
        if (isConnecting) {
            CircularProgressIndicator(
                modifier = Modifier.size(18.dp),
                color = foreground,
                strokeWidth = 2.dp
            )
        } else {
            Icon(
                imageVector = Icons.Rounded.ErrorOutline,
                contentDescription = null,
                tint = foreground,
                modifier = Modifier.size(18.dp)
            )
        }
        Text(
            text = message,
            color = foreground,
            fontSize = WireWaveTheme.Typography.badge,
            fontWeight = FontWeight.Medium
        )
    }
}

@Composable
private fun EmptyComputersView(
    isScanning: Boolean,
    onScan: () -> Unit,
    modifier: Modifier = Modifier
) {
    val cardShape = RoundedCornerShape(WireWaveTheme.Metrics.radiusCard)
    Box(
        modifier = modifier.fillMaxWidth(),
        contentAlignment = Alignment.Center
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .clip(cardShape)
                .background(WireWaveTheme.Colors.bgCard)
                .border(1.dp, WireWaveTheme.Colors.borderSubtle, cardShape)
                .padding(22.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(14.dp)
            ) {
                Box(
                    modifier = Modifier
                        .size(48.dp)
                        .clip(RoundedCornerShape(14.dp))
                        .background(WireWaveTheme.Colors.bgCardHover)
                        .border(1.dp, WireWaveTheme.Colors.borderSubtle, RoundedCornerShape(14.dp)),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.Rounded.DesktopWindows,
                        contentDescription = null,
                        tint = WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(24.dp)
                    )
                }
                Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
                    Text(
                        text = "No computers found",
                        color = WireWaveTheme.Colors.textPrimary,
                        fontSize = 20.sp,
                        fontWeight = FontWeight.Bold
                    )
                    Text(
                        text = "Looking on local network and Bluetooth",
                        color = WireWaveTheme.Colors.textSecondary,
                        fontSize = WireWaveTheme.Typography.badge
                    )
                }
            }

            HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)

            Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                ChecklistRow("1", "Connect phone and PC to the same Wi-Fi network or pair Bluetooth.")
                ChecklistRow("2", "Make sure ${AppConfig.appName} is open and ready on your Windows PC.")
            }

            Button(
                onClick = onScan,
                enabled = !isScanning,
                shape = RoundedCornerShape(14.dp),
                colors = ButtonDefaults.buttonColors(
                    containerColor = WireWaveTheme.Colors.textPrimary,
                    contentColor = WireWaveTheme.Colors.bgCanvas,
                    disabledContainerColor = WireWaveTheme.Colors.bgCardHover,
                    disabledContentColor = WireWaveTheme.Colors.textSecondary
                ),
                modifier = Modifier
                    .fillMaxWidth()
                    .height(48.dp)
            ) {
                Text(
                    text = if (isScanning) "Scanning..." else "Scan again",
                    fontSize = WireWaveTheme.Typography.body,
                    fontWeight = FontWeight.SemiBold
                )
            }
        }
    }
}

@Composable
private fun ChecklistRow(step: String, text: String) {
    Row(
        verticalAlignment = Alignment.Top,
        horizontalArrangement = Arrangement.spacedBy(10.dp)
    ) {
        Box(
            modifier = Modifier
                .size(22.dp)
                .clip(CircleShape)
                .background(WireWaveTheme.Colors.bgSidebar)
                .border(1.dp, WireWaveTheme.Colors.borderSubtle, CircleShape),
            contentAlignment = Alignment.Center
        ) {
            Text(
                text = step,
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = 11.sp,
                fontWeight = FontWeight.SemiBold
            )
        }
        Text(
            text = text,
            color = WireWaveTheme.Colors.textSecondary,
            fontSize = WireWaveTheme.Typography.badge,
            modifier = Modifier.weight(1f)
        )
    }
}

@Composable
private fun NearbyComputersView(
    hosts: List<DiscoveredHost>,
    isScanning: Boolean,
    onScan: () -> Unit,
    onConnect: (DiscoveredHost) -> Unit
) {
    Spacer(modifier = Modifier.height(18.dp))
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = "Nearby computers",
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = 24.sp,
                fontWeight = FontWeight.Bold
            )
            Spacer(modifier = Modifier.height(2.dp))
            Text(
                text = "${hosts.size} ${if (hosts.size == 1) "computer" else "computers"} found",
                color = WireWaveTheme.Colors.textSecondary,
                fontSize = WireWaveTheme.Typography.badge
            )
        }
        Box(
            modifier = Modifier
                .defaultMinSize(minHeight = 44.dp)
                .clip(RoundedCornerShape(WireWaveTheme.Metrics.radiusPill))
                .background(WireWaveTheme.Colors.bgCard)
                .border(1.dp, WireWaveTheme.Colors.borderSubtle, RoundedCornerShape(WireWaveTheme.Metrics.radiusPill))
                .clickable(enabled = !isScanning, role = Role.Button, onClick = onScan)
                .padding(horizontal = 14.dp, vertical = 10.dp),
            contentAlignment = Alignment.Center
        ) {
            Text(
                text = if (isScanning) "Scanning..." else "Scan",
                color = if (isScanning) WireWaveTheme.Colors.textSecondary else WireWaveTheme.Colors.accentBlue,
                fontSize = WireWaveTheme.Typography.badge,
                fontWeight = FontWeight.SemiBold
            )
        }
    }
    Spacer(modifier = Modifier.height(16.dp))

    val shape = RoundedCornerShape(WireWaveTheme.Metrics.radiusCard)
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .clip(shape)
            .background(WireWaveTheme.Colors.bgCard)
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, shape)
    ) {
        hosts.forEachIndexed { idx, host ->
            if (idx > 0) {
                HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)
            }
            HostRow(host = host, onConnect = { onConnect(host) })
        }
    }
}

@Composable
private fun HostRow(
    host: DiscoveredHost,
    onConnect: () -> Unit
) {
    val icon = when (host.transport) {
        TransportKind.WIFI -> Icons.Rounded.Wifi
        TransportKind.BLUETOOTH -> Icons.Rounded.Bluetooth
    }
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 14.dp, vertical = 14.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Box(
            modifier = Modifier
                .size(44.dp)
                .clip(RoundedCornerShape(12.dp))
                .background(WireWaveTheme.Colors.bgCardHover)
                .border(1.dp, WireWaveTheme.Colors.borderSubtle, RoundedCornerShape(12.dp)),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = Icons.Rounded.DesktopWindows,
                contentDescription = null,
                tint = WireWaveTheme.Colors.textPrimary,
                modifier = Modifier.size(20.dp)
            )
        }

        Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(3.dp)) {
            Text(
                text = host.pcName,
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = WireWaveTheme.Typography.cardTitle,
                fontWeight = FontWeight.SemiBold,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis
            )
            Row(verticalAlignment = Alignment.CenterVertically) {
                Box(
                    modifier = Modifier
                        .size(7.dp)
                        .clip(CircleShape)
                        .background(
                            if (host.busy) WireWaveTheme.Colors.statusErrFg
                            else WireWaveTheme.Colors.statusOkFg
                        )
                )
                Spacer(modifier = Modifier.width(6.dp))
                Icon(
                    imageVector = icon,
                    contentDescription = null,
                    tint = WireWaveTheme.Colors.textSecondary,
                    modifier = Modifier.size(13.dp)
                )
                Spacer(modifier = Modifier.width(4.dp))
                Text(
                    text = if (host.busy) "Busy" else "${host.transport.label} · Ready",
                    color = WireWaveTheme.Colors.textSecondary,
                    fontSize = WireWaveTheme.Typography.badge
                )
            }
        }

        Button(
            onClick = onConnect,
            enabled = !host.busy,
            shape = RoundedCornerShape(WireWaveTheme.Metrics.radiusPill),
            contentPadding = PaddingValues(horizontal = 16.dp, vertical = 10.dp),
            colors = ButtonDefaults.buttonColors(
                containerColor = WireWaveTheme.Colors.textPrimary,
                contentColor = WireWaveTheme.Colors.bgCanvas,
                disabledContainerColor = WireWaveTheme.Colors.bgCardHover,
                disabledContentColor = WireWaveTheme.Colors.textSecondary
            ),
            modifier = Modifier.defaultMinSize(minHeight = 44.dp)
        ) {
            Text(
                text = if (host.busy) "Busy" else "Connect",
                fontSize = WireWaveTheme.Typography.badge,
                fontWeight = FontWeight.SemiBold
            )
        }
    }
}
