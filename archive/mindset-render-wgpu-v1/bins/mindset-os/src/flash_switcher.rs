//! MindSet OS — MindSet Flash Workspace Manager & Switcher Modal (`flash_switcher.rs`)
//! Replaces legacy virtual desktops with sovereign "MindSet Flash" workspaces.
//! Renders visual thumbnail cards showing open windows and interactive switching.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use crate::theme::*;
use mindset_render::SdfInstance;
use mindset_surface::{WindowId, WindowManager};
use mindset_ui::LayoutRect;

/// A MindSet Flash Workspace definition
#[derive(Debug, Clone)]
pub struct FlashWorkspaceInfo {
    pub index: u8,
    pub name: &'static str,
    pub tagline: &'static str,
    pub symbol: &'static str,
    pub accent_color: [f32; 4],
}

impl FlashWorkspaceInfo {
    pub fn all() -> [FlashWorkspaceInfo; 4] {
        [
            FlashWorkspaceInfo {
                index: 0,
                name: "MindSet Flash 1",
                tagline: "Focus & Academic Study",
                symbol: "⚡1",
                accent_color: [0.063, 0.725, 0.506, 1.0], // Emerald green
            },
            FlashWorkspaceInfo {
                index: 1,
                name: "MindSet Flash 2",
                tagline: "Academic Browser & Shield",
                symbol: "⚡2",
                accent_color: [0.231, 0.510, 0.965, 1.0], // Royal blue
            },
            FlashWorkspaceInfo {
                index: 2,
                name: "MindSet Flash 3",
                tagline: "File Vault & Documents",
                symbol: "⚡3",
                accent_color: [0.984, 0.749, 0.141, 1.0], // Golden amber
            },
            FlashWorkspaceInfo {
                index: 3,
                name: "MindSet Flash 4",
                tagline: "System Doctor & Analytics",
                symbol: "⚡4",
                accent_color: [0.659, 0.333, 0.965, 1.0], // Purple
            },
        ]
    }
}

