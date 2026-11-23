//! Liquidmorphism Design System & Theme Engine for MindSet OS (Iced 0.14.0)
//!
//! Provides WCAG AAA frosted glass, Solar Amber, Violet Nebula, Dark Space, and Alpine Glass palettes,
//! high-contrast button styling, hardware-accelerated border glows, and zero window bleed-through.

pub mod claymorphism;
pub mod neumorphism;

pub use claymorphism::{clay_card, clay_orb, ClayPalette};
pub use neumorphism::{neu_card, neu_inset, NeuPalette};

/// Standardized 4-Tier Cognitive Typography Tokens (NERO-ENGAGEING-INSPECTOR Rubric)
pub const TYPE_CAPTION: u32 = 10;   // Eyebrows, categories, timestamps, badges
pub const TYPE_BODY: u32 = 12;      // Primary reading content, body copy, descriptions
pub const TYPE_SUBHEAD: u32 = 15;   // Card headers, section subheads, prompt anchors
pub const TYPE_METRIC: u32 = 22;    // Big metric countdowns, scores, victory XP numbers

use iced::border::Radius;
use iced::widget::{button, container, slider, text_input, Space};
use iced::{Background, Border, Color, Length, Shadow, Vector};
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
                // Authentic translucent frosted glass with anti-glare scrim:
                glass_surface: Color::from_rgba(1.0, 1.0, 1.0, 0.76),
                glass_surface_elevated: Color::from_rgba(1.0, 1.0, 1.0, 0.82),
                glass_border: Color::from_rgba(1.0, 1.0, 1.0, 0.50),
                glass_border_focused: Color::from_rgba(0.12, 0.48, 0.96, 0.90),
                text_primary: Color::from_rgb(0.06, 0.10, 0.20),
                text_secondary: Color::from_rgb(0.24, 0.32, 0.48),
                text_muted: Color::from_rgb(0.42, 0.50, 0.64),
                accent: Color::from_rgb(0.12, 0.48, 0.96),
                accent_hover: Color::from_rgb(0.20, 0.58, 1.00),
                accent_text: Color::from_rgb(1.0, 1.0, 1.0),
                danger: Color::from_rgb(0.92, 0.28, 0.28),
                success: Color::from_rgb(0.10, 0.72, 0.42),
                warning: Color::from_rgb(0.94, 0.62, 0.16),
            },
            ThemeMode::DarkSpace => Self {
                mode,
                background: Color::from_rgb(0.04, 0.06, 0.10),
                // Translucent obsidian frosted glass (76-82% opacity to eliminate wallpaper bleed):
                glass_surface: Color::from_rgba(0.08, 0.11, 0.18, 0.76),
                glass_surface_elevated: Color::from_rgba(0.10, 0.14, 0.24, 0.82),
                glass_border: Color::from_rgba(1.0, 1.0, 1.0, 0.45),
                glass_border_focused: Color::from_rgba(0.40, 0.70, 1.00, 0.80),
                text_primary: Color::from_rgb(0.98, 0.99, 1.0),
                text_secondary: Color::from_rgb(0.70, 0.78, 0.90),
                text_muted: Color::from_rgb(0.48, 0.55, 0.68),
                accent: Color::from_rgb(0.20, 0.60, 1.00), // Sky Cyan
                accent_hover: Color::from_rgb(0.35, 0.70, 1.00),
                accent_text: Color::from_rgb(0.98, 0.99, 1.0),
                danger: Color::from_rgb(0.95, 0.30, 0.30),
                success: Color::from_rgb(0.15, 0.80, 0.50),
                warning: Color::from_rgb(0.96, 0.68, 0.20),
            },
            ThemeMode::VioletNebula => Self {
                mode,
                background: Color::from_rgb(0.06, 0.04, 0.12),
                glass_surface: Color::from_rgba(0.13, 0.09, 0.22, 0.76),
                glass_surface_elevated: Color::from_rgba(0.16, 0.11, 0.28, 0.82),
                glass_border: Color::from_rgba(1.0, 1.0, 1.0, 0.45),
                glass_border_focused: Color::from_rgba(0.75, 0.40, 0.98, 0.85),
                text_primary: Color::from_rgb(0.98, 0.98, 1.0),
                text_secondary: Color::from_rgb(0.78, 0.72, 0.90),
                text_muted: Color::from_rgb(0.55, 0.48, 0.68),
                accent: Color::from_rgb(0.70, 0.30, 0.95), // Violet Nebula
                accent_hover: Color::from_rgb(0.80, 0.42, 1.00),
                accent_text: Color::from_rgb(0.98, 0.98, 1.0),
                danger: Color::from_rgb(0.95, 0.30, 0.30),
                success: Color::from_rgb(0.15, 0.80, 0.50),
                warning: Color::from_rgb(0.96, 0.68, 0.20),
            },
            ThemeMode::SolarAmber => Self {
                mode,
                background: Color::from_rgb(0.06, 0.06, 0.08),
                glass_surface: Color::from_rgba(0.12, 0.11, 0.14, 0.76),
                glass_surface_elevated: Color::from_rgba(0.15, 0.13, 0.18, 0.82),
                glass_border: Color::from_rgba(1.0, 1.0, 1.0, 0.45),
                glass_border_focused: Color::from_rgba(0.96, 0.68, 0.20, 0.85),
                text_primary: Color::from_rgb(0.98, 0.99, 1.0),
                text_secondary: Color::from_rgb(0.82, 0.78, 0.72),
                text_muted: Color::from_rgb(0.58, 0.54, 0.48),
                accent: Color::from_rgb(0.96, 0.68, 0.20), // Solar Amber
                accent_hover: Color::from_rgb(1.00, 0.75, 0.30),
                accent_text: Color::from_rgb(0.067, 0.094, 0.153),
                danger: Color::from_rgb(0.95, 0.30, 0.30),
                success: Color::from_rgb(0.15, 0.80, 0.50),
                warning: Color::from_rgb(0.96, 0.68, 0.20),
            },
        }
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
// Container Styles — High-Opacity Frosted Glass with Layered Shadows
// ============================================================================

