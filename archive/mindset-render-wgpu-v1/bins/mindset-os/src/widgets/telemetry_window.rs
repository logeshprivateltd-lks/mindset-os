//! MindSet OS — System Telemetry & Doctor Window Widget (`telemetry_window.rs`)
//! Pure Rust native component featuring iOS 18/visionOS liquidmorphism glass surfaces,
//! live system telemetry metrics, 5 self-healing diagnostic subsystem rows, and power profile controls.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::theme::*;
use mindset_render::{optimal_text_color_for_fill, SdfInstance, SdfPipeline, TextRenderer};
use mindset_ui::LayoutRect;

/// Power Profile Governor State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    EcoStudy,
    Balanced,
    AiBoost,
}

impl PowerProfile {
    pub fn all() -> [PowerProfile; 3] {
        [
            PowerProfile::EcoStudy,
            PowerProfile::Balanced,
            PowerProfile::AiBoost,
        ]
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::EcoStudy => "Eco Study",
            Self::Balanced => "Balanced",
            Self::AiBoost => "AI Boost",
        }
    }
}

/// A diagnostic subsystem row within the doctor window
#[derive(Debug, Clone)]
pub struct DiagnosticRow {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon_symbol: &'static str,
    pub icon_color: [f32; 4],
    pub is_repairing: bool,
    pub shimmer_progress: f32,
}

impl DiagnosticRow {
    pub fn default_rows() -> Vec<DiagnosticRow> {
        vec![
            DiagnosticRow {
                id: "audio",
                title: "Audio Pipeline Reset",
                subtitle: "Healthy (48kHz, Low Latency, 2.4ms)",
                icon_symbol: "🔊",
                icon_color: [0.220, 0.741, 0.973, 1.0], // Neon soundwave cyan
                is_repairing: false,
                shimmer_progress: 0.0,
            },
            DiagnosticRow {
                id: "wifi",
                title: "Wi-Fi & DNS Stack Flush",
                subtitle: "Connected (TP-Link Archer, 85% Link, 12ms)",
                icon_symbol: "📶",
                icon_color: [0.063, 0.725, 0.506, 1.0], // Emerald Wi-Fi
                is_repairing: false,
                shimmer_progress: 0.0,
            },
            DiagnosticRow {
                id: "bluetooth",
                title: "Bluetooth Stack Restart",
                subtitle: "Active (2 Wireless Peripherals Linked)",
                icon_symbol: "ᛒ",
                icon_color: [0.231, 0.510, 0.965, 1.0], // Royal Blue BT
                is_repairing: false,
                shimmer_progress: 0.0,
            },
            DiagnosticRow {
                id: "sqlite",
                title: "SQLite Core DB & WAL Checkpoint",
                subtitle: "Integrity Verified (WAL Active, B-Tree OK)",
                icon_symbol: "🗄️",
                icon_color: [0.984, 0.749, 0.141, 1.0], // Pastel Gold DB
                is_repairing: false,
                shimmer_progress: 0.0,
            },
            DiagnosticRow {
                id: "maindy_ai",
                title: "Maindy AI Stream & VRAM Purge",
                subtitle: "Optimal (Port 11434, Sovereign Tutor Ready)",
                icon_symbol: "🧠",
                icon_color: [0.659, 0.333, 0.965, 1.0], // Purple AI chip
                is_repairing: false,
                shimmer_progress: 0.0,
            },
        ]
    }
}

/// Telemetry state snapshot
#[derive(Debug, Clone)]
pub struct TelemetrySnapshot {
    pub rss_mb: f32,
    pub free_ram_mb: f32,
    pub fps: u32,
    pub frame_time_ms: f32,
    pub health_percent: f32,
    pub is_full_repairing: bool,
    pub port_11434_online: bool,
}

impl Default for TelemetrySnapshot {
    fn default() -> Self {
        Self {
            rss_mb: 1120.5,
            free_ram_mb: 3628.5,
            fps: 30,
            frame_time_ms: 20.80,
            health_percent: 99.4,
            is_full_repairing: false,
            port_11434_online: true,
        }
    }
}

