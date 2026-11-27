package com.wirewave.core.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.KeyboardArrowRight
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme

/** Presentation and navigation action for a feature entry. */
data class FeatureNavigationItem(
    val title: String,
    val subtitle: String,
    val icon: ImageVector,
    val iconBackground: Color,
    val onClick: () -> Unit
)

/** Renders feature entries without knowing their destinations or business logic. */
@Composable
fun FeatureNavigationList(
    title: String,
    rows: List<FeatureNavigationItem>,
    modifier: Modifier = Modifier
) {
    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(10.dp)) {
        Text(
            text = title,
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
                        .clickable(role = Role.Button, onClick = row.onClick)
                        .padding(horizontal = 16.dp, vertical = 14.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Box(
                            modifier = Modifier
                                .size(40.dp)
                                .clip(RoundedCornerShape(10.dp))
                                .background(row.iconBackground),
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
                    Icon(
                        imageVector = Icons.AutoMirrored.Rounded.KeyboardArrowRight,
                        contentDescription = "Open ${row.title}",
                        tint = WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(20.dp)
                    )
                }
            }
        }
    }
}
