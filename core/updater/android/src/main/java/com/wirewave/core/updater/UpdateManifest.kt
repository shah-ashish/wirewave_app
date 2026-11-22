package com.wirewave.core.updater

import android.util.Log
import com.wirewave.core.config.AppConfig
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull
import org.json.JSONException
import org.json.JSONObject
import java.io.IOException
import java.util.logging.Level
import java.util.logging.Logger

data class UpdateRelease(
    val versionName: String,
    val versionCode: Int,
    val downloadUrl: String,
    val sha256: String?,
    val notes: String
)

sealed interface UpdateCheckOutcome {
    data object UpToDate : UpdateCheckOutcome
    data class Available(val release: UpdateRelease) : UpdateCheckOutcome
    data class Failure(val message: String) : UpdateCheckOutcome
}

class UpdateManifestClient(
    private val httpClient: OkHttpClient,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) {
    suspend fun checkForUpdate(): UpdateCheckOutcome = withContext(ioDispatcher) {
        val url = "${AppConfig.updateEndpoint.trimEnd('/')}/latest.json"
        try {
            val request = Request.Builder()
                .url(url)
                .header("User-Agent", "${AppConfig.appName}/${AppConfig.versionName} (${AppConfig.updateAppId})")
                .header("X-Update-App-Id", AppConfig.updateAppId)
                .build()
            httpClient.newCall(request).execute().use { response ->
                if (!response.isSuccessful) {
                    Log.e("WireWaveUpdater", "Update check returned HTTP ${response.code}")
                    return@withContext UpdateCheckOutcome.Failure(
                        "Update service is unavailable. Try again later."
                    )
                }
                val body = response.body?.string().orEmpty()
                parseAndEvaluate(body, AppConfig.versionCode, AppConfig.updateAppId)
            }
        } catch (e: IOException) {
            Log.e("WireWaveUpdater", "Update check failed", e)
            UpdateCheckOutcome.Failure("Couldn’t connect. Check your internet and try again.")
        } catch (e: IllegalArgumentException) {
            Log.e("WireWaveUpdater", "Invalid update endpoint", e)
            UpdateCheckOutcome.Failure("Update service configuration is invalid.")
        }
    }

    companion object {
        private val logger = Logger.getLogger("WireWaveUpdater")

        /** Pure parser/evaluator for JVM unit testing. */
        fun parseAndEvaluate(
            rawJson: String,
            currentVersionCode: Int,
            expectedUpdateAppId: String
        ): UpdateCheckOutcome = try {
            val root = JSONObject(rawJson)
            val remoteCode = when (val value = root.opt("version_code")) {
                is Int -> value.takeIf { it > 0 }
                is Long -> value.takeIf { it in 1..Int.MAX_VALUE }?.toInt()
                else -> null
            }
            val remoteName = root.optString("version_name", "").trim()
            val notes = root.optString("notes", "")

            when {
                remoteCode == null || remoteName.isEmpty() ->
                    UpdateCheckOutcome.Failure("Update information is invalid.")
                remoteCode <= currentVersionCode -> UpdateCheckOutcome.UpToDate
                else -> resolveAndroidArtifact(root, remoteCode, remoteName, notes, expectedUpdateAppId)
            }
        } catch (e: JSONException) {
            logger.log(Level.SEVERE, "Invalid update manifest", e)
            UpdateCheckOutcome.Failure("Update information is unavailable. Try again later.")
        }

        private fun resolveAndroidArtifact(
            root: JSONObject,
            remoteCode: Int,
            remoteName: String,
            notes: String,
            expectedUpdateAppId: String
        ): UpdateCheckOutcome {
            val androidObj = root.optJSONObject("android")
            if (androidObj != null) {
                val manifestUpdateId = androidObj.optString("update_app_id", "")
                if (manifestUpdateId != expectedUpdateAppId) {
                    logger.severe(
                        "Update channel mismatch: $manifestUpdateId != $expectedUpdateAppId"
                    )
                    return UpdateCheckOutcome.Failure("This update is not compatible with this app.")
                }
                val url = androidObj.optString("download_url", "").trim()
                if (url.toHttpUrlOrNull() == null) {
                    logger.severe("Manifest has invalid Android download URL")
                    return UpdateCheckOutcome.Failure("Update download information is invalid.")
                }
                val sha256 = androidObj.optString("sha256", "").trim().takeIf { it.isNotEmpty() }
                if (sha256 == null || !sha256.matches(Regex("^[0-9a-fA-F]{64}$"))) {
                    logger.severe("Manifest has missing or invalid Android SHA-256")
                    return UpdateCheckOutcome.Failure("Update verification information is unavailable.")
                }
                return UpdateCheckOutcome.Available(
                    UpdateRelease(remoteName, remoteCode, url, sha256, notes)
                )
            }

            logger.severe("Manifest is missing Android artifact")
            return UpdateCheckOutcome.Failure("Update information is unavailable. Try again later.")
        }
    }
}
