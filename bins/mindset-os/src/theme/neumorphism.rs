//! Neumorphic Matte Styling & Dual-Shadow Containers for MindSet OS
//!
//! Provides soft matte card elevation (top-left light highlight + bottom-right sculpted shadow)
//! and soft inset wells (scratchpad, pressed toggles) with zero multi-pass GPU shaders.

use iced::widget::container;
use iced::{Background, Border, Color, Element, Shadow, Theme, Vector};
use mindset_core::ThemeMode;

#[derive(Debug, Clone, Copy)]
pub struct NeuPalette {
    pub surface_base: Color,
    pub surface_elevated: Color,
    pub surface_inset: Color,
    pub shadow_light: Color,
    pub shadow_dark: Color,
    pub text_primary: Color,
    pub text_muted: Color,
    pub accent_primary: Color,
    pub status_streak: Color,
    pub status_done: Color,
    pub status_boss: Color,
}

impl NeuPalette {
    /// Alpine Matte Light palette (WCAG AAA contrast over light/alpine wallpaper)
    pub fn light() -> Self {
        Self {
            surface_base: Color::from_rgb(0.91, 0.933, 0.957), // #E8EEF4
            surface_elevated: Color::from_rgb(0.941, 0.957, 0.976), // #F0F4F9
            surface_inset: Color::from_rgb(0.865, 0.895, 0.933), // #DDE4EE
            shadow_light: Color::from_rgba(1.0, 1.0, 1.0, 0.90), // Crisp top-left light highlight
            shadow_dark: Color::from_rgba(0.639, 0.694, 0.776, 0.60), // #A3B1C6 at 0.60
            text_primary: Color::from_rgb(0.067, 0.102, 0.161), // #111A29
            text_muted: Color::from_rgb(0.255, 0.314, 0.392),  // #415064 (>= 6.5:1)
            accent_primary: Color::from_rgb(0.06, 0.44, 0.78), // #0F70C7
            status_streak: Color::from_rgb(0.85, 0.40, 0.06),  // High-contrast Amber-Orange
            status_done: Color::from_rgb(0.08, 0.56, 0.34),    // #148F57
            status_boss: Color::from_rgb(0.70, 0.45, 0.02),    // Deep Amber Gold
        }
    }

    /// Dark Space Obsidian Matte Dark palette
    pub fn dark_space() -> Self {
        Self {
            surface_base: Color::from_rgb(0.106, 0.125, 0.161), // #1B2029
            surface_elevated: Color::from_rgb(0.133, 0.157, 0.204), // #222834
            surface_inset: Color::from_rgb(0.082, 0.098, 0.129), // #151921
            shadow_light: Color::from_rgba(1.0, 1.0, 1.0, 0.08), // Subtle top-left light bevel
            shadow_dark: Color::from_rgba(0.0, 0.0, 0.0, 0.75), // Deep dark drop shadow
            text_primary: Color::from_rgb(0.95, 0.97, 0.99),    // #F2F7FC
            text_muted: Color::from_rgb(0.69, 0.75, 0.84),      // #B0BFD6 (>= 6.5:1)
            accent_primary: Color::from_rgb(0.24, 0.74, 0.96),  // #3DBDF5
            status_streak: Color::from_rgb(0.98, 0.62, 0.28),   // #FB9E47
            status_done: Color::from_rgb(0.24, 0.86, 0.62),     // #3DDB9E
            status_boss: Color::from_rgb(0.98, 0.82, 0.22),     // #FAD138
        }
    }

    /// Violet Nebula theme-tinted dark matte palette
    pub fn violet_nebula() -> Self {
        Self {
            surface_base: Color::from_rgb(0.118, 0.094, 0.165), // #1E182A
            surface_elevated: Color::from_rgb(0.157, 0.125, 0.220), // #282038
            surface_inset: Color::from_rgb(0.086, 0.067, 0.125), // #161120
            shadow_light: Color::from_rgba(1.0, 1.0, 1.0, 0.10),
            shadow_dark: Color::from_rgba(0.0, 0.0, 0.0, 0.80),
            text_primary: Color::from_rgb(0.96, 0.93, 1.0), // #F5EDFF
            text_muted: Color::from_rgb(0.78, 0.68, 0.96),  // #C7ADF5 (>= 7:1)
            accent_primary: Color::from_rgb(0.78, 0.56, 1.0), // #C78FFF
            status_streak: Color::from_rgb(0.98, 0.62, 0.28),
            status_done: Color::from_rgb(0.24, 0.86, 0.62),
            status_boss: Color::from_rgb(0.98, 0.82, 0.22),
        }
    }

