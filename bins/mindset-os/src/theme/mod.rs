//! Liquidmorphism Design System & Theme Engine for MindSet OS (Iced 0.14.0)
//!
//! Provides WCAG AAA frosted glass, Solar Amber, Violet Nebula, Dark Space, and Alpine Glass palettes,
//! high-contrast button styling, hardware-accelerated border glows, and zero window bleed-through.

pub mod claymorphism;
pub mod liquidmorphism;
pub mod neumorphism;

pub use claymorphism::{clay_card, clay_orb, ClayPalette};
pub use liquidmorphism::{
    directional_specular_rim, fresnel_sheen, liquid_card, liquid_card_style, liquid_notch,
    liquid_notch_style, liquid_widget, liquid_widget_style, sdf_liquid_normal, sdf_rounded_box,
    LiquidGlassProperties, LiquidMetrics, LiquidPalette,
};
pub use neumorphism::{neu_card, neu_inset, NeuPalette};

/// Standardized 4-Tier Cognitive Typography Tokens (NERO-ENGAGEING-INSPECTOR Rubric)
pub const TYPE_CAPTION: u32 = 10; // Eyebrows, categories, timestamps, badges
pub const TYPE_BODY: u32 = 12; // Primary reading content, body copy, descriptions
pub const TYPE_SUBHEAD: u32 = 15; // Card headers, section subheads, prompt anchors
pub const TYPE_METRIC: u32 = 22; // Big metric countdowns, scores, victory XP numbers

use iced::border::Radius;
use iced::gradient::Linear;
use iced::widget::{button, container, slider, text_input, Space};
use iced::{
    Background, Border, Color, Degrees, Element, Gradient, Length, Padding, Shadow, Vector,
};
use mindset_core::ThemeMode;

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub mode: ThemeMode,
    pub background: Color,
    pub glass_surface: Color,
    pub glass_surface_elevated: Color,
    pub glass_border: Color,
    pub glass_border_focused: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_text: Color,
    pub danger: Color,
    pub success: Color,
    pub warning: Color,
}

