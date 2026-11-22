package com.wirewave.core.config

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

/** Global OLED theme tokens synced from `config/theme.json` at build time. */
object WireWaveTheme {
    object Colors {
        val bgCanvas = Color(BuildConfig.COLOR_BG_CANVAS)
        val bgSidebar = Color(BuildConfig.COLOR_BG_SIDEBAR)
        val bgCard = Color(BuildConfig.COLOR_BG_CARD)
        val bgCardHover = Color(BuildConfig.COLOR_BG_CARD_HOVER)
        val borderSubtle = Color(BuildConfig.COLOR_BORDER_SUBTLE)
        val divider = Color(BuildConfig.COLOR_DIVIDER)
        val textPrimary = Color(BuildConfig.COLOR_TEXT_PRIMARY)
        val textSecondary = Color(BuildConfig.COLOR_TEXT_SECONDARY)
        val textSection = Color(BuildConfig.COLOR_TEXT_SECTION)
        val accentBlue = Color(BuildConfig.COLOR_ACCENT_BLUE)
        val statusOkBg = Color(BuildConfig.COLOR_STATUS_OK_BG)
        val statusOkFg = Color(BuildConfig.COLOR_STATUS_OK_FG)
        val statusErrBg = Color(BuildConfig.COLOR_STATUS_ERR_BG)
        val statusErrFg = Color(BuildConfig.COLOR_STATUS_ERR_FG)
    }

    object Typography {
        val title = BuildConfig.TYPE_FONT_TITLE.sp
        val cardTitle = BuildConfig.TYPE_FONT_CARD_TITLE.sp
        val body = BuildConfig.TYPE_FONT_BODY.sp
        val mono = BuildConfig.TYPE_FONT_MONO.sp
        val section = BuildConfig.TYPE_FONT_SECTION.sp
        val badge = BuildConfig.TYPE_FONT_BADGE.sp
    }

    object Metrics {
        val radiusCard = BuildConfig.METRIC_RADIUS_CARD.dp
        val radiusPill = BuildConfig.METRIC_RADIUS_PILL.dp
        val spacingXs = BuildConfig.METRIC_SPACING_XS.dp
        val spacingSm = BuildConfig.METRIC_SPACING_SM.dp
        val spacingMd = BuildConfig.METRIC_SPACING_MD.dp
        val spacingLg = BuildConfig.METRIC_SPACING_LG.dp
        val rowHeight = BuildConfig.METRIC_ROW_HEIGHT.dp
    }
}
