package com.wirewave.core.session

import android.content.Context
import com.wirewave.core.stream.ChannelId
import com.wirewave.core.config.AppConfig
import com.wirewave.core.crypto.EphemeralX25519
import com.wirewave.core.crypto.CipherRole
import com.wirewave.core.crypto.IdentityKeypair
import com.wirewave.core.crypto.SessionCipher
import com.wirewave.core.crypto.WireWaveCrypto
import com.wirewave.core.crypto.toHex
import com.wirewave.core.discovery.DiscoveredHost
import com.wirewave.core.identity.DeviceIdentity
import com.wirewave.core.transport.FrameCodec
import com.wirewave.core.transport.TransportKind
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancelAndJoin
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import org.json.JSONObject
import java.io.DataInputStream
import java.io.DataOutputStream
import java.util.concurrent.atomic.AtomicReference

/**
 * Manages the encrypted WireWave session lifecycle on Android:
 * trusted challenge-response, 6-digit OTP pairing, encrypted heartbeat, and explicit disconnect.
 */
class SessionController(
    private val scope: CoroutineScope,
    private val identity: DeviceIdentity,
    private val deviceName: String,
    private val keypair: IdentityKeypair,
    context: Context,
    val store: TrustAndPrefsStore,
    private val onForegroundStateChanged: (active: Boolean, pcName: String?) -> Unit = { _, _ -> },
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) {
    private val transportRunner = SessionTransportRunner(
        context = context,
        onConnectionLost = { message ->
            onForegroundStateChanged(false, null)
            _state.update { ClientSessionState.Disconnected(message) }
        },
        onFailure = ::failSession,
        runProtocol = ::runSessionProtocol
    )
    private val _state = MutableStateFlow<ClientSessionState>(ClientSessionState.Disconnected())
    val state: StateFlow<ClientSessionState> = _state.asStateFlow()
    private val speakerStateStore = RemoteFeatureStateStore(ChannelId.SPEAKER, allowedByDefault = true)
    val speakerState: StateFlow<FeatureRemoteState> = speakerStateStore.state
    private val micStateStore = RemoteFeatureStateStore(ChannelId.MIC, allowedByDefault = false)
    val micState: StateFlow<FeatureRemoteState> = micStateStore.state
    private val mouseStateStore = RemoteFeatureStateStore(ChannelId.MOUSE, allowedByDefault = true)
    val mouseState: StateFlow<FeatureRemoteState> = mouseStateStore.state
    private val outboundControls = AtomicReference<OutboundControlQueue?>(null)

    fun featureState(channel: ChannelId): StateFlow<FeatureRemoteState> = when (channel) {
        ChannelId.SPEAKER -> speakerState
        ChannelId.MIC -> micState
        ChannelId.MOUSE -> mouseState
        else -> MutableStateFlow(FeatureRemoteState(channel = channel, allowed = false)).asStateFlow()
    }

    fun requestFeatureActive(channel: ChannelId, active: Boolean) {
        when (channel) {
            ChannelId.SPEAKER -> requestSpeakerActive(active)
            ChannelId.MIC -> requestMicActive(active)
            ChannelId.MOUSE -> requestMouseActive(active)
            else -> outboundControls.get()?.trySendFeature(channel, active)
        }
    }

    private var connectionJob: Job? = null
    private var otpInputDeferred: CompletableDeferred<String>? = null

    fun connect(host: DiscoveredHost) {
        if (host.busy) {
            _state.update { ClientSessionState.Disconnected("This computer is currently connected to another phone.") }
            return
        }
        cancelActiveJob()
        store.setExplicitDisconnect(false)
        _state.update { ClientSessionState.Connecting(host) }

        connectionJob = scope.launch(ioDispatcher) {
            transportRunner.run(host)
        }
    }

    fun submitOtp(code: String) {
        otpInputDeferred?.complete(code.trim())
    }

    fun requestSpeakerActive(active: Boolean) {
        if (active && !speakerState.value.allowed) return
        outboundControls.get()?.trySendSpeaker(active)
    }

    fun requestMicActive(active: Boolean) {
        if (active && !micState.value.allowed) return
        outboundControls.get()?.trySendMic(active)
    }

    fun requestMouseActive(active: Boolean) {
        if (active && !mouseState.value.allowed) return
        outboundControls.get()?.trySendFeature(ChannelId.MOUSE, active)
    }

    fun sendKeyInput(text: String? = null, keyCode: String? = null) {
        outboundControls.get()?.trySendKeyInput(text, keyCode)
    }

    fun cancelPairing() {
        otpInputDeferred?.cancel()
        cancelActiveJob()
        _state.update { ClientSessionState.Disconnected() }
    }

    fun disconnectExplicitly() {
        store.setExplicitDisconnect(true)
        cancelActiveJob()
        onForegroundStateChanged(false, null)
        _state.update { ClientSessionState.Disconnected() }
    }

    fun disconnectForBackground() {
        cancelActiveJob()
        onForegroundStateChanged(false, null)
        _state.update { ClientSessionState.Disconnected("Disconnected because keep alive in background is turned off.") }
    }

    private fun cancelActiveJob() {
        otpInputDeferred?.cancel()
        otpInputDeferred = null
        transportRunner.close()
        connectionJob?.cancel()
        connectionJob = null
    }

    private suspend fun runSessionProtocol(
        host: DiscoveredHost,
        rawInput: java.io.InputStream,
        rawOutput: java.io.OutputStream
    ) {
        val input = DataInputStream(rawInput)
        val output = DataOutputStream(rawOutput)
        val ephemeral = EphemeralX25519()
        val isTrusted = store.getTrustedDesktopForHost(host.desktopId, host.transport, host.hostAddress) != null
        val hello = JSONObject().apply {
            put("type", "hello")
            put("protocol_domain", AppConfig.connection.protocolDomain)
            put("device_id", identity.deviceId)
            put("device_name", deviceName)
            put("public_key_hex", keypair.publicKeyHex)
            put("ephemeral_x25519_hex", ephemeral.publicKeyHex)
            put("transport", host.transport.wireName)
            put("require_pairing", !isTrusted)
        }
        FrameCodec.writeFrame(output, hello.toString().toByteArray(Charsets.UTF_8))

        val response = JSONObject(String(FrameCodec.readFrame(input), Charsets.UTF_8))
        when (response.optString("type")) {
            "challenge" -> handleTrustedChallenge(host, response, ephemeral, input, output)
            "otp_required" -> handleOtpFlow(host, response, ephemeral, input, output)
            "rejected" -> {
                val reason = mapRejectCode(response.optString("code"))
                _state.update { ClientSessionState.Disconnected(reason) }
            }
            else -> _state.update { ClientSessionState.Disconnected("Unexpected response from desktop.") }
        }
    }

    private suspend fun handleTrustedChallenge(
        host: DiscoveredHost,
        challenge: JSONObject,
        ephemeral: EphemeralX25519,
        input: DataInputStream,
        output: DataOutputStream
    ) {
        val desktopId = challenge.getString("desktop_id")
        val desktopName = challenge.getString("desktop_name")
        val desktopPub = challenge.getString("desktop_public_key_hex")
        val serverEph = challenge.getString("ephemeral_x25519_hex")
        val nonceHex = challenge.getString("nonce_hex")

        if (host.publicKeyHex != null &&
            !host.publicKeyHex.equals(desktopPub, ignoreCase = true)
        ) {
            return failSession("Desktop discovery identity changed.")
        }

        val trustedDesktop = store.getTrustedDesktop(desktopId)
            ?: return failSession("Desktop is not trusted. Pair again.")
        if (!trustedDesktop.publicKeyHex.equals(desktopPub, ignoreCase = true)) {
            return failSession("Desktop identity changed. Pair again.")
        }

        val shared = ephemeral.deriveSharedSecret(serverEph)
            ?: return failSession("Key exchange failed.")

        val exchangeHash = WireWaveCrypto.computeExchangeHash(
            desktopId = desktopId,
            deviceId = identity.deviceId,
            desktopPubkeyHex = desktopPub,
            devicePubkeyHex = keypair.publicKeyHex,
            serverX25519Hex = serverEph,
            clientX25519Hex = ephemeral.publicKeyHex,
            nonceHex = nonceHex
        )
        val sig = keypair.sign(exchangeHash).toHex()
        val reply = JSONObject().apply {
            put("type", "challenge_reply")
            put("signature_hex", sig)
        }
        FrameCodec.writeFrame(output, reply.toString().toByteArray(Charsets.UTF_8))

        val ready = JSONObject(String(FrameCodec.readFrame(input), Charsets.UTF_8))
        if (ready.optString("type") != "session_ready") {
            return failSession(mapRejectCode(ready.optString("code")))
        }
        val desktopSig = ready.getString("desktop_signature_hex")
        if (!WireWaveCrypto.verifyPeerSignature(desktopPub, exchangeHash, desktopSig)) {
            return failSession("Desktop authenticity check failed.")
        }
        store.saveTrustedDesktop(TrustedDesktop(desktopId, desktopName, desktopPub))
        if (host.transport == TransportKind.BLUETOOTH) {
            store.associateBluetoothAddress(desktopId, host.hostAddress)
        }

        val sessionKey = WireWaveCrypto.deriveSessionKey(shared, exchangeHash)
        enterEncryptedLoop(
            desktopId,
            desktopName,
            host.transport,
            host.hostAddress,
            SessionCipher(sessionKey, CipherRole.ANDROID),
            input,
            output
        )
    }

    private suspend fun handleOtpFlow(
        host: DiscoveredHost,
        prompt: JSONObject,
        ephemeral: EphemeralX25519,
        input: DataInputStream,
        output: DataOutputStream
    ) {
        val desktopId = prompt.getString("desktop_id")
        val desktopName = prompt.getString("desktop_name")
        val desktopPub = prompt.getString("desktop_public_key_hex")
        val serverEph = prompt.getString("ephemeral_x25519_hex")

        if (host.publicKeyHex != null &&
            !host.publicKeyHex.equals(desktopPub, ignoreCase = true)
        ) {
            return failSession("Desktop discovery identity changed.")
        }

        val shared = ephemeral.deriveSharedSecret(serverEph)
            ?: return failSession("Key exchange failed.")

        val exchangeHash = WireWaveCrypto.computeExchangeHash(
            desktopId = desktopId,
            deviceId = identity.deviceId,
            desktopPubkeyHex = desktopPub,
            devicePubkeyHex = keypair.publicKeyHex,
            serverX25519Hex = serverEph,
            clientX25519Hex = ephemeral.publicKeyHex,
            nonceHex = ""
        )
        _state.update { ClientSessionState.AwaitingOtp(host) }

        repeat(AppConfig.connection.otpMaxAttempts) {
            val deferred = CompletableDeferred<String>()
            otpInputDeferred = deferred
            val code = deferred.await()

            val proofHex = WireWaveCrypto.computeOtpProof(shared, code, exchangeHash).toHex()
            val sigHex = keypair.sign(exchangeHash).toHex()
            val msg = JSONObject().apply {
                put("type", "otp_proof")
                put("proof_hex", proofHex)
                put("signature_hex", sigHex)
            }
            FrameCodec.writeFrame(output, msg.toString().toByteArray(Charsets.UTF_8))

            val reply = JSONObject(String(FrameCodec.readFrame(input), Charsets.UTF_8))
            when (reply.optString("type")) {
                "session_ready" -> {
                    val desktopSig = reply.getString("desktop_signature_hex")
                    if (!WireWaveCrypto.verifyPeerSignature(desktopPub, exchangeHash, desktopSig)) {
                        return failSession("Desktop authenticity check failed.")
                    }
                    store.saveTrustedDesktop(TrustedDesktop(desktopId, desktopName, desktopPub))
        if (host.transport == TransportKind.BLUETOOTH) {
            store.associateBluetoothAddress(desktopId, host.hostAddress)
        }
                    val sessionKey = WireWaveCrypto.deriveSessionKey(shared, exchangeHash)
                    enterEncryptedLoop(
                        desktopId,
                        desktopName,
                        host.transport,
                        host.hostAddress,
                        SessionCipher(sessionKey, CipherRole.ANDROID),
                        input,
                        output
                    )
                    return
                }
                "rejected" -> {
                    val codeStr = reply.optString("code")
                    if (codeStr == "invalid_otp") {
                        _state.update { ClientSessionState.AwaitingOtp(host, "Incorrect code. Try again.") }
                    } else {
                        return failSession(mapRejectCode(codeStr))
                    }
                }
                else -> return failSession("Pairing failed.")
            }
        }
        failSession("Too many incorrect attempts.")
    }

    private suspend fun enterEncryptedLoop(
        desktopId: String,
        pcName: String,
        transport: TransportKind,
        hostAddress: String,
        cipher: SessionCipher,
        input: DataInputStream,
        output: DataOutputStream
    ) {
        val controls = OutboundControlQueue()
        outboundControls.set(controls)
        onForegroundStateChanged(true, pcName)
        _state.update { ClientSessionState.Connected(desktopId, pcName, transport) }

        try {
            cipher.use { activeCipher ->
                coroutineScope {
                    val writer = launch(ioDispatcher) {
                        for (command in controls.commands) {
                            FrameCodec.writeFrame(output, activeCipher.encrypt(command.encode()))
                        }
                    }

                    try {
                        var seq = 0L
                        while (currentCoroutineContext().isActive) {
                            val frame = FrameCodec.readFrame(input)
                            val plain = activeCipher.decrypt(frame) ?: break
                            val msg = JSONObject(String(plain, Charsets.UTF_8))
                            when (msg.optString("type")) {
                                "heartbeat_ping" -> controls.sendHeartbeatPong(
                                    msg.optLong("seq", ++seq)
                                )
                                "speaker_state" -> speakerStateStore.updateFromMessage(msg, hostAddress)
                                "mic_state" -> micStateStore.updateFromMessage(msg, hostAddress)
                                "mouse_state" -> mouseStateStore.updateFromMessage(msg, hostAddress)
                                "disconnect" -> break
                            }
                        }
                    } finally {
                        controls.close()
                        writer.cancelAndJoin()
                    }
                }
            }
        } finally {
            outboundControls.compareAndSet(controls, null)
            controls.close()
            onForegroundStateChanged(false, null)
            speakerStateStore.reset()
            micStateStore.reset()
            mouseStateStore.reset()
            _state.update { ClientSessionState.Disconnected() }
        }
    }

    private fun failSession(message: String) {
        onForegroundStateChanged(false, null)
        outboundControls.getAndSet(null)?.close()
        speakerStateStore.reset()
        micStateStore.reset()
        mouseStateStore.reset()
        _state.update { ClientSessionState.Disconnected(message) }
    }

    private fun mapRejectCode(code: String): String = when (code) {
        "busy" -> "This computer is already connected to another phone."
        "blocked" -> "This phone is blocked by the computer."
        "otp_expired" -> "Pairing code expired. Try again."
        "cancelled" -> "Pairing cancelled on computer."
        else -> "Authentication failed."
    }
}
