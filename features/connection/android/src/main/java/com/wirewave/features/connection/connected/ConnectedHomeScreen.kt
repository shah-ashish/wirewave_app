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
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.ime
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.KeyboardArrowRight
import androidx.compose.material.icons.rounded.Keyboard
import androidx.compose.material.icons.rounded.Mouse
import androidx.compose.material.icons.rounded.Settings
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.AppConfig
import com.wirewave.core.config.WireWaveTheme
import com.wirewave.core.transport.TransportKind
import com.wirewave.features.connection.overlay.ScreenOverlay
import com.wirewave.features.connection.overlay.ScreenOverlayHost

private data class ServiceCircleSpec(val label: String, val icon: ImageVector)
private data class UsePhoneRowSpec(
    val title: String,
    val subtitle: String,
    val icon: ImageVector,
    val iconBg: Color
)

@Composable
fun ConnectedHomeScreen(
    pcName: String,
    transport: TransportKind,
    onDisconnect: () -> Unit,
    onOpenSettings: () -> Unit,
    brandMark: @Composable () -> Unit,
    speakerTile: @Composable () -> Unit = {},
    micTile: @Composable () -> Unit = {},
    onNavigateToMouse: () -> Unit = {},
    onSendKeyInput: (text: String?, keyCode: String?) -> Unit = { _, _ -> },
    activeOverlay: ScreenOverlay = ScreenOverlay.NONE,
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    val keyboardController = LocalSoftwareKeyboardController.current
    val focusRequester = remember { FocusRequester() }
    val density = LocalDensity.current
    val imeBottom = WindowInsets.ime.getBottom(density)
    val isKeyboardVisible = imeBottom > 0

    val sentinel = "  "
    var textValue by remember {
        mutableStateOf(TextFieldValue(sentinel, TextRange(sentinel.length)))
    }

    val rows = listOf(
        UsePhoneRowSpec("Mouse", "Move the cursor, scroll, click", Icons.Rounded.Mouse, WireWaveTheme.Colors.accentBlue)
    )

    Box(
        modifier = modifier
            .fillMaxSize()
            .background(WireWaveTheme.Colors.bgCanvas)
    ) {
        // Hidden input field that triggers and captures Android native keyboard input (Gboard, Samsung, etc.)
        BasicTextField(
            value = textValue,
            onValueChange = { newText ->
                val currentStr = newText.text
                if (currentStr.length > sentinel.length) {
                    val added = currentStr.substring(sentinel.length)
                    if (added.contains("\n")) {
                        val parts = added.split("\n")
                        parts.forEachIndexed { index, part ->
                            if (part.isNotEmpty()) {
                                onSendKeyInput(part, null)
                            }
                            if (index < parts.size - 1) {
                                onSendKeyInput(null, "Enter")
                            }
                        }
                    } else {
                        onSendKeyInput(added, null)
                    }
                    textValue = TextFieldValue(sentinel, TextRange(sentinel.length))
                } else if (currentStr.length < sentinel.length) {
                    val count = sentinel.length - currentStr.length
                    repeat(count) {
                        onSendKeyInput(null, "Backspace")
                    }
                    textValue = TextFieldValue(sentinel, TextRange(sentinel.length))
                } else if (currentStr != sentinel) {
                    onSendKeyInput(currentStr, null)
                    textValue = TextFieldValue(sentinel, TextRange(sentinel.length))
                }
            },
            keyboardOptions = KeyboardOptions(
                imeAction = ImeAction.Default,
                autoCorrect = true
            ),
            keyboardActions = KeyboardActions(
                onDone = { onSendKeyInput(null, "Enter") },
                onSend = { onSendKeyInput(null, "Enter") },
                onGo = { onSendKeyInput(null, "Enter") }
            ),
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .size(1.dp)
                .alpha(0f)
                .focusRequester(focusRequester)
        )

        Column(
            modifier = Modifier
                .fillMaxSize()
                .statusBarsPadding()
                .imePadding()
                .verticalScroll(rememberScrollState())
                .padding(horizontal = WireWaveTheme.Metrics.spacingMd, vertical = WireWaveTheme.Metrics.spacingSm)
                .padding(bottom = if (activeOverlay != ScreenOverlay.NONE) 280.dp else 16.dp),
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
                        imageVector = Icons.Rounded.Settings,
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
                    speakerTile()
                    micTile()
                    val isKeyboardActive = isKeyboardVisible
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally,
                        modifier = Modifier
                            .clip(RoundedCornerShape(12.dp))
                            .clickable(role = Role.Button) {
                                if (isKeyboardVisible) {
                                    keyboardController?.hide()
                                } else {
                                    focusRequester.requestFocus()
                                    keyboardController?.show()
                                }
                            }
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
                                    when (row.title) {
                                        "Mouse" -> onNavigateToMouse()
                                        else -> Toast.makeText(context, "${row.title} will be available in Stage 3", Toast.LENGTH_SHORT).show()
                                    }
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
                            if (row.title == "Mouse") {
                                Icon(
                                    imageVector = Icons.AutoMirrored.Rounded.KeyboardArrowRight,
                                    contentDescription = "Open ${row.title}",
                                    tint = WireWaveTheme.Colors.textSecondary,
                                    modifier = Modifier.size(20.dp)
                                )
                            } else {
                                ComingSoonBadge()
                            }
                        }
                    }
                }
            }
        }

        // Screen Overlay Host (Controls popping up on the current screen)
        ScreenOverlayHost(
            overlay = activeOverlay,
            onControlKeyClick = { code ->
                onSendKeyInput(null, code)
            },
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .navigationBarsPadding()
        )
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