impl ThemePalette {
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::AlpineGlass => Self {
                mode,
                background: Color::from_rgb(0.72, 0.84, 0.96),
                // Glacier frosted glass (crisp icy slate base eliminating washed-out milky slabs):
                glass_surface: Color::from_rgba(0.89, 0.93, 0.98, 0.84),
                glass_surface_elevated: Color::from_rgba(0.91, 0.94, 0.98, 0.90),
                glass_border: Color::from_rgba(0.10, 0.18, 0.30, 0.26),
                glass_border_focused: Color::from_rgba(0.08, 0.42, 0.88, 0.92),
                text_primary: Color::from_rgb(0.04, 0.07, 0.14),
                text_secondary: Color::from_rgb(0.11, 0.16, 0.26),
                text_muted: Color::from_rgb(0.16, 0.22, 0.33),
                accent: Color::from_rgb(0.08, 0.44, 0.90),
                accent_hover: Color::from_rgb(0.14, 0.52, 0.96),
                accent_text: Color::from_rgb(1.0, 1.0, 1.0),
                danger: Color::from_rgb(0.82, 0.18, 0.18),
                success: Color::from_rgb(0.08, 0.58, 0.34),
                warning: Color::from_rgb(0.68, 0.36, 0.02),
            },
            ThemeMode::DarkSpace => Self {
                mode,
                background: Color::from_rgb(0.04, 0.06, 0.10),
                // Translucent obsidian frosted glass with adaptive cool tint (>= 76% stability over any wallpaper):
                glass_surface: Color::from_rgba(0.06, 0.09, 0.16, 0.78),
                glass_surface_elevated: Color::from_rgba(0.08, 0.12, 0.21, 0.86),
                glass_border: Color::from_rgba(1.0, 1.0, 1.0, 0.32),
                glass_border_focused: Color::from_rgba(0.35, 0.72, 1.00, 0.88),
                text_primary: Color::from_rgb(0.98, 0.99, 1.0),
                text_secondary: Color::from_rgb(0.82, 0.87, 0.94),
                text_muted: Color::from_rgb(0.67, 0.73, 0.83),
                accent: Color::from_rgb(0.20, 0.62, 1.00), // Sky Cyan
                accent_hover: Color::from_rgb(0.35, 0.72, 1.00),
                accent_text: Color::from_rgb(0.98, 0.99, 1.0),
                danger: Color::from_rgb(0.96, 0.34, 0.34),
                success: Color::from_rgb(0.20, 0.84, 0.54),
                warning: Color::from_rgb(0.98, 0.74, 0.24),
            },
            ThemeMode::VioletNebula => Self {
                mode,
                background: Color::from_rgb(0.06, 0.04, 0.12),
                glass_surface: Color::from_rgba(0.11, 0.08, 0.20, 0.78),
                glass_surface_elevated: Color::from_rgba(0.14, 0.10, 0.25, 0.86),
                glass_border: Color::from_rgba(0.92, 0.84, 1.0, 0.34),
                glass_border_focused: Color::from_rgba(0.75, 0.44, 0.98, 0.90),
                text_primary: Color::from_rgb(0.98, 0.98, 1.0),
                text_secondary: Color::from_rgb(0.86, 0.82, 0.96),
                text_muted: Color::from_rgb(0.72, 0.66, 0.88),
                accent: Color::from_rgb(0.72, 0.34, 0.96),
                accent_hover: Color::from_rgb(0.82, 0.46, 1.00),
                accent_text: Color::from_rgb(0.98, 0.98, 1.0),
                danger: Color::from_rgb(0.96, 0.34, 0.34),
                success: Color::from_rgb(0.20, 0.84, 0.54),
                warning: Color::from_rgb(0.98, 0.74, 0.24),
            },
            ThemeMode::SolarAmber => Self {
                mode,
                background: Color::from_rgb(0.06, 0.06, 0.08),
                glass_surface: Color::from_rgba(0.11, 0.10, 0.13, 0.78),
                glass_surface_elevated: Color::from_rgba(0.14, 0.12, 0.17, 0.86),
                glass_border: Color::from_rgba(1.0, 0.92, 0.78, 0.34),
                glass_border_focused: Color::from_rgba(0.96, 0.68, 0.20, 0.90),
                text_primary: Color::from_rgb(0.98, 0.99, 1.0),
                text_secondary: Color::from_rgb(0.88, 0.84, 0.76),
                text_muted: Color::from_rgb(0.74, 0.69, 0.60),
                accent: Color::from_rgb(0.96, 0.68, 0.20),
                accent_hover: Color::from_rgb(1.00, 0.76, 0.32),
                accent_text: Color::from_rgb(0.067, 0.094, 0.153),
                danger: Color::from_rgb(0.96, 0.34, 0.34),
                success: Color::from_rgb(0.20, 0.84, 0.54),
                warning: Color::from_rgb(0.98, 0.74, 0.24),
            },
        }
    }
}

/// Composites a foreground RGBA color over a background RGB color (`alpha` blending).
pub fn composite_over(fg: Color, bg: Color) -> Color {
    let a = fg.a.clamp(0.0, 1.0);
    Color {
        r: fg.r * a + bg.r * (1.0 - a),
        g: fg.g * a + bg.g * (1.0 - a),
        b: fg.b * a + bg.b * (1.0 - a),
        a: 1.0,
    }
}

/// Computes sRGB relative luminance according to WCAG 2.1.
pub fn relative_luminance(c: Color) -> f32 {
    fn linearize(channel: f32) -> f32 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * linearize(c.r) + 0.7152 * linearize(c.g) + 0.0722 * linearize(c.b)
}

/// Computes WCAG 2.1 contrast ratio between foreground and background colors (`>= 4.5` AA, `>= 7.0` AAA).
pub fn wcag_contrast_ratio(fg: Color, bg: Color) -> f32 {
    let fg_rgb = composite_over(fg, bg);
    let l1 = relative_luminance(fg_rgb);
    let l2 = relative_luminance(bg);
    let (hi, lo) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (hi + 0.05) / (lo + 0.05)
}

/// Blends an adaptive tint layer over a frosted scrim while keeping a subtle alpha channel.
pub fn blend_frost_tint(tint: Color, scrim: Color) -> Color {
    let a = tint.a.clamp(0.0, 1.0);
    Color {
        r: tint.r * a + scrim.r * (1.0 - a),
        g: tint.g * a + scrim.g * (1.0 - a),
        b: tint.b * a + scrim.b * (1.0 - a),
        a: (scrim.a + a * (1.0 - scrim.a)).min(0.94),
    }
}

