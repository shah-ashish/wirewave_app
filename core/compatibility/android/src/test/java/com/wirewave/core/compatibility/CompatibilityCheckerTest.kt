package com.wirewave.core.compatibility

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class CompatibilityCheckerTest {
    @Test
    fun `android 11 api 30 is supported`() {
        val report = CompatibilityChecker.evaluateApiLevel(sdkInt = 30, releaseLabel = "11", minSdk = 30)
        assertTrue(report.isSupported)
    }

    @Test
    fun `android 10 api 29 is unsupported`() {
        val report = CompatibilityChecker.evaluateApiLevel(sdkInt = 29, releaseLabel = "10", minSdk = 30)
        assertFalse(report.isSupported)
    }
}
