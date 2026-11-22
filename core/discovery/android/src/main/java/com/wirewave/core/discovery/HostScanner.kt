package com.wirewave.core.discovery

import android.annotation.SuppressLint
import android.bluetooth.BluetoothManager
import android.content.Context
import com.wirewave.core.config.AppConfig
import com.wirewave.core.crypto.WireWaveCrypto
import com.wirewave.core.crypto.toHex
import com.wirewave.core.transport.TransportKind
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONException
import org.json.JSONObject
import java.io.IOException
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.net.SocketTimeoutException

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
        val found = linkedMapOf<String, DiscoveredHost>()
        scanWifiUdp().forEach { host -> found["${host.desktopId}:wifi"] = host }
        scanBluetoothBonded().forEach { host -> found["${host.desktopId}:bt"] = host }
        found.values.toList()
    }

    private fun scanWifiUdp(): List<DiscoveredHost> {
        val results = mutableListOf<DiscoveredHost>()
        val probe = encodeProbe()

        try {
            DatagramSocket().use { socket ->
                socket.broadcast = true
                socket.soTimeout = 1200

                val broadcast = InetAddress.getByName("255.255.255.255")
                val outPacket = DatagramPacket(probe, probe.size, broadcast, AppConfig.connection.udpDiscoveryPort)
                socket.send(outPacket)

                val deadline = System.currentTimeMillis() + 1400L
                val buf = ByteArray(1024)
                while (System.currentTimeMillis() < deadline) {
                    val reply = DatagramPacket(buf, buf.size)
                    try {
                        socket.receive(reply)
                    } catch (_: SocketTimeoutException) {
                        break
                    }
                    val raw = reply.data.copyOf(reply.length)
                    val announcement = decodeSignedPacket(raw) ?: continue
                    val parsed = parseAnnouncement(
                        announcement.payload,
                        reply.address.hostAddress.orEmpty(),
                        announcement.publicKeyHex
                    )
                    if (parsed != null) {
                        results.add(parsed)
                    }
                }
            }
        } catch (_: IOException) {
            // Wi-Fi may be unavailable; Bluetooth scan still proceeds.
        }
        return results
    }

    @SuppressLint("MissingPermission")
    private fun scanBluetoothBonded(): List<DiscoveredHost> {
        val btManager = appContext.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
            ?: return emptyList()
        val adapter = btManager.adapter ?: return emptyList()
        if (!adapter.isEnabled) return emptyList()

        return try {
            adapter.bondedDevices.orEmpty().mapNotNull { device ->
                val name = device.name?.trim()?.takeIf { it.isNotEmpty() } ?: return@mapNotNull null
                val address = device.address ?: return@mapNotNull null
                DiscoveredHost(
                    desktopId = "bt_$address",
                    pcName = name,
                    hostAddress = address,
                    tcpPort = 0,
                    transport = TransportKind.BLUETOOTH,
                    busy = false,
                    publicKeyHex = null
                )
            }
        } catch (_: SecurityException) {
            emptyList()
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
