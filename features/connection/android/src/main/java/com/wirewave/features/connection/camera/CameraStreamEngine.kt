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

data class CameraStreamState(val isStreaming: Boolean = false, val error: String? = null)

class CameraStreamEngine(
    private val scope: CoroutineScope,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) {
    private val _state = MutableStateFlow(CameraStreamState())
    val state: StateFlow<CameraStreamState> = _state.asStateFlow()
    private var streamClient: UdpStreamClient? = null
    private var activeJob: Job? = null

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

        scope.launch(ioDispatcher) {
            try {
                val jpegBytes = imageProxyToJpeg(imageProxy)
                if (jpegBytes != null && jpegBytes.isNotEmpty()) {
                    client.sendDatagram(ChannelId.CAMERA, jpegBytes)
                }
            } catch (_: Exception) {
                // Drop frame on transient network error
            } finally {
                imageProxy.close()
            }
        }
    }

    private fun imageProxyToJpeg(image: ImageProxy): ByteArray? {
        val yBuffer = image.planes[0].buffer
        val uBuffer = image.planes[1].buffer
        val vBuffer = image.planes[2].buffer

        val ySize = yBuffer.remaining()
        val uSize = uBuffer.remaining()
        val vSize = vBuffer.remaining()

        val nv21 = ByteArray(ySize + uSize + vSize)
        yBuffer.get(nv21, 0, ySize)
        vBuffer.get(nv21, ySize, vSize)
        uBuffer.get(nv21, ySize + vSize, uSize)

        val yuvImage = YuvImage(nv21, ImageFormat.NV21, image.width, image.height, null)
        val out = ByteArrayOutputStream()
        val rect = Rect(0, 0, image.width, image.height)
        val compressed = yuvImage.compressToJpeg(rect, 70, out)
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
