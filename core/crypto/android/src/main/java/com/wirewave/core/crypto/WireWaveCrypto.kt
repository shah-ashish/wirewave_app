package com.wirewave.core.crypto

import android.content.Context
import com.wirewave.core.config.AppConfig
import org.bouncycastle.crypto.agreement.X25519Agreement
import org.bouncycastle.crypto.modes.ChaCha20Poly1305
import org.bouncycastle.crypto.params.AEADParameters
import org.bouncycastle.crypto.params.Ed25519PrivateKeyParameters
import org.bouncycastle.crypto.params.Ed25519PublicKeyParameters
import org.bouncycastle.crypto.params.KeyParameter
import org.bouncycastle.crypto.params.X25519PrivateKeyParameters
import org.bouncycastle.crypto.params.X25519PublicKeyParameters
import org.bouncycastle.crypto.signers.Ed25519Signer
import java.nio.ByteBuffer
import java.security.MessageDigest
import java.security.SecureRandom
import javax.crypto.Mac
import javax.crypto.spec.SecretKeySpec

class IdentityKeypair(private val privateKeyBytes: ByteArray) {
    private val privateParams = Ed25519PrivateKeyParameters(privateKeyBytes, 0)
    private val publicParams: Ed25519PublicKeyParameters = privateParams.generatePublicKey()

    val publicKeyHex: String
        get() = publicParams.encoded.toHex()

    fun sign(payload: ByteArray): ByteArray {
        val signer = Ed25519Signer()
        signer.init(true, privateParams)
        signer.update(payload, 0, payload.size)
        return signer.generateSignature()
    }

    companion object {
        fun generate(): IdentityKeypair {
            val priv = Ed25519PrivateKeyParameters(SecureRandom())
            return IdentityKeypair(priv.encoded)
        }
    }
}

class KeypairStore(context: Context) {
    private val prefs = context.applicationContext.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    fun fetchOrCreate(): IdentityKeypair {
        val existingHex = prefs.getString(KEY_SECRET_HEX, null)?.trim()
        val decoded = existingHex?.takeIf { it.length == 64 }?.hexToBytesOrNull()
        if (decoded != null) {
            return IdentityKeypair(decoded)
        }
        val privBytes = ByteArray(32)
        SecureRandom().nextBytes(privBytes)
        val kp = IdentityKeypair(privBytes)
        prefs.edit().putString(KEY_SECRET_HEX, privBytes.toHex()).apply()
        privBytes.fill(0)
        return kp
    }

    companion object {
        private const val PREFS_NAME = "wirewave_keypair"
        private const val KEY_SECRET_HEX = "ed25519_secret_hex"
    }
}

class EphemeralX25519 {
    private val privateKey = X25519PrivateKeyParameters(SecureRandom())
    val publicKeyHex: String = privateKey.generatePublicKey().encoded.toHex()

    fun deriveSharedSecret(peerPublicKeyHex: String): ByteArray? {
        val peerBytes = peerPublicKeyHex.hexToBytesOrNull()?.takeIf { it.size == 32 } ?: return null
        val agreement = X25519Agreement()
        agreement.init(privateKey)
        val secret = ByteArray(32)
        agreement.calculateAgreement(X25519PublicKeyParameters(peerBytes, 0), secret, 0)
        return secret
    }
}

enum class CipherRole {
    DESKTOP,
    ANDROID
}

class SessionCipher(rawKey: ByteArray, role: CipherRole) : AutoCloseable {
    private val sendKey: ByteArray
    private val recvKey: ByteArray
    private var sendCounter: Long = 0L
    private var recvCounter: Long = 0L

    init {
        val desktopToAndroid = deriveDirectionalKey(rawKey, "desktop-to-android")
        val androidToDesktop = deriveDirectionalKey(rawKey, "android-to-desktop")
        if (role == CipherRole.DESKTOP) {
            sendKey = desktopToAndroid
            recvKey = androidToDesktop
        } else {
            sendKey = androidToDesktop
            recvKey = desktopToAndroid
        }
    }

    fun encrypt(plaintext: ByteArray): ByteArray {
        val counter = sendCounter
        if (sendCounter == Long.MAX_VALUE) {
            throw IllegalStateException("Nonce counter exhausted")
        }
        sendCounter++
        val nonce = buildNonce(counter)
        val cipher = ChaCha20Poly1305()
        cipher.init(true, AEADParameters(KeyParameter(sendKey), 128, nonce))
        val out = ByteArray(cipher.getOutputSize(plaintext.size))
        val len = cipher.processBytes(plaintext, 0, plaintext.size, out, 0)
        cipher.doFinal(out, len)

        return ByteBuffer.allocate(8 + out.size)
            .putLong(counter)
            .put(out)
            .array()
    }

