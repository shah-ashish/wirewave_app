package com.wirewave.features.connection.pairing

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
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.Backspace
import androidx.compose.material.icons.rounded.Bluetooth
import androidx.compose.material.icons.rounded.ChevronLeft
import androidx.compose.material.icons.rounded.DesktopWindows
import androidx.compose.material.icons.rounded.Settings
import androidx.compose.material.icons.rounded.Wifi
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.discovery.DiscoveredHost
import com.wirewave.core.transport.TransportKind

/** Matches 2b - Pairing code.png with 6-box code display and custom numeric keypad. */
@Composable
fun PairingCodeScreen(
    host: DiscoveredHost,
    errorMessage: String?,
    onSubmitCode: (String) -> Unit,
    onBack: () -> Unit,
    onOpenSettings: () -> Unit,
    modifier: Modifier = Modifier
) {
    var code by rememberSaveable(host.desktopId) { mutableStateOf("") }
    val transportIcon = when (host.transport) {
        TransportKind.WIFI -> Icons.Rounded.Wifi
        TransportKind.BLUETOOTH -> Icons.Rounded.Bluetooth
    }

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(WireWaveTheme.Colors.bgCanvas)
            .statusBarsPadding()
            .navigationBarsPadding()
            .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = WireWaveTheme.Metrics.spacingSm),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
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
                Text(
                    text = "Back",
                    color = WireWaveTheme.Colors.accentBlue,
                    fontSize = WireWaveTheme.Typography.title
                )
            }
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

        Spacer(modifier = Modifier.height(14.dp))
        Box(
            modifier = Modifier
                .size(60.dp)
                .clip(RoundedCornerShape(16.dp))
                .background(WireWaveTheme.Colors.bgCard),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = Icons.Rounded.DesktopWindows,
                contentDescription = null,
                tint = WireWaveTheme.Colors.textSecondary,
                modifier = Modifier.size(26.dp)
            )
        }
        Spacer(modifier = Modifier.height(16.dp))
        Text(
            text = "Enter pairing code",
            color = WireWaveTheme.Colors.textPrimary,
            fontSize = 24.sp,
            fontWeight = FontWeight.Bold
        )
        Spacer(modifier = Modifier.height(6.dp))
        Text(
            text = "Type the code shown on ${host.pcName} to connect.",
            color = WireWaveTheme.Colors.textSecondary,
            fontSize = WireWaveTheme.Typography.badge,
            textAlign = TextAlign.Center
        )
        Spacer(modifier = Modifier.height(10.dp))
        Row(
            modifier = Modifier
                .clip(RoundedCornerShape(WireWaveTheme.Metrics.radiusPill))
                .background(WireWaveTheme.Colors.bgCard)
                .padding(horizontal = 12.dp, vertical = 5.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = transportIcon,
                contentDescription = null,
                tint = WireWaveTheme.Colors.textSecondary,
                modifier = Modifier.size(13.dp)
            )
            Spacer(modifier = Modifier.width(6.dp))
            Text(
                text = host.transport.label,
                color = WireWaveTheme.Colors.textSecondary,
                fontSize = WireWaveTheme.Typography.badge
            )
        }

        Spacer(modifier = Modifier.height(20.dp))
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            repeat(6) { idx ->
                val digit = code.getOrNull(idx)?.toString().orEmpty()
                Box(
                    modifier = Modifier
                        .weight(1f)
                        .height(54.dp)
                        .clip(RoundedCornerShape(12.dp))
                        .background(WireWaveTheme.Colors.bgCard)
                        .border(1.dp, WireWaveTheme.Colors.borderSubtle, RoundedCornerShape(12.dp)),
                    contentAlignment = Alignment.Center
                ) {
                    Text(
                        text = digit,
                        color = WireWaveTheme.Colors.textPrimary,
                        fontSize = 22.sp,
                        fontWeight = FontWeight.Bold
                    )
                }
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

        Spacer(modifier = Modifier.height(20.dp))
        Button(
            onClick = { onSubmitCode(code) },
            enabled = code.length == 6,
            shape = RoundedCornerShape(14.dp),
            colors = ButtonDefaults.buttonColors(containerColor = WireWaveTheme.Colors.accentBlue),
            modifier = Modifier
                .fillMaxWidth()
                .height(50.dp)
        ) {
            Text(
                text = "Connect",
                color = WireWaveTheme.Colors.textPrimary,
                fontSize = WireWaveTheme.Typography.body,
                fontWeight = FontWeight.SemiBold
            )
        }

        Spacer(modifier = Modifier.weight(1f))

        NumericKeypad(
            onDigit = { digit -> if (code.length < 6) code += digit },
            onBackspace = { if (code.isNotEmpty()) code = code.dropLast(1) }
        )
    }
}

@Composable
private fun NumericKeypad(
    onDigit: (String) -> Unit,
    onBackspace: () -> Unit
) {
    val rows = listOf(
        listOf("1", "2", "3"),
        listOf("4", "5", "6"),
        listOf("7", "8", "9"),
        listOf("", "0", "DEL")
    )

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        rows.forEach { row ->
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(10.dp)
            ) {
                row.forEach { key ->
                    when (key) {
                        "" -> Spacer(modifier = Modifier.weight(1f).height(52.dp))
                        "DEL" -> Box(
                            modifier = Modifier
                                .weight(1f)
                                .height(52.dp)
                                .clip(RoundedCornerShape(12.dp))
                                .clickable(role = Role.Button, onClick = onBackspace),
                            contentAlignment = Alignment.Center
                        ) {
                            Icon(
                                imageVector = Icons.AutoMirrored.Rounded.Backspace,
                                contentDescription = "Delete",
                                tint = WireWaveTheme.Colors.textPrimary,
                                modifier = Modifier.size(22.dp)
                            )
                        }
                        else -> Box(
                            modifier = Modifier
                                .weight(1f)
                                .height(52.dp)
                                .clip(RoundedCornerShape(12.dp))
                                .background(WireWaveTheme.Colors.bgCard)
                                .clickable(role = Role.Button) { onDigit(key) },
                            contentAlignment = Alignment.Center
                        ) {
                            Text(
                                text = key,
                                color = WireWaveTheme.Colors.textPrimary,
                                fontSize = 22.sp,
                                fontWeight = FontWeight.Medium
                            )
                        }
                    }
                }
            }
        }
    }
}
