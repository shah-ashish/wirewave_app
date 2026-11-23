package com.wirewave.core.discovery

import android.annotation.SuppressLint
import android.bluetooth.BluetoothClass
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothSocket
import android.content.Context
import com.wirewave.core.config.AppConfig
import com.wirewave.core.crypto.WireWaveCrypto
import com.wirewave.core.crypto.toHex
import com.wirewave.core.transport.TransportKind
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.withContext
import kotlinx.coroutines.withTimeoutOrNull
import org.json.JSONException
import org.json.JSONObject
import java.io.IOException
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.Inet4Address
import java.net.InetAddress
import java.net.NetworkInterface
import java.net.SocketTimeoutException
import java.util.UUID

data class DiscoveredHost(
    val desktopId: String,
    val pcName: String,
    val hostAddress: String,
    val tcpPort: Int,
    val transport: TransportKind,
    val busy: Boolean,
    val publicKeyHex: String? = null
)

/**
 * Discovers WireWave Desktop hosts over Wi-Fi (authenticated binary UDP beacon)
 * and paired Bluetooth Classic adapters.
 */
class HostScanner(
    context: Context,
    private val ioDispatcher: CoroutineDispatcher = Dispatchers.IO
) {
    private val appContext = context.applicationContext

    suspend fun scanAll(): List<DiscoveredHost> = withContext(ioDispatcher) {
        val wifiHosts = scanWifiUdp()
        val wifiPcNames = wifiHosts.map { it.pcName.lowercase() }.toSet()

        val btHosts = scanBluetoothBonded().filter { btHost ->
            btHost.pcName.lowercase() !in wifiPcNames
        }

        wifiHosts + btHosts
    }

    private fun scanWifiUdp(): List<DiscoveredHost> {
        val results = mutableListOf<DiscoveredHost>()
        val probe = encodeProbe()
        val seen = linkedSetOf<String>()

        // Enumerate every active IPv4 interface and send the probe on each one.
        // This is essential when the phone itself is a Wi-Fi hotspot: the default
        // DatagramSocket binds to the mobile-data (cellular) interface and the
        // broadcast never reaches PCs connected to the hotspot. By iterating all
        // interfaces we also cover the AP (hotspot) interface and regular Wi-Fi.
        val interfaces = try {
            NetworkInterface.getNetworkInterfaces()?.toList() ?: emptyList()
        } catch (_: IOException) {
            emptyList()
        }

        val bindAddresses = interfaces
            .filter { iface -> iface.isUp && !iface.isLoopback }
            .flatMap { iface -> iface.interfaceAddresses }
            .mapNotNull { ifAddr ->
                val addr = ifAddr.address
                if (addr is Inet4Address && !addr.isLoopbackAddress) addr else null
            }
            .ifEmpty {
                // Fallback: use the unbound socket (original behaviour).
                listOf(null)
            }

        for (bindAddr in bindAddresses) {
            try {
                val socket = if (bindAddr != null) DatagramSocket(0, bindAddr) else DatagramSocket()
                socket.use {
                    it.broadcast = true
                    it.soTimeout = 1200

                    val broadcast = InetAddress.getByName("255.255.255.255")
                    val outPacket = DatagramPacket(probe, probe.size, broadcast, AppConfig.connection.udpDiscoveryPort)
                    it.send(outPacket)

                    val deadline = System.currentTimeMillis() + 1400L
                    val buf = ByteArray(1024)
                    while (System.currentTimeMillis() < deadline) {
                        val reply = DatagramPacket(buf, buf.size)
                        try {
                            it.receive(reply)
                        } catch (_: SocketTimeoutException) {
                            break
                        }
                        val raw = reply.data.copyOf(reply.length)
                        val announcement = decodeSignedPacket(raw) ?: continue
                        val parsed = parseAnnouncement(
                            announcement.payload,
                            reply.address.hostAddress.orEmpty(),
                            announcement.publicKeyHex
                        ) ?: continue
                        // Deduplicate by desktopId so a multi-homed PC only appears once.
                        val key = parsed.desktopId
                        if (seen.add(key)) results.add(parsed)
                    }
                }
            } catch (_: IOException) {
                // This interface failed; continue to the next one.
            }
        }
        return results
    }

    @SuppressLint("MissingPermission")
    private suspend fun scanBluetoothBonded(): List<DiscoveredHost> = withContext(ioDispatcher) {
        val btManager = appContext.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
            ?: return@withContext emptyList()
        val adapter = btManager.adapter ?: return@withContext emptyList()
        if (!adapter.isEnabled) return@withContext emptyList()

        val serviceUuid = try {
            UUID.fromString(AppConfig.connection.btServiceUuid)
        } catch (_: IllegalArgumentException) {
            return@withContext emptyList()
        }

        // Only scan candidate bonded devices that are computers or uncategorized
        val candidateDevices = try {
            adapter.bondedDevices.orEmpty().filter { device ->
                val majorClass = device.bluetoothClass?.majorDeviceClass
                majorClass == null || majorClass == BluetoothClass.Device.Major.COMPUTER ||
                    majorClass == BluetoothClass.Device.Major.UNCATEGORIZED
            }
        } catch (_: SecurityException) {
            return@withContext emptyList()
        }

        if (candidateDevices.isEmpty()) return@withContext emptyList()

        coroutineScope {
            candidateDevices.map { device ->
                async {
                    verifyWireWaveBluetoothDevice(device, serviceUuid)
                }
            }.awaitAll().filterNotNull()
        }
    }

    @SuppressLint("MissingPermission")
    private suspend fun verifyWireWaveBluetoothDevice(
        device: BluetoothDevice,
        serviceUuid: UUID
    ): DiscoveredHost? = withTimeoutOrNull(800L) {
        val name = device.name?.trim()?.takeIf { it.isNotEmpty() } ?: return@withTimeoutOrNull null
        val address = device.address ?: return@withTimeoutOrNull null

        val cachedUuids = device.uuids
        if (cachedUuids != null && cachedUuids.isNotEmpty()) {
            val matchesWireWave = cachedUuids.any {
                it.uuid.toString().equals(AppConfig.connection.btServiceUuid, ignoreCase = true)
            }
            if (!matchesWireWave) {
                return@withTimeoutOrNull null
            }
        }

        var socket: BluetoothSocket? = null
        try {
            socket = device.createRfcommSocketToServiceRecord(serviceUuid)
            withContext(Dispatchers.IO) {
                socket.connect()
            }
            DiscoveredHost(
                desktopId = "bt_$address",
                pcName = name,
                hostAddress = address,
                tcpPort = 0,
                transport = TransportKind.BLUETOOTH,
                busy = false,
                publicKeyHex = null
            )
        } catch (_: Exception) {
            null
        } finally {
            try {
                socket?.close()
            } catch (_: IOException) {
                // Ignore socket close error
            }
        }
    }

    private fun parseAnnouncement(
        payload: ByteArray,
        hostIp: String,
        publicKeyHex: String
    ): DiscoveredHost? = try {
        val json = JSONObject(String(payload, Charsets.UTF_8))
        val desktopId = json.optString("desktop_id", "").trim()
        val pcName = json.optString("pc_name", "").trim()
        val port = json.optInt("tcp_port", AppConfig.connection.tcpPort)
        val busy = json.optBoolean("busy", false)
        if (desktopId.isEmpty() || pcName.isEmpty() || hostIp.isEmpty()) null
        else DiscoveredHost(
            desktopId,
            pcName,
            hostIp,
            port,
            TransportKind.WIFI,
            busy,
            publicKeyHex
        )
    } catch (_: JSONException) {
        null
    }

    companion object {
        private val BEACON_MAGIC = byteArrayOf('W'.code.toByte(), 'W'.code.toByte(), '2'.code.toByte(), 'D'.code.toByte())
        private const val PUBLIC_KEY_LEN = 32
        private const val SIGNATURE_LEN = 64

        data class SignedAnnouncement(
            val payload: ByteArray,
            val publicKeyHex: String
        )

        fun encodeProbe(): ByteArray {
            return BEACON_MAGIC + "PROBE".toByteArray(Charsets.UTF_8)
        }

        fun decodeSignedPacket(raw: ByteArray): SignedAnnouncement? {
            val headerLength = BEACON_MAGIC.size + PUBLIC_KEY_LEN + SIGNATURE_LEN
            if (raw.size <= headerLength) return null
            if (!raw.copyOfRange(0, 4).contentEquals(BEACON_MAGIC)) return null
            val publicKey = raw.copyOfRange(4, 4 + PUBLIC_KEY_LEN)
            val signature = raw.copyOfRange(
                4 + PUBLIC_KEY_LEN,
                headerLength
            )
            val payload = raw.copyOfRange(headerLength, raw.size)
            val publicKeyHex = publicKey.toHex()
            val verified = WireWaveCrypto.verifyPeerSignature(
                publicKeyHex,
                WireWaveCrypto.discoverySigningPayload(payload),
                signature.toHex()
            )
            return if (verified) SignedAnnouncement(payload, publicKeyHex) else null
        }
    }
}