/// Returns the 24–32% adaptive tint layer color used over frosted glass chrome to guarantee contrast
/// across bright minimalist, dark neon, and busy high-detail wallpapers.
pub fn adaptive_frost_tint(palette: ThemePalette) -> Color {
    match palette.mode {
        ThemeMode::AlpineGlass => Color::from_rgba(0.74, 0.83, 0.95, 0.28), // 28% glacier slate tint
        ThemeMode::DarkSpace => Color::from_rgba(0.06, 0.14, 0.28, 0.28), // 28% obsidian cyan tint
        ThemeMode::VioletNebula => Color::from_rgba(0.18, 0.10, 0.32, 0.28), // 28% nebula tint
        ThemeMode::SolarAmber => Color::from_rgba(0.18, 0.14, 0.08, 0.28), // 28% amber obsidian tint
    }
}

// ============================================================================
// Layout Helpers
// ============================================================================

pub fn h_space() -> Space {
    Space::new().width(Length::Fill)
}

pub fn v_space() -> Space {
    Space::new().height(Length::Fill)
}

pub fn px_space(w: f32, h: f32) -> Space {
    Space::new().width(w).height(h)
}

// ============================================================================
// Container Styles — Frosted Glass Chrome & Matte Neomorphic Windows
// ============================================================================

/// Frosted glass chrome style (floating OS bars, notch, control center, dock, popovers)
/// with a 24–32% adaptive tint layer blended over an elevated frost scrim and specular border.
pub fn frosted_glass_style(palette: ThemePalette, radius: f32, elevated: bool) -> container::Style {
    let scrim = if elevated {
        palette.glass_surface_elevated
    } else {
        palette.glass_surface
    };
    let tint = adaptive_frost_tint(palette);
    let top_left_tint = blend_frost_tint(
        Color {
            a: if elevated { 0.32 } else { 0.28 },
            ..tint
        },
        scrim,
    );
    let bottom_right_tint = blend_frost_tint(Color { a: 0.24, ..tint }, scrim);

    let gradient = Linear::new(Degrees(135.0))
        .add_stop(0.0, top_left_tint)
        .add_stop(1.0, bottom_right_tint);

    let border_color = match (palette.mode, elevated) {
        (ThemeMode::AlpineGlass, true) => Color::from_rgba(1.0, 1.0, 1.0, 0.65),
        (ThemeMode::AlpineGlass, false) => Color::from_rgba(1.0, 1.0, 1.0, 0.50),
        (_, true) => Color::from_rgba(1.0, 1.0, 1.0, 0.26),
        (_, false) => Color::from_rgba(1.0, 1.0, 1.0, 0.18),
    };

    container::Style {
        text_color: Some(palette.text_primary),
        background: Some(Background::Gradient(Gradient::Linear(gradient))),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: Radius::from(radius),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.02, 0.06, 0.16, if elevated { 0.28 } else { 0.20 }),
            offset: Vector::new(0.0, if elevated { 10.0 } else { 6.0 }),
            blur_radius: if elevated { 28.0 } else { 18.0 },
        },
        ..Default::default()
    }
}

/// Wraps floating chrome in a 1px top-left specular rim highlight framing an inner frosted surface.
pub fn frosted_glass_container<'a, M: 'a>(
    palette: ThemePalette,
    radius: f32,
    elevated: bool,
    content: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
    let rim_color = match palette.mode {
        ThemeMode::AlpineGlass => Color::from_rgba(1.0, 1.0, 1.0, 0.78),
        _ => Color::from_rgba(1.0, 1.0, 1.0, 0.28),
    };
    let inner_radius = (radius - 1.0).max(4.0);
    let inner = container(content)
        .style(move |_| frosted_glass_style(palette, inner_radius, elevated))
        .clip(true);

    container(inner)
        .padding(Padding {
            top: 1.0,
            left: 1.0,
            bottom: 0.0,
            right: 0.0,
        })
        .style(move |_| container::Style {
            background: Some(Background::Color(rim_color)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(radius),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.02, 0.06, 0.16, if elevated { 0.28 } else { 0.18 }),
                offset: Vector::new(0.0, if elevated { 10.0 } else { 5.0 }),
                blur_radius: if elevated { 26.0 } else { 16.0 },
            },
            ..Default::default()
        })
        .into()
}

pub fn glass_card(palette: ThemePalette) -> container::Style {
    frosted_glass_style(palette, 16.0, false)
}

pub fn glass_elevated(palette: ThemePalette) -> container::Style {
    frosted_glass_style(palette, 18.0, true)
}

pub fn glass_panel(palette: ThemePalette) -> container::Style {
    frosted_glass_style(palette, 10.0, false)
}

