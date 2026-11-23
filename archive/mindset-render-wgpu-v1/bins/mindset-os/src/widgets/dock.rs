//! MindSet OS — Liquidmorphism Floating Dock Widget (`dock.rs`)
//! Pure Rust native component featuring 8 3D tactile squircle app icons,
//! smooth spring-physics hover scaling, pill badges, and glassmorphic reflection.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::theme::*;
use mindset_render::{SdfInstance, SdfPipeline, TextRenderer};

/// A dock icon specification
#[derive(Debug, Clone)]
pub struct DockIconItem {
    pub id: &'static str,
    pub title: &'static str,
    pub badge: &'static str,
    pub symbol: &'static str,
    pub bg_color: [f32; 4],
    pub hover_scale: f32,
    pub is_active: bool,
}

impl DockIconItem {
    pub fn default_items() -> Vec<DockIconItem> {
        vec![
            DockIconItem {
                id: "tasks",
                title: "Tasks",
                badge: "✓",
                symbol: "✓",
                bg_color: DOCK_COLOR_TASKS,
                hover_scale: 1.0,
                is_active: false,
            },
            DockIconItem {
                id: "reader",
                title: "Reader",
                badge: "RD",
                symbol: "📖",
                bg_color: DOCK_COLOR_READER,
                hover_scale: 1.0,
                is_active: false,
            },
            DockIconItem {
                id: "create",
                title: "Add Note",
                badge: "+",
                symbol: "+",
                bg_color: DOCK_COLOR_CREATE,
                hover_scale: 1.0,
                is_active: false,
            },
            DockIconItem {
                id: "search",
                title: "Browser & Shield",
                badge: "SRC",
                symbol: "🌐",
                bg_color: DOCK_COLOR_SEARCH,
                hover_scale: 1.0,
                is_active: false,
            },
            DockIconItem {
                id: "files",
                title: "Files & Vault",
                badge: "DIR",
                symbol: "📁",
                bg_color: DOCK_COLOR_DIR,
                hover_scale: 1.0,
                is_active: false,
            },
            DockIconItem {
                id: "stats",
                title: "Study Analytics",
                badge: "ST",
                symbol: "📊",
                bg_color: DOCK_COLOR_STATS,
                hover_scale: 1.0,
                is_active: false,
            },
            DockIconItem {
                id: "system",
                title: "System Doctor",
                badge: "SYS",
                symbol: "💻",
                bg_color: DOCK_COLOR_SYS,
                hover_scale: 1.0,
                is_active: true,
            },
            DockIconItem {
                id: "settings",
                title: "Settings",
                badge: "SET",
                symbol: "⚙️",
                bg_color: DOCK_COLOR_SET,
                hover_scale: 1.0,
                is_active: false,
            },
        ]
    }
}

/// Render the bottom compact floating liquid dock
pub fn render_liquid_dock(
    sdf: &mut SdfPipeline,
    txt: &mut TextRenderer,
    queue: &wgpu::Queue,
    screen_w: f32,
    screen_h: f32,
    items: &[DockIconItem],
    hovered_idx: Option<usize>,
) {
    let num_items = items.len() as f32;
    let icon_base_size = 44.0;
    let icon_gap = 10.0;
    let pad_h = 18.0;

    let dock_w = pad_h * 2.0 + num_items * icon_base_size + (num_items - 1.0) * icon_gap;
    let dock_h = DOCK_HEIGHT;
    let dock_x = (screen_w - dock_w) * 0.5;
    let dock_y = screen_h - dock_h - 16.0;

    // 1. Frosted Liquid Glass Dock Background (rgba(255, 255, 255, 0.22) with 24px radius)
    sdf.push(SdfInstance::glass_panel(
        dock_x,
        dock_y,
        dock_w,
        dock_h,
        DOCK_RADIUS,
        DOCK_GLASS_FILL,
        DOCK_BORDER,
    ));

    // Top rim specular highlight line
    sdf.push(SdfInstance::button(
        dock_x + 16.0,
        dock_y + 2.0,
        dock_w - 32.0,
        10.0,
        5.0,
        [1.0, 1.0, 1.0, 0.35],
        [0.0; 4],
        0.0,
    ));

    // 2. Render 8 3D Squircle Icons
    for (i, item) in items.iter().enumerate() {
        let is_hovered = hovered_idx == Some(i);
        let scale = if is_hovered {
            1.15
        } else {
            item.hover_scale.clamp(1.0, 1.15)
        };

        let scaled_size = icon_base_size * scale;
        let delta = (scaled_size - icon_base_size) * 0.5;

        let base_x = dock_x + pad_h + (i as f32) * (icon_base_size + icon_gap);
        let item_x = base_x - delta;
        let item_y = dock_y + 10.0 - delta;

        // Outer 3D squircle glow / shadow on hover or active
        if is_hovered || item.is_active {
            sdf.push(SdfInstance::button(
                item_x - 3.0,
                item_y - 3.0,
                scaled_size + 6.0,
                scaled_size + 6.0,
                14.0 * scale,
                [item.bg_color[0], item.bg_color[1], item.bg_color[2], 0.35],
                [0.0; 4],
                0.0,
            ));
        }

        // Squircle body with 3D liquid border
        sdf.push(SdfInstance::button(
            item_x,
            item_y,
            scaled_size,
            scaled_size,
            12.0 * scale,
            item.bg_color,
            [1.0, 1.0, 1.0, 0.65],
            0.15,
        ));

        // Top glossy specular highlight
        sdf.push(SdfInstance::button(
            item_x + 4.0 * scale,
            item_y + 2.0 * scale,
            scaled_size - 8.0 * scale,
            (scaled_size * 0.35).min(14.0),
            6.0 * scale,
            [1.0, 1.0, 1.0, 0.45],
            [0.0; 4],
            0.0,
        ));

        // Center icon symbol
        txt.queue_text_centered_in_rect(
            queue,
            item.symbol,
            [item_x, item_y, scaled_size, scaled_size - 4.0],
            16.0 * scale,
            [1.0, 1.0, 1.0, 0.98],
        );

        // Small frosted pill text badge directly underneath each icon
        let badge_w = 26.0;
        let badge_h = 13.0;
        let badge_x = base_x + (icon_base_size - badge_w) * 0.5;
        let badge_y = dock_y + dock_h - 18.0;

        sdf.push(SdfInstance::button(
            badge_x,
            badge_y,
            badge_w,
            badge_h,
            6.5,
            [0.10, 0.15, 0.25, 0.70],
            [1.0, 1.0, 1.0, 0.35],
            0.05,
        ));
        txt.queue_text_centered_in_rect(
            queue,
            item.badge,
            [badge_x, badge_y, badge_w, badge_h],
            9.0,
            [1.0, 1.0, 1.0, 0.92],
        );

        // Active indicator dot under active items
        if item.is_active {
            let dot_size = 4.0;
            let dot_x = base_x + (icon_base_size - dot_size) * 0.5;
            let dot_y = dock_y + dock_h - 4.0;
            sdf.push(SdfInstance::button(
                dot_x,
                dot_y,
                dot_size,
                dot_size,
                2.0,
                [1.0, 1.0, 1.0, 0.85],
                [0.0; 4],
                0.0,
            ));
        }
    }
}
