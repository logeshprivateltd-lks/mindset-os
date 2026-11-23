//! Primary, Secondary, Outline, Subtle, and Pill button widgets with 5 visual states
//! and spring-animated micro-interactions.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::WidgetState;
use crate::input::InputState;
use crate::layout::LayoutRect;
use crate::theme::Theme;
use mindset_motion::{Spring1D, SpringPreset};

/// Button style variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Subtle,
    Pill,
}

/// Spring animation target values and running spring animators for button micro-interactions.
#[derive(Debug, Clone, Copy)]
pub struct ButtonAnimationTargets {
    pub scale_target: f32,
    pub hover_target: f32,
    pub press_target: f32,
    pub focus_target: f32,
}

impl Default for ButtonAnimationTargets {
    fn default() -> Self {
        Self {
            scale_target: 1.0,
            hover_target: 0.0,
            press_target: 0.0,
            focus_target: 0.0,
        }
    }
}

/// Interactive button widget supporting 5 distinct visual states:
/// Normal, Hover, Active (Pressed), Focused, and Disabled.
pub struct Button {
    pub rect: LayoutRect,
    pub label: String,
    pub variant: ButtonVariant,
    pub focused: bool,
    pub disabled: bool,
    /// Current interaction state.
    pub state: WidgetState,
    /// Hover animation progress 0.0..=1.0 (driven by Spring1D).
    pub hover_t: f32,
    /// Press animation progress 0.0..=1.0 (driven by Spring1D).
    pub press_t: f32,
    /// Focus ring animation progress 0.0..=1.0 (driven by Spring1D).
    pub focus_t: f32,
    /// Current geometric scale (1.0 = normal, 1.03 = hover, 0.96 = pressed).
    pub scale: f32,
    // Internal spring animators (preset: SnappyMicro, stiffness: 420.0, damping: 32.0)
    hover_spring: Spring1D,
    press_spring: Spring1D,
    focus_spring: Spring1D,
    scale_spring: Spring1D,
}

impl Button {
    /// Creates a new button with default dimensions and state.
    pub fn new(rect: LayoutRect, label: impl Into<String>, variant: ButtonVariant) -> Self {
        Self {
            rect,
            label: label.into(),
            variant,
            focused: false,
            disabled: false,
            state: WidgetState::Normal,
            hover_t: 0.0,
            press_t: 0.0,
            focus_t: 0.0,
            scale: 1.0,
            hover_spring: Spring1D::with_preset(0.0, SpringPreset::SnappyMicro),
            press_spring: Spring1D::with_preset(0.0, SpringPreset::SnappyMicro),
            focus_spring: Spring1D::with_preset(0.0, SpringPreset::SnappyMicro),
            scale_spring: Spring1D::with_preset(1.0, SpringPreset::SnappyMicro),
        }
    }

    /// Helper to construct a fully rounded Pill button (`radius = height * 0.5`).
    pub fn new_pill(rect: LayoutRect, label: impl Into<String>) -> Self {
        Self::new(rect, label, ButtonVariant::Pill)
    }

