package com.wirewave.features.connection.connected

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.DesktopWindows
import androidx.compose.material.icons.rounded.Settings
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
import com.wirewave.core.transport.TransportKind

@Composable
internal fun ConnectedHomeHeader(
    brandMark: @Composable () -> Unit,
    onOpenSettings: () -> Unit
) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        brandMark()
        Text(
            text = AppConfig.appName,
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = 19.sp,
            fontWeight = FontWeight.Bold
        )
        Box(
            modifier = Modifier
                .size(44.dp)
                .clip(CircleShape)
                .background(WireWaveTheme.Colors.bgCard)
                .border(1.dp, WireWaveTheme.Colors.borderSubtle, CircleShape)
                .clickable(role = Role.Button, onClick = onOpenSettings),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = Icons.Rounded.Settings,
                contentDescription = "Settings",
                tint = WireWaveTheme.Colors.textPrimary,
                modifier = Modifier.size(20.dp)
            )
        }
    }
}

@Composable
internal fun ConnectionSummary(
    pcName: String,
    transport: TransportKind,
    onDisconnect: () -> Unit
) {
    val cardShape = RoundedCornerShape(WireWaveTheme.Metrics.radiusCard)
    val pillShape = RoundedCornerShape(WireWaveTheme.Metrics.radiusPill)

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clip(cardShape)
            .background(WireWaveTheme.Colors.bgCard)
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, cardShape)
            .padding(horizontal = 16.dp, vertical = 14.dp),
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
                modifier = Modifier.size(22.dp)
            )
        }

        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = pcName,
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = 20.sp,
                fontWeight = FontWeight.Bold,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis
            )
            Spacer(modifier = Modifier.height(4.dp))
            Row(verticalAlignment = Alignment.CenterVertically) {
                Box(
                    modifier = Modifier
                        .size(7.dp)
                        .clip(CircleShape)
                        .background(WireWaveTheme.Colors.statusOkFg)
                )
                Spacer(modifier = Modifier.width(6.dp))
                Text(
                    text = "Connected · ${transport.label}",
                    color = WireWaveTheme.Colors.statusOkFg,
                    fontSize = WireWaveTheme.Typography.badge,
                    fontWeight = FontWeight.Medium
                )
            }
        }

        Box(
            modifier = Modifier
                .defaultMinSize(minHeight = 44.dp)
                .clip(pillShape)
                .background(WireWaveTheme.Colors.textPrimary)
                .clickable(role = Role.Button, onClick = onDisconnect)
                .padding(horizontal = 16.dp, vertical = 10.dp),
            contentAlignment = Alignment.Center
        ) {
            Text(
                text = "Disconnect",
                color = WireWaveTheme.Colors.bgCanvas,
                fontSize = WireWaveTheme.Typography.badge,
                fontWeight = FontWeight.SemiBold
            )
        }
    }
}
