package com.wirewave.core.crypto

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class WireWaveCryptoTest {
    @Test
    fun `ed25519 signature verifies only untampered payloads`() {
        val kp = IdentityKeypair.generate()
        val sigHex = kp.sign("challenge-123".toByteArray()).toHex()
        assertTrue(WireWaveCrypto.verifyPeerSignature(kp.publicKeyHex, "challenge-123".toByteArray(), sigHex))
        assertFalse(WireWaveCrypto.verifyPeerSignature(kp.publicKeyHex, "tampered".toByteArray(), sigHex))
    }

    @Test
    fun `session cipher roundtrips and rejects replayed frames`() {
        val key = ByteArray(32) { it.toByte() }
        val tx = SessionCipher(key, CipherRole.DESKTOP)
        val rx = SessionCipher(key, CipherRole.ANDROID)

        val encrypted = tx.encrypt("hello".toByteArray())
        val decrypted = rx.decrypt(encrypted)
        assertNotNull(decrypted)
        assertArrayEquals("hello".toByteArray(), decrypted)
        assertNull(rx.decrypt(encrypted))

        tx.close()
        rx.close()
    }
}