    /// Sets whether the button is disabled.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.state = WidgetState::Disabled;
            self.focused = false;
        } else if self.state == WidgetState::Disabled {
            self.state = WidgetState::Normal;
        }
    }

    /// Updates internal state from cursor and mouse input.
    pub fn update_state(&mut self, input: &InputState) {
        if self.disabled {
            self.state = WidgetState::Disabled;
            return;
        }

        let hovered = input.hovered(self.rect);
        let pressed = hovered && input.left_down;

        if pressed {
            self.state = WidgetState::Pressed;
        } else if hovered {
            self.state = WidgetState::Hover;
        } else if self.focused {
            self.state = WidgetState::Focused;
        } else {
            self.state = WidgetState::Normal;
        }
    }

    /// Computes target animation values according to current visual state.
    pub fn target_values(&self) -> ButtonAnimationTargets {
        if self.disabled {
            return ButtonAnimationTargets {
                scale_target: 1.0,
                hover_target: 0.0,
                press_target: 0.0,
                focus_target: 0.0,
            };
        }

        match self.state {
            WidgetState::Normal => ButtonAnimationTargets {
                scale_target: 1.0,
                hover_target: 0.0,
                press_target: 0.0,
                focus_target: if self.focused { 1.0 } else { 0.0 },
            },
            WidgetState::Hover => ButtonAnimationTargets {
                scale_target: 1.03, // Slight expansion on hover
                hover_target: 1.0,
                press_target: 0.0,
                focus_target: if self.focused { 1.0 } else { 0.0 },
            },
            WidgetState::Pressed => ButtonAnimationTargets {
                scale_target: 0.96, // Tactile depression on press
                hover_target: 1.0,
                press_target: 1.0,
                focus_target: if self.focused { 1.0 } else { 0.0 },
            },
            WidgetState::Focused => ButtonAnimationTargets {
                scale_target: 1.00,
                hover_target: 0.0,
                press_target: 0.0,
                focus_target: 1.0,
            },
            WidgetState::Disabled => ButtonAnimationTargets {
                scale_target: 1.0,
                hover_target: 0.0,
                press_target: 0.0,
                focus_target: 0.0,
            },
            _ => ButtonAnimationTargets::default(),
        }
    }

    /// Advances spring physics by `dt` seconds, respecting `reduced_motion`.
    pub fn update_springs(&mut self, dt: f32, reduced_motion: bool) {
        let targets = self.target_values();

        if reduced_motion {
            self.hover_spring.snap(targets.hover_target);
            self.press_spring.snap(targets.press_target);
            self.focus_spring.snap(targets.focus_target);
            self.scale_spring.snap(targets.scale_target);

            self.hover_t = targets.hover_target;
            self.press_t = targets.press_target;
            self.focus_t = targets.focus_target;
            self.scale = targets.scale_target;
        } else {
            self.hover_spring.set_target(targets.hover_target);
            self.press_spring.set_target(targets.press_target);
            self.focus_spring.set_target(targets.focus_target);
            self.scale_spring.set_target(targets.scale_target);

            self.hover_t = self.hover_spring.update(dt);
            self.press_t = self.press_spring.update(dt);
            self.focus_t = self.focus_spring.update(dt);
            self.scale = self.scale_spring.update(dt);
        }
    }

    fn lerp([a0, a1, a2, a3]: [f32; 4], [b0, b1, b2, b3]: [f32; 4], t: f32) -> [f32; 4] {
        [
            a0 + (b0 - a0) * t,
            a1 + (b1 - a1) * t,
            a2 + (b2 - a2) * t,
            a3 + (b3 - a3) * t,
        ]
    }

    /// Corner radius (Pill uses half-height for fully rounded ends; standard buttons use 8.0px).
    pub fn corner_radius(&self) -> f32 {
        match self.variant {
            ButtonVariant::Pill => self.rect.h * 0.5,
            _ => 8.0,
        }
    }

    /// Overall opacity (0.38 when disabled; 1.0 when active).
    pub fn opacity(&self) -> f32 {
        if self.disabled {
            0.38
        } else {
            1.0
        }
    }

    /// Fill color based on variant, hover/press progress, and current theme.
    pub fn fill_color(&self, theme: &Theme) -> [f32; 4] {
        let p = &theme.palette;
        let mut col = match self.variant {
            ButtonVariant::Primary | ButtonVariant::Pill => {
                let hover = [
                    (p.accent_primary[0] * 1.12).min(1.0),
                    (p.accent_primary[1] * 1.12).min(1.0),
                    (p.accent_primary[2] * 1.12).min(1.0),
                    1.0,
                ];
                let pressed = [
                    p.accent_primary[0] * 0.88,
                    p.accent_primary[1] * 0.88,
                    p.accent_primary[2] * 0.88,
                    1.0,
                ];
                let base = Self::lerp(p.accent_primary, hover, self.hover_t);
                Self::lerp(base, pressed, self.press_t)
            }
            ButtonVariant::Secondary => {
                let base = p.surface_raised;
                let hover = [
                    p.surface_raised[0] + 0.08,
                    p.surface_raised[1] + 0.08,
                    p.surface_raised[2] + 0.08,
                    0.98,
                ];
                let pressed = [
                    p.surface_raised[0] - 0.04,
                    p.surface_raised[1] - 0.04,
                    p.surface_raised[2] - 0.04,
                    1.0,
                ];
                let interim = Self::lerp(base, hover, self.hover_t);
                Self::lerp(interim, pressed, self.press_t)
            }
            ButtonVariant::Outline => {
                let base = [0.0, 0.0, 0.0, 0.0];
                let hover = [
                    p.accent_primary[0],
                    p.accent_primary[1],
                    p.accent_primary[2],
                    0.12,
                ];
                let pressed = [
                    p.accent_primary[0],
                    p.accent_primary[1],
                    p.accent_primary[2],
                    0.22,
                ];
                let interim = Self::lerp(base, hover, self.hover_t);
                Self::lerp(interim, pressed, self.press_t)
            }
            ButtonVariant::Subtle => {
                let base = [0.0, 0.0, 0.0, 0.0];
                let hover = [
                    p.text_secondary[0],
                    p.text_secondary[1],
                    p.text_secondary[2],
                    0.10,
                ];
                let pressed = [
                    p.text_secondary[0],
                    p.text_secondary[1],
                    p.text_secondary[2],
                    0.18,
                ];
                let interim = Self::lerp(base, hover, self.hover_t);
                Self::lerp(interim, pressed, self.press_t)
            }
        };

        if self.disabled {
            col[3] *= 0.38;
        }
        col
    }

    /// Border color based on variant, hover/press progress, and current theme.
    pub fn border_color(&self, theme: &Theme) -> [f32; 4] {
        let p = &theme.palette;
        let mut col = match self.variant {
            ButtonVariant::Primary | ButtonVariant::Pill => {
                let base = [1.0, 1.0, 1.0, 0.25];
                let hover = [1.0, 1.0, 1.0, 0.50];
                Self::lerp(base, hover, self.hover_t)
            }
            ButtonVariant::Secondary => {
                let base = p.border;
                let hover = [
                    p.border[0] * 1.3,
                    p.border[1] * 1.3,
                    p.border[2] * 1.3,
                    0.60,
                ];
                Self::lerp(base, hover, self.hover_t)
            }
            ButtonVariant::Outline => {
                let base = p.accent_primary;
                let hover = [
                    (p.accent_primary[0] * 1.15).min(1.0),
                    (p.accent_primary[1] * 1.15).min(1.0),
                    (p.accent_primary[2] * 1.15).min(1.0),
                    1.0,
                ];
                Self::lerp(base, hover, self.hover_t)
            }
            ButtonVariant::Subtle => [0.0, 0.0, 0.0, 0.0],
        };

        if self.disabled {
            col[3] *= 0.30;
        }
        col
    }

    /// Text color based on variant, theme, and disabled state.
    pub fn text_color(&self, theme: &Theme) -> [f32; 4] {
        let p = &theme.palette;
        let mut col = match self.variant {
            ButtonVariant::Primary | ButtonVariant::Pill => [1.0, 1.0, 1.0, 1.0],
            ButtonVariant::Secondary => p.text_primary,
            ButtonVariant::Outline => p.accent_primary,
            ButtonVariant::Subtle => p.text_secondary,
        };

        if self.disabled {
            col = [p.text_muted[0], p.text_muted[1], p.text_muted[2], 0.45];
        }
        col
    }

    /// High-visibility focus ring outer rectangle (2.0px outward offset).
    pub fn focus_ring_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x - 2.0,
            self.rect.y - 2.0,
            self.rect.w + 4.0,
            self.rect.h + 4.0,
        )
    }

    /// Focus ring color (Solar Amber glow with smooth opacity).
    pub fn focus_ring_color(&self, theme: &Theme) -> [f32; 4] {
        let p = &theme.palette;
        [
            p.accent_primary[0],
            p.accent_primary[1],
            p.accent_primary[2],
            0.85 * self.focus_t,
        ]
    }

    /// Returns true if this button was clicked this frame (disabled buttons ignore clicks).
    pub fn clicked(&self, input: &InputState) -> bool {
        !self.disabled && input.clicked(self.rect)
    }

    /// Returns true if cursor is hovering this button (disabled buttons ignore hover).
    pub fn hovered(&self, input: &InputState) -> bool {
        !self.disabled && input.hovered(self.rect)
    }

    /// Returns true if button is focused and activated by Enter or Space.
    pub fn activate_if_focused(&self, enter_or_space: bool) -> bool {
        !self.disabled && self.focused && enter_or_space
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::ThemeMode;

    #[test]
    fn test_button_colors_and_variants() {
        let theme = Theme::new(ThemeMode::Light);
        let rect = LayoutRect::new(0.0, 0.0, 100.0, 40.0);

        let btn_primary = Button::new(rect, "Primary", ButtonVariant::Primary);
        assert_eq!(btn_primary.text_color(&theme), [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(btn_primary.corner_radius(), 8.0);

        let btn_pill = Button::new_pill(rect, "Pill");
        assert_eq!(btn_pill.corner_radius(), 20.0); // 40.0 * 0.5
        assert_eq!(btn_pill.variant, ButtonVariant::Pill);

        let btn_sec = Button::new(rect, "Secondary", ButtonVariant::Secondary);
        assert_eq!(btn_sec.text_color(&theme), theme.palette.text_primary);

        let btn_outline = Button::new(rect, "Outline", ButtonVariant::Outline);
        assert_eq!(btn_outline.text_color(&theme), theme.palette.accent_primary);
    }

    #[test]
    fn test_button_hover_and_click() {
        let rect = LayoutRect::new(10.0, 10.0, 100.0, 40.0);
        let mut btn = Button::new(rect, "ClickMe", ButtonVariant::Primary);

        let mut input = InputState {
            cursor: (50.0, 25.0),
            ..Default::default()
        };
        assert!(btn.hovered(&input));

        btn.update_state(&input);
        assert_eq!(btn.state, WidgetState::Hover);

        input.left_down = true;
        btn.update_state(&input);
        assert_eq!(btn.state, WidgetState::Pressed);

        input.just_released = true;
        assert!(btn.clicked(&input));

        // Test disabled suppression
        btn.set_disabled(true);
        assert_eq!(btn.state, WidgetState::Disabled);
        assert!(!btn.hovered(&input));
        assert!(!btn.clicked(&input));
    }

    #[test]
    fn test_button_spring_animations() {
        let rect = LayoutRect::new(0.0, 0.0, 100.0, 40.0);
        let mut btn = Button::new(rect, "Animate", ButtonVariant::Primary);

        // Hover transition
        btn.state = WidgetState::Hover;
        for _ in 0..60 {
            btn.update_springs(1.0 / 60.0, false);
        }
        assert!(btn.hover_t > 0.95);
        assert!(btn.scale > 1.02);

        // Press transition
        btn.state = WidgetState::Pressed;
        for _ in 0..60 {
            btn.update_springs(1.0 / 60.0, false);
        }
        assert!(btn.press_t > 0.95);
        assert!(btn.scale < 0.98);

        // Reduced motion instant snap
        btn.state = WidgetState::Normal;
        btn.update_springs(0.016, true);
        assert_eq!(btn.hover_t, 0.0);
        assert_eq!(btn.press_t, 0.0);
        assert_eq!(btn.scale, 1.0);
    }

    #[test]
    fn test_focus_ring_and_keyboard_activation() {
        let rect = LayoutRect::new(10.0, 10.0, 80.0, 32.0);
        let mut btn = Button::new(rect, "FocusBtn", ButtonVariant::Primary);
        btn.focused = true;

        assert!(btn.activate_if_focused(true));
        assert!(!btn.activate_if_focused(false));

        let ring_r = btn.focus_ring_rect();
        assert_eq!(ring_r.x, 8.0);
        assert_eq!(ring_r.y, 8.0);
        assert_eq!(ring_r.w, 84.0);
        assert_eq!(ring_r.h, 36.0);
    }
}