/// Render the central System Telemetry & Doctor window
#[allow(clippy::too_many_arguments)]
pub fn render_system_telemetry_window(
    sdf: &mut SdfPipeline,
    txt: &mut TextRenderer,
    queue: &wgpu::Queue,
    window_rect: LayoutRect,
    telemetry: &TelemetrySnapshot,
    subsystems: &[DiagnosticRow],
    power_profile: PowerProfile,
    _is_active: bool,
) {
    // 1. Main Frosted Glass Panel Container (rgba(30, 50, 80, 0.45) with 24px radius)
    sdf.push(SdfInstance::glass_panel(
        window_rect.x,
        window_rect.y,
        window_rect.w,
        window_rect.h,
        MAIN_WINDOW_RADIUS,
        MAIN_WINDOW_GLASS_FILL,
        MAIN_WINDOW_BORDER_TOP,
    ));

    // Subtle drop shadow / inner depth line
    sdf.push(SdfInstance::button(
        window_rect.x + 12.0,
        window_rect.y + 40.0,
        window_rect.w - 24.0,
        1.0,
        0.0,
        [1.0, 1.0, 1.0, 0.12],
        [0.0; 4],
        0.0,
    ));

    // 2. Title Bar: Traffic Lights (Left), Title (Center-Left), Window Controls (Right)
    let traffic_y = window_rect.y + 14.0;
    sdf.push(SdfInstance::button(
        window_rect.x + 16.0,
        traffic_y,
        12.0,
        12.0,
        6.0,
        TRAFFIC_RED,
        [0.0; 4],
        0.0,
    ));
    sdf.push(SdfInstance::button(
        window_rect.x + 34.0,
        traffic_y,
        12.0,
        12.0,
        6.0,
        TRAFFIC_YELLOW,
        [0.0; 4],
        0.0,
    ));
    sdf.push(SdfInstance::button(
        window_rect.x + 52.0,
        traffic_y,
        12.0,
        12.0,
        6.0,
        TRAFFIC_GREEN,
        [0.0; 4],
        0.0,
    ));

    // Window Title
    txt.queue_text_bounded(
        queue,
        "System Telemetry & Doctor",
        window_rect.x + 76.0,
        window_rect.y + 12.0,
        window_rect.w - 180.0,
        13.5,
        TEXT_PRIMARY_HEADER,
    );

    // Right-aligned circular window controls: [-], [□], [×]
    let win_btn_y = window_rect.y + 10.0;
    let min_x = window_rect.x + window_rect.w - 74.0;
    let max_x = window_rect.x + window_rect.w - 48.0;
    let cls_x = window_rect.x + window_rect.w - 24.0;

    sdf.push(SdfInstance::button(
        min_x,
        win_btn_y,
        18.0,
        18.0,
        9.0,
        [1.0, 1.0, 1.0, 0.12],
        [0.0; 4],
        0.0,
    ));
    txt.queue_text_centered_in_rect(
        queue,
        "-",
        [min_x, win_btn_y, 18.0, 18.0],
        11.0,
        TEXT_SECONDARY_MUTED,
    );

    sdf.push(SdfInstance::button(
        max_x,
        win_btn_y,
        18.0,
        18.0,
        9.0,
        [1.0, 1.0, 1.0, 0.12],
        [0.0; 4],
        0.0,
    ));
    txt.queue_text_centered_in_rect(
        queue,
        "□",
        [max_x, win_btn_y, 18.0, 18.0],
        10.0,
        TEXT_SECONDARY_MUTED,
    );

    sdf.push(SdfInstance::button(
        cls_x,
        win_btn_y,
        18.0,
        18.0,
        9.0,
        [1.0, 0.3, 0.3, 0.20],
        [0.0; 4],
        0.0,
    ));
    txt.queue_text_centered_in_rect(
        queue,
        "×",
        [cls_x, win_btn_y, 18.0, 18.0],
        11.0,
        [1.0, 0.6, 0.6, 0.9],
    );

    // 3. Header Block (y = window_rect.y + 52.0)
    let header_y = window_rect.y + 52.0;

    // 3D glowing cyan SYS orb icon (64x64)
    let orb_x = window_rect.x + 24.0;
    let orb_size = 64.0;
    sdf.push(SdfInstance::button(
        orb_x - 3.0,
        header_y - 3.0,
        orb_size + 6.0,
        orb_size + 6.0,
        35.0,
        [0.22, 0.74, 0.97, 0.25],
        [0.0; 4],
        0.0,
    ));
    sdf.push(SdfInstance::button(
        orb_x,
        header_y,
        orb_size,
        orb_size,
        32.0,
        [0.08, 0.28, 0.48, 0.85],
        [0.4, 0.85, 1.0, 0.70],
        0.2,
    ));
    sdf.push(SdfInstance::button(
        orb_x + 12.0,
        header_y + 6.0,
        orb_size - 24.0,
        14.0,
        7.0,
        [1.0, 1.0, 1.0, 0.40],
        [0.0; 4],
        0.0,
    ));
    txt.queue_text_centered_in_rect(
        queue,
        "SYS",
        [orb_x, header_y + 14.0, orb_size, 36.0],
        15.0,
        [1.0, 1.0, 1.0, 0.95],
    );

    // Telemetry Text Block
    let text_x = orb_x + orb_size + 18.0;
    txt.queue_text_bounded(
        queue,
        "Hardware Telemetry & Health Monitor",
        text_x,
        header_y + 8.0,
        window_rect.w - 380.0,
        18.0,
        TEXT_PRIMARY_HEADER,
    );

    let telemetry_str = format!(
        "RSS {:.1} MB • Free {:.1} MB • {} FPS • {:.2} ms",
        telemetry.rss_mb, telemetry.free_ram_mb, telemetry.fps, telemetry.frame_time_ms
    );
    txt.queue_text_bounded(
        queue,
        &telemetry_str,
        text_x,
        header_y + 36.0,
        window_rect.w - 380.0,
        12.5,
        TEXT_SECONDARY_MUTED,
    );

    // Right-aligned Emerald Pill Badge: "System Health: 99.4%"
    let badge_w = 175.0;
    let badge_h = 32.0;
    let badge_x = window_rect.x + window_rect.w - badge_w - 24.0;
    let badge_y = header_y + 16.0;
    sdf.push(SdfInstance::button(
        badge_x,
        badge_y,
        badge_w,
        badge_h,
        16.0,
        HEALTH_BADGE_FILL,
        HEALTH_BADGE_BORDER,
        0.1,
    ));
    let health_str = format!("System Health: {:.1}%", telemetry.health_percent);
    txt.queue_text_centered_in_rect(
        queue,
        &health_str,
        [badge_x, badge_y, badge_w, badge_h],
        11.5,
        HEALTH_BADGE_TEXT,
    );

    // 4. Primary CTA Button: Full-width glossy cyan liquid pill button
    // "Run Full System Auto-Repair & Reclaim Memory" with wrench icon [🔧]
    let cta_y = header_y + orb_size + 18.0;
    let cta_h = 48.0;
    let cta_w = window_rect.w - 48.0;
    let cta_x = window_rect.x + 24.0;

    let (cta_fill, cta_label) = if telemetry.is_full_repairing {
        (
            [0.02, 0.40, 0.65, 0.95],
            "Auto-Repairing Subsystems & Flushing Buffers...",
        )
    } else {
        (
            PRIMARY_CTA_TOP,
            "Run Full System Auto-Repair & Reclaim Memory",
        )
    };

    sdf.push(SdfInstance::button(
        cta_x,
        cta_y,
        cta_w,
        cta_h,
        24.0,
        cta_fill,
        [0.5, 0.9, 1.0, 0.8],
        0.2,
    ));
    sdf.push(SdfInstance::button(
        cta_x + 16.0,
        cta_y + 3.0,
        cta_w - 32.0,
        14.0,
        7.0,
        PRIMARY_CTA_GLOW,
        [0.0; 4],
        0.0,
    ));

    txt.queue_text_centered_in_rect(
        queue,
        cta_label,
        [cta_x, cta_y, cta_w - 36.0, cta_h],
        13.5,
        [1.0, 1.0, 1.0, 0.98],
    );
    txt.queue_text_centered_in_rect(
        queue,
        "🔧",
        [cta_x + cta_w - 38.0, cta_y, 30.0, cta_h],
        15.0,
        [1.0, 1.0, 1.0, 0.95],
    );

    // 5. Section Header: "Self Healing Doctor & Subsystem Repair"
    let section_y = cta_y + cta_h + 16.0;
    txt.queue_text_bounded(
        queue,
        "Self Healing Doctor & Subsystem Repair",
        window_rect.x + 24.0,
        section_y,
        window_rect.w - 48.0,
        14.5,
        TEXT_PRIMARY_HEADER,
    );

    // 6. 5 Diagnostic Subsystem Rows (stacked cards with 64px height, 14px radius)
    let mut row_y = section_y + 24.0;
    let card_w = window_rect.w - 48.0;
    let card_h = 60.0;
    let card_x = window_rect.x + 24.0;

    for row in subsystems {
        sdf.push(SdfInstance::button(
            card_x,
            row_y,
            card_w,
            card_h,
            DIAGNOSTIC_ROW_RADIUS,
            DIAGNOSTIC_ROW_FILL,
            DIAGNOSTIC_ROW_BORDER,
            0.1,
        ));

        // Subsystem 3D Icon Badge (38x38 squircle)
        let icon_box_size = 38.0;
        let icon_box_x = card_x + 12.0;
        let icon_box_y = row_y + (card_h - icon_box_size) * 0.5;

        sdf.push(SdfInstance::button(
            icon_box_x,
            icon_box_y,
            icon_box_size,
            icon_box_size,
            10.0,
            [
                row.icon_color[0],
                row.icon_color[1],
                row.icon_color[2],
                0.22,
            ],
            [
                row.icon_color[0],
                row.icon_color[1],
                row.icon_color[2],
                0.60,
            ],
            0.1,
        ));
        txt.queue_text_centered_in_rect(
            queue,
            row.icon_symbol,
            [icon_box_x, icon_box_y, icon_box_size, icon_box_size],
            16.0,
            row.icon_color,
        );

        // Two-line low-contrast text block
        let text_block_x = icon_box_x + icon_box_size + 14.0;
        let text_block_w = card_w - 200.0;

        txt.queue_text_bounded(
            queue,
            row.title,
            text_block_x,
            row_y + 10.0,
            text_block_w,
            13.0,
            TEXT_PRIMARY_HEADER,
        );
        txt.queue_text_bounded(
            queue,
            row.subtitle,
            text_block_x,
            row_y + 30.0,
            text_block_w,
            11.0,
            TEXT_SECONDARY_MUTED,
        );

        // Right-aligned glossy amber-orange pill button: "One-Click Repair"
        let btn_w = 135.0;
        let btn_h = 32.0;
        let btn_x = card_x + card_w - btn_w - 14.0;
        let btn_y = row_y + (card_h - btn_h) * 0.5;

        let (btn_top, btn_text) = if row.is_repairing {
            ([0.35, 0.40, 0.45, 0.85], "Repairing...")
        } else {
            (REPAIR_PILL_TOP, "One-Click Repair")
        };

        sdf.push(SdfInstance::button(
            btn_x,
            btn_y,
            btn_w,
            btn_h,
            16.0,
            btn_top,
            [1.0, 0.85, 0.4, 0.6],
            0.15,
        ));
        sdf.push(SdfInstance::button(
            btn_x + 8.0,
            btn_y + 2.0,
            btn_w - 16.0,
            8.0,
            4.0,
            [1.0, 1.0, 1.0, 0.35],
            [0.0; 4],
            0.0,
        ));

        txt.queue_text_centered_in_rect(
            queue,
            btn_text,
            [btn_x, btn_y, btn_w, btn_h],
            11.0,
            optimal_text_color_for_fill(btn_top),
        );

        row_y += card_h + 8.0;
    }

    // 7. Footer: Power Profile Governor Segmented Control
    let footer_y = window_rect.y + window_rect.h - 50.0;
    txt.queue_text_bounded(
        queue,
        "Power Profile Governor:",
        window_rect.x + 24.0,
        footer_y + 8.0,
        180.0,
        12.5,
        TEXT_SECONDARY_MUTED,
    );

    let seg_x = window_rect.x + 210.0;
    let seg_w = 340.0;
    let seg_h = 34.0;

    sdf.push(SdfInstance::button(
        seg_x,
        footer_y,
        seg_w,
        seg_h,
        17.0,
        [0.05, 0.10, 0.18, 0.60],
        [1.0, 1.0, 1.0, 0.18],
        0.1,
    ));

    let item_w = (seg_w - 8.0) / 3.0;
    let profiles = PowerProfile::all();
    for (i, p) in profiles.iter().enumerate() {
        let pill_x = seg_x + 4.0 + (i as f32) * item_w;
        let is_selected = *p == power_profile;

        if is_selected {
            // Active Warm Yellow Highlight with Dark Slate Text (#FACC15 / #1E293B)
            sdf.push(SdfInstance::button(
                pill_x,
                footer_y + 3.0,
                item_w,
                seg_h - 6.0,
                14.0,
                BALANCED_PILL_FILL,
                [0.0; 4],
                0.0,
            ));
            txt.queue_text_centered_in_rect(
                queue,
                p.label(),
                [pill_x, footer_y + 3.0, item_w, seg_h - 6.0],
                11.5,
                CONTROL_CENTER_TEXT_DARK,
            );
        } else {
            txt.queue_text_centered_in_rect(
                queue,
                p.label(),
                [pill_x, footer_y + 3.0, item_w, seg_h - 6.0],
                11.5,
                TEXT_SECONDARY_MUTED,
            );
        }
    }
}
