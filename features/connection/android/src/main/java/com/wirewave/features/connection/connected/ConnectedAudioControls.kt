package com.wirewave.features.connection.connected

import androidx.compose.foundation.background
import androidx.compose.foundation.border
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
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Keyboard
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme

@Composable
internal fun ConnectedAudioControls(
    speakerTile: @Composable () -> Unit,
    micTile: @Composable () -> Unit,
    isKeyboardActive: Boolean,
    onToggleKeyboard: () -> Unit
) {
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
            speakerTile()
            micTile()
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                modifier = Modifier
                    .clip(RoundedCornerShape(12.dp))
                    .clickable(role = Role.Button, onClick = onToggleKeyboard)
                    .padding(4.dp)
            ) {
                Box(
                    modifier = Modifier
                        .size(76.dp)
                        .clip(CircleShape)
                        .background(if (isKeyboardActive) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.bgCard)
                        .border(
                            1.dp,
                            if (isKeyboardActive) WireWaveTheme.Colors.accentBlue else WireWaveTheme.Colors.borderSubtle,
                            CircleShape
                        ),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.Rounded.Keyboard,
                        contentDescription = "Keyboard",
                        tint = if (isKeyboardActive) Color.White else WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(28.dp)
                    )
                }
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Keyboard",
                    color = WireWaveTheme.Colors.textPrimary,
                    fontSize = WireWaveTheme.Typography.badge,
                    fontWeight = FontWeight.SemiBold
                )
            }
        }
    }
}
