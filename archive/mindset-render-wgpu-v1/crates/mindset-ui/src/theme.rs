//! MindSet OS — Production-Grade Design Token Architecture & Theming Pipeline
//! Defines Primitive Tokens, Semantic Tokens, Typography Scale Tokens, Elevation & Blur Tokens,
//! Corner Radii Tokens, and WCAG 2.1 AA/AAA Contrast Verification for Pure-Rust wgpu 24.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

/// Theme mode selector: LightFrosted (warm daylight) vs DarkObsidian (OLED deep obsidian)
/// Meets the exact specifications of `/mindset-os-ui-rust`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    LightFrosted,
    #[default]
    DarkObsidian,
    /// Deprecated backward-compatible alias for LightFrosted
    #[doc(hidden)]
    Light,
    /// Deprecated backward-compatible alias for DarkObsidian
    #[doc(hidden)]
    Dark,
}

impl PartialEq for ThemeMode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_dark() == other.is_dark()
    }
}

impl Eq for ThemeMode {}

impl ThemeMode {
    #[inline]
    pub const fn is_dark(&self) -> bool {
        matches!(self, ThemeMode::DarkObsidian | ThemeMode::Dark)
    }

    #[inline]
    pub const fn is_light(&self) -> bool {
        matches!(self, ThemeMode::LightFrosted | ThemeMode::Light)
    }

    #[inline]
    pub const fn canonical(&self) -> Self {
        match self {
            ThemeMode::LightFrosted | ThemeMode::Light => ThemeMode::LightFrosted,
            ThemeMode::DarkObsidian | ThemeMode::Dark => ThemeMode::DarkObsidian,
        }
    }
}

/// Accent color options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AccentColor {
    #[default]
    SolarAmber,
    Blue,
    Emerald,
    Indigo,
    Orange,
}

/// RGBA color tuple (normalized linear sRGB 0.0..=1.0)
pub type Rgba = [f32; 4];

// ═══════════════════════════════════════════════════════════════════════════════
// 1. PRIMITIVE TOKENS: Base Color Constants (Hex & Normalized [f32; 4] RGBA)
// ═══════════════════════════════════════════════════════════════════════════════

pub mod primitives {
    use super::Rgba;

    // ── Primitive Color Hex Strings ──────────────────────────────────────────
    pub const HEX_DEEP_CHARCOAL: &str = "#0B0D12";
    pub const HEX_RAISED_GLASS: &str = "#151922";
    pub const HEX_SECONDARY_SURFACE: &str = "#1C2230";
    pub const HEX_SOLAR_AMBER: &str = "#E5AD45";
    pub const HEX_SOLAR_AMBER_GLOW: &str = "#F59E0B";
    pub const HEX_SOLAR_AMBER_DEEP: &str = "#D97706";
    pub const HEX_PRIMARY_TEXT: &str = "#F5F6FA";
    pub const HEX_SECONDARY_TEXT: &str = "#929AAA";
    pub const HEX_IVORY_CANVAS: &str = "#FFFBEB";
    pub const HEX_IVORY_SURFACE: &str = "#FFFDF5";
    pub const HEX_OBSIDIAN_TEXT: &str = "#1C1917";
    pub const HEX_STONE_TEXT: &str = "#57534E";

    // ── Frosted Glass Scrim & Meniscus Primitives (mindset-os-ui-rust) ────────
    pub const HEX_GLASS_LIGHT_WINDOW: &str = "#F8FAFC";
    pub const HEX_GLASS_DARK_WINDOW: &str = "#0F172A";
    pub const HEX_GLASS_LIGHT_CARD: &str = "#FFFFFF";
    pub const HEX_GLASS_DARK_CARD: &str = "#1E293B";
    pub const HEX_GLASS_LIGHT_OVERLAY: &str = "#F1F5F9";
    pub const HEX_GLASS_DARK_OVERLAY: &str = "#090D16";
    pub const HEX_READING_SOLID_LIGHT: &str = "#FAF8F5";
    pub const HEX_READING_SOLID_DARK: &str = "#111827";
    pub const HEX_SLATE_900: &str = "#0F172A";
    pub const HEX_SLATE_50: &str = "#F8FAFC";
    pub const HEX_SLATE_700: &str = "#334155";
    pub const HEX_SLATE_300: &str = "#CBD5E1";
    pub const HEX_SKY_600: &str = "#0284C7";
    pub const HEX_SKY_400: &str = "#38BDF8";
    pub const HEX_AMBER_500: &str = "#F59E0B";
    pub const HEX_AMBER_400: &str = "#FBBF24";

    // ── Base Dark Mode Primitives (Normalized [f32; 4] RGBA) ─────────────────
    /// Deep Charcoal backdrop: #0B0D12 [0.0431, 0.0510, 0.0706, 1.0]
    pub const DEEP_CHARCOAL: Rgba = [0.0431, 0.0510, 0.0706, 1.0000];

    /// Raised Glass surface: #151922 [0.0824, 0.0980, 0.1333, 1.0]
    pub const RAISED_GLASS: Rgba = [0.0824, 0.0980, 0.1333, 1.0000];

    /// Raised Glass Frosted Plate with liquidmorphism transmission (94% opacity)
    pub const RAISED_GLASS_FROSTED: Rgba = [0.0824, 0.0980, 0.1333, 0.9400];

    /// Secondary Surface: #1C2230 [0.1098, 0.1333, 0.1882, 1.0]
    pub const SECONDARY_SURFACE: Rgba = [0.1098, 0.1333, 0.1882, 1.0000];

    /// Secondary Surface Elevated with card plate transmission (96% opacity)
    pub const SECONDARY_SURFACE_RAISED: Rgba = [0.1098, 0.1333, 0.1882, 0.9600];

    /// Solar Amber primary brand accent: #E5AD45 [0.8980, 0.6784, 0.2706, 1.0]
    pub const SOLAR_AMBER: Rgba = [0.8980, 0.6784, 0.2706, 1.0000];

    /// Solar Amber vibrant highlight / focus ring: #F59E0B
    pub const SOLAR_AMBER_GLOW: Rgba = [0.9608, 0.6196, 0.0431, 1.0000];

    /// Solar Amber deep CTA fill for daylight canvas: #D97706
    pub const SOLAR_AMBER_DEEP: Rgba = [0.8510, 0.4667, 0.0235, 1.0000];

    /// Solar Amber soft warm gold glow: #FBBF24
    pub const SOLAR_AMBER_LIGHT: Rgba = [0.9843, 0.7490, 0.1412, 1.0000];

    /// Primary Text: #F5F6FA (OLED high-contrast white)
    pub const PRIMARY_TEXT: Rgba = [0.9608, 0.9647, 0.9804, 1.0000];

    /// Secondary Text: #929AAA (Warm metallic grey)
    pub const SECONDARY_TEXT: Rgba = [0.5725, 0.6039, 0.6667, 1.0000];

    /// Muted Text: #6B7280 (Subtle metadata)
    pub const TEXT_MUTED: Rgba = [0.4196, 0.4471, 0.5020, 1.0000];

    /// Specular Top Rim Reflection: rgba(255, 255, 255, 0.40)
    pub const SPECULAR_RIM_TOP: Rgba = [1.0000, 1.0000, 1.0000, 0.4000];

    /// Specular Bottom Falloff Rim: rgba(255, 255, 255, 0.10)
    pub const SPECULAR_RIM_BOTTOM: Rgba = [1.0000, 1.0000, 1.0000, 0.1000];

    /// Solar Amber Top-Lit Specular Rim: rgba(245, 158, 11, 0.48)
    pub const SPECULAR_RIM_AMBER: Rgba = [0.9608, 0.6196, 0.0431, 0.4800];

    // ── Base Light Mode Primitives (Normalized [f32; 4] RGBA) ────────────────
    /// Warm Ivory daylight canvas: #FFFBEB
    pub const IVORY_CANVAS: Rgba = [1.0000, 0.9843, 0.9216, 1.0000];

    /// Ivory frosted glass backing plate: rgba(255, 253, 245, 0.95)
    pub const IVORY_SURFACE: Rgba = [1.0000, 0.9922, 0.9608, 0.9500];

    /// Soft golden glass card plate: rgba(254, 243, 199, 0.96)
    pub const GOLDEN_GLASS_CARD: Rgba = [0.9961, 0.9529, 0.7804, 0.9600];

    /// Deep Obsidian primary text for light daylight: #1C1917
    pub const OBSIDIAN_TEXT: Rgba = [0.1100, 0.0980, 0.0900, 1.0000];

    /// Warm Stone secondary text for light daylight: #57534E
    pub const STONE_TEXT: Rgba = [0.3412, 0.3255, 0.3059, 1.0000];

    // ── Status & Telemetry Primitives ───────────────────────────────────────
    /// Status Success Emerald (Standard): #10B981
    pub const STATUS_SUCCESS_EMERALD: Rgba = [0.0627, 0.7255, 0.5059, 1.0000];
    /// Status Success Bright (Dark theme indicator): #34D399
    pub const STATUS_SUCCESS_BRIGHT: Rgba = [0.2039, 0.8275, 0.6000, 1.0000];
    /// Status Success Dark (Light theme indicator): #059669
    pub const STATUS_SUCCESS_DARK: Rgba = [0.0196, 0.5882, 0.4118, 1.0000];

