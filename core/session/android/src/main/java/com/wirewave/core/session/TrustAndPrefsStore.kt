package com.wirewave.core.session

import android.content.Context
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

data class ConnectionPrefs(
    val autoConnect: Boolean = true,
    val keepAliveInBackground: Boolean = true
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

    fun getTrustedDesktop(desktopId: String): TrustedDesktop? {
        val pub = prefs.getString("$TRUST_PREFIX${desktopId}_pub", null)?.trim() ?: return null
        val name = prefs.getString("$TRUST_PREFIX${desktopId}_name", null)?.trim() ?: "Windows PC"
        return TrustedDesktop(desktopId, name, pub)
    }

    fun lastDesktopId(): String? = prefs.getString(KEY_LAST_DESKTOP_ID, null)

    fun setExplicitDisconnect(disconnected: Boolean) {
        prefs.edit().putBoolean(KEY_EXPLICIT_DISCONNECT, disconnected).apply()
    }

    fun wasExplicitlyDisconnected(): Boolean = prefs.getBoolean(KEY_EXPLICIT_DISCONNECT, false)

    private fun loadPrefs(): ConnectionPrefs = ConnectionPrefs(
        autoConnect = prefs.getBoolean(KEY_AUTO_CONNECT, true),
        keepAliveInBackground = prefs.getBoolean(KEY_KEEP_ALIVE, true)
    )

    companion object {
        private const val PREFS_NAME = "wirewave_session_store"
        private const val KEY_AUTO_CONNECT = "auto_connect"
        private const val KEY_KEEP_ALIVE = "keep_alive_bg"
        private const val KEY_LAST_DESKTOP_ID = "last_desktop_id"
        private const val KEY_EXPLICIT_DISCONNECT = "explicit_disconnect"
        private const val TRUST_PREFIX = "trusted_"
    }
}
