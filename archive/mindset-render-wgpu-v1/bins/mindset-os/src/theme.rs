//! MindSet OS — Liquidmorphism & Glassmorphism Design Tokens (`theme.rs`)
//! Production-grade Design Token Architecture for MindSet OS Shell & Window System.
//! Derived from MindSet OS Design Token Hierarchy for LKS PVT LTD.

// Re-export full design token system from mindset_ui::theme
pub use mindset_ui::theme::{
    elevation::{
        CornerRadii, DualKawaseParams, ElevationLevel, ElevationTokens, LiquidGlassParams,
        SpecularHighlight,
    },
    primitives,
    typography::{TypographyScale, TypographyScaleLevel, TypographyToken},
    wcag::{WcagCompliance, WcagLevel},
    AccentColor, GlassTokens, Rgba, SemanticTokens, Theme, ThemeMode, ThemePalette,
};

// ── Corner Radii Tokens (Cards 14px, Badges 12px, Pills 18px, Dock 24px) ─────
pub const BADGE_RADIUS: f32 = 12.0;
pub const CARD_RADIUS: f32 = 14.0;
pub const WINDOW_RADIUS: f32 = 16.0;
pub const PILL_RADIUS: f32 = 18.0;
pub const CONTROL_CENTER_RADIUS: f32 = 22.0;
pub const DOCK_RADIUS: f32 = 24.0;
pub const MAIN_WINDOW_RADIUS: f32 = 24.0;
pub const DIAGNOSTIC_ROW_RADIUS: f32 = 14.0;

// ── Spacing Tokens ────────────────────────────────────────────────────────────
pub const INNER_GAP: f32 = 8.0;
pub const DIAGNOSTIC_ROW_HEIGHT: f32 = 74.0;
pub const DOCK_HEIGHT: f32 = 76.0;

// ── Surface & Blur Tokens ─────────────────────────────────────────────────────
/// Main Application Window Glass Tint: rgba(30, 50, 80, 0.45)
pub const MAIN_WINDOW_GLASS_FILL: [f32; 4] = [0.118, 0.196, 0.314, 0.450];
/// Specular top inner border: rgba(255, 255, 255, 0.40)
pub const MAIN_WINDOW_BORDER_TOP: [f32; 4] = primitives::SPECULAR_RIM_TOP;
/// Specular bottom inner border: rgba(255, 255, 255, 0.10)
pub const MAIN_WINDOW_BORDER_BOTTOM: [f32; 4] = primitives::SPECULAR_RIM_BOTTOM;
pub const MAIN_WINDOW_BLUR_RADIUS: f32 = 28.0;

/// Diagnostic Row Glass: rgba(255, 255, 255, 0.08)
pub const DIAGNOSTIC_ROW_FILL: [f32; 4] = [1.000, 1.000, 1.000, 0.080];
/// Diagnostic Row Border: rgba(255, 255, 255, 0.18)
pub const DIAGNOSTIC_ROW_BORDER: [f32; 4] = [1.000, 1.000, 1.000, 0.180];

/// Top-Right Floating Control Center Glass: rgba(225, 238, 255, 0.72)
pub const CONTROL_CENTER_GLASS_FILL: [f32; 4] = [0.882, 0.933, 1.000, 0.720];
pub const CONTROL_CENTER_BORDER: [f32; 4] = [1.000, 1.000, 1.000, 0.450];
pub const CONTROL_CENTER_BLUR_RADIUS: f32 = 25.0;
/// Dark Slate Typography for Control Center: #1E293B (rgba(30, 41, 59, 0.88))
pub const CONTROL_CENTER_TEXT_DARK: [f32; 4] = [0.118, 0.161, 0.231, 0.880];
pub const CONTROL_CENTER_TEXT_MUTED: [f32; 4] = [0.278, 0.333, 0.412, 0.750];

/// Bottom Compact Floating Dock Glass: rgba(255, 255, 255, 0.22)
pub const DOCK_GLASS_FILL: [f32; 4] = [1.000, 1.000, 1.000, 0.220];
pub const DOCK_BORDER: [f32; 4] = [1.000, 1.000, 1.000, 0.350];

// ─── Low-Contrast Eye-Friendly Typography Tokens ─────────────────────────────
/// Primary Headers: rgba(255, 255, 255, 0.85) (#F1F5F9 at 85%)
pub const TEXT_PRIMARY_HEADER: [f32; 4] = [0.945, 0.961, 0.976, 0.850];
/// Secondary Telemetry & Subtitles: rgba(203, 213, 225, 0.68) (#CBD5E1)
pub const TEXT_SECONDARY_MUTED: [f32; 4] = [0.796, 0.835, 0.882, 0.680];
/// Accent Cyan for SYS highlights: #38BDF8
pub const TEXT_ACCENT_CYAN: [f32; 4] = primitives::STATUS_INFO_BRIGHT;

