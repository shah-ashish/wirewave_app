/// Global design tokens compiled from `config/theme.json`.
#[derive(Debug, Clone, Copy)]
pub struct ThemeTokens {
    pub colors: ColorTokens,
    pub typography: TypographyTokens,
    pub metrics: MetricTokens,
}

/// Colors stored in `0xRRGGBBAA` format for direct use with `gpui::rgba(...)`.
#[derive(Debug, Clone, Copy)]
pub struct ColorTokens {
    pub bg_canvas: u32,
    pub bg_sidebar: u32,
    pub bg_card: u32,
    pub bg_card_hover: u32,
    pub bg_nav_active: u32,
    pub border_subtle: u32,
    pub divider: u32,
    pub text_primary: u32,
    pub text_secondary: u32,
    pub text_section: u32,
    pub accent_blue: u32,
    pub accent_blue_hover: u32,
    pub status_ok_bg: u32,
    pub status_ok_fg: u32,
    pub status_err_bg: u32,
    pub status_err_fg: u32,
    pub scrim: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct TypographyTokens {
    pub font_title: f32,
    pub font_card_title: f32,
    pub font_body: f32,
    pub font_mono: f32,
    pub font_section: f32,
    pub font_badge: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct MetricTokens {
    pub radius_card: f32,
    pub radius_nav: f32,
    pub radius_pill: f32,
    pub spacing_xs: f32,
    pub spacing_sm: f32,
    pub spacing_md: f32,
    pub spacing_lg: f32,
    pub row_height: f32,
    pub titlebar_height: f32,
    pub sidebar_width: f32,
    pub content_max_width: f32,
}
