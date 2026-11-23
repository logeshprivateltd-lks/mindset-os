//! ToggleSwitch, Slider, ProgressBar, StatusBadge, and SquircleBadge interactive controls.
//! Pure Rust hardware-accelerated widgets for MindSet OS liquidmorphic UI.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::input::InputState;
use crate::layout::LayoutRect;
use crate::theme::Theme;
use mindset_motion::{Spring1D, SpringPreset};

// ─── ToggleSwitch ─────────────────────────────────────────────────────────────

/// Color scheme for the active toggle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleColorScheme {
    #[default]
    SolarAmber,
    Blue,
    Emerald,
}

/// iOS-Style Toggle Switch with continuous sliding thumb knob and color interpolation.
pub struct ToggleSwitch {
    pub rect: LayoutRect, // Standard: 44px wide × 24px tall
    pub value: bool,
    pub focused: bool,
    pub disabled: bool,
    pub color_scheme: ToggleColorScheme,
    /// Slide animation progress 0.0=off, 1.0=on (driven by Spring1D).
    pub anim_t: f32,
    /// Internal spring oscillator.
    spring: Spring1D,
}

impl ToggleSwitch {
    pub const TRACK_WIDTH: f32 = 44.0;
    pub const TRACK_HEIGHT: f32 = 24.0;
    pub const KNOB_DIAMETER: f32 = 20.0;
    pub const KNOB_PADDING: f32 = 2.0;

    /// Inactive track color: Slate gray rgba(71, 85, 105, 0.60)
    pub const INACTIVE_TRACK: [f32; 4] = [0.278, 0.333, 0.412, 0.600];
    /// Vibrant Solar Amber active track: #F59E0B
    pub const SOLAR_AMBER_TRACK: [f32; 4] = [0.961, 0.620, 0.043, 1.000];
    /// Vibrant iOS Blue active track: #2563EB
    pub const BLUE_TRACK: [f32; 4] = [0.145, 0.388, 0.922, 1.000];
    /// Vibrant Emerald active track: #10B981
    pub const EMERALD_TRACK: [f32; 4] = [0.063, 0.725, 0.506, 1.000];

    pub fn new(rect: LayoutRect, value: bool) -> Self {
        let init_t = if value { 1.0 } else { 0.0 };
        Self {
            rect,
            value,
            focused: false,
            disabled: false,
            color_scheme: ToggleColorScheme::SolarAmber,
            anim_t: init_t,
            spring: Spring1D::with_preset(init_t, SpringPreset::FluidPanel),
        }
    }

