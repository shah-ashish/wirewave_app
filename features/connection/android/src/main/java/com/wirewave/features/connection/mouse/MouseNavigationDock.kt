package com.wirewave.features.connection.mouse

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Home
import androidx.compose.material.icons.rounded.Keyboard
import androidx.compose.material.icons.rounded.Tune
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp
import com.wirewave.core.config.WireWaveTheme

/** Floating bottom navigation dock containing Home, Controls, and Keyboard. */
@Composable
internal fun MouseNavigationDock(
    controlsActive: Boolean,
    isKeyboardActive: Boolean,
    onHomeClick: () -> Unit,
    onControlsToggle: () -> Unit,
    onKeyboardToggle: () -> Unit,
    modifier: Modifier = Modifier
) {
    val dockShape = RoundedCornerShape(28.dp)

    Row(
        modifier = modifier
            .fillMaxWidth()
            .height(56.dp)
            .clip(dockShape)
            .background(WireWaveTheme.Colors.bgCard)
            .border(1.dp, WireWaveTheme.Colors.borderSubtle, dockShape)
            .padding(horizontal = 8.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        DockIconButton(
            icon = Icons.Rounded.Home,
            contentDescription = "Home",
            isActive = false,
            onClick = onHomeClick
        )
        DockIconButton(
            icon = Icons.Rounded.Tune,
            contentDescription = "Controls",
            isActive = controlsActive,
            onClick = onControlsToggle
        )
        DockIconButton(
            icon = Icons.Rounded.Keyboard,
            contentDescription = "Keyboard",
            isActive = isKeyboardActive,
            onClick = onKeyboardToggle
        )
    }
}

@Composable
private fun DockIconButton(
    icon: ImageVector,
    contentDescription: String,
    isActive: Boolean,
    onClick: () -> Unit
) {
    val pillShape = RoundedCornerShape(18.dp)
    Box(
        modifier = Modifier
            .width(56.dp)
            .height(44.dp)
            .clip(pillShape)
            .background(if (isActive) WireWaveTheme.Colors.accentBlue else Color.Transparent)
            .clickable(role = Role.Button, onClick = onClick),
        contentAlignment = Alignment.Center
    ) {
        Icon(
            imageVector = icon,
            contentDescription = contentDescription,
            tint = if (isActive) WireWaveTheme.Colors.textPrimary else WireWaveTheme.Colors.textSecondary,
            modifier = Modifier.size(22.dp)
        )
    }
}
