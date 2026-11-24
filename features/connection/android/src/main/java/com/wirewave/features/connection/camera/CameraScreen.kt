package com.wirewave.features.connection.camera

import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
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
import androidx.compose.material.icons.automirrored.rounded.ArrowBack
import androidx.compose.material.icons.automirrored.rounded.VolumeUp
import androidx.compose.material.icons.rounded.AutoAwesome
import androidx.compose.material.icons.rounded.FlipCameraAndroid
import androidx.compose.material.icons.rounded.Mic
import androidx.compose.material.icons.rounded.MicOff
import androidx.compose.material.icons.rounded.PhotoCamera
import androidx.compose.material.icons.rounded.Refresh
import androidx.compose.material.icons.rounded.SwapVert
import androidx.compose.material.icons.rounded.Videocam
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.scale
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wirewave.core.config.WireWaveTheme

enum class CameraCaptureMode {
    PHOTO,
    VIDEO
}

@Composable
fun CameraScreen(
    speakerActive: Boolean,
    micActive: Boolean,
    onToggleSpeaker: () -> Unit,
    onToggleMic: () -> Unit,
    onBack: () -> Unit,
    modifier: Modifier = Modifier
) {
    var captureMode by remember { mutableStateOf(CameraCaptureMode.PHOTO) }
    var focusPoint by remember { mutableStateOf<Offset?>(null) }

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(Color.Black)
            .statusBarsPadding()
            .navigationBarsPadding()
    ) {
        // --- Top Bar ---
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp, vertical = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(10.dp)
            ) {
                // Back Button
                Box(
                    modifier = Modifier
                        .size(38.dp)
                        .clip(CircleShape)
                        .background(WireWaveTheme.Colors.bgCard)
                        .clickable(role = Role.Button, onClick = onBack),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.AutoMirrored.Rounded.ArrowBack,
                        contentDescription = "Back",
                        tint = WireWaveTheme.Colors.textPrimary,
                        modifier = Modifier.size(20.dp)
                    )
                }

                Text(
                    text = "Camera",
                    color = Color.White,
                    fontSize = 24.sp,
                    fontWeight = FontWeight.Bold
                )
            }

            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                // Speaker Button
                Box(
                    modifier = Modifier
                        .size(38.dp)
                        .clip(CircleShape)
                        .background(
                            if (speakerActive) WireWaveTheme.Colors.accentBlue
                            else WireWaveTheme.Colors.bgCard
                        )
                        .clickable(role = Role.Button, onClick = onToggleSpeaker),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.AutoMirrored.Rounded.VolumeUp,
                        contentDescription = "Speaker",
                        tint = if (speakerActive) Color.White else WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(20.dp)
                    )
                }

                // Microphone Button
                Box(
                    modifier = Modifier
                        .size(38.dp)
                        .clip(CircleShape)
                        .background(
                            if (micActive) WireWaveTheme.Colors.accentBlue
                            else WireWaveTheme.Colors.bgCard
                        )
                        .clickable(role = Role.Button, onClick = onToggleMic),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = if (micActive) Icons.Rounded.Mic else Icons.Rounded.MicOff,
                        contentDescription = "Microphone",
                        tint = if (micActive) Color.White else WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(20.dp)
                    )
                }

                // Swap / Status Button (Visual only)
                Box(
                    modifier = Modifier
                        .size(38.dp)
                        .clip(CircleShape)
                        .background(WireWaveTheme.Colors.bgCard),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.Rounded.SwapVert,
                        contentDescription = "Transport status",
                        tint = WireWaveTheme.Colors.textSecondary,
                        modifier = Modifier.size(20.dp)
                    )
                }
            }
        }

        // --- Camera Viewfinder Preview Area ---
        Box(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
                .pointerInput(Unit) {
                    detectTapGestures { offset ->
                        focusPoint = offset
                    }
                },
            contentAlignment = Alignment.Center
        ) {
            val centerReticle = focusPoint
            Canvas(modifier = Modifier.fillMaxSize()) {
                val center = centerReticle ?: Offset(size.width / 2f, size.height / 2f)
                // Reticle outer ring
                drawCircle(
                    color = WireWaveTheme.Colors.accentBlue.copy(alpha = 0.8f),
                    radius = 24.dp.toPx(),
                    center = center,
                    style = Stroke(width = 1.5.dp.toPx())
                )
                // Reticle center dot
                drawCircle(
                    color = WireWaveTheme.Colors.accentBlue,
                    radius = 2.5.dp.toPx(),
                    center = center
                )
            }
        }

        // --- Bottom Controls ---
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 20.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(20.dp)
        ) {
            // Photo / Video Toggle Pill
            ModeSelectorPill(
                currentMode = captureMode,
                onSelectMode = { captureMode = it }
            )

            // Shutter & Actions Row
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 36.dp),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                // Flip camera icon button
                Box(
                    modifier = Modifier
                        .size(48.dp)
                        .clip(CircleShape)
                        .background(Color(0xFF1C1C1E))
                        .clickable(role = Role.Button, onClick = {}),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.Rounded.Refresh,
                        contentDescription = "Flip camera",
                        tint = Color.White,
                        modifier = Modifier.size(22.dp)
                    )
                }

                // Shutter button (White ring + circle)
                ShutterButton(
                    isVideo = captureMode == CameraCaptureMode.VIDEO
                )

                // Effects / Filter button
                Box(
                    modifier = Modifier
                        .size(48.dp)
                        .clip(CircleShape)
                        .background(Color(0xFF1C1C1E))
                        .clickable(role = Role.Button, onClick = {}),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.Rounded.AutoAwesome,
                        contentDescription = "Effects",
                        tint = Color.White,
                        modifier = Modifier.size(22.dp)
                    )
                }
            }
        }
    }
}

