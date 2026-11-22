package com.wirewave.core.updater

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class UpdateManifestClientTest {
    @Test
    fun `newer version with matching update_app_id returns available`() {
        val json = """
            {
              "version_code": 10100,
              "version_name": "1.1.0",
              "android": {
                "update_app_id": "ww-upd-and",
                "download_url": "https://example.com/wirewave.apk",
                "sha256": "0000000000000000000000000000000000000000000000000000000000000000"
              }
            }
        """.trimIndent()

        val outcome = UpdateManifestClient.parseAndEvaluate(json, 10000, "ww-upd-and")
        assertTrue(outcome is UpdateCheckOutcome.Available)
        assertEquals("1.1.0", (outcome as UpdateCheckOutcome.Available).release.versionName)
    }

    @Test
    fun `missing version code fails check`() {
        val result = UpdateManifestClient.parseAndEvaluate(
            """{"version_name":"1.1.0"}""",
            currentVersionCode = 10000,
            expectedUpdateAppId = "ww-upd-and"
        )
        org.junit.Assert.assertTrue(result is UpdateCheckOutcome.Failure)
    }

    @Test
    fun `missing version name fails check`() {
        val result = UpdateManifestClient.parseAndEvaluate(
            """{"version_code":10100}""",
            currentVersionCode = 10000,
            expectedUpdateAppId = "ww-upd-and"
        )
        org.junit.Assert.assertTrue(result is UpdateCheckOutcome.Failure)
    }

    @Test
    fun `mismatched update_app_id fails check`() {
        val json = """
            {
              "version_code": 10100,
              "version_name": "1.1.0",
              "android": {
                "update_app_id": "wrong-channel",
                "download_url": "https://example.com/wirewave.apk"
              }
            }
        """.trimIndent()

        val outcome = UpdateManifestClient.parseAndEvaluate(json, 10000, "ww-upd-and")
        assertTrue(outcome is UpdateCheckOutcome.Failure)
    }

    @Test
    fun `same version code returns up to date`() {
        val json = """{"version_code": 10000, "version_name": "1.0.0"}"""
        val outcome = UpdateManifestClient.parseAndEvaluate(json, 10000, "ww-upd-and")
        assertEquals(UpdateCheckOutcome.UpToDate, outcome)
    }
}
