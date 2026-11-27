package com.wirewave.core.session

data class ConnectionPrefs(
    val autoConnect: Boolean = true,
    val keepAliveInBackground: Boolean = true
)