pub fn top_bar_style(palette: ThemePalette) -> container::Style {
    frosted_glass_style(palette, 20.0, true)
}

pub fn dock_style(palette: ThemePalette) -> container::Style {
    frosted_glass_style(palette, 22.0, true)
}

pub fn modal_backdrop() -> container::Style {
    container::Style {
        text_color: None,
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.42))),
        border: Border::default(),
        shadow: Shadow::default(),
        ..Default::default()
    }
}

/// Window chrome style using matte Neomorphism (guaranteed zero bleed-through over desktop).
pub fn window_chrome_style(palette: ThemePalette, is_focused: bool) -> container::Style {
    let neu = NeuPalette::for_theme(palette.mode);
    let border_color = if is_focused {
        palette.glass_border_focused
    } else {
        Color {
            a: 0.35,
            ..neu.shadow_light
        }
    };

    container::Style {
        text_color: Some(neu.text_primary),
        background: Some(Background::Color(neu.surface_base)),
        border: Border {
            color: border_color,
            width: if is_focused { 1.5 } else { 1.0 },
            radius: Radius::from(16.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(
                neu.shadow_dark.r,
                neu.shadow_dark.g,
                neu.shadow_dark.b,
                if is_focused { 0.56 } else { 0.30 },
            ),
            offset: Vector::new(0.0, if is_focused { 14.0 } else { 6.0 }),
            blur_radius: if is_focused { 36.0 } else { 18.0 },
        },
        ..Default::default()
    }
}

/// Native OS window titlebar style using extruded matte Neomorphic surface.
pub fn window_titlebar_style(palette: ThemePalette, is_focused: bool) -> container::Style {
    let neu = NeuPalette::for_theme(palette.mode);
    let bg_color = if is_focused {
        neu.surface_elevated
    } else {
        neu.surface_base
    };

    container::Style {
        text_color: Some(neu.text_primary),
        background: Some(Background::Color(bg_color)),
        border: Border {
            color: Color {
                a: 0.35,
                ..neu.shadow_light
            },
            width: 0.5,
            radius: Radius {
                top_left: 16.0,
                top_right: 16.0,
                bottom_left: 0.0,
                bottom_right: 0.0,
            },
        },
        shadow: Shadow::default(),
        ..Default::default()
    }
}

// ============================================================================
// Button Styles
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    WindowControl(Color),
}

pub fn styled_button(
    palette: ThemePalette,
    variant: ButtonVariant,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border, text_color) = match variant {
            ButtonVariant::Primary => {
                let (b, bo) = match status {
                    button::Status::Active => (palette.accent, palette.glass_border),
                    button::Status::Hovered => (palette.accent_hover, palette.glass_border_focused),
                    button::Status::Pressed => (palette.accent, palette.glass_border_focused),
                    button::Status::Disabled => {
                        (Color::from_rgba(0.3, 0.3, 0.3, 0.5), Color::TRANSPARENT)
                    }
                };
                (b, bo, palette.accent_text)
            }
            ButtonVariant::Secondary => {
                let is_light = palette.mode == ThemeMode::AlpineGlass;
                let (b, bo) = match status {
                    button::Status::Active => (
                        if is_light {
                            Color::from_rgba(0.08, 0.14, 0.26, 0.09)
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.10)
                        },
                        palette.glass_border,
                    ),
                    button::Status::Hovered => (
                        if is_light {
                            Color::from_rgba(0.08, 0.14, 0.26, 0.16)
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.20)
                        },
                        palette.glass_border_focused,
                    ),
                    button::Status::Pressed => (
                        if is_light {
                            Color::from_rgba(0.08, 0.14, 0.26, 0.24)
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.28)
                        },
                        palette.glass_border_focused,
                    ),
                    button::Status::Disabled => (
                        if is_light {
                            Color::from_rgba(0.08, 0.14, 0.26, 0.03)
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.03)
                        },
                        Color::TRANSPARENT,
                    ),
                };
                (b, bo, palette.text_primary)
            }
            ButtonVariant::Danger => {
                let (b, bo) = match status {
                    button::Status::Active => (palette.danger, palette.glass_border),
                    button::Status::Hovered => {
                        (Color::from_rgb(1.0, 0.4, 0.4), palette.glass_border_focused)
                    }
                    button::Status::Pressed => (palette.danger, palette.glass_border_focused),
                    button::Status::Disabled => {
                        (Color::from_rgba(0.4, 0.2, 0.2, 0.5), Color::TRANSPARENT)
                    }
                };
                (b, bo, Color::WHITE)
            }
            ButtonVariant::WindowControl(color) => {
                let b = match status {
                    button::Status::Active => color,
                    button::Status::Hovered => Color {
                        a: (color.a * 1.3).min(1.0),
                        ..color
                    },
                    button::Status::Pressed => Color {
                        a: (color.a * 0.8),
                        ..color
                    },
                    button::Status::Disabled => Color::from_rgba(0.3, 0.3, 0.3, 0.3),
                };
                (b, Color::TRANSPARENT, Color::WHITE)
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color,
            border: Border {
                color: border,
                width: 1.0,
                radius: Radius::from(8.0),
            },
            shadow: Shadow::default(),
            ..Default::default()
        }
    }
}