// ─── Buttons, Pills & Badges ──────────────────────────────────────────────────
/// Primary CTA Pill Top: #38BDF8 (rgba(56, 189, 248, 1.0))
pub const PRIMARY_CTA_TOP: [f32; 4] = primitives::STATUS_INFO_BRIGHT;
/// Primary CTA Pill Bottom: #0284C7 (rgba(2, 132, 199, 1.0))
pub const PRIMARY_CTA_BOTTOM: [f32; 4] = [0.008, 0.518, 0.780, 1.000];
/// Primary CTA Top Glow: rgba(255, 255, 255, 0.60)
pub const PRIMARY_CTA_GLOW: [f32; 4] = [1.000, 1.000, 1.000, 0.600];

/// One-Click Repair Pill Top: #FBBF24 (rgba(251, 191, 36, 1.0))
pub const REPAIR_PILL_TOP: [f32; 4] = primitives::STATUS_WARNING_BRIGHT;
/// One-Click Repair Pill Bottom: #D97706 (rgba(217, 119, 6, 1.0))
pub const REPAIR_PILL_BOTTOM: [f32; 4] = primitives::SOLAR_AMBER_DEEP;

/// Health Badge Pill Fill: rgba(74, 222, 128, 0.35)
pub const HEALTH_BADGE_FILL: [f32; 4] = [0.290, 0.871, 0.502, 0.350];
/// Health Badge Border: #86EFAC (rgba(134, 239, 172, 0.90))
pub const HEALTH_BADGE_BORDER: [f32; 4] = [0.525, 0.937, 0.675, 0.900];
/// Health Badge Text: #34D399 (rgba(52, 211, 153, 1.00))
pub const HEALTH_BADGE_TEXT: [f32; 4] = primitives::STATUS_SUCCESS_BRIGHT;

/// Active Balanced Pill Fill: #FDE047 (Soft Pastel Yellow)
pub const BALANCED_PILL_FILL: [f32; 4] = [0.992, 0.878, 0.278, 1.000];
/// Inactive Segmented Button Fill: rgba(255, 255, 255, 0.08)
pub const SEGMENTED_INACTIVE_FILL: [f32; 4] = [1.000, 1.000, 1.000, 0.080];

/// Traffic Light Red: #FF5F56
pub const TRAFFIC_RED: [f32; 4] = [1.000, 0.373, 0.337, 1.000];
/// Traffic Light Yellow: #FFBD2E
pub const TRAFFIC_YELLOW: [f32; 4] = [1.000, 0.741, 0.180, 1.000];
/// Traffic Light Green: #27C93F
pub const TRAFFIC_GREEN: [f32; 4] = [0.153, 0.788, 0.247, 1.000];

/// iOS Active Toggle Blue: #3B82F6
pub const TOGGLE_ACTIVE_BLUE: [f32; 4] = [0.231, 0.510, 0.965, 1.000];
/// iOS Inactive Toggle Track: rgba(203, 213, 225, 0.50)
pub const TOGGLE_INACTIVE_TRACK: [f32; 4] = [0.796, 0.835, 0.882, 0.500];

// ─── 8 Dock Icon Theme Colors ─────────────────────────────────────────────────
pub const DOCK_COLOR_TASKS: [f32; 4] = [0.133, 0.773, 0.369, 1.000]; // Green
pub const DOCK_COLOR_READER: [f32; 4] = [0.220, 0.741, 0.973, 1.000]; // Sky Blue
pub const DOCK_COLOR_CREATE: [f32; 4] = [0.659, 0.333, 0.965, 1.000]; // Violet Purple
pub const DOCK_COLOR_SEARCH: [f32; 4] = [0.149, 0.384, 0.933, 1.000]; // Royal Blue
pub const DOCK_COLOR_DIR: [f32; 4] = [0.961, 0.690, 0.098, 1.000]; // Golden Yellow
pub const DOCK_COLOR_STATS: [f32; 4] = [0.078, 0.729, 0.651, 1.000]; // Teal
pub const DOCK_COLOR_SYS: [f32; 4] = [0.957, 0.267, 0.478, 1.000]; // Rose Pink
pub const DOCK_COLOR_SET: [f32; 4] = [0.580, 0.639, 0.722, 1.000]; // Metallic Grey