    pub fn with_scheme(mut self, scheme: ToggleColorScheme) -> Self {
        self.color_scheme = scheme;
        self
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.focused = false;
        }
    }

    /// Click the toggle if the pointer was released inside the rect. Returns true if changed.
    pub fn toggle_if_clicked(&mut self, input: &InputState) -> bool {
        if !self.disabled && input.clicked(self.rect) {
            self.value = !self.value;
            true
        } else {
            false
        }
    }

    /// Toggles the switch via keyboard Enter or Space when focused.
    pub fn toggle_if_enter_or_space(&mut self, enter_or_space: bool) -> bool {
        if !self.disabled && self.focused && enter_or_space {
            self.value = !self.value;
            true
        } else {
            false
        }
    }

    /// Update animation progress using the internal spring respecting `reduced_motion`.
    pub fn tick_spring(&mut self, dt: f32, reduced_motion: bool) {
        let target = if self.value { 1.0 } else { 0.0 };
        if reduced_motion {
            self.spring.snap(target);
            self.anim_t = target;
        } else {
            self.spring.set_target(target);
            self.anim_t = self.spring.update(dt);
        }
    }

    /// Legacy external spring updater (retains backward compatibility).
    pub fn update_spring(&mut self, spring: &mut Spring1D, reduced_motion: bool, dt: f32) {
        let target = if self.value { 1.0 } else { 0.0 };
        if reduced_motion {
            spring.snap(target);
            self.anim_t = target;
        } else {
            spring.set_target(target);
            self.anim_t = spring.update(dt);
        }
    }

    /// X position of the sliding thumb center.
    pub fn thumb_x(&self) -> f32 {
        let min_x = self.rect.x + Self::KNOB_PADDING + Self::KNOB_DIAMETER * 0.5;
        let max_x = self.rect.x + self.rect.w - Self::KNOB_PADDING - Self::KNOB_DIAMETER * 0.5;
        min_x + (max_x - min_x) * self.anim_t
    }

    pub fn thumb_y(&self) -> f32 {
        self.rect.y + self.rect.h * 0.5
    }

    /// Continuous sliding thumb knob bounding rectangle.
    pub fn thumb_rect(&self) -> LayoutRect {
        let min_x = self.rect.x + Self::KNOB_PADDING;
        let max_x = self.rect.x + self.rect.w - Self::KNOB_DIAMETER - Self::KNOB_PADDING;
        let kx = min_x + (max_x - min_x) * self.anim_t;
        let ky = self.rect.y + (self.rect.h - Self::KNOB_DIAMETER) * 0.5;
        LayoutRect::new(kx, ky, Self::KNOB_DIAMETER, Self::KNOB_DIAMETER)
    }

    fn lerp([a0, a1, a2, a3]: [f32; 4], [b0, b1, b2, b3]: [f32; 4], t: f32) -> [f32; 4] {
        [
            a0 + (b0 - a0) * t,
            a1 + (b1 - a1) * t,
            a2 + (b2 - a2) * t,
            a3 + (b3 - a3) * t,
        ]
    }

    /// Active accent color depending on the selected color scheme.
    pub fn active_color(&self, theme: &Theme) -> [f32; 4] {
        match self.color_scheme {
            ToggleColorScheme::SolarAmber => theme.palette.accent_primary,
            ToggleColorScheme::Blue => Self::BLUE_TRACK,
            ToggleColorScheme::Emerald => Self::EMERALD_TRACK,
        }
    }

    /// Smoothly interpolated track color from inactive slate gray to vibrant accent.
    pub fn interpolated_track_color(&self, theme: &Theme) -> [f32; 4] {
        let inactive = Self::INACTIVE_TRACK;
        let active = self.active_color(theme);
        let mut col = Self::lerp(inactive, active, self.anim_t);
        if self.disabled {
            col[3] *= 0.40;
        }
        col
    }

    /// Legacy track color (maintained for compatibility).
    pub fn track_color(&self, theme: &Theme) -> [f32; 4] {
        self.interpolated_track_color(theme)
    }

    /// Thumb knob surface color (pure white with subtle opacity reduction if disabled).
    pub fn thumb_color(&self, _theme: &Theme) -> [f32; 4] {
        if self.disabled {
            [1.0, 1.0, 1.0, 0.45]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        }
    }

    /// Specular rim highlight on the sliding thumb knob.
    pub fn thumb_border_color(&self) -> [f32; 4] {
        [0.80, 0.85, 0.90, 0.50]
    }

    /// High-visibility focus ring outer rectangle.
    pub fn focus_ring_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x - 2.0,
            self.rect.y - 2.0,
            self.rect.w + 4.0,
            self.rect.h + 4.0,
        )
    }
}

// ─── Continuous Sliders ────────────────────────────────────────────────────────

/// Specific hardware or OS parameter driven by a slider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderKind {
    #[default]
    Brightness,
    Volume,
    Backlight,
    Custom(&'static str),
}

impl SliderKind {
    /// Associated UI icon symbol.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Brightness => "☀️",
            Self::Volume => "🔊",
            Self::Backlight => "💡",
            Self::Custom(_) => "⚙️",
        }
    }

    /// Semantic accent color for the filled portion of the track.
    pub fn fill_color(&self, theme: &Theme) -> [f32; 4] {
        match self {
            Self::Brightness | Self::Backlight => theme.palette.accent_primary, // Solar Amber
            Self::Volume => [0.220, 0.741, 0.973, 1.000],                       // Sky Blue
            Self::Custom(_) => theme.palette.accent_secondary,
        }
    }
}

/// Dual-track continuous slider with circular thumb knob, drag hit-testing, and value clamping.
pub struct Slider {
    pub rect: LayoutRect,
    /// Current value clamped within 0.0..=1.0.
    pub value: f32,
    pub dragging: bool,
    pub focused: bool,
    pub disabled: bool,
    pub kind: SliderKind,
}

