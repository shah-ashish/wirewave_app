package com.wirewave.core.session

import android.annotation.SuppressLint
import android.bluetooth.BluetoothManager
import android.content.Context
import com.wirewave.core.config.AppConfig
import com.wirewave.core.discovery.DiscoveredHost
import com.wirewave.core.transport.TransportKind
import kotlinx.coroutines.CancellationException
import java.io.Closeable
import java.io.IOException
import java.io.InputStream
import java.io.OutputStream
import java.net.InetSocketAddress
import java.net.Socket
import java.util.UUID

/** Owns transport sockets; session authentication and state belong to the caller. */
internal class SessionTransportRunner(
    context: Context,
    private val onConnectionLost: (String) -> Unit,
    private val onFailure: (String) -> Unit,
    private val runProtocol: suspend (DiscoveredHost, InputStream, OutputStream) -> Unit
) : AutoCloseable {
    private val appContext = context.applicationContext
    private var activeConnection: Closeable? = null

    suspend fun run(host: DiscoveredHost) {
        when (host.transport) {
            TransportKind.WIFI -> runWifiSession(host)
            TransportKind.BLUETOOTH -> runBluetoothSession(host)
        }
    }

    override fun close() {
        try {
            activeConnection?.close()
        } catch (_: IOException) {
            // Closing active socket to unblock I/O.
        }
        activeConnection = null
    }

    private suspend fun runWifiSession(host: DiscoveredHost) {
        val socket = Socket()
        activeConnection = socket
        try {
            socket.connect(InetSocketAddress(host.hostAddress, host.tcpPort), 5000)
            socket.soTimeout = AppConfig.connection.heartbeatTimeoutSecs * 1000
            runProtocol(host, socket.getInputStream(), socket.getOutputStream())
        } catch (e: CancellationException) {
            throw e
        } catch (_: IOException) {
            onConnectionLost("Connection lost.")
        } finally {
            try {
                socket.close()
            } catch (_: IOException) {
                // Socket cleanup.
            }
            if (activeConnection === socket) activeConnection = null
        }
    }

    @SuppressLint("MissingPermission")
    private suspend fun runBluetoothSession(host: DiscoveredHost) {
        val manager = appContext.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
            ?: return onFailure("Bluetooth is unavailable.")
        val adapter = manager.adapter ?: return onFailure("Bluetooth is unavailable.")
        val device = try {
            adapter.getRemoteDevice(host.hostAddress)
        } catch (_: IllegalArgumentException) {
            return onFailure("Invalid Bluetooth device address.")
        }
        val socket = try {
            device.createRfcommSocketToServiceRecord(UUID.fromString(AppConfig.connection.btServiceUuid))
        } catch (_: SecurityException) {
            return onFailure("Bluetooth permission is required.")
        }
        activeConnection = socket
        try {
            adapter.cancelDiscovery()
            socket.connect()
            runProtocol(host, socket.inputStream, socket.outputStream)
        } catch (e: CancellationException) {
            throw e
        } catch (_: SecurityException) {
            onFailure("Bluetooth permission is required.")
        } catch (_: IOException) {
            onConnectionLost("Bluetooth connection lost.")
        } finally {
            try {
                socket.close()
            } catch (_: IOException) {
                // Socket cleanup.
            }
            if (activeConnection === socket) activeConnection = null
        }
    }
}
