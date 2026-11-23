package com.wirewave.features.speaker

import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioTrack
import android.os.Process
import java.util.concurrent.atomic.AtomicBoolean

/** Dedicated high-priority AudioTrack consumer writing 48 kHz stereo S16LE frames. */
class AudioTrackPlayer(private val jitterBuffer: JitterBuffer) {
    private val running = AtomicBoolean(false)
    private var thread: Thread? = null

    fun start() {
        if (!running.compareAndSet(false, true)) return
        thread = Thread({
            Process.setThreadPriority(Process.THREAD_PRIORITY_URGENT_AUDIO)
            runPlaybackLoop()
        }, "WireWave-Speaker-Play").apply { start() }
    }

    fun stop() {
        running.set(false)
        thread?.join(500)
        thread = null
        jitterBuffer.clear()
    }

    private fun runPlaybackLoop() {
        val minBuf = AudioTrack.getMinBufferSize(
            LaudPacket.TARGET_SAMPLE_RATE,
            AudioFormat.CHANNEL_OUT_STEREO,
            AudioFormat.ENCODING_PCM_16BIT
        ).coerceAtLeast(LaudPacket.PCM_BYTES_PER_PACKET * 4)

        val track = AudioTrack.Builder()
            .setAudioAttributes(
                AudioAttributes.Builder()
                    .setUsage(AudioAttributes.USAGE_MEDIA)
                    .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC)
                    .build()
            )
            .setAudioFormat(
                AudioFormat.Builder()
                    .setEncoding(AudioFormat.ENCODING_PCM_16BIT)
                    .setSampleRate(LaudPacket.TARGET_SAMPLE_RATE)
                    .setChannelMask(AudioFormat.CHANNEL_OUT_STEREO)
                    .build()
            )
            .setTransferMode(AudioTrack.MODE_STREAM)
            .setBufferSizeInBytes(minBuf)
            .setPerformanceMode(AudioTrack.PERFORMANCE_MODE_LOW_LATENCY)
            .build()

        try {
            track.play()
            while (running.get()) {
                val pcm = jitterBuffer.popNext()
                track.write(pcm, 0, pcm.size, AudioTrack.WRITE_BLOCKING)
            }
        } finally {
            track.stop()
            track.release()
        }
    }
}
