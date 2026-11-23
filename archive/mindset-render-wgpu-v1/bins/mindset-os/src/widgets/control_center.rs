//! MindSet OS — Top-Right Floating Control Center Widget (`control_center.rs`)
//! Pure Rust native component featuring bright frosted glassmorphism,
//! iOS-style toggle switches, live connectivity readouts, and quick actions.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::theme::*;
use mindset_render::{SdfInstance, SdfPipeline, TextRenderer};
use mindset_ui::LayoutRect;

/// Control Center State
#[derive(Debug, Clone)]
pub struct ControlCenterState {
    pub alerts_enabled: bool,
    pub search_enabled: bool,
    pub focus_mode_enabled: bool,
    pub wifi_ssid: String,
    pub bluetooth_status: String,
}

impl Default for ControlCenterState {
    fn default() -> Self {
        Self {
            alerts_enabled: true,
            search_enabled: false,
            focus_mode_enabled: false,
            wifi_ssid: "TP-Link".to_string(),
            bluetooth_status: "Active".to_string(),
        }
    }
}

/// Render the floating top-right Control Center widget
pub fn render_control_center_widget(
    sdf: &mut SdfPipeline,
    txt: &mut TextRenderer,
    queue: &wgpu::Queue,
    rect: LayoutRect,
    state: &ControlCenterState,
    timestamp_str: &str,
) {
    // 1. Frosted Glass Container (rgba(225, 238, 255, 0.72) with 22px radius)
    sdf.push(SdfInstance::glass_panel(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        CONTROL_CENTER_RADIUS,
        CONTROL_CENTER_GLASS_FILL,
        CONTROL_CENTER_BORDER,
    ));

    // Subtle drop shadow / rim lighting
    sdf.push(SdfInstance::button(
        rect.x + 8.0,
        rect.y + 3.0,
        rect.w - 16.0,
        10.0,
        5.0,
        [1.0, 1.0, 1.0, 0.40],
        [0.0; 4],
        0.0,
    ));

    // 2. Header: Timestamp
    let header_y = rect.y + 14.0;
    txt.queue_text_centered_in_rect(
        queue,
        timestamp_str,
        [rect.x, header_y, rect.w, 22.0],
        12.5,
        CONTROL_CENTER_TEXT_DARK,
    );

    // Thin divider line
    let div1_y = header_y + 26.0;
    sdf.push(SdfInstance::button(
        rect.x + 14.0,
        div1_y,
        rect.w - 28.0,
        1.0,
        0.0,
        [0.12, 0.16, 0.23, 0.15],
        [0.0; 4],
        0.0,
    ));

    // 3. 2-Column Quick Connectivity Grid: Wi-Fi ("TP-Link") & Bluetooth ("Active")
    let grid_y = div1_y + 10.0;
    let col_w = (rect.w - 36.0) / 2.0;
    let col_h = 44.0;

    // Left Column: Wi-Fi
    let wifi_x = rect.x + 14.0;
    sdf.push(SdfInstance::button(
        wifi_x,
        grid_y,
        col_w,
        col_h,
        12.0,
        [1.0, 1.0, 1.0, 0.65],
        [0.23, 0.51, 0.96, 0.35],
        0.1,
    ));
    txt.queue_text_centered_in_rect(
        queue,
        "📶",
        [wifi_x + 8.0, grid_y + 6.0, 24.0, 32.0],
        16.0,
        TOGGLE_ACTIVE_BLUE,
    );
    txt.queue_text_bounded(
        queue,
        &state.wifi_ssid,
        wifi_x + 36.0,
        grid_y + 14.0,
        col_w - 40.0,
        11.5,
        CONTROL_CENTER_TEXT_DARK,
    );

    // Right Column: Bluetooth
    let bt_x = wifi_x + col_w + 8.0;
    sdf.push(SdfInstance::button(
        bt_x,
        grid_y,
        col_w,
        col_h,
        12.0,
        [1.0, 1.0, 1.0, 0.65],
        [0.23, 0.51, 0.96, 0.35],
        0.1,
    ));
    txt.queue_text_centered_in_rect(
        queue,
        "ᛒ",
        [bt_x + 8.0, grid_y + 6.0, 24.0, 32.0],
        16.0,
        TOGGLE_ACTIVE_BLUE,
    );
    txt.queue_text_bounded(
        queue,
        &state.bluetooth_status,
        bt_x + 36.0,
        grid_y + 14.0,
        col_w - 40.0,
        11.5,
        CONTROL_CENTER_TEXT_DARK,
    );

    // Thin divider line
    let div2_y = grid_y + col_h + 10.0;
    sdf.push(SdfInstance::button(
        rect.x + 14.0,
        div2_y,
        rect.w - 28.0,
        1.0,
        0.0,
        [0.12, 0.16, 0.23, 0.15],
        [0.0; 4],
        0.0,
    ));

    // 4. 3 iOS-style Animated Toggle Rows
    let mut toggle_y = div2_y + 10.0;
    let toggle_h = 36.0;
    let row_w = rect.w - 28.0;
    let row_x = rect.x + 14.0;

    let toggles = [
        ("Alerts (4)", state.alerts_enabled),
        ("Search", state.search_enabled),
        ("Mode: Standard", state.focus_mode_enabled),
    ];

    for (label, is_on) in toggles {
        // Row background
        sdf.push(SdfInstance::button(
            row_x,
            toggle_y,
            row_w,
            toggle_h,
            10.0,
            [1.0, 1.0, 1.0, 0.45],
            [0.0; 4],
            0.05,
        ));

        // Label
        txt.queue_text_bounded(
            queue,
            label,
            row_x + 12.0,
            toggle_y + 10.0,
            row_w - 60.0,
            12.0,
            CONTROL_CENTER_TEXT_DARK,
        );

        // iOS Pill Switch (44x22)
        let sw_w = 44.0;
        let sw_h = 22.0;
        let sw_x = row_x + row_w - sw_w - 8.0;
        let sw_y = toggle_y + (toggle_h - sw_h) * 0.5;

        let track_color = if is_on {
            TOGGLE_ACTIVE_BLUE
        } else {
            TOGGLE_INACTIVE_TRACK
        };
        sdf.push(SdfInstance::button(
            sw_x,
            sw_y,
            sw_w,
            sw_h,
            11.0,
            track_color,
            [0.0; 4],
            0.0,
        ));

        // Switch thumb circle
        let thumb_x = if is_on {
            sw_x + sw_w - 20.0
        } else {
            sw_x + 2.0
        };
        sdf.push(SdfInstance::button(
            thumb_x,
            sw_y + 2.0,
            18.0,
            18.0,
            9.0,
            [1.0, 1.0, 1.0, 0.98],
            [0.0; 4],
            0.0,
        ));

        toggle_y += toggle_h + 8.0;
    }

    // Thin divider line before footer
    let div3_y = toggle_y + 4.0;
    sdf.push(SdfInstance::button(
        rect.x + 14.0,
        div3_y,
        rect.w - 28.0,
        1.0,
        0.0,
        [0.12, 0.16, 0.23, 0.15],
        [0.0; 4],
        0.0,
    ));

    // 5. 2-Column Footer Actions: [Lock] and [Power]
    let footer_y = div3_y + 10.0;
    let act_w = (rect.w - 36.0) / 2.0;
    let act_h = 36.0;

    // Lock Button
    let lock_x = rect.x + 14.0;
    sdf.push(SdfInstance::button(
        lock_x,
        footer_y,
        act_w,
        act_h,
        10.0,
        [1.0, 1.0, 1.0, 0.60],
        [0.12, 0.16, 0.23, 0.20],
        0.1,
    ));
    txt.queue_text_centered_in_rect(
        queue,
        "🔒 Lock",
        [lock_x, footer_y, act_w, act_h],
        12.0,
        CONTROL_CENTER_TEXT_DARK,
    );

    // Power Button
    let power_x = lock_x + act_w + 8.0;
    sdf.push(SdfInstance::button(
        power_x,
        footer_y,
        act_w,
        act_h,
        10.0,
        [1.0, 0.9, 0.9, 0.65],
        [0.9, 0.3, 0.3, 0.30],
        0.1,
    ));
    txt.queue_text_centered_in_rect(
        queue,
        "⏻ Power",
        [power_x, footer_y, act_w, act_h],
        12.0,
        [0.85, 0.20, 0.20, 1.0],
    );
}