pub fn primary_button(
    palette: ThemePalette,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    styled_button(palette, ButtonVariant::Primary)
}

pub fn secondary_button(
    palette: ThemePalette,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    styled_button(palette, ButtonVariant::Secondary)
}

pub fn toggle_button(
    palette: ThemePalette,
    is_active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let variant = if is_active {
        ButtonVariant::Primary
    } else {
        ButtonVariant::Secondary
    };
    styled_button(palette, variant)
}

pub fn danger_button(
    palette: ThemePalette,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    styled_button(palette, ButtonVariant::Danger)
}

pub fn window_control_button(
    color: Color,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    styled_button(
        ThemePalette::for_mode(ThemeMode::DarkSpace),
        ButtonVariant::WindowControl(color),
    )
}

/// Native OS Traffic Light circular button style (macOS / NeXT desktop style).
/// Smoothly reveals dark glyph symbols (×, −, +) on hover while showing pure jewel-toned
/// circular dots in normal state, with spatial dimming when the window is not focused.
pub fn traffic_light_button(
    base_color: Color,
    border_color: Color,
    symbol_color: Color,
    is_focused: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, text_c) = match status {
            button::Status::Hovered => (base_color, symbol_color),
            button::Status::Pressed => (
                Color {
                    r: (base_color.r * 0.82),
                    g: (base_color.g * 0.82),
                    b: (base_color.b * 0.82),
                    a: 1.0,
                },
                symbol_color,
            ),
            button::Status::Disabled => (Color::from_rgba(0.4, 0.4, 0.4, 0.25), Color::TRANSPARENT),
            button::Status::Active => {
                let color = if is_focused {
                    base_color
                } else {
                    Color {
                        a: 0.45,
                        ..base_color
                    }
                };
                (color, Color::TRANSPARENT)
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_c,
            border: Border {
                color: if is_focused {
                    border_color
                } else {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.12)
                },
                width: 0.5,
                radius: Radius::from(6.0),
            },
            shadow: Shadow::default(),
            ..Default::default()
        }
    }
}

// ============================================================================
// Slider & Input Styles
// ============================================================================

pub fn modern_slider(
    palette: ThemePalette,
) -> impl Fn(&iced::Theme, slider::Status) -> slider::Style {
    move |_theme, _status| slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Background::Color(palette.accent),
                Background::Color(Color::from_rgba(0.5, 0.6, 0.7, 0.25)),
            ),
            width: 6.0,
            border: Border {
                radius: Radius::from(3.0),
                ..Border::default()
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Background::Color(palette.text_primary),
            border_width: 2.0,
            border_color: palette.accent,
        },
    }
}

pub fn modern_text_input(
    palette: ThemePalette,
) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    move |_theme, status| {
        let border_color = match status {
            text_input::Status::Active => palette.glass_border,
            text_input::Status::Hovered => palette.glass_border_focused,
            text_input::Status::Focused { .. } => palette.accent,
            text_input::Status::Disabled => Color::from_rgba(0.2, 0.2, 0.2, 0.4),
        };
        text_input::Style {
            background: Background::Color(if palette.mode == ThemeMode::AlpineGlass {
                Color::from_rgba(1.0, 1.0, 1.0, 0.85)
            } else {
                Color::from_rgba(0.04, 0.06, 0.10, 0.85)
            }),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: Radius::from(8.0),
            },
            icon: palette.text_secondary,
            placeholder: palette.text_muted,
            value: palette.text_primary,
            selection: Color::from_rgba(0.3, 0.6, 1.0, 0.35),
        }
    }
}
