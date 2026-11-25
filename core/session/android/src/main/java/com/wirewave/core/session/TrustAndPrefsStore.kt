package com.wirewave.core.session

import android.content.Context
import com.wirewave.core.transport.TransportKind
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

data class ConnectionPrefs(
    val autoConnect: Boolean = true,
    val keepAliveInBackground: Boolean = true
)

data class MousePrefs(
    val pointerSpeed: Float = 1.0f,
    val scrollSpeed: Float = 1.0f,
    val naturalScroll: Boolean = true,
    val showMouseButtons: Boolean = true
)

data class TrustedDesktop(
    val desktopId: String,
    val pcName: String,
    val publicKeyHex: String
)

class TrustAndPrefsStore(context: Context) {
    private val prefs = context.applicationContext.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
    private val _connectionPrefs = MutableStateFlow(loadPrefs())
    val connectionPrefs: StateFlow<ConnectionPrefs> = _connectionPrefs.asStateFlow()
    private val _mousePrefs = MutableStateFlow(loadMousePrefs())
    val mousePrefs: StateFlow<MousePrefs> = _mousePrefs.asStateFlow()

    fun setPointerSpeed(speed: Float) {
        val clamped = speed.coerceIn(0.5f, 3.0f)
        prefs.edit().putFloat(KEY_MOUSE_POINTER_SPEED, clamped).apply()
        _mousePrefs.update { it.copy(pointerSpeed = clamped) }
    }

    fun setScrollSpeed(speed: Float) {
        val clamped = speed.coerceIn(0.5f, 3.0f)
        prefs.edit().putFloat(KEY_MOUSE_SCROLL_SPEED, clamped).apply()
        _mousePrefs.update { it.copy(scrollSpeed = clamped) }
    }

    fun setNaturalScroll(natural: Boolean) {
        prefs.edit().putBoolean(KEY_MOUSE_NATURAL_SCROLL, natural).apply()
        _mousePrefs.update { it.copy(naturalScroll = natural) }
    }

    fun setShowMouseButtons(show: Boolean) {
        prefs.edit().putBoolean(KEY_MOUSE_SHOW_BUTTONS, show).apply()
        _mousePrefs.update { it.copy(showMouseButtons = show) }
    }

    fun setAutoConnect(enabled: Boolean) {
        prefs.edit().putBoolean(KEY_AUTO_CONNECT, enabled).apply()
        _connectionPrefs.update { it.copy(autoConnect = enabled) }
    }

    fun setKeepAliveInBackground(enabled: Boolean) {
        prefs.edit().putBoolean(KEY_KEEP_ALIVE, enabled).apply()
        _connectionPrefs.update { it.copy(keepAliveInBackground = enabled) }
    }

    fun saveTrustedDesktop(desktop: TrustedDesktop) {
        prefs.edit()
            .putString("$TRUST_PREFIX${desktop.desktopId}_name", desktop.pcName)
            .putString("$TRUST_PREFIX${desktop.desktopId}_pub", desktop.publicKeyHex)
            .putString(KEY_LAST_DESKTOP_ID, desktop.desktopId)
            .apply()
    }

    fun associateBluetoothAddress(desktopId: String, btAddress: String) {
        if (btAddress.isBlank()) return
        prefs.edit()
            .putString("$BT_PREFIX$btAddress", desktopId)
            .putString("$TRUST_PREFIX${desktopId}_bt", btAddress)
            .apply()
    }

    fun getDesktopIdForBluetoothAddress(btAddress: String): String? {
        return prefs.getString("$BT_PREFIX$btAddress", null)
    }

    fun getTrustedDesktopForHost(desktopId: String, transport: TransportKind, hostAddress: String): TrustedDesktop? {
        val direct = getTrustedDesktop(desktopId)
        if (direct != null) return direct
        if (transport == TransportKind.BLUETOOTH) {
            val mappedId = getDesktopIdForBluetoothAddress(hostAddress) ?: return null
            return getTrustedDesktop(mappedId)
        }
        return null
    }

    fun getTrustedDesktop(desktopId: String): TrustedDesktop? {
        val pub = prefs.getString("$TRUST_PREFIX${desktopId}_pub", null)?.trim() ?: return null
        val name = prefs.getString("$TRUST_PREFIX${desktopId}_name", null)?.trim() ?: "Windows PC"
        return TrustedDesktop(desktopId, name, pub)
    }

    fun lastDesktopId(): String? = prefs.getString(KEY_LAST_DESKTOP_ID, null)

    fun forgetDesktop(desktopId: String) {
        val btAddress = prefs.getString("$TRUST_PREFIX${desktopId}_bt", null)
        val editor = prefs.edit()
            .remove("$TRUST_PREFIX${desktopId}_name")
            .remove("$TRUST_PREFIX${desktopId}_pub")
            .remove("$TRUST_PREFIX${desktopId}_bt")
        if (btAddress != null) {
            editor.remove("$BT_PREFIX$btAddress")
        }
        editor.apply()
        if (lastDesktopId() == desktopId) {
            prefs.edit().remove(KEY_LAST_DESKTOP_ID).apply()
        }
    }

    fun listTrustedDesktops(): List<TrustedDesktop> {
        return prefs.all.keys
            .filter { it.startsWith(TRUST_PREFIX) && it.endsWith("_pub") }
            .mapNotNull { key ->
                val desktopId = key.removePrefix(TRUST_PREFIX).removeSuffix("_pub")
                getTrustedDesktop(desktopId)
            }
    }

    fun setExplicitDisconnect(disconnected: Boolean) {
        prefs.edit().putBoolean(KEY_EXPLICIT_DISCONNECT, disconnected).apply()
    }

    fun wasExplicitlyDisconnected(): Boolean = prefs.getBoolean(KEY_EXPLICIT_DISCONNECT, false)

    private fun loadPrefs(): ConnectionPrefs = ConnectionPrefs(
        autoConnect = prefs.getBoolean(KEY_AUTO_CONNECT, true),
        keepAliveInBackground = prefs.getBoolean(KEY_KEEP_ALIVE, true)
    )

    private fun loadMousePrefs(): MousePrefs = MousePrefs(
        pointerSpeed = prefs.getFloat(KEY_MOUSE_POINTER_SPEED, 1.0f).coerceIn(0.5f, 3.0f),
        scrollSpeed = prefs.getFloat(KEY_MOUSE_SCROLL_SPEED, 1.0f).coerceIn(0.5f, 3.0f),
        naturalScroll = prefs.getBoolean(KEY_MOUSE_NATURAL_SCROLL, true),
        showMouseButtons = prefs.getBoolean(KEY_MOUSE_SHOW_BUTTONS, true)
    )

    companion object {
        private const val PREFS_NAME = "wirewave_session_store"
        private const val KEY_AUTO_CONNECT = "auto_connect"
        private const val KEY_KEEP_ALIVE = "keep_alive_bg"
        private const val KEY_MOUSE_POINTER_SPEED = "mouse_pointer_speed"
        private const val KEY_MOUSE_SCROLL_SPEED = "mouse_scroll_speed"
        private const val KEY_MOUSE_NATURAL_SCROLL = "mouse_natural_scroll"
        private const val KEY_MOUSE_SHOW_BUTTONS = "mouse_show_buttons"
        private const val KEY_LAST_DESKTOP_ID = "last_desktop_id"
        private const val KEY_EXPLICIT_DISCONNECT = "explicit_disconnect"
        private const val TRUST_PREFIX = "trusted_"
        private const val BT_PREFIX = "bt_map_"
    }
}
