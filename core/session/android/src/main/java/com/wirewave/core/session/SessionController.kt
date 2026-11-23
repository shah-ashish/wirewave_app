package com.wirewave.core.session

import android.annotation.SuppressLint
import android.bluetooth.BluetoothManager
import android.content.Context
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
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import org.json.JSONObject
import java.io.DataInputStream
import java.io.DataOutputStream
import java.io.IOException
import java.io.Closeable
import java.net.InetSocketAddress
import java.net.Socket
import java.util.UUID

sealed interface ClientSessionState {
    data class Disconnected(val errorMessage: String? = null) : ClientSessionState
    data class Connecting(val host: DiscoveredHost) : ClientSessionState
    data class AwaitingOtp(
        val host: DiscoveredHost,
        val errorMessage: String? = null
    ) : ClientSessionState
    data class Connected(
        val desktopId: String,
        val pcName: String,
        val transport: TransportKind
    ) : ClientSessionState
}

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
    private val appContext = context.applicationContext
    private val _state = MutableStateFlow<ClientSessionState>(ClientSessionState.Disconnected())
    val state: StateFlow<ClientSessionState> = _state.asStateFlow()

    private var connectionJob: Job? = null
    private var activeConnection: Closeable? = null
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
            when (host.transport) {
                TransportKind.WIFI -> runWifiSession(host)
                TransportKind.BLUETOOTH -> runBluetoothSession(host)
            }
        }
    }

    fun submitOtp(code: String) {
        otpInputDeferred?.complete(code.trim())
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
        try {
            activeConnection?.close()
        } catch (_: IOException) {
            // Closing active socket to unblock I/O.
        }
        activeConnection = null
        connectionJob?.cancel()
        connectionJob = null
    }

    private suspend fun runWifiSession(host: DiscoveredHost) {
        val socket = Socket()
        activeConnection = socket
        try {
            socket.connect(InetSocketAddress(host.hostAddress, host.tcpPort), 5000)
            socket.soTimeout = AppConfig.connection.heartbeatTimeoutSecs * 1000
            runSessionProtocol(host, socket.getInputStream(), socket.getOutputStream())
        } catch (e: CancellationException) {
            throw e
        } catch (_: IOException) {
            onForegroundStateChanged(false, null)
            _state.update { ClientSessionState.Disconnected("Connection lost.") }
        } finally {
            try {
                socket.close()
            } catch (_: IOException) {
                // Socket cleanup.
            }
            if (activeConnection === socket) activeConnection = null
        }
    }

    @SuppressLint("MissingPermission")
    private suspend fun runBluetoothSession(host: DiscoveredHost) {
        val manager = appContext.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
            ?: return failSession("Bluetooth is unavailable.")
        val adapter = manager.adapter ?: return failSession("Bluetooth is unavailable.")
        val device = try {
            adapter.getRemoteDevice(host.hostAddress)
        } catch (_: IllegalArgumentException) {
            return failSession("Invalid Bluetooth device address.")
        }
        val socket = try {
            device.createRfcommSocketToServiceRecord(UUID.fromString(AppConfig.connection.btServiceUuid))
        } catch (_: SecurityException) {
            return failSession("Bluetooth permission is required.")
        }
        activeConnection = socket
        try {
            adapter.cancelDiscovery()
            socket.connect()
            runSessionProtocol(host, socket.inputStream, socket.outputStream)
        } catch (e: CancellationException) {
            throw e
        } catch (_: SecurityException) {
            failSession("Bluetooth permission is required.")
        } catch (_: IOException) {
            onForegroundStateChanged(false, null)
            _state.update { ClientSessionState.Disconnected("Bluetooth connection lost.") }
        } finally {
            try {
                socket.close()
            } catch (_: IOException) {
                // Socket cleanup.
            }
            if (activeConnection === socket) activeConnection = null
        }
    }

    private suspend fun runSessionProtocol(
        host: DiscoveredHost,
        rawInput: java.io.InputStream,
        rawOutput: java.io.OutputStream
    ) {
        val input = DataInputStream(rawInput)
        val output = DataOutputStream(rawOutput)
        val ephemeral = EphemeralX25519()
        val hello = JSONObject().apply {
            put("type", "hello")
            put("protocol_domain", AppConfig.connection.protocolDomain)
            put("device_id", identity.deviceId)
            put("device_name", deviceName)
            put("public_key_hex", keypair.publicKeyHex)
            put("ephemeral_x25519_hex", ephemeral.publicKeyHex)
            put("transport", host.transport.wireName)
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

        val transcript = "$desktopId:${identity.deviceId}:$nonceHex".toByteArray(Charsets.UTF_8)
        val sig = keypair.sign(transcript).toHex()
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
        if (!WireWaveCrypto.verifyPeerSignature(desktopPub, transcript, desktopSig)) {
            return failSession("Desktop authenticity check failed.")
        }
        store.saveTrustedDesktop(TrustedDesktop(desktopId, desktopName, desktopPub))

        val sessionKey = WireWaveCrypto.deriveSessionKey(shared, transcript)
        enterEncryptedLoop(
            desktopId,
            desktopName,
            host.transport,
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

        val transcript = "$desktopId:${identity.deviceId}".toByteArray(Charsets.UTF_8)
        _state.update { ClientSessionState.AwaitingOtp(host) }

        repeat(AppConfig.connection.otpMaxAttempts) {
            val deferred = CompletableDeferred<String>()
            otpInputDeferred = deferred
            val code = deferred.await()

            val proofHex = WireWaveCrypto.computeOtpProof(shared, code, transcript).toHex()
            val sigHex = keypair.sign(transcript).toHex()
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
                    if (!WireWaveCrypto.verifyPeerSignature(desktopPub, transcript, desktopSig)) {
                        return failSession("Desktop authenticity check failed.")
                    }
                    store.saveTrustedDesktop(TrustedDesktop(desktopId, desktopName, desktopPub))
                    val sessionKey = WireWaveCrypto.deriveSessionKey(shared, transcript)
                    enterEncryptedLoop(
                        desktopId,
                        desktopName,
                        host.transport,
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
        cipher: SessionCipher,
        input: DataInputStream,
        output: DataOutputStream
    ) {
        onForegroundStateChanged(true, pcName)
        _state.update { ClientSessionState.Connected(desktopId, pcName, transport) }

        cipher.use { activeCipher ->
            var seq = 0L
            while (scope.isActive) {
                val frame = FrameCodec.readFrame(input)
                val plain = activeCipher.decrypt(frame) ?: break
                val msg = JSONObject(String(plain, Charsets.UTF_8))
                when (msg.optString("type")) {
                    "heartbeat_ping" -> {
                        val pong = JSONObject().apply {
                            put("type", "heartbeat_pong")
                            put("seq", msg.optLong("seq", ++seq))
                        }
                        FrameCodec.writeFrame(output, activeCipher.encrypt(pong.toString().toByteArray(Charsets.UTF_8)))
                    }
                    "disconnect" -> break
                }
                delay(10)
            }
        }
        onForegroundStateChanged(false, null)
        _state.update { ClientSessionState.Disconnected() }
    }

    private fun failSession(message: String) {
        onForegroundStateChanged(false, null)
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
