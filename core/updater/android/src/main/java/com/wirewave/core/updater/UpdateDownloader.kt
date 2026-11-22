package com.wirewave.core.updater

import android.util.Log
import android.content.Context
import com.wirewave.core.config.AppConfig
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.OkHttpClient
import okhttp3.Request
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.security.MessageDigest

sealed interface DownloadOutcome {
    data class Success(val apkFile: File) : DownloadOutcome
    data class Failure(val message: String) : DownloadOutcome
}

class UpdateDownloader(
    context: Context,
    private val httpClient: OkHttpClient,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) {
    private val updateDir = File(context.applicationContext.cacheDir, "updates")

    /**
     * Streams the APK into a `.part` file, validates length & optional SHA-256,
     * then atomically promotes it to the final `.apk` file.
     */
    suspend fun download(
        release: UpdateRelease,
        onProgress: (percent: Int) -> Unit
    ): DownloadOutcome = withContext(ioDispatcher) {
        updateDir.mkdirs()
        val safeVersion = release.versionName.filter { it.isLetterOrDigit() || it == '.' || it == '-' }
        val finalFile = File(updateDir, "${AppConfig.appName}-$safeVersion.apk")
        val partFile = File(updateDir, "${AppConfig.appName}-$safeVersion.apk.part")
        partFile.delete()

        try {
            val request = Request.Builder()
                .url(release.downloadUrl)
                .header("X-Update-App-Id", AppConfig.updateAppId)
                .build()
            httpClient.newCall(request).execute().use { response ->
                if (!response.isSuccessful) {
                    Log.e("WireWaveUpdater", "Update download returned HTTP ${response.code}")
                    return@withContext DownloadOutcome.Failure(
                        "Couldn’t download the update. Try again later."
                    )
                }
                val body = response.body
                    ?: run {
                        Log.e("WireWaveUpdater", "Update download returned an empty response")
                        return@withContext DownloadOutcome.Failure(
                            "Couldn’t download the update. Try again later."
                        )
                    }

                val totalBytes = body.contentLength()
                var downloaded = 0L
                val buffer = ByteArray(CHUNK_SIZE)

                body.byteStream().use { input ->
                    FileOutputStream(partFile).use { output ->
                        while (true) {
                            val read = input.read(buffer)
                            if (read <= 0) break
                            output.write(buffer, 0, read)
                            downloaded += read
                            if (totalBytes > 0) {
                                val pct = ((downloaded * 100L) / totalBytes).toInt().coerceIn(0, 100)
                                onProgress(pct)
                            }
                        }
                        output.flush()
                    }
                }

                if (!isValidFile(partFile, totalBytes)) {
                    partFile.delete()
                    Log.e("WireWaveUpdater", "Downloaded APK is incomplete")
                    return@withContext DownloadOutcome.Failure(
                        "The update download was incomplete. Try again."
                    )
                }

                val expectedSha = release.sha256
                if (expectedSha == null || !expectedSha.matches(Regex("^[0-9a-fA-F]{64}$"))) {
                    partFile.delete()
                    return@withContext DownloadOutcome.Failure("Update verification information is unavailable.")
                }
                if (!verifySha256(partFile, expectedSha)) {
                    partFile.delete()
                    Log.e("WireWaveUpdater", "APK checksum verification failed")
                    return@withContext DownloadOutcome.Failure(
                        "Update verification failed. Download it again."
                    )
                }

                finalFile.delete()
                if (!partFile.renameTo(finalFile)) {
                    partFile.delete()
                    Log.e("WireWaveUpdater", "Could not finalize downloaded APK")
                    return@withContext DownloadOutcome.Failure("Couldn’t save the update. Try again.")
                }
                onProgress(100)
                DownloadOutcome.Success(finalFile)
            }
        } catch (e: CancellationException) {
            partFile.delete()
            throw e
        } catch (e: IOException) {
            partFile.delete()
            Log.e("WireWaveUpdater", "Update download failed", e)
            DownloadOutcome.Failure("Couldn’t download the update. Check your internet and try again.")
        } catch (e: IllegalArgumentException) {
            partFile.delete()
            Log.e("WireWaveUpdater", "Invalid update download URL", e)
            DownloadOutcome.Failure("Update download information is invalid.")
        }
    }

    companion object {
        private const val CHUNK_SIZE = 32 * 1024
        internal const val MIN_APK_BYTES = 100_000L

        internal fun isValidFile(file: File, expectedLength: Long): Boolean {
            if (!file.exists()) return false
            val size = file.length()
            val matchesExpected = expectedLength <= 0L || size == expectedLength
            return size >= MIN_APK_BYTES && matchesExpected
        }

        internal fun verifySha256(file: File, expectedHex: String): Boolean {
            val digest = MessageDigest.getInstance("SHA-256")
            file.inputStream().use { input ->
                val buf = ByteArray(CHUNK_SIZE)
                while (true) {
                    val n = input.read(buf)
                    if (n <= 0) break
                    digest.update(buf, 0, n)
                }
            }
            val actual = digest.digest().joinToString("") { "%02x".format(it) }
            return actual.equals(expectedHex.trim(), ignoreCase = true)
        }
    }
}
