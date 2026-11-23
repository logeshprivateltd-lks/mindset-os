//! MindSet OS — Floating Capsule Top Bar (`top_bar.rs`)
//! Replaces edge-to-edge taskbars with visionOS floating liquid capsule islands.
//! Left: MindSet Flash workspace switcher & system menus.
//! Right: Date/time, connectivity telemetry, notifications, and Control Center launcher.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use mindset_render::SdfInstance;
use mindset_ui::LayoutRect;

pub struct TopCapsuleBarLayout {
    pub left_capsule: LayoutRect,
    pub flash_pill: LayoutRect,
    pub map_pill: LayoutRect,
    pub menu_file: LayoutRect,
    pub menu_edit: LayoutRect,
    pub menu_view: LayoutRect,
    pub menu_help: LayoutRect,

    pub right_capsule: LayoutRect,
    pub clock_pill: LayoutRect,
    pub wifi_pill: LayoutRect,
    pub notif_pill: LayoutRect,
    pub cc_button: LayoutRect,
}

impl TopCapsuleBarLayout {
    pub fn compute(screen_w: f32, ui_scale: f32) -> Self {
        let s = ui_scale.clamp(0.80, 1.50);
        let top_y = 12.0;
        let bar_h = 36.0;
        let left_pad = 16.0;

        // When ui_scale > 1.15, compress horizontal gaps and widths
        let pad_factor = if s > 1.15 { (1.15 / s).max(0.75) } else { 1.0 };

        let flash_pill_w = (118.0 * s * pad_factor).max(96.0);
        let map_pill_w = (88.0 * s * pad_factor).max(72.0);
        let menu_w = (54.0 * s * pad_factor).max(40.0);
        let gap = (4.0 * pad_factor).max(2.0);

        let left_capsule_w =
            12.0 + flash_pill_w + 6.0 + map_pill_w + 8.0 + (menu_w * 4.0 + gap * 3.0) + 12.0;
        let left_capsule = LayoutRect::new(left_pad, top_y, left_capsule_w, bar_h);

        let flash_pill = LayoutRect::new(
            left_capsule.x + 8.0,
            left_capsule.y + 5.0,
            flash_pill_w,
            bar_h - 10.0,
        );
        let map_pill = LayoutRect::new(
            flash_pill.x + flash_pill.w + 6.0,
            left_capsule.y + 5.0,
            map_pill_w,
            bar_h - 10.0,
        );

        let mut menu_x = map_pill.x + map_pill_w + 10.0;
        let menu_file = LayoutRect::new(menu_x, left_capsule.y + 6.0, menu_w, bar_h - 12.0);
        menu_x += menu_w + gap;
        let menu_edit = LayoutRect::new(menu_x, left_capsule.y + 6.0, menu_w, bar_h - 12.0);
        menu_x += menu_w + gap;
        let menu_view = LayoutRect::new(menu_x, left_capsule.y + 6.0, menu_w, bar_h - 12.0);
        menu_x += menu_w + gap;
        let menu_help = LayoutRect::new(menu_x, left_capsule.y + 6.0, menu_w, bar_h - 12.0);

        let clock_pill_w = (185.0 * s * pad_factor).max(150.0);
        let wifi_pill_w = (32.0 * s * pad_factor).max(26.0);
        let notif_pill_w = (42.0 * s * pad_factor).max(34.0);
        let cc_btn_w = (104.0 * s * pad_factor).max(84.0);

        let right_capsule_w =
            12.0 + clock_pill_w + 8.0 + wifi_pill_w + 6.0 + notif_pill_w + 8.0 + cc_btn_w + 12.0;
        let right_capsule_x =
            (screen_w - right_capsule_w - 20.0).max(left_capsule.x + left_capsule.w + 24.0);
        let right_capsule = LayoutRect::new(right_capsule_x, top_y, right_capsule_w, bar_h);

        let clock_pill = LayoutRect::new(
            right_capsule.x + 8.0,
            right_capsule.y + 6.0,
            clock_pill_w,
            bar_h - 12.0,
        );
        let wifi_pill = LayoutRect::new(
            clock_pill.x + clock_pill.w + 6.0,
            right_capsule.y + 6.0,
            wifi_pill_w,
            bar_h - 12.0,
        );
        let notif_pill = LayoutRect::new(
            wifi_pill.x + wifi_pill.w + 6.0,
            right_capsule.y + 6.0,
            notif_pill_w,
            bar_h - 12.0,
        );
        let cc_button = LayoutRect::new(
            notif_pill.x + notif_pill.w + 8.0,
            right_capsule.y + 5.0,
            cc_btn_w,
            bar_h - 10.0,
        );

        Self {
            left_capsule,
            flash_pill,
            map_pill,
            menu_file,
            menu_edit,
            menu_view,
            menu_help,
            right_capsule,
            clock_pill,
            wifi_pill,
            notif_pill,
            cc_button,
        }
    }

