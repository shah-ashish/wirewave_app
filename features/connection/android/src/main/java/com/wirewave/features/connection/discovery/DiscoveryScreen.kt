package com.wirewave.features.connection.discovery

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
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
import androidx.compose.material.icons.rounded.Tune
import androidx.compose.material.icons.rounded.Wifi
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.AppConfig
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.discovery.DiscoveredHost
import com.wirewave.core.transport.TransportKind

@Composable
fun DiscoveryScreen(
    uiState: DiscoveryUiState,
    errorMessage: String?,
    onScan: () -> Unit,
    onConnect: (DiscoveredHost) -> Unit,
    onOpenSettings: () -> Unit,
    brandMark: @Composable () -> Unit,
    modifier: Modifier = Modifier
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
            Box(
                modifier = Modifier
                    .size(40.dp)
                    .clip(CircleShape)
                    .background(WireWaveTheme.Colors.bgCard)
                    .clickable(role = Role.Button, onClick = onOpenSettings),
                contentAlignment = Alignment.Center
            ) {
                Icon(
                    imageVector = Icons.Rounded.Tune,
                    contentDescription = "Settings",
                    tint = WireWaveTheme.Colors.textPrimary,
                    modifier = Modifier.size(20.dp)
                )
            }
        }

        if (errorMessage != null) {
            Spacer(modifier = Modifier.height(10.dp))
            Text(
                text = errorMessage,
                color = WireWaveTheme.Colors.statusErrFg,
                fontSize = WireWaveTheme.Typography.badge
            )
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
private fun EmptyComputersView(
    isScanning: Boolean,
    onScan: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Box(
            modifier = Modifier
                .size(64.dp)
                .clip(RoundedCornerShape(18.dp))
                .background(WireWaveTheme.Colors.bgCard),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = Icons.Rounded.DesktopWindows,
                contentDescription = null,
                tint = WireWaveTheme.Colors.textSecondary,
                modifier = Modifier.size(28.dp)
            )
        }
        Spacer(modifier = Modifier.height(20.dp))
        Text(
            text = "No computers found",
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = 22.sp,
            fontWeight = FontWeight.Bold
        )
        Spacer(modifier = Modifier.height(10.dp))
        Text(
            text = "Make sure your phone and PC are on the same Wi-Fi network, and that ${AppConfig.appName} is running on your PC.",
            color = WireWaveTheme.Colors.textSecondary,
            fontSize = WireWaveTheme.Typography.body,
            textAlign = TextAlign.Center,
            modifier = Modifier.padding(horizontal = 24.dp)
        )
        Spacer(modifier = Modifier.height(28.dp))
        Button(
            onClick = onScan,
            enabled = !isScanning,
            shape = RoundedCornerShape(14.dp),
            colors = ButtonDefaults.buttonColors(containerColor = WireWaveTheme.Colors.accentBlue),
            modifier = Modifier
                .fillMaxWidth(0.72f)
                .height(48.dp)
        ) {
            Text(
                text = if (isScanning) "Scanning..." else "Scan again",
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = WireWaveTheme.Typography.body,
                fontWeight = FontWeight.Medium
            )
        }
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
        Text(
            text = "Nearby computers",
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = 26.sp,
            fontWeight = FontWeight.Bold
        )
        Text(
            text = if (isScanning) "Scanning..." else "Scan",
            color = WireWaveTheme.Colors.accentBlue,
            fontSize = WireWaveTheme.Typography.body,
            modifier = Modifier
                .clickable(enabled = !isScanning, role = Role.Button, onClick = onScan)
                .padding(6.dp)
        )
    }
    Spacer(modifier = Modifier.height(4.dp))
    Text(
        text = "${hosts.size} ${if (hosts.size == 1) "computer" else "computers"} found",
        color = WireWaveTheme.Colors.textSecondary,
        fontSize = WireWaveTheme.Typography.badge
    )
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
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Box(
                modifier = Modifier
                    .size(40.dp)
                    .clip(RoundedCornerShape(10.dp))
                    .background(WireWaveTheme.Colors.bgCardHover),
                contentAlignment = Alignment.Center
            ) {
                Icon(
                    imageVector = Icons.Rounded.DesktopWindows,
                    contentDescription = null,
                    tint = WireWaveTheme.Colors.textPrimary,
                    modifier = Modifier.size(20.dp)
                )
            }
            Spacer(modifier = Modifier.width(12.dp))
            Column {
                Text(
                    text = host.pcName,
                    color = WireWaveTheme.Colors.textPrimary,
                    fontSize = WireWaveTheme.Typography.cardTitle,
                    fontWeight = FontWeight.Medium
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
                        text = if (host.busy) "Busy" else "Ready on ${host.transport.label}",
                        color = WireWaveTheme.Colors.textSecondary,
                        fontSize = WireWaveTheme.Typography.badge
                    )
                }
            }
        }

        Button(
            onClick = onConnect,
            enabled = !host.busy,
            shape = RoundedCornerShape(WireWaveTheme.Metrics.radiusPill),
            colors = ButtonDefaults.buttonColors(containerColor = WireWaveTheme.Colors.accentBlue)
        ) {
            Text(
                text = "Connect",
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = WireWaveTheme.Typography.badge,
                fontWeight = FontWeight.Medium
            )
        }
    }
}