/** Segmented pill selector to toggle Photo vs Video mode. */
@Composable
private fun ModeSelectorPill(
    currentMode: CameraCaptureMode,
    onSelectMode: (CameraCaptureMode) -> Unit
) {
    val pillShape = RoundedCornerShape(20.dp)

    Row(
        modifier = Modifier
            .clip(pillShape)
            .background(Color(0xFF1C1C1E))
            .padding(3.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        // Photo Mode
        val isPhoto = currentMode == CameraCaptureMode.PHOTO
        Box(
            modifier = Modifier
                .clip(RoundedCornerShape(16.dp))
                .background(if (isPhoto) Color.White else Color.Transparent)
                .clickable(role = Role.Tab) { onSelectMode(CameraCaptureMode.PHOTO) }
                .padding(horizontal = 14.dp, vertical = 6.dp),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = Icons.Rounded.PhotoCamera,
                contentDescription = "Photo mode",
                tint = if (isPhoto) Color.Black else Color.White.copy(alpha = 0.6f),
                modifier = Modifier.size(18.dp)
            )
        }

        // Video Mode
        val isVideo = currentMode == CameraCaptureMode.VIDEO
        Box(
            modifier = Modifier
                .clip(RoundedCornerShape(16.dp))
                .background(if (isVideo) Color.White else Color.Transparent)
                .clickable(role = Role.Tab) { onSelectMode(CameraCaptureMode.VIDEO) }
                .padding(horizontal = 14.dp, vertical = 6.dp),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = Icons.Rounded.Videocam,
                contentDescription = "Video mode",
                tint = if (isVideo) Color.Black else Color.White.copy(alpha = 0.6f),
                modifier = Modifier.size(18.dp)
            )
        }
    }
}

/** Big circular shutter button with touch press animation. */
@Composable
private fun ShutterButton(
    isVideo: Boolean,
    modifier: Modifier = Modifier
) {
    val interaction = remember { MutableInteractionSource() }
    val isPressed by interaction.collectIsPressedAsState()
    val scale by animateFloatAsState(targetValue = if (isPressed) 0.90f else 1f, label = "shutter_scale")

    Box(
        modifier = modifier
            .size(76.dp)
            .scale(scale)
            .clip(CircleShape)
            .border(3.dp, Color.White, CircleShape)
            .clickable(
                interactionSource = interaction,
                indication = null,
                role = Role.Button,
                onClick = {}
            ),
        contentAlignment = Alignment.Center
    ) {
        Box(
            modifier = Modifier
                .size(60.dp)
                .clip(CircleShape)
                .background(if (isVideo) WireWaveTheme.Colors.statusErrFg else Color.White)
        )
    }
}
