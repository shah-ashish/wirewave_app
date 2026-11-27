package com.wirewave.core.session

data class MousePrefs(
    val pointerSpeed: Float = 1.0f,
    val scrollSpeed: Float = 1.0f,
    val naturalScroll: Boolean = true,
    val showMouseButtons: Boolean = true
)
