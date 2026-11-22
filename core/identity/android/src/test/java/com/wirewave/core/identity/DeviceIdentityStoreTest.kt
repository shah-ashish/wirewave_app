package com.wirewave.core.identity

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class DeviceIdentityStoreTest {
    @Test
    fun `salted id is deterministic and hides raw seed`() {
        val first = DeviceIdentityStore.deriveSaltedId("sample-android-id")
        val second = DeviceIdentityStore.deriveSaltedId("sample-android-id")
        assertEquals(first, second)
        assertTrue(first.startsWith("wwd_"))
        assertFalse(first.contains("sample-android-id"))
    }
}