impl Slider {
    pub const TRACK_HEIGHT: f32 = 6.0;
    pub const THUMB_DIAMETER: f32 = 18.0;
    pub const HIT_PADDING_Y: f32 = 8.0;
    pub const HIT_PADDING_X: f32 = 9.0;

    pub fn new(rect: LayoutRect, value: f32) -> Self {
        Self {
            rect,
            value: value.clamp(0.0, 1.0),
            dragging: false,
            focused: false,
            disabled: false,
            kind: SliderKind::Brightness,
        }
    }

    pub fn with_kind(mut self, kind: SliderKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.dragging = false;
            self.focused = false;
        }
    }

    /// Expanded hit-testing bounds for effortless cursor or touch dragging.
    pub fn hit_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x - Self::HIT_PADDING_X,
            self.rect.y - Self::HIT_PADDING_Y,
            self.rect.w + Self::HIT_PADDING_X * 2.0,
            self.rect.h + Self::HIT_PADDING_Y * 2.0,
        )
    }

    /// Inactive background track groove (height 6.0px, centered vertically).
    pub fn track_rect(&self) -> LayoutRect {
        let ty = self.rect.y + (self.rect.h - Self::TRACK_HEIGHT) * 0.5;
        LayoutRect::new(self.rect.x, ty, self.rect.w, Self::TRACK_HEIGHT)
    }

    /// Active filled track portion from left edge to thumb position.
    pub fn fill_rect(&self) -> LayoutRect {
        let tr = self.track_rect();
        let fw = (tr.w * self.value).max(0.0);
        LayoutRect::new(tr.x, tr.y, fw, tr.h)
    }

    /// Center X coordinate of the thumb knob.
    pub fn thumb_x(&self) -> f32 {
        self.rect.x + self.rect.w * self.value
    }

    /// Center Y coordinate of the thumb knob.
    pub fn thumb_y(&self) -> f32 {
        self.rect.y + self.rect.h * 0.5
    }

    /// Circular sliding thumb knob bounding rectangle (18.0px diameter).
    pub fn thumb_rect(&self) -> LayoutRect {
        let tx = self.thumb_x() - Self::THUMB_DIAMETER * 0.5;
        let ty = self.thumb_y() - Self::THUMB_DIAMETER * 0.5;
        LayoutRect::new(tx, ty, Self::THUMB_DIAMETER, Self::THUMB_DIAMETER)
    }

    /// Inactive background groove color.
    pub fn track_color(&self, _theme: &Theme) -> [f32; 4] {
        [0.180, 0.220, 0.300, 0.800]
    }

    /// Active filled groove color.
    pub fn fill_color(&self, theme: &Theme) -> [f32; 4] {
        self.kind.fill_color(theme)
    }

    /// Circular thumb knob surface color.
    pub fn thumb_color(&self) -> [f32; 4] {
        if self.disabled {
            [0.60, 0.60, 0.65, 0.60]
        } else {
            [0.98, 0.98, 1.00, 1.00]
        }
    }

    /// Thumb knob specular rim border color.
    pub fn thumb_border_color(&self) -> [f32; 4] {
        [0.80, 0.85, 0.95, 0.50]
    }

    /// High-visibility focus ring outer rectangle.
    pub fn focus_ring_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x - 2.0,
            self.rect.y - 2.0,
            self.rect.w + 4.0,
            self.rect.h + 4.0,
        )
    }

    /// Formatted display percentage, e.g. "84%".
    pub fn formatted_pct(&self) -> String {
        format!("{:.0}%", self.value * 100.0)
    }

    /// Update drag state and position from input.
    pub fn update(&mut self, input: &InputState) {
        if self.disabled {
            self.dragging = false;
            return;
        }

        let hit = self.hit_rect();
        if input.left_down && (self.dragging || hit.contains(input.cursor.0, input.cursor.1)) {
            self.dragging = true;
            let rel = (input.cursor.0 - self.rect.x) / self.rect.w;
            self.value = rel.clamp(0.0, 1.0);
        } else {
            self.dragging = false;
        }
    }

    pub fn step_left(&mut self, delta: f32) -> f32 {
        if !self.disabled {
            self.value = (self.value - delta).clamp(0.0, 1.0);
        }
        self.value
    }

    pub fn step_right(&mut self, delta: f32) -> f32 {
        if !self.disabled {
            self.value = (self.value + delta).clamp(0.0, 1.0);
        }
        self.value
    }

    /// Handles keyboard Left/Right arrows, PageUp/PageDown, and Home/End.
    pub fn handle_arrow_keys(&mut self, left: bool, right: bool, step: f32) -> bool {
        if self.disabled || !self.focused {
            return false;
        }
        if left {
            self.step_left(step);
            true
        } else if right {
            self.step_right(step);
            true
        } else {
            false
        }
    }

    pub fn fill_ratio(&self) -> f32 {
        self.value
    }
}