    /// Status Warning Amber (Standard): #F59E0B
    pub const STATUS_WARNING_AMBER: Rgba = [0.9608, 0.6196, 0.0431, 1.0000];
    /// Status Warning Bright (Dark theme indicator): #FBBF24
    pub const STATUS_WARNING_BRIGHT: Rgba = [0.9843, 0.7490, 0.1412, 1.0000];

    /// Status Danger Crimson (Standard): #EF4444
    pub const STATUS_DANGER_CRIMSON: Rgba = [0.9373, 0.2667, 0.2667, 1.0000];
    /// Status Danger Bright (Dark theme indicator): #F87171
    pub const STATUS_DANGER_BRIGHT: Rgba = [0.9725, 0.4431, 0.4431, 1.0000];
    /// Status Danger Dark (Light theme indicator): #DC2626
    pub const STATUS_DANGER_DARK: Rgba = [0.8627, 0.1490, 0.1490, 1.0000];

    /// Status Info Sky Cyan (Standard): #0EA5E9
    pub const STATUS_INFO_CYAN: Rgba = [0.0549, 0.6471, 0.9137, 1.0000];
    /// Status Info Bright (Dark theme indicator): #38BDF8
    pub const STATUS_INFO_BRIGHT: Rgba = [0.2196, 0.7412, 0.9725, 1.0000];

    /// Parse integer hex 0xRRGGBB and alpha into normalized [f32; 4].
    pub const fn from_hex_rgb(hex: u32, alpha: f32) -> Rgba {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        [r, g, b, alpha]
    }

    /// Alias matching `/mindset-os-ui-rust` specification.
    pub const fn from_hex_alpha(hex: u32, alpha: f32) -> Rgba {
        from_hex_rgb(hex, alpha)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 2. SEMANTIC TOKENS: Contextual Intent & High-Level Roles
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SemanticTokens {
    /// Base canvas / desktop workspace surface
    pub surface_primary: Rgba,
    /// Raised card, dialog, and panel surface
    pub surface_elevated: Rgba,
    /// Frosted liquid glass backdrop with blur and refraction
    pub surface_frosted: Rgba,
    /// Primary interactive accent, focus ring, active selection
    pub accent_focus: Rgba,
    /// Verified status, healthy telemetry, secure lock
    pub status_success: Rgba,
    /// Quota warning, throttling, pending action
    pub status_warning: Rgba,
    /// Diagnostic alert, blocked content, emergency action
    pub status_danger: Rgba,
    /// Hairline separator, subtle grid lines
    pub border_subtle: Rgba,
    /// Top-lit glass specular refractive highlight rim
    pub border_specular: Rgba,
}

impl SemanticTokens {
    /// Dark Mode Semantic Mapping (Solar Amber Deep Glass)
    pub fn dark() -> Self {
        Self {
            surface_primary: primitives::DEEP_CHARCOAL,
            surface_elevated: primitives::RAISED_GLASS_FROSTED,
            surface_frosted: primitives::SECONDARY_SURFACE_RAISED,
            accent_focus: primitives::SOLAR_AMBER_GLOW,
            status_success: primitives::STATUS_SUCCESS_BRIGHT,
            status_warning: primitives::STATUS_WARNING_BRIGHT,
            status_danger: primitives::STATUS_DANGER_BRIGHT,
            border_subtle: [1.000, 1.000, 1.000, 0.120],
            border_specular: primitives::SPECULAR_RIM_TOP,
        }
    }

    /// Light Mode Semantic Mapping (Solar Amber Warm Daylight)
    pub fn light() -> Self {
        Self {
            surface_primary: primitives::IVORY_CANVAS,
            surface_elevated: primitives::IVORY_SURFACE,
            surface_frosted: primitives::GOLDEN_GLASS_CARD,
            accent_focus: primitives::SOLAR_AMBER_DEEP,
            status_success: primitives::STATUS_SUCCESS_DARK,
            status_warning: primitives::SOLAR_AMBER_DEEP,
            status_danger: primitives::STATUS_DANGER_DARK,
            border_subtle: [0.706, 0.325, 0.035, 0.180],
            border_specular: primitives::SPECULAR_RIM_AMBER,
        }
    }

    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::DarkObsidian | ThemeMode::Dark => Self::dark(),
            ThemeMode::LightFrosted | ThemeMode::Light => Self::light(),
        }
    }