pub fn glass_card(palette: ThemePalette) -> container::Style {
    container::Style {
        text_color: Some(palette.text_primary),
        background: Some(Background::Color(palette.glass_surface)),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.38),
            width: 1.0,
            radius: Radius::from(16.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.04, 0.10, 0.22, 0.18),
            offset: Vector::new(1.0, 6.0),
            blur_radius: 20.0,
        },
        ..Default::default()
    }
}

pub fn glass_elevated(palette: ThemePalette) -> container::Style {
    container::Style {
        text_color: Some(palette.text_primary),
        background: Some(Background::Color(palette.glass_surface_elevated)),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.45),
            width: 1.0,
            radius: Radius::from(18.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.02, 0.08, 0.20, 0.24),
            offset: Vector::new(1.5, 8.0),
            blur_radius: 26.0,
        },
        ..Default::default()
    }
}

pub fn glass_panel(palette: ThemePalette) -> container::Style {
    container::Style {
        text_color: Some(palette.text_primary),
        background: Some(Background::Color(palette.glass_surface)),
        border: Border {
            color: palette.glass_border,
            width: 1.0,
            radius: Radius::from(10.0),
        },
        shadow: Shadow::default(),
        ..Default::default()
    }
}

pub fn top_bar_style(palette: ThemePalette) -> container::Style {
    container::Style {
        text_color: Some(palette.text_primary),
        background: Some(Background::Color(palette.glass_surface_elevated)),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.42),
            width: 1.0,
            radius: Radius::from(22.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.02, 0.06, 0.15, 0.18),
            offset: Vector::new(1.0, 5.0),
            blur_radius: 18.0,
        },
        ..Default::default()
    }
}

pub fn dock_style(palette: ThemePalette) -> container::Style {
    container::Style {
        text_color: Some(palette.text_primary),
        background: Some(Background::Color(palette.glass_surface_elevated)),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.40),
            width: 1.2,
            radius: Radius::from(24.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.02, 0.06, 0.15, 0.32),
            offset: Vector::new(0.0, 10.0),
            blur_radius: 30.0,
        },
        ..Default::default()
    }
}

pub fn modal_backdrop() -> container::Style {
    container::Style {
        text_color: None,
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.55))),
        border: Border::default(),
        shadow: Shadow::default(),
        ..Default::default()
    }
}

/// Window chrome style guaranteeing zero bleed-through from desktop widgets.
pub fn window_chrome_style(palette: ThemePalette, is_focused: bool) -> container::Style {
    let border_color = if is_focused {
        palette.glass_border_focused
    } else {
        palette.glass_border
    };

    let bg_color = if palette.mode == ThemeMode::AlpineGlass {
        Color::from_rgba(0.96, 0.98, 1.0, 0.98)
    } else {
        Color::from_rgba(0.08, 0.11, 0.18, 0.98)
    };

    container::Style {
        text_color: Some(palette.text_primary),
        background: Some(Background::Color(bg_color)),
        border: Border {
            color: border_color,
            width: if is_focused { 1.5 } else { 1.0 },
            radius: Radius::from(16.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.02, 0.08, if is_focused { 0.48 } else { 0.22 }),
            offset: Vector::new(0.0, if is_focused { 16.0 } else { 6.0 }),
            blur_radius: if is_focused { 40.0 } else { 20.0 },
        },
        ..Default::default()
    }
}

/// Native OS window titlebar style with top border radii, subtle specular rim, and bottom border.
pub fn window_titlebar_style(palette: ThemePalette, is_focused: bool) -> container::Style {
    let bg_color = if palette.mode == ThemeMode::AlpineGlass {
        Color::from_rgba(0.92, 0.95, 0.99, if is_focused { 0.95 } else { 0.88 })
    } else {
        Color::from_rgba(0.06, 0.08, 0.13, if is_focused { 0.96 } else { 0.88 })
    };

    container::Style {
        text_color: Some(palette.text_primary),
        background: Some(Background::Color(bg_color)),
        border: Border {
            color: if is_focused {
                Color::from_rgba(1.0, 1.0, 1.0, 0.12)
            } else {
                Color::from_rgba(1.0, 1.0, 1.0, 0.05)
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
                let (b, bo) = match status {
                    button::Status::Active => {
                        (Color::from_rgba(1.0, 1.0, 1.0, 0.10), palette.glass_border)
                    }
                    button::Status::Hovered => (
                        Color::from_rgba(1.0, 1.0, 1.0, 0.20),
                        palette.glass_border_focused,
                    ),
                    button::Status::Pressed => (
                        Color::from_rgba(1.0, 1.0, 1.0, 0.28),
                        palette.glass_border_focused,
                    ),
                    button::Status::Disabled => {
                        (Color::from_rgba(1.0, 1.0, 1.0, 0.02), Color::TRANSPARENT)
                    }
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
            button::Status::Disabled => (
                Color::from_rgba(0.4, 0.4, 0.4, 0.25),
                Color::TRANSPARENT,
            ),
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
