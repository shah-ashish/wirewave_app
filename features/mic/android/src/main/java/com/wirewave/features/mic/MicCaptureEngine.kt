package com.wirewave.features.mic

import android.annotation.SuppressLint
import android.media.AudioFormat
import android.media.AudioRecord
import android.media.MediaRecorder
import android.media.audiofx.AcousticEchoCanceler
import android.media.audiofx.NoiseSuppressor
import android.os.Process
import com.wirewave.core.stream.ChannelId
import com.wirewave.core.stream.UdpStreamClient
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import java.io.IOException

data class MicCaptureState(val isOn: Boolean = false, val error: String? = null)

class MicCaptureEngine(
    private val scope: CoroutineScope,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) {
    private data class RecorderConfig(val recorder: AudioRecord, val sampleRate: Int)

    private val _state = MutableStateFlow(MicCaptureState())
    val state: StateFlow<MicCaptureState> = _state.asStateFlow()
    private var captureJob: Job? = null
    private var client: UdpStreamClient? = null

    @SuppressLint("MissingPermission")
    fun start(host: String, port: Int, key: ByteArray, streamId: Int) {
        stop()
        captureJob = scope.launch(ioDispatcher) {
            val previousPriority = Process.getThreadPriority(Process.myTid())
            Process.setThreadPriority(Process.THREAD_PRIORITY_URGENT_AUDIO)
            try {
                capture(host, port, key, streamId)
            } finally {
                Process.setThreadPriority(previousPriority)
            }
        }
    }

    @SuppressLint("MissingPermission")
    private suspend fun kotlinx.coroutines.CoroutineScope.capture(
        host: String,
        port: Int,
        key: ByteArray,
        streamId: Int
    ) {
        val recorderConfig = try {
            createCompatibleRecorder()
        } catch (_: SecurityException) {
            _state.value = MicCaptureState(error = "Microphone permission is required")
            return
        }
        if (recorderConfig == null) {
            _state.value = MicCaptureState(error = "No compatible microphone configuration found")
            return
        }
        val recorder = recorderConfig.recorder

        val noiseSuppressor = if (NoiseSuppressor.isAvailable()) {
            NoiseSuppressor.create(recorder.audioSessionId)?.apply { enabled = true }
        } else null
        val echoCanceler = if (AcousticEchoCanceler.isAvailable()) {
            AcousticEchoCanceler.create(recorder.audioSessionId)?.apply { enabled = true }
        } else null
        val udp = try {
            UdpStreamClient(host, port, key, streamId)
        } catch (_: IOException) {
            noiseSuppressor?.release()
            echoCanceler?.release()
            recorder.release()
            _state.value = MicCaptureState(error = "Unable to open microphone stream")
            return
        }
        client = udp

        try {
            repeat(3) { udp.sendHello(ChannelId.MIC) }
            recorder.startRecording()
            _state.value = MicCaptureState(isOn = true)
            val pcm = ByteArray(LaudMonoEncoder.PCM_BYTES_PER_PACKET)
            val resampler = MonoPcmResampler(recorderConfig.sampleRate)
            var sequence = 0
            var timestamp = 0L
            while (isActive) {
                val read = recorder.read(pcm, 0, pcm.size, AudioRecord.READ_BLOCKING)
                if (read <= 0 || read % 2 != 0) continue
                val targetPcm = resampler.process(pcm, read)
                if (targetPcm.isEmpty()) continue
                udp.sendDatagram(
                    ChannelId.MIC,
                    LaudMonoEncoder.encode(targetPcm, targetPcm.size, sequence++, timestamp)
                )
                timestamp += targetPcm.size / 2
            }
        } catch (cancelled: CancellationException) {
            throw cancelled
        } catch (_: IOException) {
            _state.value = MicCaptureState(error = "Microphone stream interrupted")
        } catch (_: SecurityException) {
            _state.value = MicCaptureState(error = "Microphone permission is no longer available")
        } catch (_: IllegalStateException) {
            _state.value = MicCaptureState(error = "Phone microphone became unavailable")
        } finally {
            try {
                recorder.stop()
            } catch (_: IllegalStateException) {
                // Recorder may have failed before recording started.
            }
            noiseSuppressor?.release()
            echoCanceler?.release()
            recorder.release()
            udp.close()
            if (client === udp) client = null
            _state.value = _state.value.copy(isOn = false)
        }
    }

    @SuppressLint("MissingPermission")
    private fun createCompatibleRecorder(): RecorderConfig? {
        val sources = intArrayOf(
            MediaRecorder.AudioSource.VOICE_COMMUNICATION,
            MediaRecorder.AudioSource.MIC
        )
        val sampleRates = intArrayOf(48_000, 44_100, 32_000)
        for (source in sources) {
            for (sampleRate in sampleRates) {
                val minimum = AudioRecord.getMinBufferSize(
                    sampleRate,
                    AudioFormat.CHANNEL_IN_MONO,
                    AudioFormat.ENCODING_PCM_16BIT
                )
                if (minimum <= 0) continue
                val recorder = try {
                    AudioRecord(
                        source,
                        sampleRate,
                        AudioFormat.CHANNEL_IN_MONO,
                        AudioFormat.ENCODING_PCM_16BIT,
                        minimum.coerceAtLeast(LaudMonoEncoder.PCM_BYTES_PER_PACKET * 4)
                    )
                } catch (_: IllegalArgumentException) {
                    continue
                }
                if (recorder.state == AudioRecord.STATE_INITIALIZED) {
                    return RecorderConfig(recorder, sampleRate)
                }
                recorder.release()
            }
        }
        return null
    }

    fun stop() {
        captureJob?.cancel()
        captureJob = null
        client?.close()
        client = null
        _state.value = MicCaptureState()
    }
}
