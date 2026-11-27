package com.wirewave.features.connection.connected

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Settings
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
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
                .size(40.dp)
                .clip(CircleShape)
                .background(WireWaveTheme.Colors.bgCard)
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
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Column {
            Text(
                text = pcName,
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = 26.sp,
                fontWeight = FontWeight.Bold
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
                    text = "Connected on ${transport.label}",
                    color = WireWaveTheme.Colors.textSecondary,
                    fontSize = WireWaveTheme.Typography.badge
                )
            }
        }
        Text(
            text = "Disconnect",
            color = WireWaveTheme.Colors.statusErrFg,
            fontSize = WireWaveTheme.Typography.body,
            fontWeight = FontWeight.Medium,
            modifier = Modifier
                .clickable(role = Role.Button, onClick = onDisconnect)
                .padding(6.dp)
        )
    }
}
