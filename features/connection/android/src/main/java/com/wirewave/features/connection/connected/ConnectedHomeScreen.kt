package com.wirewave.features.connection.connected

import android.widget.Toast
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
import androidx.compose.material.icons.rounded.Keyboard
import androidx.compose.material.icons.rounded.Mouse
import androidx.compose.material.icons.rounded.PhotoCamera
import androidx.compose.material.icons.rounded.Tune
import androidx.compose.material.icons.rounded.VolumeUp
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.AppConfig
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.transport.TransportKind
import com.wirewave.features.speaker.SpeakerCircleTile
import com.wirewave.features.mic.MicCircleTile

private data class ServiceCircleSpec(val label: String, val icon: ImageVector)
private data class UsePhoneRowSpec(
    val title: String,
    val subtitle: String,
    val icon: ImageVector,
    val iconBg: Color
)

/**
 * Android Connected Home screen matching 3 - Connected.png.
 * All 5 hardware services are UI-only placeholders carrying a `Coming soon` badge until Stage 3.
 */
@Composable
fun ConnectedHomeScreen(
    pcName: String,
    transport: TransportKind,
    onDisconnect: () -> Unit,
    onOpenSettings: () -> Unit,
    brandMark: @Composable () -> Unit,
    isSpeakerOn: Boolean = false,
    isSpeakerBlocked: Boolean = false,
    onToggleSpeaker: () -> Unit = {},
    isMicOn: Boolean = false,
    isMicBlocked: Boolean = true,
    micBlockedDetail: String? = null,
    onToggleMic: () -> Unit = {},
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    val remainingCircles = listOf(
        ServiceCircleSpec("Keyboard", Icons.Rounded.Keyboard)
    )
    val rows = listOf(
        UsePhoneRowSpec("Mouse", "Move the cursor, scroll, click", Icons.Rounded.Mouse, WireWaveTheme.Colors.accentBlue),
        UsePhoneRowSpec("Camera", "Live stream, photos, recording", Icons.Rounded.PhotoCamera, WireWaveTheme.Colors.statusOkFg)
    )

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(WireWaveTheme.Colors.bgCanvas)
            .statusBarsPadding()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = WireWaveTheme.Metrics.spacingSm),
        verticalArrangement = Arrangement.spacedBy(22.dp)
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
                    imageVector = Icons.Rounded.Tune,
                    contentDescription = "Settings",
                    tint = WireWaveTheme.Colors.textPrimary,
                    modifier = Modifier.size(20.dp)
                )
            }
        }

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

        Column(verticalArrangement = Arrangement.spacedBy(14.dp)) {
            Text(
                text = "Audio & input",
                color = WireWaveTheme.Colors.textSection,
                fontSize = WireWaveTheme.Typography.section,
                fontWeight = FontWeight.Medium
            )
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceEvenly
            ) {
                SpeakerCircleTile(
                    isOn = isSpeakerOn,
                    isBlocked = isSpeakerBlocked,
                    onToggle = onToggleSpeaker
                )
                MicCircleTile(
                    isOn = isMicOn,
                    isBlocked = isMicBlocked,
                    blockedDetail = micBlockedDetail,
                    onToggle = onToggleMic
                )
                remainingCircles.forEach { spec ->
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally,
                        modifier = Modifier
                            .clip(RoundedCornerShape(12.dp))
                            .clickable(role = Role.Button) {
                                Toast.makeText(context, "${spec.label} will be available in Stage 3", Toast.LENGTH_SHORT).show()
                            }
                            .padding(4.dp)
                    ) {
                        Box(
                            modifier = Modifier
                                .size(76.dp)
                                .clip(CircleShape)
                                .background(WireWaveTheme.Colors.bgCard)
                                .border(1.dp, WireWaveTheme.Colors.borderSubtle, CircleShape),
                            contentAlignment = Alignment.Center
                        ) {
                            Icon(
                                imageVector = spec.icon,
                                contentDescription = spec.label,
                                tint = WireWaveTheme.Colors.textSecondary,
                                modifier = Modifier.size(28.dp)
                            )
                        }
                        Spacer(modifier = Modifier.height(8.dp))
                        Text(
                            text = spec.label,
                            color = WireWaveTheme.Colors.textPrimary,
                            fontSize = WireWaveTheme.Typography.badge,
                            fontWeight = FontWeight.SemiBold
                        )
                        Spacer(modifier = Modifier.height(4.dp))
                        ComingSoonBadge()
                    }
                }
            }
        }

        Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
            Text(
                text = "Use phone as",
                color = WireWaveTheme.Colors.textSection,
                fontSize = WireWaveTheme.Typography.section,
                fontWeight = FontWeight.Medium
            )
            val shape = RoundedCornerShape(WireWaveTheme.Metrics.radiusCard)
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .clip(shape)
                    .background(WireWaveTheme.Colors.bgCard)
                    .border(1.dp, WireWaveTheme.Colors.borderSubtle, shape)
            ) {
                rows.forEachIndexed { idx, row ->
                    if (idx > 0) {
                        HorizontalDivider(thickness = 1.dp, color = WireWaveTheme.Colors.divider)
                    }
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .clickable(role = Role.Button) {
                                Toast.makeText(context, "${row.title} will be available in Stage 3", Toast.LENGTH_SHORT).show()
                            }
                            .padding(horizontal = 16.dp, vertical = 14.dp),
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            Box(
                                modifier = Modifier
                                    .size(40.dp)
                                    .clip(RoundedCornerShape(10.dp))
                                    .background(row.iconBg),
                                contentAlignment = Alignment.Center
                            ) {
                                Icon(
                                    imageVector = row.icon,
                                    contentDescription = null,
                                    tint = WireWaveTheme.Colors.textPrimary,
                                    modifier = Modifier.size(20.dp)
                                )
                            }
                            Spacer(modifier = Modifier.width(12.dp))
                            Column {
                                Text(
                                    text = row.title,
                                    color = WireWaveTheme.Colors.textPrimary,
                                    fontSize = WireWaveTheme.Typography.cardTitle,
                                    fontWeight = FontWeight.SemiBold
                                )
                                Text(
                                    text = row.subtitle,
                                    color = WireWaveTheme.Colors.textSecondary,
                                    fontSize = WireWaveTheme.Typography.badge
                                )
                            }
                        }
                        ComingSoonBadge()
                    }
                }
            }
        }
    }
}

@Composable
private fun ComingSoonBadge() {
    Box(
        modifier = Modifier
            .clip(RoundedCornerShape(WireWaveTheme.Metrics.radiusPill))
            .background(WireWaveTheme.Colors.bgCardHover)
            .padding(horizontal = 8.dp, vertical = 3.dp)
    ) {
        Text(
            text = "Coming soon",
            color = WireWaveTheme.Colors.textSecondary,
            fontSize = 10.sp,
            fontWeight = FontWeight.Medium
        )
    }
}
