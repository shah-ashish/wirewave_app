package com.wirewave.android

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import com.wirewave.core.session.ClientSessionState
import com.wirewave.core.session.FeatureRemoteState

/** Connects remote session state to the local feature engines at the app boundary. */
@Composable
internal fun SessionFeatureEffects(
    container: AppContainer,
    sessionState: ClientSessionState,
    speakerRemoteState: FeatureRemoteState,
    micRemoteState: FeatureRemoteState,
    mouseRemoteState: FeatureRemoteState,
    micError: String?
) {
    LaunchedEffect(
        speakerRemoteState.active,
        speakerRemoteState.hostAddress,
        speakerRemoteState.udpPort,
        speakerRemoteState.streamId,
        speakerRemoteState.udpKey
    ) {
        val host = speakerRemoteState.hostAddress
        val port = speakerRemoteState.udpPort
        val streamId = speakerRemoteState.streamId
        val key = speakerRemoteState.udpKey
        if (speakerRemoteState.allowed && speakerRemoteState.active &&
            host != null && port != null && streamId != null && key != null
        ) {
            container.speakerEngine.start(host, port, key, streamId)
        } else {
            container.speakerEngine.stop()
        }
    }

    LaunchedEffect(
        micRemoteState.active,
        micRemoteState.hostAddress,
        micRemoteState.udpPort,
        micRemoteState.streamId,
        micRemoteState.udpKey,
        sessionState
    ) {
        val host = micRemoteState.hostAddress
        val port = micRemoteState.udpPort
        val streamId = micRemoteState.streamId
        val key = micRemoteState.udpKey
        val connected = sessionState as? ClientSessionState.Connected
        val pcName = connected?.pcName ?: "PC"
        if (connected != null && micRemoteState.allowed && micRemoteState.active &&
            host != null && port != null && streamId != null && key != null
        ) {
            container.updateMicrophoneForeground(true, pcName)
            container.micCaptureEngine.start(host, port, key, streamId)
        } else {
            container.micCaptureEngine.stop()
            if (connected != null) {
                container.updateMicrophoneForeground(false, pcName)
            }
        }
    }

    LaunchedEffect(micError) {
        if (micError != null && micRemoteState.active) {
            container.sessionController.requestMicActive(false)
        }
    }

    LaunchedEffect(
        mouseRemoteState.active,
        mouseRemoteState.hostAddress,
        mouseRemoteState.udpPort,
        mouseRemoteState.streamId,
        mouseRemoteState.udpKey
    ) {
        val host = mouseRemoteState.hostAddress
        val port = mouseRemoteState.udpPort
        val streamId = mouseRemoteState.streamId
        val key = mouseRemoteState.udpKey
        if (mouseRemoteState.allowed && mouseRemoteState.active &&
            host != null && port != null && streamId != null && key != null
        ) {
            container.mouseEngine.start(host, port, key, streamId)
        } else {
            container.mouseEngine.stop()
        }
    }
}