/// Render the floating MindSet Flash Switcher modal (top-center floating cards)
pub fn render_flash_switcher_modal(
    draw: &mut ClippedDraw,
    screen_w: f32,
    _screen_h: f32,
    active_flash: u8,
    wm: &WindowManager,
) -> LayoutRect {
    let flashes = FlashWorkspaceInfo::all();
    let num_flashes = flashes.len() as f32;

    let card_w = 170.0;
    let card_h = 148.0;
    let card_gap = 14.0;
    let pad_h = 20.0;

    let modal_w = pad_h * 2.0 + num_flashes * card_w + (num_flashes - 1.0) * card_gap;
    let modal_h = 224.0;
    let modal_x = (screen_w - modal_w) * 0.5;
    let modal_y = 64.0;
    let modal_rect = LayoutRect::new(modal_x, modal_y, modal_w, modal_h);

    // 1. Frosted Liquid Glass Container
    draw.sdf.push(SdfInstance::glass_panel(
        modal_rect.x,
        modal_rect.y,
        modal_rect.w,
        modal_rect.h,
        22.0,
        [0.08, 0.12, 0.20, 0.88],
        [1.0, 1.0, 1.0, 0.35],
    ));

    // Top specular highlight rim
    draw.sdf.push(SdfInstance::button(
        modal_rect.x + 16.0,
        modal_rect.y + 2.0,
        modal_rect.w - 32.0,
        8.0,
        4.0,
        [1.0, 1.0, 1.0, 0.30],
        [0.0; 4],
        0.0,
    ));

    // 2. Header & Subtitle
    draw.queue_text(
        "⚡ MindSet Flash Switcher",
        modal_rect.x + pad_h,
        modal_rect.y + 14.0,
        14.5,
        TEXT_PRIMARY_HEADER,
    );
    draw.queue_text(
        "Which Flash do you want to switch to? • Click a card or press [1 - 4]",
        modal_rect.x + pad_h,
        modal_rect.y + 32.0,
        11.0,
        TEXT_SECONDARY_MUTED,
    );

    // Close pill on top right
    let esc_pill = LayoutRect::new(
        modal_rect.x + modal_rect.w - pad_h - 70.0,
        modal_rect.y + 14.0,
        70.0,
        24.0,
    );
    draw.push_btn(esc_pill, [0.20, 0.25, 0.35, 0.60], [1.0, 1.0, 1.0, 0.20]);
    draw.queue_text_centered_in_rect("[Esc Close]", esc_pill, 10.0, [1.0, 1.0, 1.0, 0.90]);

    // 3. Render 4 Flash Preview Cards
    let start_y = modal_rect.y + 58.0;
    for (i, flash) in flashes.iter().enumerate() {
        let card_x = modal_rect.x + pad_h + (i as f32) * (card_w + card_gap);
        let card_rect = LayoutRect::new(card_x, start_y, card_w, card_h);
        let is_active = flash.index == active_flash;

        // Card glass background
        let bg_fill = if is_active {
            [0.15, 0.24, 0.38, 0.95]
        } else {
            [0.10, 0.15, 0.24, 0.70]
        };
        let border_col = if is_active {
            flash.accent_color
        } else {
            [1.0, 1.0, 1.0, 0.15]
        };

        draw.sdf.push(SdfInstance::glass_panel(
            card_rect.x,
            card_rect.y,
            card_rect.w,
            card_rect.h,
            14.0,
            bg_fill,
            border_col,
        ));

        // Miniature Desktop Window Preview Thumbnail Box
        let thumb_rect = LayoutRect::new(
            card_rect.x + 10.0,
            card_rect.y + 10.0,
            card_rect.w - 20.0,
            64.0,
        );
        draw.sdf.push(SdfInstance::button(
            thumb_rect.x,
            thumb_rect.y,
            thumb_rect.w,
            thumb_rect.h,
            8.0,
            [0.05, 0.08, 0.14, 0.95],
            [1.0, 1.0, 1.0, 0.12],
            0.1,
        ));

        // Draw miniature window representations in this Flash
        let open_wins: Vec<WindowId> = wm
            .windows
            .iter()
            .filter(|w| w.workspace_id == flash.index && w.is_open)
            .map(|w| w.id)
            .collect();

        if open_wins.is_empty() {
            draw.queue_text_centered_in_rect(
                "Empty Flash",
                thumb_rect,
                10.0,
                [0.50, 0.60, 0.70, 0.60],
            );
        } else {
            // Draw miniature floating window boxes
            let mini_w = (thumb_rect.w - 16.0) / (open_wins.len().min(2) as f32);
            let mini_h = thumb_rect.h - 14.0;
            for (w_idx, wid) in open_wins.iter().take(2).enumerate() {
                let mw_x = thumb_rect.x + 8.0 + (w_idx as f32) * (mini_w + 4.0);
                let mw_y = thumb_rect.y + 7.0;
                let mini_rect = LayoutRect::new(mw_x, mw_y, mini_w - 4.0, mini_h);

                draw.sdf.push(SdfInstance::button(
                    mini_rect.x,
                    mini_rect.y,
                    mini_rect.w,
                    mini_rect.h,
                    5.0,
                    [0.18, 0.26, 0.40, 0.95],
                    [1.0, 1.0, 1.0, 0.25],
                    0.05,
                ));
                // Mini titlebar dot
                draw.sdf.push(SdfInstance::button(
                    mini_rect.x + 3.0,
                    mini_rect.y + 3.0,
                    4.0,
                    4.0,
                    2.0,
                    wid.badge_color(),
                    [0.0; 4],
                    0.0,
                ));
                let sym = wid.icon_symbol();
                draw.queue_text_centered_in_rect(sym, mini_rect, 9.0, [1.0, 1.0, 1.0, 0.85]);
            }
        }

        // Flash Symbol & Name
        draw.queue_text(
            flash.name,
            card_rect.x + 10.0,
            card_rect.y + 82.0,
            12.0,
            TEXT_PRIMARY_HEADER,
        );

        // Flash Tagline
        draw.queue_text_bounded(
            flash.tagline,
            card_rect.x + 10.0,
            card_rect.y + 98.0,
            card_rect.w - 20.0,
            10.0,
            TEXT_SECONDARY_MUTED,
        );

        // Bottom Pill Badge: Active or Switch
        let pill_rect = LayoutRect::new(
            card_rect.x + 10.0,
            card_rect.y + 118.0,
            card_rect.w - 20.0,
            20.0,
        );
        if is_active {
            draw.push_btn(pill_rect, flash.accent_color, [0.0; 4]);
            draw.queue_text_centered_in_rect(
                "✓ CURRENT FLASH",
                pill_rect,
                9.5,
                [1.0, 1.0, 1.0, 1.0],
            );
        } else {
            draw.push_btn(pill_rect, [0.18, 0.22, 0.32, 0.60], [1.0, 1.0, 1.0, 0.15]);
            let label = format!("Switch to [{}]", i + 1);
            draw.queue_text_centered_in_rect(&label, pill_rect, 9.5, [0.85, 0.90, 0.98, 0.90]);
        }
    }

    modal_rect
}

/// Hit-test which Flash card was clicked inside the switcher modal
pub fn hit_test_flash_switcher(
    click_x: f32,
    click_y: f32,
    screen_w: f32,
    __screen_h: f32,
) -> Option<u8> {
    let flashes = FlashWorkspaceInfo::all();
    let num_flashes = flashes.len() as f32;

    let card_w = 170.0;
    let card_h = 148.0;
    let card_gap = 14.0;
    let pad_h = 20.0;

    let modal_w = pad_h * 2.0 + num_flashes * card_w + (num_flashes - 1.0) * card_gap;
    let modal_x = (screen_w - modal_w) * 0.5;
    let modal_y = 64.0;
    let start_y = modal_y + 58.0;

    for (i, _) in flashes.iter().enumerate() {
        let card_x = modal_x + pad_h + (i as f32) * (card_w + card_gap);
        let card_rect = LayoutRect::new(card_x, start_y, card_w, card_h);
        if card_rect.contains(click_x, click_y) {
            return Some(i as u8);
        }
    }
    None
}