    fun decrypt(frame: ByteArray): ByteArray? {
        if (frame.size < 24) return null
        val counter = ByteBuffer.wrap(frame, 0, 8).long
        if (java.lang.Long.compareUnsigned(counter, recvCounter) < 0) return null

        val nonce = buildNonce(counter)
        val cipher = ChaCha20Poly1305()
        cipher.init(false, AEADParameters(KeyParameter(recvKey), 128, nonce))
        val ctLen = frame.size - 8
        val out = ByteArray(cipher.getOutputSize(ctLen))
        return try {
            val len = cipher.processBytes(frame, 8, ctLen, out, 0)
            val finalLen = cipher.doFinal(out, len)
            recvCounter = counter + 1
            out.copyOf(len + finalLen)
        } catch (_: Exception) {
            null
        }
    }

    override fun close() {
        sendKey.fill(0)
        recvKey.fill(0)
    }

    private fun buildNonce(counter: Long): ByteArray =
        ByteBuffer.allocate(12).putInt(0).putLong(counter).array()

    private fun deriveDirectionalKey(sessionKey: ByteArray, direction: String): ByteArray {
        val digest = MessageDigest.getInstance("SHA-256")
        digest.update(AppConfig.connection.protocolDomain.toByteArray(Charsets.UTF_8))
        digest.update(":directional-key:".toByteArray(Charsets.UTF_8))
        digest.update(sessionKey)
        digest.update(":".toByteArray(Charsets.UTF_8))
        digest.update(direction.toByteArray(Charsets.UTF_8))
        return digest.digest()
    }
}

object WireWaveCrypto {
    fun computeExchangeHash(
        desktopId: String,
        deviceId: String,
        desktopPubkeyHex: String,
        devicePubkeyHex: String,
        serverX25519Hex: String,
        clientX25519Hex: String,
        nonceHex: String
    ): ByteArray {
        val digest = MessageDigest.getInstance("SHA-256")
        digest.update(AppConfig.connection.protocolDomain.toByteArray(Charsets.UTF_8))
        digest.update(":exchange-hash:".toByteArray(Charsets.UTF_8))
        digest.update(desktopId.toByteArray(Charsets.UTF_8))
        digest.update(":".toByteArray(Charsets.UTF_8))
        digest.update(deviceId.toByteArray(Charsets.UTF_8))
        digest.update(":".toByteArray(Charsets.UTF_8))
        digest.update(desktopPubkeyHex.toByteArray(Charsets.UTF_8))
        digest.update(":".toByteArray(Charsets.UTF_8))
        digest.update(devicePubkeyHex.toByteArray(Charsets.UTF_8))
        digest.update(":".toByteArray(Charsets.UTF_8))
        digest.update(serverX25519Hex.toByteArray(Charsets.UTF_8))
        digest.update(":".toByteArray(Charsets.UTF_8))
        digest.update(clientX25519Hex.toByteArray(Charsets.UTF_8))
        digest.update(":".toByteArray(Charsets.UTF_8))
        digest.update(nonceHex.toByteArray(Charsets.UTF_8))
        return digest.digest()
    }

    fun verifyPeerSignature(publicKeyHex: String, payload: ByteArray, signatureHex: String): Boolean {
        val pubBytes = publicKeyHex.hexToBytesOrNull()?.takeIf { it.size == 32 } ?: return false
        val sigBytes = signatureHex.hexToBytesOrNull()?.takeIf { it.size == 64 } ?: return false
        val verifier = Ed25519Signer()
        verifier.init(false, Ed25519PublicKeyParameters(pubBytes, 0))
        verifier.update(payload, 0, payload.size)
        return verifier.verifySignature(sigBytes)
    }

    fun computeOtpProof(sharedSecret: ByteArray, otp: String, transcript: ByteArray): ByteArray {
        val mac = Mac.getInstance("HmacSHA256")
        mac.init(SecretKeySpec(sharedSecret, "HmacSHA256"))
        mac.update(AppConfig.connection.protocolDomain.toByteArray(Charsets.UTF_8))
        mac.update(":otp:".toByteArray(Charsets.UTF_8))
        mac.update(otp.trim().toByteArray(Charsets.UTF_8))
        mac.update(":".toByteArray(Charsets.UTF_8))
        mac.update(transcript)
        return mac.doFinal()
    }

    fun deriveSessionKey(sharedSecret: ByteArray, transcript: ByteArray): ByteArray {
        val digest = MessageDigest.getInstance("SHA-256")
        digest.update(AppConfig.connection.protocolDomain.toByteArray(Charsets.UTF_8))
        digest.update(":session-key:".toByteArray(Charsets.UTF_8))
        digest.update(sharedSecret)
        digest.update(transcript)
        return digest.digest()
    }

    fun discoverySigningPayload(payload: ByteArray): ByteArray {
        return AppConfig.connection.protocolDomain.toByteArray(Charsets.UTF_8) +
            ":discovery:".toByteArray(Charsets.UTF_8) +
            payload
    }
}

fun ByteArray.toHex(): String = joinToString("") { "%02x".format(it) }

fun String.hexToBytesOrNull(): ByteArray? {
    val clean = trim()
    if (clean.length % 2 != 0) return null
    return try {
        ByteArray(clean.length / 2) { idx ->
            clean.substring(idx * 2, idx * 2 + 2).toInt(16).toByte()
        }
    } catch (_: NumberFormatException) {
        null
    }
}