    pub fn calculate(screen_w: f32, _screen_h: f32) -> Self {
        Self::compute(screen_w, 1.0)
    }
}

pub fn render_top_capsule_bar(
    draw: &mut ClippedDraw,
    screen_w: f32,
    _screen_h: f32,
    _active_flash: u8,
    unread_notifs: usize,
    cc_is_open: bool,
    now_str: &str,
) -> TopCapsuleBarLayout {
    let layout = TopCapsuleBarLayout::compute(screen_w, 1.0);
    let flash_label = "⚡ MindSet OS ▾";

    // ─── 1. Left Capsule Island: MindSet Branding & Menus ─────────────────────
    draw.sdf.push(SdfInstance::glass_panel(
        layout.left_capsule.x,
        layout.left_capsule.y,
        layout.left_capsule.w,
        layout.left_capsule.h,
        18.0,
        [0.85, 0.92, 1.0, 0.65], // Frosted visionOS glass
        [1.0, 1.0, 1.0, 0.50],   // Specular rim
    ));

    // MindSet OS Trigger Pill
    draw.push_btn(
        layout.flash_pill,
        [0.0, 0.48, 1.0, 0.85],
        [1.0, 1.0, 1.0, 0.40],
    );
    draw.queue_text_centered_in_rect(flash_label, layout.flash_pill, 11.5, [1.0, 1.0, 1.0, 1.0]);

    // Odyssey Map Trigger Pill
    draw.push_btn(
        layout.map_pill,
        [0.98, 0.75, 0.14, 0.85],
        [1.0, 1.0, 1.0, 0.40],
    );
    draw.queue_text_centered_in_rect("🗺️ Map", layout.map_pill, 11.5, [0.10, 0.08, 0.02, 1.0]);

    // System Menus: File, Edit, View, Help
    draw.queue_text_centered_in_rect("File", layout.menu_file, 11.5, [0.12, 0.16, 0.23, 1.0]);
    draw.queue_text_centered_in_rect("Edit", layout.menu_edit, 11.5, [0.12, 0.16, 0.23, 1.0]);
    draw.queue_text_centered_in_rect("View", layout.menu_view, 11.5, [0.12, 0.16, 0.23, 1.0]);
    draw.queue_text_centered_in_rect("Help", layout.menu_help, 11.5, [0.12, 0.16, 0.23, 1.0]);

    // ─── 2. Right Capsule Island: Status & Control Center ──────────────────────
    draw.sdf.push(SdfInstance::glass_panel(
        layout.right_capsule.x,
        layout.right_capsule.y,
        layout.right_capsule.w,
        layout.right_capsule.h,
        18.0,
        [0.85, 0.92, 1.0, 0.65],
        [1.0, 1.0, 1.0, 0.50],
    ));

    // Date & Time Pill
    draw.queue_text_centered_in_rect(now_str, layout.clock_pill, 11.5, [0.12, 0.16, 0.23, 1.0]);

    // Wi-Fi Icon Badge
    draw.queue_text_centered_in_rect("📶", layout.wifi_pill, 12.0, [0.10, 0.45, 0.95, 1.0]);

    // Notifications Badge
    let notif_str = if unread_notifs > 0 {
        format!("🔔 {}", unread_notifs)
    } else {
        "🔔".to_string()
    };
    draw.queue_text_centered_in_rect(&notif_str, layout.notif_pill, 11.5, [0.12, 0.16, 0.23, 1.0]);

    // Control Center Trigger Button
    let cc_fill = if cc_is_open {
        [0.0, 0.48, 1.0, 0.90]
    } else {
        [1.0, 1.0, 1.0, 0.60]
    };
    let cc_tc = if cc_is_open {
        [1.0, 1.0, 1.0, 1.0]
    } else {
        [0.12, 0.16, 0.23, 1.0]
    };
    draw.push_btn(layout.cc_button, cc_fill, [1.0, 1.0, 1.0, 0.35]);
    draw.queue_text_centered_in_rect("⚙️ Controls", layout.cc_button, 11.0, cc_tc);

    layout
}

