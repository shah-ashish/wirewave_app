package com.wirewave.features.mic

import java.io.ByteArrayOutputStream

/** Streaming linear PCM16 mono resampler that preserves phase between reads. */
class MonoPcmResampler(
    sourceRate: Int,
    targetRate: Int = LaudMonoEncoder.TARGET_SAMPLE_RATE
) {
    private val step = sourceRate.coerceAtLeast(1).toDouble() / targetRate.coerceAtLeast(1)
    private val passthrough = sourceRate == targetRate
    private var position = 0.0
    private var previousSample: Short? = null

    fun process(input: ByteArray, length: Int): ByteArray {
        require(length in 0..input.size && length % 2 == 0)
        if (passthrough) return input.copyOf(length)
        val inputFrames = length / 2
        if (inputFrames == 0) return ByteArray(0)

        val prefix = if (previousSample == null) 0 else 1
        val samples = ShortArray(inputFrames + prefix)
        if (prefix == 1) samples[0] = previousSample!!
        for (index in 0 until inputFrames) {
            val offset = index * 2
            samples[index + prefix] = ((input[offset + 1].toInt() shl 8) or
                (input[offset].toInt() and 0xff)).toShort()
        }

        val estimatedFrames = (inputFrames / step).toInt().coerceAtLeast(1)
        val output = ByteArrayOutputStream(estimatedFrames * 2)
        while (position + 1.0 < samples.size) {
            val leftIndex = position.toInt()
            val fraction = position - leftIndex
            val left = samples[leftIndex].toDouble()
            val right = samples[leftIndex + 1].toDouble()
            val value = (left + (right - left) * fraction)
                .toInt()
                .coerceIn(Short.MIN_VALUE.toInt(), Short.MAX_VALUE.toInt())
                .toShort()
            output.write(value.toInt() and 0xff)
            output.write((value.toInt() ushr 8) and 0xff)
            position += step
        }

        position -= (samples.size - 1).toDouble()
        previousSample = samples.last()
        return output.toByteArray()
    }
}
