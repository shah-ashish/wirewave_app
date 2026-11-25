package com.wirewave.features.connection.camera

import android.graphics.ImageFormat
import android.graphics.Rect
import android.graphics.YuvImage
import androidx.camera.core.ImageProxy
import com.wirewave.core.stream.ChannelId
import com.wirewave.core.stream.UdpStreamClient
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.nio.ByteBuffer
import java.util.concurrent.atomic.AtomicInteger

data class CameraStreamState(val isStreaming: Boolean = false, val error: String? = null)

class CameraStreamEngine(
    private val scope: CoroutineScope,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) {
    companion object {
        private const val CHUNK_HEADER_BYTES = 8
        private const val MAX_CHUNK_PAYLOAD_BYTES = 1200
    }

    private val _state = MutableStateFlow(CameraStreamState())
    val state: StateFlow<CameraStreamState> = _state.asStateFlow()
    private var streamClient: UdpStreamClient? = null
    private var activeJob: Job? = null
    private val frameCounter = AtomicInteger(0)

    fun start(host: String, port: Int, key: ByteArray, streamId: Int) {
        stop()
        activeJob = scope.launch(ioDispatcher) {
            try {
                val udp = UdpStreamClient(host, port, key, streamId)
                streamClient = udp
                repeat(3) { udp.sendHello(ChannelId.CAMERA) }
                _state.value = CameraStreamState(isStreaming = true)
            } catch (e: IOException) {
                _state.value = CameraStreamState(error = "Unable to connect camera stream: ${e.message}")
            }
        }
    }

    fun sendFrame(imageProxy: ImageProxy) {
        val client = streamClient ?: run {
            imageProxy.close()
            return
        }

        val jpegBytes = try {
            imageProxyToJpeg(imageProxy)
        } catch (_: Exception) {
            null
        } finally {
            imageProxy.close()
        }

        if (jpegBytes == null || jpegBytes.isEmpty()) return

        val frameId = frameCounter.incrementAndGet()
        scope.launch(ioDispatcher) {
            try {
                sendChunkedFrame(client, frameId, jpegBytes)
            } catch (_: Exception) {
                // Drop frame on transient network error
            }
        }
    }

    private fun sendChunkedFrame(client: UdpStreamClient, frameId: Int, jpeg: ByteArray) {
        val totalChunks = (jpeg.size + MAX_CHUNK_PAYLOAD_BYTES - 1) / MAX_CHUNK_PAYLOAD_BYTES
        if (totalChunks <= 0) return

        val packet = ByteArray(CHUNK_HEADER_BYTES + MAX_CHUNK_PAYLOAD_BYTES)
        var offset = 0
        for (chunkIdx in 0 until totalChunks) {
            val chunkLen = minOf(MAX_CHUNK_PAYLOAD_BYTES, jpeg.size - offset)
            val buf = ByteBuffer.wrap(packet, 0, CHUNK_HEADER_BYTES + chunkLen)
            buf.putInt(frameId)
            buf.putShort(chunkIdx.toShort())
            buf.putShort(totalChunks.toShort())
            System.arraycopy(jpeg, offset, packet, CHUNK_HEADER_BYTES, chunkLen)

            val slice = if (chunkLen == MAX_CHUNK_PAYLOAD_BYTES) packet else packet.copyOf(CHUNK_HEADER_BYTES + chunkLen)
            client.sendDatagram(ChannelId.CAMERA, slice)
            offset += chunkLen
        }
    }

    private fun imageProxyToJpeg(image: ImageProxy): ByteArray? {
        val width = image.width
        val height = image.height
        val yPlane = image.planes[0]
        val uPlane = image.planes[1]
        val vPlane = image.planes[2]

        val nv21 = ByteArray(width * height * 3 / 2)

        // Copy Y plane respecting rowStride
        val yBuf = yPlane.buffer
        val yRowStride = yPlane.rowStride
        var dstPos = 0
        for (row in 0 until height) {
            yBuf.position(row * yRowStride)
            yBuf.get(nv21, dstPos, width)
            dstPos += width
        }

        // Copy interleaved VU for NV21 respecting rowStride and pixelStride
        val uBuf = uPlane.buffer
        val vBuf = vPlane.buffer
        val uvRowStride = uPlane.rowStride
        val uvPixelStride = uPlane.pixelStride
        val halfW = width / 2
        val halfH = height / 2

        for (row in 0 until halfH) {
            for (col in 0 until halfW) {
                val uvIndex = row * uvRowStride + col * uvPixelStride
                nv21[dstPos++] = vBuf.get(uvIndex)
                nv21[dstPos++] = uBuf.get(uvIndex)
            }
        }

        val yuvImage = YuvImage(nv21, ImageFormat.NV21, width, height, null)
        val out = ByteArrayOutputStream()
        val rect = Rect(0, 0, width, height)
        val compressed = yuvImage.compressToJpeg(rect, 85, out)
        return if (compressed) out.toByteArray() else null
    }

    fun stop() {
        activeJob?.cancel()
        activeJob = null
        try {
            streamClient?.close()
        } catch (_: Exception) {}
        streamClient = null
        _state.value = CameraStreamState(isStreaming = false)
    }
}