    pub fn apply_warmth(&mut self, warmth: f32) {
        let shift = |c: &mut Rgba| {
            c[0] = (c[0] + warmth * 0.08).clamp(0.0, 1.0);
            c[1] = (c[1] - warmth * 0.03).clamp(0.0, 1.0);
            c[2] = (c[2] - warmth * 0.12).clamp(0.0, 1.0);
        };
        shift(&mut self.surface_primary);
        shift(&mut self.surface_elevated);
        shift(&mut self.surface_frosted);
        shift(&mut self.accent_focus);
        shift(&mut self.status_success);
        shift(&mut self.status_warning);
        shift(&mut self.status_danger);
        shift(&mut self.border_subtle);
        shift(&mut self.border_specular);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 3. TYPOGRAPHY SCALE TOKENS: Logical pt Sizes, Line Heights & Hi-DPI Multipliers
// ═══════════════════════════════════════════════════════════════════════════════

pub mod typography {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum TypographyScaleLevel {
        Display,
        Header1,
        Header2,
        Header3,
        Body,
        BodySmall,
        Caption,
        CaptionSmall,
        Code,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct TypographyToken {
        pub level: TypographyScaleLevel,
        /// Exact logical point size at 1.0x baseline scale
        pub logical_pt: f32,
        /// Line height multiplier (e.g. 1.45 = 145% of font size)
        pub line_height_multiplier: f32,
        /// Optical letter tracking in em units
        pub letter_spacing_em: f32,
        /// OpenType font weight (400 Regular, 500 Medium, 600 SemiBold, 700 Bold)
        pub font_weight: u16,
        /// Whether this style targets monospace rendering (Parley / Cosmic text)
        pub is_monospace: bool,
        /// Minimum point clamp to guarantee absolute legibility under downscaling
        pub min_pt_clamp: f32,
    }

    impl TypographyToken {
        /// Computes crisp physical pixel rendering size given global ui_scale and user font_scale.
        #[inline]
        pub fn compute_effective_pt(&self, ui_scale: f32, font_scale: f32) -> f32 {
            (self.logical_pt * ui_scale * font_scale)
                .round()
                .max(self.min_pt_clamp)
        }

        /// Computes exact layout line height in physical pixels.
        #[inline]
        pub fn line_height_px(&self, ui_scale: f32, font_scale: f32) -> f32 {
            let pt = self.compute_effective_pt(ui_scale, font_scale);
            (pt * self.line_height_multiplier).round()
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TypographyScale {
        pub display: TypographyToken,
        pub header_1: TypographyToken,
        pub header_2: TypographyToken,
        pub header_3: TypographyToken,
        pub body: TypographyToken,
        pub body_small: TypographyToken,
        pub caption: TypographyToken,
        pub caption_small: TypographyToken,
        pub code: TypographyToken,
    }

    impl TypographyScale {
        pub fn standard() -> Self {
            Self {
                display: TypographyToken {
                    level: TypographyScaleLevel::Display,
                    logical_pt: 32.0,
                    line_height_multiplier: 1.25,
                    letter_spacing_em: -0.02,
                    font_weight: 700,
                    is_monospace: false,
                    min_pt_clamp: 24.0,
                },
                header_1: TypographyToken {
                    level: TypographyScaleLevel::Header1,
                    logical_pt: 26.0,
                    line_height_multiplier: 1.30,
                    letter_spacing_em: -0.015,
                    font_weight: 700,
                    is_monospace: false,
                    min_pt_clamp: 20.0,
                },
                header_2: TypographyToken {
                    level: TypographyScaleLevel::Header2,
                    logical_pt: 20.0,
                    line_height_multiplier: 1.35,
                    letter_spacing_em: -0.01,
                    font_weight: 600,
                    is_monospace: false,
                    min_pt_clamp: 16.0,
                },
                header_3: TypographyToken {
                    level: TypographyScaleLevel::Header3,
                    logical_pt: 17.0,
                    line_height_multiplier: 1.40,
                    letter_spacing_em: 0.0,
                    font_weight: 600,
                    is_monospace: false,
                    min_pt_clamp: 14.0,
                },
                body: TypographyToken {
                    level: TypographyScaleLevel::Body,
                    logical_pt: 15.5,
                    line_height_multiplier: 1.45,
                    letter_spacing_em: 0.0,
                    font_weight: 400,
                    is_monospace: false,
                    min_pt_clamp: 14.0,
                },
                body_small: TypographyToken {
                    level: TypographyScaleLevel::BodySmall,
                    logical_pt: 14.0,
                    line_height_multiplier: 1.45,
                    letter_spacing_em: 0.005,
                    font_weight: 400,
                    is_monospace: false,
                    min_pt_clamp: 12.0,
                },
                caption: TypographyToken {
                    level: TypographyScaleLevel::Caption,
                    logical_pt: 13.5,
                    line_height_multiplier: 1.40,
                    letter_spacing_em: 0.01,
                    font_weight: 400,
                    is_monospace: false,
                    min_pt_clamp: 13.0,
                },
                caption_small: TypographyToken {
                    level: TypographyScaleLevel::CaptionSmall,
                    logical_pt: 11.0,
                    line_height_multiplier: 1.35,
                    letter_spacing_em: 0.015,
                    font_weight: 500,
                    is_monospace: false,
                    min_pt_clamp: 10.0,
                },
                code: TypographyToken {
                    level: TypographyScaleLevel::Code,
                    logical_pt: 13.0,
                    line_height_multiplier: 1.50,
                    letter_spacing_em: 0.0,
                    font_weight: 400,
                    is_monospace: true,
                    min_pt_clamp: 11.0,
                },
            }
        }

        pub fn get(&self, level: TypographyScaleLevel) -> TypographyToken {
            match level {
                TypographyScaleLevel::Display => self.display,
                TypographyScaleLevel::Header1 => self.header_1,
                TypographyScaleLevel::Header2 => self.header_2,
                TypographyScaleLevel::Header3 => self.header_3,
                TypographyScaleLevel::Body => self.body,
                TypographyScaleLevel::BodySmall => self.body_small,
                TypographyScaleLevel::Caption => self.caption,
                TypographyScaleLevel::CaptionSmall => self.caption_small,
                TypographyScaleLevel::Code => self.code,
            }
        }
    }

    impl Default for TypographyScale {
        fn default() -> Self {
            Self::standard()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 4. ELEVATION, BLUR & CORNER RADII TOKENS (Dual-Kawase & liquid_glass.wgsl)
// ═══════════════════════════════════════════════════════════════════════════════

pub mod elevation {
    use super::{primitives, Rgba, ThemeMode};
    use serde::{Deserialize, Serialize};

    /// Exact corner radii across MindSet OS surfaces.
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct CornerRadii {
        /// Small badges, telemetry tags, micro-chips (12px)
        pub badge: f32,
        /// Standard cards, diagnostic rows, search tiles (14px)
        pub card: f32,
        /// Windows, app surfaces, modals (16px)
        pub window: f32,
        /// Action buttons, segmented pills, status pills (18px)
        pub pill: f32,
        /// Floating Control Center panel (22px)
        pub control_center: f32,
        /// Floating bottom dock & main shell containers (24px)
        pub dock: f32,
        /// Fully rounded circular chips, toggles, avatars (999px)
        pub fully_rounded: f32,
    }

    impl CornerRadii {
        pub const fn standard() -> Self {
            Self {
                badge: 12.0,
                card: 14.0,
                window: 16.0,
                pill: 18.0,
                control_center: 22.0,
                dock: 24.0,
                fully_rounded: 999.0,
            }
        }
    }

    impl Default for CornerRadii {
        fn default() -> Self {
            Self::standard()
        }
    }

    /// Dual-Kawase multi-pass GPU blur configuration
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct DualKawaseParams {
        pub passes: u32,
        pub base_offset: f32,
        pub blur_radius: f32,
        pub luminance_clamp: f32,
    }

    /// Liquidmorphism shader vertex parameters (matches liquid_glass.wgsl in.params)
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct LiquidGlassParams {
        /// in.params.x: inner specular border stroke width in logical pixels
        pub border_width: f32,
        /// in.params.y: smoothstep drop shadow blur radius
        pub shadow_blur: f32,
        /// in.params.z: vertical drop shadow offset
        pub shadow_offset_y: f32,
        /// in.params.w: background blur mix factor (0.0 = solid, 1.0 = full liquid glass)
        pub glass_mix: f32,
    }

    impl LiquidGlassParams {
        pub fn as_params_vec4(&self) -> [f32; 4] {
            [
                self.border_width,
                self.shadow_blur,
                self.shadow_offset_y,
                self.glass_mix,
            ]
        }
    }

    /// Top-to-bottom inner specular highlight gradient
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct SpecularHighlight {
        pub border_top: Rgba,
        pub border_bottom: Rgba,
        pub border_width: f32,
    }

    impl SpecularHighlight {
        /// Evaluates specular gradient at vertical fraction t (0.0 = top, 1.0 = bottom)
        pub fn sample_gradient(&self, t: f32) -> Rgba {
            if t <= 0.0 {
                return self.border_top;
            }
            if t >= 1.0 {
                return self.border_bottom;
            }
            [
                self.border_top[0] + (self.border_bottom[0] - self.border_top[0]) * t,
                self.border_top[1] + (self.border_bottom[1] - self.border_top[1]) * t,
                self.border_top[2] + (self.border_bottom[2] - self.border_top[2]) * t,
                self.border_top[3] + (self.border_bottom[3] - self.border_top[3]) * t,
            ]
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum ElevationLevel {
        Level0Flat,
        Level1SubtleCard,
        Level2WindowSurface,
        Level3FloatingDock,
    }

    /// Combined elevation token bundle for rendering pipeline
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct ElevationTokens {
        pub blur: DualKawaseParams,
        pub glass: LiquidGlassParams,
        pub specular: SpecularHighlight,
    }

    impl ElevationTokens {
        pub fn for_level(level: ElevationLevel, mode: ThemeMode) -> Self {
            let (top_spec, bot_spec) = match mode {
                ThemeMode::DarkObsidian | ThemeMode::Dark => (
                    primitives::SPECULAR_RIM_TOP,
                    primitives::SPECULAR_RIM_BOTTOM,
                ),
                ThemeMode::LightFrosted | ThemeMode::Light => {
                    (primitives::SPECULAR_RIM_AMBER, [0.851, 0.467, 0.024, 0.120])
                }
            };

            match level {
                ElevationLevel::Level0Flat => Self {
                    blur: DualKawaseParams {
                        passes: 0,
                        base_offset: 0.0,
                        blur_radius: 0.0,
                        luminance_clamp: 1.0,
                    },
                    glass: LiquidGlassParams {
                        border_width: 0.0,
                        shadow_blur: 0.0,
                        shadow_offset_y: 0.0,
                        glass_mix: 0.0,
                    },
                    specular: SpecularHighlight {
                        border_top: [0.0; 4],
                        border_bottom: [0.0; 4],
                        border_width: 0.0,
                    },
                },
                ElevationLevel::Level1SubtleCard => Self {
                    blur: DualKawaseParams {
                        passes: 2,
                        base_offset: 1.5,
                        blur_radius: 14.0,
                        luminance_clamp: 0.95,
                    },
                    glass: LiquidGlassParams {
                        border_width: 1.0,
                        shadow_blur: 8.0,
                        shadow_offset_y: 2.0,
                        glass_mix: 0.70,
                    },
                    specular: SpecularHighlight {
                        border_top: top_spec,
                        border_bottom: bot_spec,
                        border_width: 1.0,
                    },
                },
                ElevationLevel::Level2WindowSurface => Self {
                    blur: DualKawaseParams {
                        passes: 3,
                        base_offset: 2.0,
                        blur_radius: 28.0,
                        luminance_clamp: 0.92,
                    },
                    glass: LiquidGlassParams {
                        border_width: 1.0,
                        shadow_blur: 16.0,
                        shadow_offset_y: 6.0,
                        glass_mix: 0.85,
                    },
                    specular: SpecularHighlight {
                        border_top: top_spec,
                        border_bottom: bot_spec,
                        border_width: 1.0,
                    },
                },
                ElevationLevel::Level3FloatingDock => Self {
                    blur: DualKawaseParams {
                        passes: 4,
                        base_offset: 2.5,
                        blur_radius: 36.0,
                        luminance_clamp: 0.88,
                    },
                    glass: LiquidGlassParams {
                        border_width: 1.25,
                        shadow_blur: 24.0,
                        shadow_offset_y: 10.0,
                        glass_mix: 0.92,
                    },
                    specular: SpecularHighlight {
                        border_top: top_spec,
                        border_bottom: bot_spec,
                        border_width: 1.25,
                    },
                },
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 4b. FROSTED GLASS TOKENS: WCAG AAA Adaptive Luminance Scrims & Meniscus Rims
// ═══════════════════════════════════════════════════════════════════════════════

/// Frosted Glass & Visual Design Tokens conforming to `/mindset-os-ui-rust`.
/// Guarantees WCAG AAA (>= 7.0:1) contrast over bright wallpapers.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GlassTokens {
    /// Multi-window shell scrim; blocks wallpaper saturation bleed
    pub window_scrim: Rgba,
    /// Doctor repair rows, desktop widgets, Control Center tiles
    pub card_scrim: Rgba,
    /// Top bar, floating Dock, App Launcher, Ctrl+K palette
    pub overlay_scrim: Rgba,
    /// Inner document canvas for Study Reader, Notes, and Word
    pub reading_sheet_solid: Rgba,
    /// Primary headings & labels
    pub text_primary: Rgba,
    /// Subtitles, telemetry metadata
    pub text_secondary: Rgba,
    /// Active toggles, sliders, primary CTAs
    pub accent_primary: Rgba,
    /// High-contrast foreground text for accent_primary
    pub accent_primary_fg: Rgba,
    /// One-Click Repair warning CTA fill
    pub accent_warning_cta: Rgba,
    /// Alias for accent_warning_cta matching skill template
    pub warning_cta_bg: Rgba,
    /// High-contrast ink for warning CTA pill
    pub warning_cta_fg: Rgba,
    /// Directional optical meniscus rim top-left specular highlight
    pub specular_rim_top: Rgba,
    /// Directional optical meniscus rim bottom-right specular shadow
    pub specular_rim_bottom: Rgba,
    /// Standard Dual-Kawase blur radius in pixels (24.0px)
    pub blur_radius_px: f32,
    /// High-frequency dithering amplitude to prevent 8-bit banding
    pub noise_dither_amp: f32,
}

impl GlassTokens {
    /// Light Frosted Mode:
    /// - window_scrim: #F8FAFC @ 80% + 24px Dual-Kawase blur
    /// - card_scrim: #FFFFFF @ 72%
    /// - overlay_scrim: #F1F5F9 @ 88%
    /// - reading_sheet_solid: #FAF8F5 @ 98% (warm paper)
    /// - text_primary: #0F172A (Slate 900) (~15.5:1)
    /// - text_secondary: #334155 (Slate 700) (~9.0:1)
    /// - accent_primary: #0284C7 (Sky 600) + #FFFFFF text
    /// - accent_warning_cta: #F59E0B (Amber 500) + #020617 ink (~9.6:1)
    /// - specular rim: Top-left #FFFFFF 65% / Bottom-right #94A3B8 35%
    pub const LIGHT: Self = Self {
        window_scrim: primitives::from_hex_rgb(0xF8FAFC, 0.80),
        card_scrim: primitives::from_hex_rgb(0xFFFFFF, 0.72),
        overlay_scrim: primitives::from_hex_rgb(0xF1F5F9, 0.88),
        reading_sheet_solid: primitives::from_hex_rgb(0xFAF8F5, 0.98),
        text_primary: primitives::from_hex_rgb(0x0F172A, 1.00),
        text_secondary: primitives::from_hex_rgb(0x334155, 1.00),
        accent_primary: primitives::from_hex_rgb(0x0284C7, 1.00),
        accent_primary_fg: primitives::from_hex_rgb(0xFFFFFF, 1.00),
        accent_warning_cta: primitives::from_hex_rgb(0xF59E0B, 1.00),
        warning_cta_bg: primitives::from_hex_rgb(0xF59E0B, 1.00),
        warning_cta_fg: primitives::from_hex_rgb(0x020617, 1.00),
        specular_rim_top: primitives::from_hex_rgb(0xFFFFFF, 0.65),
        specular_rim_bottom: primitives::from_hex_rgb(0x94A3B8, 0.35),
        blur_radius_px: 24.0,
        noise_dither_amp: 0.015,
    };

    /// Dark Obsidian Mode:
    /// - window_scrim: #0F172A @ 82% + 24px Dual-Kawase blur
    /// - card_scrim: #1E293B @ 74%
    /// - overlay_scrim: #090D16 @ 88%
    /// - reading_sheet_solid: #111827 @ 98% (matte slate)
    /// - text_primary: #F8FAFC (Slate 50) (~11.5:1)
    /// - text_secondary: #CBD5E1 (Slate 300) (~8.1:1)
    /// - accent_primary: #38BDF8 (Sky 400) + #082F49 text
    /// - accent_warning_cta: #FBBF24 (Amber 400) + #020617 ink (~12.3:1)
    /// - specular rim: Top-left #FFFFFF 18% / Bottom-right #000000 45%
    pub const DARK: Self = Self {
        window_scrim: primitives::from_hex_rgb(0x0F172A, 0.82),
        card_scrim: primitives::from_hex_rgb(0x1E293B, 0.74),
        overlay_scrim: primitives::from_hex_rgb(0x090D16, 0.88),
        reading_sheet_solid: primitives::from_hex_rgb(0x111827, 0.98),
        text_primary: primitives::from_hex_rgb(0xF8FAFC, 1.00),
        text_secondary: primitives::from_hex_rgb(0xCBD5E1, 1.00),
        accent_primary: primitives::from_hex_rgb(0x38BDF8, 1.00),
        accent_primary_fg: primitives::from_hex_rgb(0x082F49, 1.00),
        accent_warning_cta: primitives::from_hex_rgb(0xFBBF24, 1.00),
        warning_cta_bg: primitives::from_hex_rgb(0xFBBF24, 1.00),
        warning_cta_fg: primitives::from_hex_rgb(0x020617, 1.00),
        specular_rim_top: primitives::from_hex_rgb(0xFFFFFF, 0.18),
        specular_rim_bottom: primitives::from_hex_rgb(0x000000, 0.45),
        blur_radius_px: 24.0,
        noise_dither_amp: 0.012,
    };

    pub const fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::LightFrosted | ThemeMode::Light => Self::LIGHT,
            ThemeMode::DarkObsidian | ThemeMode::Dark => Self::DARK,
        }
    }

    /// Evaluates composited surface color over a bright wallpaper background
    pub fn composite_over(&self, scrim: Rgba, wallpaper: Rgba) -> Rgba {
        let alpha = scrim[3];
        [
            scrim[0] * alpha + wallpaper[0] * (1.0 - alpha),
            scrim[1] * alpha + wallpaper[1] * (1.0 - alpha),
            scrim[2] * alpha + wallpaper[2] * (1.0 - alpha),
            1.0,
        ]
    }

    /// Evaluates specular rim color along the sun vector alignment [-0.6, -0.8]
    /// (sun_align = 1.0 is full specular top-left, 0.0 is shadowed bottom-right)
    pub fn specular_rim_at(&self, sun_align: f32) -> Rgba {
        let t = sun_align.clamp(0.0, 1.0);
        [
            self.specular_rim_bottom[0]
                + (self.specular_rim_top[0] - self.specular_rim_bottom[0]) * t,
            self.specular_rim_bottom[1]
                + (self.specular_rim_top[1] - self.specular_rim_bottom[1]) * t,
            self.specular_rim_bottom[2]
                + (self.specular_rim_top[2] - self.specular_rim_bottom[2]) * t,
            self.specular_rim_bottom[3]
                + (self.specular_rim_top[3] - self.specular_rim_bottom[3]) * t,
        ]
    }

    /// Evaluates WCAG AAA (>= 7.0:1) contrast guarantee over bright wallpapers.
    /// Returns vector of (label, contrast_ratio, meets_aaa).
    pub fn verify_wcag_aaa_contrast(&self, wallpaper: Rgba) -> Vec<(&'static str, f32, bool)> {
        let window_bg = self.composite_over(self.window_scrim, wallpaper);
        let card_bg = self.composite_over(self.card_scrim, window_bg);
        let overlay_bg = self.composite_over(self.overlay_scrim, wallpaper);
        let reading_bg = self.composite_over(self.reading_sheet_solid, wallpaper);

        let pairs = [
            ("Primary Text on Window Scrim", self.text_primary, window_bg),
            ("Primary Text on Card Scrim", self.text_primary, card_bg),
            (
                "Primary Text on Overlay Scrim",
                self.text_primary,
                overlay_bg,
            ),
            (
                "Primary Text on Reading Sheet",
                self.text_primary,
                reading_bg,
            ),
            (
                "Warning CTA Ink on Pill",
                self.warning_cta_fg,
                self.accent_warning_cta,
            ),
        ];

        pairs
            .into_iter()
            .map(|(label, fg, bg)| {
                let ratio = Theme::contrast_ratio(fg, bg);
                (label, ratio, ratio >= 7.0)
            })
            .collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 5. WCAG 2.1 AA / AAA CONTRAST VERIFICATION ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

pub mod wcag {
    use super::{Rgba, Theme, ThemeMode, ThemePalette};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub enum WcagLevel {
        Fail,
        AA,
        AAA,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct WcagCompliance {
        pub contrast_ratio: f32,
        pub normal_text: WcagLevel,
        pub large_text: WcagLevel,
        pub ui_components: WcagLevel,
    }

    impl WcagCompliance {
        pub fn is_aa(&self) -> bool {
            self.normal_text >= WcagLevel::AA
        }

        pub fn is_aaa(&self) -> bool {
            self.normal_text >= WcagLevel::AAA
        }
    }

    /// Evaluates WCAG 2.1 contrast ratio and rating levels
    pub fn evaluate_compliance(fg: Rgba, bg: Rgba) -> WcagCompliance {
        let ratio = Theme::contrast_ratio(fg, bg);

        let normal_text = if ratio >= 7.0 {
            WcagLevel::AAA
        } else if ratio >= 4.5 {
            WcagLevel::AA
        } else {
            WcagLevel::Fail
        };

        let large_text = if ratio >= 4.5 {
            WcagLevel::AAA
        } else if ratio >= 3.0 {
            WcagLevel::AA
        } else {
            WcagLevel::Fail
        };

        let ui_components = if ratio >= 3.0 {
            WcagLevel::AA
        } else {
            WcagLevel::Fail
        };

        WcagCompliance {
            contrast_ratio: ratio,
            normal_text,
            large_text,
            ui_components,
        }
    }

    /// Validates compliance across primary theme pairings
    pub fn verify_theme_palette_compliance(
        palette: &ThemePalette,
        _mode: ThemeMode,
    ) -> Vec<(&'static str, f32, WcagLevel)> {
        let pairs = [
            (
                "Primary Text on Surface",
                palette.text_primary,
                palette.surface,
            ),
            (
                "Primary Text on Raised Surface",
                palette.text_primary,
                palette.surface_raised,
            ),
            (
                "Secondary Text on Surface",
                palette.text_secondary,
                palette.surface,
            ),
            (
                "Primary Text on Background",
                palette.text_primary,
                palette.background,
            ),
            (
                "Secondary Text on Background",
                palette.text_secondary,
                palette.background,
            ),
            (
                "Primary Accent on Background",
                palette.accent_primary,
                palette.background,
            ),
            ("Success on Surface", palette.success, palette.surface),
            ("Warning on Surface", palette.warning, palette.surface),
            ("Error on Surface", palette.error, palette.surface),
        ];

        pairs
            .into_iter()
            .map(|(label, fg, bg)| {
                let comp = evaluate_compliance(fg, bg);
                (label, comp.contrast_ratio, comp.normal_text)
            })
            .collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 6. THEME PALETTE & COMPREHENSIVE THEME STRUCTS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ThemePalette {
    pub background: Rgba,
    pub surface: Rgba,
    pub surface_raised: Rgba,
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,
    pub accent_primary: Rgba,
    pub accent_secondary: Rgba,
    pub success: Rgba,
    pub warning: Rgba,
    pub error: Rgba,
    pub info: Rgba,
    pub border: Rgba,
    pub semantic: SemanticTokens,
}

impl ThemePalette {
    /// Solar Amber Light Mode:
    /// Soft warm cream-yellow daylight canvas with high-contrast deep obsidian text (>14:1)
    pub fn solar_amber_light() -> Self {
        Self {
            background: primitives::IVORY_CANVAS,
            surface: primitives::IVORY_SURFACE,
            surface_raised: primitives::GOLDEN_GLASS_CARD,
            text_primary: primitives::OBSIDIAN_TEXT,
            text_secondary: primitives::STONE_TEXT,
            text_muted: [0.470, 0.445, 0.420, 1.00],
            accent_primary: primitives::SOLAR_AMBER_DEEP,
            accent_secondary: primitives::SOLAR_AMBER_GLOW,
            success: primitives::STATUS_SUCCESS_DARK,
            warning: primitives::SOLAR_AMBER_GLOW,
            error: primitives::STATUS_DANGER_DARK,
            info: primitives::STATUS_INFO_CYAN,
            border: primitives::SPECULAR_RIM_AMBER,
            semantic: SemanticTokens::light(),
        }
    }

    /// Solar Amber Dark Mode:
    /// Deep translucent glass backing plate with pure white text and glowing golden-amber highlights (>16:1)
    pub fn solar_amber_dark() -> Self {
        Self {
            background: primitives::DEEP_CHARCOAL,
            surface: [0.08, 0.11, 0.18, 0.88], // Deep translucent glass backing plate
            surface_raised: [0.118, 0.141, 0.212, 0.96], // Elevated inner card plate
            text_primary: [0.98, 0.99, 1.0, 1.0], // Pure white
            text_secondary: [0.839, 0.827, 0.820, 1.00],
            text_muted: [0.600, 0.580, 0.560, 1.00],
            accent_primary: primitives::SOLAR_AMBER_GLOW,
            accent_secondary: primitives::SOLAR_AMBER_LIGHT,
            success: primitives::STATUS_SUCCESS_BRIGHT,
            warning: primitives::SOLAR_AMBER_LIGHT,
            error: primitives::STATUS_DANGER_BRIGHT,
            info: primitives::STATUS_INFO_BRIGHT,
            border: [0.984, 0.749, 0.141, 0.48], // Warm gold top-lit specular border
            semantic: SemanticTokens::dark(),
        }
    }

    pub fn light() -> Self {
        Self::solar_amber_light()
    }

    pub fn dark() -> Self {
        Self::solar_amber_dark()
    }

    /// Apply an amber warmth tint across all colors (warmth in 0.0..=1.0).
    pub fn apply_night_warmth(&mut self, warmth: f32) {
        let shift = |c: &mut Rgba| {
            c[0] = (c[0] + warmth * 0.08).clamp(0.0, 1.0);
            c[1] = (c[1] - warmth * 0.03).clamp(0.0, 1.0);
            c[2] = (c[2] - warmth * 0.12).clamp(0.0, 1.0);
        };
        shift(&mut self.background);
        shift(&mut self.surface);
        shift(&mut self.surface_raised);
        shift(&mut self.text_primary);
        shift(&mut self.text_secondary);
        shift(&mut self.text_muted);
        shift(&mut self.accent_primary);
        shift(&mut self.accent_secondary);
        shift(&mut self.success);
        shift(&mut self.warning);
        shift(&mut self.error);
        shift(&mut self.info);
        shift(&mut self.border);
        self.semantic.apply_warmth(warmth);
    }
}

/// Comprehensive Theme configuration with semantic tokens and Hi-DPI scaling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub mode: ThemeMode,
    pub accent: AccentColor,
    /// Warm amber tint for night use (0.0..=1.0)
    pub night_warmth: f32,
    /// Text size multiplier (0.9..=1.3, default 1.0)
    pub font_scale: f32,
    /// Global UI scale multiplier (0.80..=1.50, default 1.15)
    pub ui_scale: f32,
    pub reduced_motion: bool,
    pub do_not_disturb: bool,
    pub high_contrast: bool,
    pub palette: ThemePalette,
    pub typography: typography::TypographyScale,
    pub radii: elevation::CornerRadii,
    pub glass: GlassTokens,
}

impl Theme {
    pub fn new(mode: ThemeMode) -> Self {
        let palette = match mode {
            ThemeMode::LightFrosted | ThemeMode::Light => ThemePalette::light(),
            ThemeMode::DarkObsidian | ThemeMode::Dark => ThemePalette::dark(),
        };
        let glass = GlassTokens::for_mode(mode);
        Self {
            mode,
            accent: AccentColor::SolarAmber,
            night_warmth: 0.0,
            font_scale: 1.0,
            ui_scale: 1.15, // Default to 1.15x out of the box for crisp legibility
            reduced_motion: false,
            do_not_disturb: false,
            high_contrast: false,
            palette,
            typography: typography::TypographyScale::standard(),
            radii: elevation::CornerRadii::standard(),
            glass,
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            ThemeMode::LightFrosted | ThemeMode::Light => ThemeMode::DarkObsidian,
            ThemeMode::DarkObsidian | ThemeMode::Dark => ThemeMode::LightFrosted,
        };
        self.palette = match self.mode {
            ThemeMode::LightFrosted | ThemeMode::Light => ThemePalette::light(),
            ThemeMode::DarkObsidian | ThemeMode::Dark => ThemePalette::dark(),
        };
        self.glass = GlassTokens::for_mode(self.mode);
        self.set_accent(self.accent);
        if self.night_warmth > 0.0 {
            let w = self.night_warmth;
            self.palette.apply_night_warmth(w);
        }
    }

    pub fn set_accent(&mut self, accent: AccentColor) {
        self.accent = accent;
        self.palette.accent_primary = match (accent, self.mode) {
            (AccentColor::SolarAmber, ThemeMode::LightFrosted | ThemeMode::Light) => {
                primitives::SOLAR_AMBER_DEEP
            }
            (AccentColor::SolarAmber, ThemeMode::DarkObsidian | ThemeMode::Dark) => {
                primitives::SOLAR_AMBER_GLOW
            }
            (AccentColor::Blue, ThemeMode::LightFrosted | ThemeMode::Light) => {
                [0.184, 0.502, 0.929, 1.0]
            }
            (AccentColor::Blue, ThemeMode::DarkObsidian | ThemeMode::Dark) => {
                [0.376, 0.647, 0.980, 1.0]
            }
            (AccentColor::Emerald, ThemeMode::LightFrosted | ThemeMode::Light) => {
                primitives::STATUS_SUCCESS_DARK
            }
            (AccentColor::Emerald, ThemeMode::DarkObsidian | ThemeMode::Dark) => {
                primitives::STATUS_SUCCESS_BRIGHT
            }
            (AccentColor::Indigo, ThemeMode::LightFrosted | ThemeMode::Light) => {
                [0.310, 0.275, 0.898, 1.0]
            }
            (AccentColor::Indigo, ThemeMode::DarkObsidian | ThemeMode::Dark) => {
                [0.506, 0.549, 0.973, 1.0]
            }
            (AccentColor::Orange, ThemeMode::LightFrosted | ThemeMode::Light) => {
                [0.949, 0.600, 0.290, 1.0]
            }
            (AccentColor::Orange, ThemeMode::DarkObsidian | ThemeMode::Dark) => {
                [0.988, 0.827, 0.302, 1.0]
            }
        };
    }

    pub fn set_night_warmth(&mut self, v: f32) {
        let v = v.clamp(0.0, 1.0);
        self.palette = match self.mode {
            ThemeMode::LightFrosted | ThemeMode::Light => ThemePalette::light(),
            ThemeMode::DarkObsidian | ThemeMode::Dark => ThemePalette::dark(),
        };
        self.set_accent(self.accent);
        self.night_warmth = v;
        if v > 0.0 {
            self.palette.apply_night_warmth(v);
        }
    }

    pub fn set_font_scale(&mut self, v: f32) {
        self.font_scale = v.clamp(0.9, 1.3);
    }

    pub fn set_ui_scale(&mut self, v: f32) {
        self.ui_scale = v.clamp(0.80, 1.50);
    }

    // ─── Semantic Typography Scale (scaled by ui_scale * font_scale) ───────────
    pub fn display_size(&self) -> f32 {
        self.typography
            .display
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }
    pub fn h1_size(&self) -> f32 {
        self.typography
            .header_1
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }
    pub fn h2_size(&self) -> f32 {
        self.typography
            .header_2
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }
    pub fn h3_size(&self) -> f32 {
        self.typography
            .header_3
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }
    pub fn body_size(&self) -> f32 {
        self.typography
            .body
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }
    pub fn body_small_size(&self) -> f32 {
        self.typography
            .body_small
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }
    pub fn caption_size(&self) -> f32 {
        self.typography
            .caption
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }
    pub fn caption_small_size(&self) -> f32 {
        self.typography
            .caption_small
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }
    pub fn code_size(&self) -> f32 {
        self.typography
            .code
            .compute_effective_pt(self.ui_scale, self.font_scale)
    }

    // ─── Semantic Spacing Tokens (scaled by ui_scale) ──────────────────────────
    pub fn spacing_xs(&self) -> f32 {
        (4.0 * self.ui_scale).round()
    }
    pub fn spacing_sm(&self) -> f32 {
        (8.0 * self.ui_scale).round()
    }
    pub fn spacing_md(&self) -> f32 {
        (12.0 * self.ui_scale).round()
    }
    pub fn spacing_lg(&self) -> f32 {
        (16.0 * self.ui_scale).round()
    }
    pub fn spacing_xl(&self) -> f32 {
        (24.0 * self.ui_scale).round()
    }
    pub fn spacing_xxl(&self) -> f32 {
        (32.0 * self.ui_scale).round()
    }

    // ─── Semantic Shape Tokens (corner radiuses) ──────────────────────────────
    pub fn radius_sm(&self) -> f32 {
        6.0
    }
    pub fn radius_md(&self) -> f32 {
        self.radii.badge
    } // 12.0px
    pub fn radius_lg(&self) -> f32 {
        self.radii.pill
    } // 18.0px
    pub fn radius_pill(&self) -> f32 {
        self.radii.fully_rounded
    } // 999.0px
    pub fn radius_window(&self) -> f32 {
        self.radii.window
    } // 16.0px
    pub fn radius_card(&self) -> f32 {
        self.radii.card
    } // 14.0px
    pub fn radius_badge(&self) -> f32 {
        self.radii.badge
    } // 12.0px
    pub fn radius_dock(&self) -> f32 {
        self.radii.dock
    } // 24.0px

    /// Elevation token bundle for a given visual elevation level
    pub fn elevation(&self, level: elevation::ElevationLevel) -> elevation::ElevationTokens {
        elevation::ElevationTokens::for_level(level, self.mode)
    }

    /// Access semantic tokens
    pub fn semantic(&self) -> &SemanticTokens {
        &self.palette.semantic
    }

    /// WCAG 2.1 contrast ratio between two sRGB Rgba values.
    pub fn contrast_ratio(fg: Rgba, bg: Rgba) -> f32 {
        fn linearize(c: f32) -> f32 {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055_f32).powf(2.4)
            }
        }
        fn luminance(c: Rgba) -> f32 {
            0.2126 * linearize(c[0]) + 0.7152 * linearize(c[1]) + 0.0722 * linearize(c[2])
        }
        let l1 = luminance(fg);
        let l2 = luminance(bg);
        let (lighter, darker) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Update a spring towards a target value respecting the `reduced_motion` preference.
    pub fn update_spring(
        &self,
        spring: &mut mindset_motion::Spring1D,
        target: f32,
        dt: f32,
    ) -> f32 {
        if self.reduced_motion {
            spring.snap(target);
            target
        } else {
            spring.set_target(target);
            spring.update(dt)
        }
    }

    /// Active frosted glass design tokens
    pub fn glass(&self) -> &GlassTokens {
        &self.glass
    }

    /// Run WCAG 2.1 accessibility verification across the active palette
    pub fn verify_accessibility(&self) -> Vec<(&'static str, f32, wcag::WcagLevel)> {
        wcag::verify_theme_palette_compliance(&self.palette, self.mode)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 7. COMPREHENSIVE UNIT TEST SUITE
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme_primary_text_contrast() {
        let p = ThemePalette::dark();
        let ratio = Theme::contrast_ratio(p.text_primary, p.surface);
        assert!(
            ratio >= 7.0,
            "Dark text contrast {ratio:.2} must be >= 7.0:1"
        );
    }

    #[test]
    fn test_light_theme_primary_text_contrast() {
        let p = ThemePalette::light();
        let ratio = Theme::contrast_ratio(p.text_primary, p.surface);
        assert!(
            ratio >= 7.0,
            "Light text contrast {ratio:.2} must be >= 7.0:1"
        );
    }

    #[test]
    fn test_night_warmth_clamps_channels() {
        let mut t = Theme::new(ThemeMode::Dark);
        t.set_night_warmth(1.0);
        for c in t.palette.text_primary.iter() {
            assert!(*c >= 0.0 && *c <= 1.0, "Channel out of range: {c}");
        }
    }

    #[test]
    fn test_ui_and_typography_scaling() {
        let mut t = Theme::new(ThemeMode::Dark);
        assert_eq!(t.ui_scale, 1.15); // Default 1.15x
        assert!(t.body_size() >= 17.0);

        t.set_ui_scale(1.50);
        assert_eq!(t.ui_scale, 1.50);
        assert!(t.display_size() >= 48.0);
        assert!(t.h1_size() >= 39.0);
        assert!(t.body_size() >= 23.0);
        assert!(t.caption_size() >= 20.0);
    }

    #[test]
    fn test_reduced_motion_instant_snap_vs_spring_motion() {
        let mut theme = Theme::new(ThemeMode::Light);
        theme.reduced_motion = false;
        let mut smooth_spring =
            mindset_motion::Spring1D::with_preset(0.0, mindset_motion::SpringPreset::FluidPanel);
        let target = 1.0;

        let pos_step1 = theme.update_spring(&mut smooth_spring, target, 0.016);
        assert!(pos_step1 > 0.0 && pos_step1 < target);
        assert!(smooth_spring.velocity > 0.0);

        theme.reduced_motion = true;
        let mut snap_spring =
            mindset_motion::Spring1D::with_preset(0.0, mindset_motion::SpringPreset::FluidPanel);
        let snap_pos = theme.update_spring(&mut snap_spring, target, 0.016);
        assert_eq!(snap_pos, target);
        assert_eq!(snap_spring.velocity, 0.0);
        assert!(snap_spring.is_at_rest());
    }

    #[test]
    fn test_solar_amber_light_and_dark_wcag_contrast() {
        let light = ThemePalette::solar_amber_light();
        let light_ratio = Theme::contrast_ratio(light.text_primary, light.surface);
        assert!(
            light_ratio >= 10.0,
            "SolarAmber Light primary text contrast {light_ratio:.2} must be >= 10.0:1"
        );

        let dark = ThemePalette::solar_amber_dark();
        let dark_ratio = Theme::contrast_ratio(dark.text_primary, dark.surface);
        assert!(
            dark_ratio >= 10.0,
            "SolarAmber Dark primary text contrast {dark_ratio:.2} must be >= 10.0:1"
        );
    }

    #[test]
    fn test_high_contrast_glass_and_specular_border() {
        let dark = ThemePalette::solar_amber_dark();
        assert_eq!(
            dark.surface,
            [0.08, 0.11, 0.18, 0.88],
            "Dark surface must be deep translucent glass"
        );
        assert_eq!(
            dark.text_primary,
            [0.98, 0.99, 1.0, 1.0],
            "Dark text_primary must be pure white"
        );
        assert!(
            dark.surface[3] >= 0.85,
            "Dark surface alpha must be >= 0.85 for frosted glass"
        );
        assert!(
            dark.surface_raised[3] >= 0.88,
            "Dark surface_raised alpha must be >= 0.88"
        );
        // Specular warm golden-amber border
        assert!(dark.border[0] > 0.9 && dark.border[1] > 0.7 && dark.border[3] >= 0.45);

        let light = ThemePalette::solar_amber_light();
        assert!(light.surface[3] >= 0.80);
        assert!(light.surface_raised[3] >= 0.85);
        assert_eq!(light.text_primary, [0.110, 0.098, 0.090, 1.0]);
    }

    #[test]
    fn test_solar_amber_never_used_as_window_or_surface_background() {
        let dark = ThemePalette::solar_amber_dark();
        let light = ThemePalette::solar_amber_light();

        let amber_accents = [
            primitives::SOLAR_AMBER,
            primitives::SOLAR_AMBER_GLOW,
            primitives::SOLAR_AMBER_DEEP,
            primitives::SOLAR_AMBER_LIGHT,
        ];

        for amber in &amber_accents {
            // Dark theme surfaces must never be yellow / Solar Amber
            assert_ne!(
                dark.background, *amber,
                "Dark background must not be Solar Amber"
            );
            assert_ne!(dark.surface, *amber, "Dark surface must not be Solar Amber");
            assert_ne!(
                dark.surface_raised, *amber,
                "Dark surface_raised must not be Solar Amber"
            );

            // Light theme surfaces must never be yellow / Solar Amber
            assert_ne!(
                light.background, *amber,
                "Light background must not be Solar Amber"
            );
            assert_ne!(
                light.surface, *amber,
                "Light surface must not be Solar Amber"
            );

            // Semantic tokens surface layers must never be Solar Amber
            let dark_sem = SemanticTokens::dark();
            assert_ne!(dark_sem.surface_primary, *amber);
            assert_ne!(dark_sem.surface_elevated, *amber);
            assert_ne!(dark_sem.surface_frosted, *amber);
        }
    }

    #[test]
    fn test_primitive_tokens_exact_constants() {
        use primitives::*;

        assert_eq!(HEX_DEEP_CHARCOAL, "#0B0D12");
        assert_eq!(HEX_RAISED_GLASS, "#151922");
        assert_eq!(HEX_SECONDARY_SURFACE, "#1C2230");
        assert_eq!(HEX_SOLAR_AMBER, "#E5AD45");
        assert_eq!(HEX_PRIMARY_TEXT, "#F5F6FA");
        assert_eq!(HEX_SECONDARY_TEXT, "#929AAA");

        // Verify normalized linear sRGB channel values
        assert!((DEEP_CHARCOAL[0] - 0.0431).abs() < 0.001);
        assert!((RAISED_GLASS[0] - 0.0824).abs() < 0.001);
        assert!((SECONDARY_SURFACE[0] - 0.1098).abs() < 0.001);
        assert!((SOLAR_AMBER[0] - 0.8980).abs() < 0.001);
        assert!((PRIMARY_TEXT[0] - 0.9608).abs() < 0.001);
        assert!((SECONDARY_TEXT[0] - 0.5725).abs() < 0.001);

        // Specular rim
        assert_eq!(SPECULAR_RIM_TOP, [1.0, 1.0, 1.0, 0.40]);
        assert_eq!(SPECULAR_RIM_BOTTOM, [1.0, 1.0, 1.0, 0.10]);
    }

    #[test]
    fn test_semantic_tokens_mapping() {
        let dark = SemanticTokens::dark();
        assert_eq!(dark.surface_primary, primitives::DEEP_CHARCOAL);
        assert_eq!(dark.surface_elevated, primitives::RAISED_GLASS_FROSTED);
        assert_eq!(dark.surface_frosted, primitives::SECONDARY_SURFACE_RAISED);
        assert_eq!(dark.accent_focus, primitives::SOLAR_AMBER_GLOW);
        assert_eq!(dark.status_success, primitives::STATUS_SUCCESS_BRIGHT);
        assert_eq!(dark.status_warning, primitives::STATUS_WARNING_BRIGHT);
        assert_eq!(dark.status_danger, primitives::STATUS_DANGER_BRIGHT);
        assert_eq!(dark.border_specular, primitives::SPECULAR_RIM_TOP);

        let light = SemanticTokens::light();
        assert_eq!(light.surface_primary, primitives::IVORY_CANVAS);
        assert_eq!(light.accent_focus, primitives::SOLAR_AMBER_DEEP);
        assert_eq!(light.status_success, primitives::STATUS_SUCCESS_DARK);
        assert_eq!(light.status_danger, primitives::STATUS_DANGER_DARK);
        assert_eq!(light.border_specular, primitives::SPECULAR_RIM_AMBER);
    }

    #[test]
    fn test_typography_scale_tokens() {
        let theme = Theme::new(ThemeMode::Dark);
        let scale = &theme.typography;

        assert_eq!(scale.display.logical_pt, 32.0);
        assert_eq!(scale.header_1.logical_pt, 26.0);
        assert_eq!(scale.header_2.logical_pt, 20.0);
        assert_eq!(scale.header_3.logical_pt, 17.0);
        assert_eq!(scale.body.logical_pt, 15.5);
        assert_eq!(scale.caption.logical_pt, 13.5);
        assert_eq!(scale.code.logical_pt, 13.0);
        assert!(scale.code.is_monospace);

        // Effective physical size at default 1.15x ui_scale
        assert_eq!(theme.display_size(), 37.0); // 32 * 1.15 = 36.8 -> 37.0
        assert_eq!(theme.h1_size(), 30.0); // 26 * 1.15 = 29.9 -> 30.0
        assert_eq!(theme.h2_size(), 23.0); // 20 * 1.15 = 23.0
        assert_eq!(theme.h3_size(), 20.0); // 17 * 1.15 = 19.55 -> 20.0
        assert_eq!(theme.body_size(), 18.0); // 15.5 * 1.15 = 17.825 -> 18.0
        assert_eq!(theme.caption_size(), 16.0); // 13.5 * 1.15 = 15.525 -> 16.0
        assert_eq!(theme.code_size(), 15.0); // 13.0 * 1.15 = 14.95 -> 15.0
    }

    #[test]
    fn test_elevation_and_corner_radii() {
        let theme = Theme::new(ThemeMode::Dark);

        // Corner radii tokens: Cards 14px, Badges 12px, Pills 18px, Dock 24px
        assert_eq!(theme.radius_card(), 14.0);
        assert_eq!(theme.radius_badge(), 12.0);
        assert_eq!(theme.radius_lg(), 18.0);
        assert_eq!(theme.radius_dock(), 24.0);
        assert_eq!(theme.radius_window(), 16.0);
        assert_eq!(theme.radius_pill(), 999.0);

        // Elevation tokens
        let l1 = theme.elevation(elevation::ElevationLevel::Level1SubtleCard);
        assert_eq!(l1.blur.passes, 2);
        assert_eq!(l1.blur.blur_radius, 14.0);
        assert_eq!(l1.glass.glass_mix, 0.70);

        let l2 = theme.elevation(elevation::ElevationLevel::Level2WindowSurface);
        assert_eq!(l2.blur.passes, 3);
        assert_eq!(l2.blur.blur_radius, 28.0);
        assert_eq!(l2.glass.glass_mix, 0.85);

        let l3 = theme.elevation(elevation::ElevationLevel::Level3FloatingDock);
        assert_eq!(l3.blur.passes, 4);
        assert_eq!(l3.blur.blur_radius, 36.0);
        assert_eq!(l3.glass.glass_mix, 0.92);

        // Specular gradient sampling
        let spec_sample_top = l3.specular.sample_gradient(0.0);
        let spec_sample_bot = l3.specular.sample_gradient(1.0);
        assert_eq!(spec_sample_top, primitives::SPECULAR_RIM_TOP);
        assert_eq!(spec_sample_bot, primitives::SPECULAR_RIM_BOTTOM);
    }

    #[test]
    fn test_wcag_21_aa_and_aaa_verification() {
        use primitives::*;

        // Dark theme: Primary text (#F5F6FA) on Deep Charcoal (#0B0D12)
        let c_pri_charcoal = Theme::contrast_ratio(PRIMARY_TEXT, DEEP_CHARCOAL);
        assert!(
            c_pri_charcoal >= 15.0,
            "Expected >15.0, got {c_pri_charcoal:.2}"
        );
        let comp = wcag::evaluate_compliance(PRIMARY_TEXT, DEEP_CHARCOAL);
        assert!(comp.is_aaa());

        // Dark theme: Primary text (#F5F6FA) on Raised Glass (#151922)
        let c_pri_raised = Theme::contrast_ratio(PRIMARY_TEXT, RAISED_GLASS);
        assert!(
            c_pri_raised >= 14.0,
            "Expected >14.0, got {c_pri_raised:.2}"
        );

        // Dark theme: Secondary text (#929AAA) on Deep Charcoal (#0B0D12)
        let c_sec_charcoal = Theme::contrast_ratio(SECONDARY_TEXT, DEEP_CHARCOAL);
        assert!(
            c_sec_charcoal >= 4.5,
            "Expected >=4.5 (WCAG AA), got {c_sec_charcoal:.2}"
        );

        // Dark theme: Solar Amber (#E5AD45) on Deep Charcoal (#0B0D12)
        let c_amber = Theme::contrast_ratio(SOLAR_AMBER, DEEP_CHARCOAL);
        assert!(
            c_amber >= 7.0,
            "Expected >=7.0 (WCAG AAA), got {c_amber:.2}"
        );

        // Light theme: Obsidian text (#1C1917) on Ivory Canvas (#FFFBEB)
        let c_light_pri = Theme::contrast_ratio(OBSIDIAN_TEXT, IVORY_CANVAS);
        assert!(c_light_pri >= 14.0, "Expected >14.0, got {c_light_pri:.2}");

        // Light theme: Warm Stone text (#57534E) on Ivory Canvas (#FFFBEB)
        let c_light_sec = Theme::contrast_ratio(STONE_TEXT, IVORY_CANVAS);
        assert!(
            c_light_sec >= 4.5,
            "Expected >=4.5 (WCAG AA), got {c_light_sec:.2}"
        );

        // Full palette verification
        let theme_dark = Theme::new(ThemeMode::Dark);
        let dark_report = theme_dark.verify_accessibility();
        for (name, ratio, level) in dark_report {
            assert!(
                ratio >= 3.0,
                "Accessibility failure on {name}: ratio {ratio:.2} < 3.0"
            );
            assert_ne!(level, wcag::WcagLevel::Fail, "Pair {name} failed WCAG");
        }
    }

    #[test]
    fn test_glass_tokens_light_frosted_spec() {
        let glass = GlassTokens::LIGHT;
        assert_eq!(glass.blur_radius_px, 24.0);
        assert_eq!(glass.noise_dither_amp, 0.015);

        // Alpha values
        assert_eq!(glass.window_scrim[3], 0.80);
        assert_eq!(glass.card_scrim[3], 0.72);
        assert_eq!(glass.overlay_scrim[3], 0.88);
        assert_eq!(glass.reading_sheet_solid[3], 0.98);

        // Primary & secondary text
        assert_eq!(glass.text_primary, primitives::from_hex_rgb(0x0F172A, 1.0));
        assert_eq!(
            glass.text_secondary,
            primitives::from_hex_rgb(0x334155, 1.0)
        );

        // Accents
        assert_eq!(
            glass.accent_primary,
            primitives::from_hex_rgb(0x0284C7, 1.0)
        );
        assert_eq!(glass.accent_primary_fg, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(
            glass.accent_warning_cta,
            primitives::from_hex_rgb(0xF59E0B, 1.0)
        );
        assert_eq!(glass.warning_cta_bg, glass.accent_warning_cta);
        assert_eq!(
            glass.warning_cta_fg,
            primitives::from_hex_rgb(0x020617, 1.0)
        );

        // Specular rim
        assert_eq!(glass.specular_rim_top, [1.0, 1.0, 1.0, 0.65]);
        assert_eq!(
            glass.specular_rim_bottom,
            primitives::from_hex_rgb(0x94A3B8, 0.35)
        );
    }

    #[test]
    fn test_glass_tokens_dark_obsidian_spec() {
        let glass = GlassTokens::DARK;
        assert_eq!(glass.blur_radius_px, 24.0);
        assert_eq!(glass.noise_dither_amp, 0.012);

        // Alpha values
        assert_eq!(glass.window_scrim[3], 0.82);
        assert_eq!(glass.card_scrim[3], 0.74);
        assert_eq!(glass.overlay_scrim[3], 0.88);
        assert_eq!(glass.reading_sheet_solid[3], 0.98);

        // Primary & secondary text
        assert_eq!(glass.text_primary, primitives::from_hex_rgb(0xF8FAFC, 1.0));
        assert_eq!(
            glass.text_secondary,
            primitives::from_hex_rgb(0xCBD5E1, 1.0)
        );

        // Accents
        assert_eq!(
            glass.accent_primary,
            primitives::from_hex_rgb(0x38BDF8, 1.0)
        );
        assert_eq!(
            glass.accent_primary_fg,
            primitives::from_hex_rgb(0x082F49, 1.0)
        );
        assert_eq!(
            glass.accent_warning_cta,
            primitives::from_hex_rgb(0xFBBF24, 1.0)
        );
        assert_eq!(glass.warning_cta_bg, glass.accent_warning_cta);
        assert_eq!(
            glass.warning_cta_fg,
            primitives::from_hex_rgb(0x020617, 1.0)
        );

        // Specular rim
        assert_eq!(glass.specular_rim_top, [1.0, 1.0, 1.0, 0.18]);
        assert_eq!(glass.specular_rim_bottom, [0.0, 0.0, 0.0, 0.45]);
    }

    #[test]
    fn test_theme_mode_canonical_and_compatibility() {
        assert_eq!(ThemeMode::default(), ThemeMode::DarkObsidian);
        assert_eq!(ThemeMode::Dark, ThemeMode::DarkObsidian);
        assert_eq!(ThemeMode::Light, ThemeMode::LightFrosted);
        assert!(ThemeMode::DarkObsidian.is_dark());
        assert!(ThemeMode::Dark.is_dark());
        assert!(!ThemeMode::LightFrosted.is_dark());
        assert!(ThemeMode::LightFrosted.is_light());

        let mut theme = Theme::new(ThemeMode::LightFrosted);
        assert_eq!(theme.glass.window_scrim, GlassTokens::LIGHT.window_scrim);
        assert_eq!(theme.glass.blur_radius_px, 24.0);

        theme.toggle_mode();
        assert_eq!(theme.mode, ThemeMode::DarkObsidian);
        assert_eq!(theme.glass.window_scrim, GlassTokens::DARK.window_scrim);

        theme.toggle_mode();
        assert_eq!(theme.mode, ThemeMode::LightFrosted);
        assert_eq!(theme.glass.window_scrim, GlassTokens::LIGHT.window_scrim);
    }

    #[test]
    fn test_wcag_aaa_contrast_guarantee_over_bright_wallpapers() {
        let white_wallpaper: Rgba = [1.0, 1.0, 1.0, 1.0];
        let sky_wallpaper: Rgba = [0.878, 0.949, 0.996, 1.0];
        let cream_wallpaper: Rgba = [1.0, 0.984, 0.922, 1.0];

        for wallpaper in [white_wallpaper, sky_wallpaper, cream_wallpaper] {
            // Light Frosted verification
            let light_glass = GlassTokens::LIGHT;
            let light_results = light_glass.verify_wcag_aaa_contrast(wallpaper);
            for (label, ratio, meets_aaa) in light_results {
                assert!(
                    meets_aaa,
                    "LightFrosted WCAG AAA failure on '{label}' over wallpaper: ratio {ratio:.2} must be >= 7.0:1"
                );
                assert!(ratio >= 7.0, "'{label}' ratio {ratio:.2} < 7.0:1");
            }

            // Dark Obsidian verification
            let dark_glass = GlassTokens::DARK;
            let dark_results = dark_glass.verify_wcag_aaa_contrast(wallpaper);
            for (label, ratio, meets_aaa) in dark_results {
                assert!(
                    meets_aaa,
                    "DarkObsidian WCAG AAA failure on '{label}' over wallpaper: ratio {ratio:.2} must be >= 7.0:1"
                );
                assert!(ratio >= 7.0, "'{label}' ratio {ratio:.2} < 7.0:1");
            }
        }
    }

    #[test]
    fn test_glass_specular_rim_sun_vector_alignment() {
        let glass = GlassTokens::LIGHT;
        let top = glass.specular_rim_at(1.0);
        assert_eq!(top, glass.specular_rim_top);

        let bottom = glass.specular_rim_at(0.0);
        assert_eq!(bottom, glass.specular_rim_bottom);

        let mid = glass.specular_rim_at(0.5);
        for (i, &val) in mid.iter().enumerate() {
            let expected = (glass.specular_rim_bottom[i] + glass.specular_rim_top[i]) * 0.5;
            assert!((val - expected).abs() < 1e-4);
        }
    }

    #[test]
    fn test_glass_tokens_serialization_roundtrip() {
        let json_light =
            serde_json::to_string(&GlassTokens::LIGHT).expect("Failed to serialize light");
        let deserialized_light: GlassTokens =
            serde_json::from_str(&json_light).expect("Failed to deserialize light");
        assert_eq!(deserialized_light, GlassTokens::LIGHT);

        let json_dark =
            serde_json::to_string(&GlassTokens::DARK).expect("Failed to serialize dark");
        let deserialized_dark: GlassTokens =
            serde_json::from_str(&json_dark).expect("Failed to deserialize dark");
        assert_eq!(deserialized_dark, GlassTokens::DARK);

        // Test ThemeMode serde aliases
        let mode_l: ThemeMode = serde_json::from_str("\"Light\"").expect("deserialize Light");
        assert_eq!(mode_l, ThemeMode::LightFrosted);
        let mode_lf: ThemeMode =
            serde_json::from_str("\"LightFrosted\"").expect("deserialize LightFrosted");
        assert_eq!(mode_lf, ThemeMode::LightFrosted);

        let mode_d: ThemeMode = serde_json::from_str("\"Dark\"").expect("deserialize Dark");
        assert_eq!(mode_d, ThemeMode::DarkObsidian);
        let mode_do: ThemeMode =
            serde_json::from_str("\"DarkObsidian\"").expect("deserialize DarkObsidian");
        assert_eq!(mode_do, ThemeMode::DarkObsidian);
    }
}