// ─── ProgressBar ──────────────────────────────────────────────────────────────

pub struct ProgressBar {
    pub rect: LayoutRect,
    /// Progress value in 0.0..=1.0.
    pub value: f32,
    /// Use success (green) color instead of accent when true.
    pub use_success_color: bool,
}

impl ProgressBar {
    pub fn new(rect: LayoutRect, value: f32) -> Self {
        Self {
            rect,
            value: value.clamp(0.0, 1.0),
            use_success_color: false,
        }
    }

    pub fn fill_color(&self, theme: &Theme) -> [f32; 4] {
        if self.use_success_color {
            theme.palette.success
        } else {
            theme.palette.accent_primary
        }
    }

    pub fn track_color(&self, theme: &Theme) -> [f32; 4] {
        theme.palette.border
    }

    /// Filled portion rect.
    pub fn fill_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.rect.x,
            self.rect.y,
            self.rect.w * self.value,
            self.rect.h,
        )
    }

    /// Formatted percentage label e.g. "73%".
    pub fn label(&self) -> String {
        format!("{:.0}%", self.value * 100.0)
    }
}

// ─── StatusBadge ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeKind {
    Success,
    Warning,
    Error,
    Info,
}

pub struct StatusBadge {
    pub rect: LayoutRect,
    pub symbol: &'static str,
    pub text: &'static str,
    pub kind: BadgeKind,
}

impl StatusBadge {
    pub fn new(
        rect: LayoutRect,
        symbol: &'static str,
        text: &'static str,
        kind: BadgeKind,
    ) -> Self {
        Self {
            rect,
            symbol,
            text,
            kind,
        }
    }

    pub fn color(&self, theme: &Theme) -> [f32; 4] {
        let p = &theme.palette;
        match self.kind {
            BadgeKind::Success => p.success,
            BadgeKind::Warning => p.warning,
            BadgeKind::Error => p.error,
            BadgeKind::Info => p.accent_primary,
        }
    }

    /// Full display label: "{symbol} {text}".
    pub fn full_label(&self) -> String {
        format!("{} {}", self.symbol, self.text)
    }
}

// ─── SquircleBadge ────────────────────────────────────────────────────────────

/// Hardware-accelerated smooth SDF squircle badge with centered icon and specular glint.
#[derive(Debug, Clone)]
pub struct SquircleBadge {
    pub rect: LayoutRect,
    pub symbol: String,
    pub badge_color: [f32; 4],
    pub text_color: [f32; 4],
    pub press_t: f32,
}

impl SquircleBadge {
    pub fn new(rect: LayoutRect, symbol: impl Into<String>, badge_color: [f32; 4]) -> Self {
        Self {
            rect,
            symbol: symbol.into(),
            badge_color,
            text_color: [1.0, 1.0, 1.0, 1.0],
            press_t: 0.0,
        }
    }

    /// Continuous squircle corner radius formula: min(h * 0.28, 14.0).
    pub fn radius(&self) -> f32 {
        (self.rect.h * 0.28).min(14.0)
    }

    /// Specular reflection glint rectangle covering top 35% of the badge.
    pub fn glint_rect(&self) -> LayoutRect {
        let rad = self.radius() * 0.6;
        let gh = (self.rect.h * 0.35).min(14.0);
        LayoutRect::new(self.rect.x + 2.0, self.rect.y + 1.0, self.rect.w - 4.0, gh)
            .inset(rad * 0.2)
    }

