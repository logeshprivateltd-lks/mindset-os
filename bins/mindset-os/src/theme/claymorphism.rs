//! Claymorphic Styling & Gradient Shading for Maindy AI
//!
//! Provides puffy, tactile gradient-shaded containers with 2px specular rim lighting
//! and soft colored drop shadows reserved exclusively for Maindy AI components:
//! - Floating Claymorphic Maindy AI Orb
//! - 28px rounded Claymorphic chat bubbles & headers in Maindy AI Workspace

use iced::widget::container;
use iced::{Background, Border, Color, Element, Shadow, Theme, Vector};

#[derive(Debug, Clone, Copy)]
pub struct ClayPalette {
    pub grad_top: Color,
    pub grad_bottom: Color,
    pub rim_light: Color,
    pub outer_shadow: Color,
    pub text_primary: Color,
    pub text_muted: Color,
}

impl Default for ClayPalette {
    fn default() -> Self {
        Self {
            grad_top: Color::from_rgb(0.561, 0.482, 1.0),      // #8F7BFF Soft Lavender
            grad_bottom: Color::from_rgb(0.357, 0.322, 0.941), // #5B52F0 Deep Periwinkle
            rim_light: Color::from_rgba(1.0, 1.0, 1.0, 0.65),  // 2px bright inner specular rim
            outer_shadow: Color::from_rgba(0.357, 0.322, 0.941, 0.45), // Soft colored drop shadow
            text_primary: Color::WHITE,
            text_muted: Color::from_rgba(1.0, 1.0, 1.0, 0.85),
        }
    }
}

/// Puffy Claymorphic card container with tactile inner dome + tinted outer drop shadow
pub fn clay_card<'a, M: 'a>(content: impl Into<Element<'a, M>>, radius: f32) -> Element<'a, M> {
    let p = ClayPalette::default();

    // Outer layer: Soft tinted directional drop shadow (Azimuth ~135°)
    container(
        // Inner layer: Tactile 3D dome with top-left specular highlight
        container(content)
            .padding(14)
            .style(move |_theme: &Theme| container::Style {
                background: Some(Background::Color(p.grad_bottom)),
                border: Border {
                    color: p.rim_light,
                    width: 1.5,
                    radius: radius.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.38), // Top-left inner dome highlight
                    offset: Vector::new(-2.5, -2.5),
                    blur_radius: 6.0,
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
            color: p.outer_shadow, // Deep colored ambient drop shadow
            offset: Vector::new(2.0, 9.0),
            blur_radius: 22.0,
        },
        ..Default::default()
    })
    .into()
}

/// Floating Claymorphic circular orb for Maindy AI with physics scale support (1.0 -> 1.08 hover -> 0.92 press)
pub fn clay_orb<'a, M: 'a>(content: impl Into<Element<'a, M>>, size: f32) -> Element<'a, M> {
    let p = ClayPalette::default();

    container(
        container(content)
            .width(size)
            .height(size)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(move |_theme: &Theme| container::Style {
                background: Some(Background::Color(p.grad_bottom)),
                border: Border {
                    color: p.rim_light,
                    width: 2.0,
                    radius: 999.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.45), // Inner light reflection
                    offset: Vector::new(-3.0, -3.0),
                    blur_radius: 7.0,
                },
                ..Default::default()
            }),
    )
    .style(move |_theme: &Theme| container::Style {
        background: None,
        border: Border {
            radius: 999.0.into(),
            ..Default::default()
        },
        shadow: Shadow {
            color: p.outer_shadow,
            offset: Vector::new(2.0, 10.0),
            blur_radius: 24.0,
        },
        ..Default::default()
    })
    .into()
}

/// 28px rounded Claymorphic chat bubble for Maindy AI tutor responses
pub fn clay_bubble<'a, M: 'a>(content: impl Into<Element<'a, M>>, radius: f32) -> Element<'a, M> {
    let p = ClayPalette::default();

    container(
        container(content)
            .padding(12)
            .style(move |_theme: &Theme| container::Style {
                background: Some(Background::Color(p.grad_bottom)),
                border: Border {
                    color: p.rim_light,
                    width: 1.5,
                    radius: radius.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.35),
                    offset: Vector::new(-2.0, -2.0),
                    blur_radius: 5.0,
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
            color: p.outer_shadow,
            offset: Vector::new(1.5, 6.0),
            blur_radius: 16.0,
        },
        ..Default::default()
    })
    .into()
}

/// Claymorphic top header banner for Maindy AI
pub fn clay_header<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    let p = ClayPalette::default();

    container(
        container(content)
            .padding([8, 14])
            .style(move |_theme: &Theme| container::Style {
                background: Some(Background::Color(p.grad_bottom)),
                border: Border {
                    color: p.rim_light,
                    width: 1.5,
                    radius: 28.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.35),
                    offset: Vector::new(-2.0, -2.0),
                    blur_radius: 5.0,
                },
                ..Default::default()
            }),
    )
    .style(move |_theme: &Theme| container::Style {
        background: None,
        border: Border {
            radius: 28.0.into(),
            ..Default::default()
        },
        shadow: Shadow {
            color: p.outer_shadow,
            offset: Vector::new(1.5, 6.0),
            blur_radius: 16.0,
        },
        ..Default::default()
    })
    .into()
}