    /// Solar Amber theme-tinted dark matte palette
    pub fn solar_amber() -> Self {
        Self {
            surface_base: Color::from_rgb(0.125, 0.110, 0.094), // #201C18
            surface_elevated: Color::from_rgb(0.173, 0.149, 0.125), // #2C2620
            surface_inset: Color::from_rgb(0.090, 0.078, 0.063), // #171410
            shadow_light: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            shadow_dark: Color::from_rgba(0.0, 0.0, 0.0, 0.75),
            text_primary: Color::from_rgb(0.99, 0.96, 0.84), // #FEF5D6
            text_muted: Color::from_rgb(0.92, 0.76, 0.46),   // #EBC275 (>= 7:1)
            accent_primary: Color::from_rgb(0.98, 0.68, 0.14), // #FAAD24
            status_streak: Color::from_rgb(0.98, 0.62, 0.28),
            status_done: Color::from_rgb(0.24, 0.86, 0.62),
            status_boss: Color::from_rgb(0.98, 0.82, 0.22),
        }
    }

    pub fn dark() -> Self {
        Self::dark_space()
    }

    pub fn for_theme(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::AlpineGlass => Self::light(),
            ThemeMode::DarkSpace => Self::dark_space(),
            ThemeMode::VioletNebula => Self::violet_nebula(),
            ThemeMode::SolarAmber => Self::solar_amber(),
        }
    }

    pub fn for_theme_mode(mode_str: &str) -> Self {
        match mode_str {
            "DarkSpace" | "ObsidianMatteDark" => Self::dark_space(),
            "VioletNebula" => Self::violet_nebula(),
            "SolarAmber" => Self::solar_amber(),
            _ => Self::light(),
        }
    }
}

/// Dual-shadow 2-layer container helper for Neumorphic raised cards
pub fn neu_card<'a, M: 'a>(
    content: impl Into<Element<'a, M>>,
    palette: &NeuPalette,
    radius: f32,
    opacity: f32,
) -> Element<'a, M> {
    let surface_base = palette.surface_base;
    let shadow_light = palette.shadow_light;
    let shadow_dark = palette.shadow_dark;

    // Inner layer: matte surface + bottom-right dark depth shadow
    let inner = container(content)
        .padding(14)
        .style(move |_theme: &Theme| container::Style {
            background: Some(Background::Color(Color {
                a: (0.92 * opacity).clamp(0.0, 1.0),
                ..surface_base
            })),
            border: Border {
                color: Color {
                    a: (0.55 * opacity).clamp(0.0, 1.0),
                    ..shadow_light
                },
                width: 1.0,
                radius: radius.into(),
            },
            shadow: Shadow {
                color: Color {
                    a: (shadow_dark.a * opacity).clamp(0.0, 1.0),
                    ..shadow_dark
                },
                offset: Vector::new(5.0, 5.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        });

    // Outer layer: top-left light highlight shadow
    container(inner)
        .style(move |_theme: &Theme| container::Style {
            background: None,
            border: Border {
                radius: radius.into(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color {
                    a: (shadow_light.a * opacity).clamp(0.0, 1.0),
                    ..shadow_light
                },
                offset: Vector::new(-4.0, -4.0),
                blur_radius: 10.0,
            },
            ..Default::default()
        })
        .into()
}

/// Inset container helper for sculpted wells (Scratchpad editor, active toggles)
pub fn neu_inset<'a, M: 'a>(
    content: impl Into<Element<'a, M>>,
    palette: &NeuPalette,
    radius: f32,
) -> Element<'a, M> {
    let surface_inset = palette.surface_inset;
    let shadow_dark = palette.shadow_dark;
    let shadow_light = palette.shadow_light;

    // Outer well: Top-left carved cavity shadow
    container(
        // Inner well: Bottom-right extruded light bevel
        container(content)
            .padding(10)
            .style(move |_theme: &Theme| container::Style {
                background: Some(Background::Color(surface_inset)),
                border: Border {
                    color: Color {
                        a: 0.18,
                        ..shadow_dark
                    },
                    width: 1.0,
                    radius: radius.into(),
                },
                shadow: Shadow {
                    color: Color {
                        a: 0.35,
                        ..shadow_light
                    }, // Bottom-right bevel
                    offset: Vector::new(2.0, 2.0),
                    blur_radius: 4.0,
                },
                ..Default::default()
            }),
    )
    .style(move |_theme: &Theme| container::Style {
        background: None,
        border: Border {
            radius: radius.into(),
            ..Default::default()
        },
        shadow: Shadow {
            color: Color {
                a: 0.40,
                ..shadow_dark
            }, // Top-left recessed cavity
            offset: Vector::new(-2.0, -2.0),
            blur_radius: 5.0,
        },
        ..Default::default()
    })
    .into()
}
