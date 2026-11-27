package com.wirewave.core.session

data class TrustedDesktop(
    val desktopId: String,
    val pcName: String,
    val publicKeyHex: String
)