use mindset_surface::workspace::WORKSPACE_LABELS;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TopBarLayout {
    pub top_bar: LayoutRect,
    pub sys_btn: LayoutRect,
    pub ws_pills: [(LayoutRect, u8); 4],
    pub ws_count: usize,
    pub win_pill: LayoutRect,
    pub center_text_x: f32,
    pub center_text_y: f32,
    pub center_text_w: f32,
    pub btn_notif: LayoutRect,
    pub btn_search: LayoutRect,
    pub btn_mode: LayoutRect,
    pub btn_cc: LayoutRect,
    pub btn_lock: LayoutRect,
    pub btn_power: LayoutRect,
}

#[allow(clippy::too_many_arguments)]
pub fn compute_top_bar_layout(
    screen_w: f32,
    ui_scale: f32,
    workspaces_enabled: [bool; 4],
    active_workspace: u8,
    open_count: usize,
    unread_cnt: usize,
    mode_name: &str,
    now_str: &str,
    measure: impl Fn(&str, f32) -> (f32, f32),
) -> TopBarLayout {
    let s = ui_scale.clamp(0.85, 2.0);
    let bar_h = 40.0;
    let top_bar = LayoutRect::new(0.0, 0.0, screen_w, bar_h);
    let pad_x = 8.0 * s;
    let btn_h = 32.0;
    let btn_y = 4.0;
    let gap = (8.0 * s).ceil();

    // 1. Left side pills flowing forward
    let font_body = (11.0 * s).clamp(10.0, 14.0);
    let (sys_tw, _) = measure("MindSet OS", font_body);
    let sys_w = (sys_tw + 24.0 * s).ceil().max(80.0 * s);
    let sys_btn = LayoutRect::new(pad_x, btn_y, sys_w, btn_h);
    let mut left_cursor = pad_x + sys_w + gap;

    let mut ws_pills = [(LayoutRect::default(), 0u8); 4];
    let mut ws_count = 0;
    let font_ws = (10.5 * s).clamp(9.5, 13.0);
    for ws_idx in 0..4u8 {
        if !workspaces_enabled[ws_idx as usize] && ws_idx != active_workspace {
            continue;
        }
        let label = WORKSPACE_LABELS[ws_idx as usize];
        let (tw, _) = measure(label, font_ws);
        let pill_w = (tw + 18.0 * s).ceil().max(44.0 * s);
        let r = LayoutRect::new(left_cursor, btn_y, pill_w, btn_h);
        left_cursor += pill_w + gap;
        ws_pills[ws_count] = (r, ws_idx);
        ws_count += 1;
    }

    // 2. Right side controls flowing backward from right edge
    let font_ctl = (11.0 * s).clamp(10.0, 14.0);
    let mut right_cursor = screen_w - pad_x;

    let (pwr_tw, _) = measure("[Power]", font_ctl);
    let pwr_w = (pwr_tw + 22.0 * s).ceil();
    right_cursor -= pwr_w;
    let btn_power = LayoutRect::new(right_cursor, btn_y, pwr_w, btn_h);
    right_cursor -= gap;

    let (lock_tw, _) = measure("[Lock]", font_ctl);
    let lock_w = (lock_tw + 22.0 * s).ceil();
    right_cursor -= lock_w;
    let btn_lock = LayoutRect::new(right_cursor, btn_y, lock_w, btn_h);
    right_cursor -= gap;

    let (cc_tw, _) = measure("[Controls]", font_ctl);
    let cc_w = (cc_tw + 22.0 * s).ceil();
    right_cursor -= cc_w;
    let btn_cc = LayoutRect::new(right_cursor, btn_y, cc_w, btn_h);
    right_cursor -= gap;

    let (mode_tw, _) = measure(mode_name, font_ctl);
    let mode_w = (mode_tw + 22.0 * s).ceil();
    right_cursor -= mode_w;
    let btn_mode = LayoutRect::new(right_cursor, btn_y, mode_w, btn_h);
    right_cursor -= gap;

    let (src_tw, _) = measure("[Search]", font_ctl);
    let src_w = (src_tw + 22.0 * s).ceil();
    right_cursor -= src_w;
    let btn_search = LayoutRect::new(right_cursor, btn_y, src_w, btn_h);
    right_cursor -= gap;

    let notif_label = format!("[Alerts ({})]", unread_cnt);
    let (notif_tw, _) = measure(&notif_label, font_ctl);
    let notif_w = (notif_tw + 22.0 * s).ceil();
    right_cursor -= notif_w;
    let btn_notif = LayoutRect::new(right_cursor, btn_y, notif_w, btn_h);

    // 3. Middle area: between left_cursor and right_cursor
    let avail_center = (right_cursor - left_cursor).max(0.0);
    let win_pill_label = format!("[Windows: {}]", open_count);
    let (win_tw, _) = measure(&win_pill_label, font_ctl);
    let win_w = (win_tw + 18.0 * s).ceil().max(68.0 * s);

    let (win_pill, center_start) = if avail_center > win_w + 30.0 * s {
        let r = LayoutRect::new(left_cursor + gap, btn_y, win_w, btn_h);
        let start = r.x + r.w + gap;
        (r, start)
    } else {
        (
            LayoutRect::new(left_cursor, btn_y, 0.0, 0.0),
            left_cursor + gap,
        )
    };

    let center_end = right_cursor - gap;
    let free_center = (center_end - center_start).max(0.0);
    let (clk_tw, _) = measure(now_str, font_ctl);
    let clk_w = clk_tw.min(free_center);
    let center_text_x = center_start + (free_center - clk_w) * 0.5;
    let center_text_y = top_bar.y + 13.0;

    TopBarLayout {
        top_bar,
        sys_btn,
        ws_pills,
        ws_count,
        win_pill,
        center_text_x,
        center_text_y,
        center_text_w: clk_w,
        btn_notif,
        btn_search,
        btn_mode,
        btn_cc,
        btn_lock,
        btn_power,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_capsule_bar_layout_branding_and_proportions() {
        let screens = [1024.0, 1280.0, 1440.0, 1920.0, 2560.0, 3840.0];
        for &screen_w in &screens {
            let layout = TopCapsuleBarLayout::calculate(screen_w, 1080.0);

            // Clean MindSet OS branding width check
            assert_eq!(layout.flash_pill.w, 118.0, "flash_pill width must be 118.0");
            assert!(
                layout.left_capsule.w >= 450.0 && layout.left_capsule.w <= 500.0,
                "left_capsule width expected ~472.0, got {}",
                layout.left_capsule.w
            );

            // Strict left-to-right containment and non-overlapping invariant in left capsule
            assert!(layout.flash_pill.x >= layout.left_capsule.x);
            assert!(layout.map_pill.x >= layout.flash_pill.x + layout.flash_pill.w);
            assert!(layout.menu_file.x >= layout.map_pill.x + layout.map_pill.w);
            assert!(layout.menu_edit.x >= layout.menu_file.x + layout.menu_file.w);
            assert!(layout.menu_view.x >= layout.menu_edit.x + layout.menu_edit.w);
            assert!(layout.menu_help.x >= layout.menu_view.x + layout.menu_view.w);
            assert!(
                layout.menu_help.x + layout.menu_help.w
                    <= layout.left_capsule.x + layout.left_capsule.w
            );

            // Separation between left and right capsule islands
            assert!(
                layout.right_capsule.x >= layout.left_capsule.x + layout.left_capsule.w + 24.0,
                "right capsule must maintain minimum gap from left capsule"
            );

            // Strict left-to-right containment in right capsule
            assert!(layout.clock_pill.x >= layout.right_capsule.x);
            assert!(layout.wifi_pill.x >= layout.clock_pill.x + layout.clock_pill.w);
            assert!(layout.notif_pill.x >= layout.wifi_pill.x + layout.wifi_pill.w);
            assert!(layout.cc_button.x >= layout.notif_pill.x + layout.notif_pill.w);
            assert!(
                layout.cc_button.x + layout.cc_button.w
                    <= layout.right_capsule.x + layout.right_capsule.w
            );
        }
    }
}