    /// Specular glint surface color (subtle 15% white reflection).
    pub fn glint_color(&self) -> [f32; 4] {
        [1.0, 1.0, 1.0, 0.15]
    }

    /// Specular rim highlight matching badge tint.
    pub fn border_rim_color(&self) -> [f32; 4] {
        [
            (self.badge_color[0] * 1.30).min(1.0),
            (self.badge_color[1] * 1.30).min(1.0),
            (self.badge_color[2] * 1.30).min(1.0),
            0.40,
        ]
    }

    /// Proportional icon font size: clamp(h * 0.40, 9.0, 16.0).
    pub fn font_size(&self) -> f32 {
        (self.rect.h * 0.40).clamp(9.0, 16.0)
    }

    /// Centered icon coordinates given measured text width.
    pub fn icon_coords(&self, sym_w: f32) -> (f32, f32) {
        let fs = self.font_size();
        let sx = self.rect.x + (self.rect.w - sym_w) * 0.5;
        let sy = self.rect.y + (self.rect.h - fs) * 0.5 - 1.0;
        (sx, sy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_switch_color_interpolation_and_dimensions() {
        let rect = LayoutRect::new(
            0.0,
            0.0,
            ToggleSwitch::TRACK_WIDTH,
            ToggleSwitch::TRACK_HEIGHT,
        );
        let mut toggle = ToggleSwitch::new(rect, false);

        // Verify inactive color
        let theme = Theme::new(crate::theme::ThemeMode::Dark);
        let inactive_col = toggle.interpolated_track_color(&theme);
        assert!((inactive_col[0] - ToggleSwitch::INACTIVE_TRACK[0]).abs() < 0.01);

        // Turn ON and simulate spring frames
        toggle.value = true;
        for _ in 0..60 {
            toggle.tick_spring(1.0 / 60.0, false);
        }

        assert!(toggle.anim_t > 0.95);
        let active_col = toggle.interpolated_track_color(&theme);
        assert!((active_col[0] - theme.palette.accent_primary[0]).abs() < 0.05);

        let thumb_r = toggle.thumb_rect();
        assert_eq!(thumb_r.w, ToggleSwitch::KNOB_DIAMETER);
        assert_eq!(thumb_r.h, ToggleSwitch::KNOB_DIAMETER);
        assert!(thumb_r.x > rect.x + 18.0);
    }

    #[test]
    fn test_dual_track_slider_clamping_and_kinds() {
        let rect = LayoutRect::new(20.0, 30.0, 200.0, 20.0);
        let mut slider = Slider::new(rect, 0.4).with_kind(SliderKind::Volume);

        assert_eq!(slider.kind.icon(), "🔊");
        assert_eq!(slider.formatted_pct(), "40%");

        let track_r = slider.track_rect();
        assert_eq!(track_r.h, Slider::TRACK_HEIGHT);
        assert_eq!(track_r.w, 200.0);

        let fill_r = slider.fill_rect();
        assert_eq!(fill_r.w, 80.0); // 200 * 0.4

        let thumb_r = slider.thumb_rect();
        assert_eq!(thumb_r.w, Slider::THUMB_DIAMETER);
        assert_eq!(thumb_r.h, Slider::THUMB_DIAMETER);

        // Test clamping via arrow keys
        slider.focused = true;
        slider.handle_arrow_keys(false, true, 0.8);
        assert_eq!(slider.value, 1.0); // Clamped at 1.0

        slider.handle_arrow_keys(true, false, 1.5);
        assert_eq!(slider.value, 0.0); // Clamped at 0.0
    }

    #[test]
    fn test_squircle_badge_geometry() {
        let rect = LayoutRect::new(0.0, 0.0, 36.0, 36.0);
        let badge = SquircleBadge::new(rect, "🌐", [0.961, 0.620, 0.043, 1.0]);

        assert!((badge.radius() - 36.0 * 0.28).abs() < 0.01);
        let glint = badge.glint_rect();
        assert!(glint.w < rect.w);
        assert!(glint.h <= 14.0);

        let (sx, sy) = badge.icon_coords(12.0);
        assert_eq!(sx, 12.0); // (36 - 12) / 2
        assert!(sy > 0.0);
    }
}
