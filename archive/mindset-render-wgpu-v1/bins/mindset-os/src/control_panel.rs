//! MindSet OS — Modern Liquid Control Center (`control_panel.rs`)
//! High-fidelity native liquidmorphic control panel with animated switches,
//! continuous brightness/volume/scale sliders, secondary tiles, and footer actions.
//! Single source of truth with `mindset_surface::ControlCenterLayout`.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use mindset_render::SdfInstance;
use mindset_surface::ControlCenterLayout;
use mindset_ui::LayoutRect;

/// State for the Modern Liquid Control Panel
#[derive(Debug, Clone)]
pub struct ModernControlPanelState {
    pub wifi_enabled: bool,
    pub wifi_ssid: String,
    pub bluetooth_enabled: bool,
    pub bluetooth_status: String,
    pub do_not_disturb: bool,
    pub airplane_mode: bool,

    // Continuous sliders (0.0 to 1.0)
    pub brightness: f32,
    pub volume: f32,
    pub keyboard_backlight: f32,

    // Secondary tiles
    pub monitor_name: String,
    pub night_light_active: bool,
}

impl Default for ModernControlPanelState {
    fn default() -> Self {
        Self {
            wifi_enabled: true,
            wifi_ssid: "HomeWiFi 5G".to_string(),
            bluetooth_enabled: true,
            bluetooth_status: "Active".to_string(),
            do_not_disturb: false,
            airplane_mode: false,
            brightness: 0.85,
            volume: 0.70,
            keyboard_backlight: 0.60,
            monitor_name: "Internal HDR".to_string(),
            night_light_active: true,
        }
    }
}

/// Render the Modern Liquid Control Center using geometry strictly from `ControlCenterLayout`.
pub fn render_modern_control_panel(
    draw: &mut ClippedDraw,
    layout: &ControlCenterLayout,
    state: &ModernControlPanelState,
    ui_scale: f32,
) {
    let s = ui_scale.clamp(0.80, 1.50);
    let rect = layout.panel_rect;

    // 1. Frosted Liquid Glass Container (Deep glass with high contrast)
    draw.sdf.push(SdfInstance::glass_panel(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        22.0 * s,
        [0.08, 0.11, 0.18, 0.92], // High-contrast deep glass
        [1.0, 1.0, 1.0, 0.35],    // Crisp specular border
    ));

    // Top rim specular highlight line
    draw.sdf.push(SdfInstance::button(
        rect.x + 16.0 * s,
        rect.y + 2.0 * s,
        rect.w - 32.0 * s,
        4.0 * s,
        2.0 * s,
        [1.0, 1.0, 1.0, 0.30],
        [0.0; 4],
        0.0,
    ));

    // 2. Header: Title & Action Buttons
    let header_y = rect.y + 14.0 * s;
    draw.queue_text(
        "Control Center",
        rect.x + 18.0 * s,
        header_y,
        15.0 * s,
        [0.98, 0.99, 1.0, 1.0],
    );

    // Settings button
    draw.sdf.push(SdfInstance::button(
        layout.settings_btn.x,
        layout.settings_btn.y,
        layout.settings_btn.w,
        layout.settings_btn.h,
        8.0 * s,
        [1.0, 1.0, 1.0, 0.12],
        [1.0, 1.0, 1.0, 0.25],
        0.05,
    ));
    draw.queue_text_centered_in_rect("⚙️", layout.settings_btn, 12.0 * s, [1.0, 1.0, 1.0, 1.0]);

    // Close button
    draw.sdf.push(SdfInstance::button(
        layout.close_btn.x,
        layout.close_btn.y,
        layout.close_btn.w,
        layout.close_btn.h,
        8.0 * s,
        [0.95, 0.30, 0.30, 0.25],
        [0.95, 0.30, 0.30, 0.45],
        0.05,
    ));
    draw.queue_text_centered_in_rect("✕", layout.close_btn, 11.5 * s, [1.0, 0.40, 0.40, 1.0]);

    // 3. Quick Tiles (2 Columns + Full-Width Night Light)
    let tiles = [
        (
            layout.wifi_tile,
            "📶",
            "Wi-Fi 5G",
            state.wifi_enabled,
            [0.0, 0.48, 1.0, 0.35],
        ),
        (
            layout.bluetooth_tile,
            "ᛒ",
            "Bluetooth",
            state.bluetooth_enabled,
            [0.0, 0.48, 1.0, 0.35],
        ),
        (
            layout.dnd_tile,
            "🌙",
            "Focus / DND",
            state.do_not_disturb,
            [0.55, 0.25, 0.85, 0.35],
        ),
        (
            layout.airplane_tile,
            "✈️",
            "Airplane",
            state.airplane_mode,
            [0.95, 0.55, 0.10, 0.35],
        ),
    ];

    for (tr, icon, label, is_active, active_bg) in tiles {
        let (bg, border) = if is_active {
            (active_bg, [1.0, 1.0, 1.0, 0.45])
        } else {
            ([1.0, 1.0, 1.0, 0.08], [1.0, 1.0, 1.0, 0.18])
        };
        draw.sdf.push(SdfInstance::button(
            tr.x,
            tr.y,
            tr.w,
            tr.h,
            10.0 * s,
            bg,
            border,
            0.05,
        ));
        let icon_rect = LayoutRect::new(
            tr.x + 8.0 * s,
            tr.y + (tr.h - 20.0 * s) * 0.5,
            20.0 * s,
            20.0 * s,
        );
        draw.queue_text_centered_in_rect(icon, icon_rect, 12.0 * s, [1.0, 1.0, 1.0, 1.0]);
        draw.queue_text_bounded(
            label,
            tr.x + 32.0 * s,
            tr.y + (tr.h - 13.0 * s) * 0.5,
            tr.w - 36.0 * s,
            11.5 * s,
            [0.95, 0.97, 1.0, 1.0],
        );
    }

    // Night Light full-width tile
    let nl = layout.night_light_tile;
    let (nl_bg, nl_border) = if state.night_light_active {
        ([0.95, 0.65, 0.15, 0.35], [1.0, 1.0, 1.0, 0.45])
    } else {
        ([1.0, 1.0, 1.0, 0.08], [1.0, 1.0, 1.0, 0.18])
    };
    draw.sdf.push(SdfInstance::button(
        nl.x,
        nl.y,
        nl.w,
        nl.h,
        10.0 * s,
        nl_bg,
        nl_border,
        0.05,
    ));
    let nl_icon_r = LayoutRect::new(
        nl.x + 8.0 * s,
        nl.y + (nl.h - 20.0 * s) * 0.5,
        20.0 * s,
        20.0 * s,
    );
    draw.queue_text_centered_in_rect("💡", nl_icon_r, 12.0 * s, [1.0, 1.0, 1.0, 1.0]);
    draw.queue_text_bounded(
        if state.night_light_active {
            "Night Warmth (Active)"
        } else {
            "Night Warmth (Off)"
        },
        nl.x + 32.0 * s,
        nl.y + (nl.h - 13.0 * s) * 0.5,
        nl.w - 36.0 * s,
        11.5 * s,
        [0.95, 0.97, 1.0, 1.0],
    );

    // 4. Continuous Sliders (Single Source of Truth)
    let thumb_r = layout.brightness_track.h * 0.5;

    // --- Brightness Slider ---
    let b_track = layout.brightness_track;
    let b_val = state.brightness.clamp(0.0, 1.0);
    draw.queue_text(
        &format!("☀️ Brightness  {}%", (b_val * 100.0).round() as u32),
        b_track.x + 2.0 * s,
        b_track.y - 15.0 * s,
        11.0 * s,
        [0.85, 0.89, 0.96, 1.0],
    );
    draw.sdf.push(SdfInstance::button(
        b_track.x,
        b_track.y,
        b_track.w,
        b_track.h,
        thumb_r,
        [0.15, 0.20, 0.30, 0.60],
        [1.0, 1.0, 1.0, 0.22],
        0.05,
    ));
    let b_fill = (b_track.w * b_val).max(0.0);
    draw.sdf.push(SdfInstance::button(
        b_track.x,
        b_track.y,
        b_fill,
        b_track.h,
        thumb_r,
        [0.98, 0.72, 0.15, 0.90],
        [0.0; 4],
        0.0,
    ));
    let b_cx = mindset_surface::slider_thumb_center_x(b_track, b_val, thumb_r);
    draw.sdf.push(SdfInstance::button(
        b_cx - thumb_r,
        b_track.y,
        thumb_r * 2.0,
        thumb_r * 2.0,
        thumb_r,
        [1.0, 1.0, 1.0, 1.0],
        [0.20, 0.25, 0.35, 0.40],
        0.15,
    ));

    // --- Master Volume Slider ---
    let v_track = layout.volume_track;
    let v_val = state.volume.clamp(0.0, 1.0);
    draw.queue_text(
        &format!("🔊 Volume  {}%", (v_val * 100.0).round() as u32),
        v_track.x + 2.0 * s,
        v_track.y - 15.0 * s,
        11.0 * s,
        [0.85, 0.89, 0.96, 1.0],
    );
    draw.sdf.push(SdfInstance::button(
        v_track.x,
        v_track.y,
        v_track.w,
        v_track.h,
        thumb_r,
        [0.15, 0.20, 0.30, 0.60],
        [1.0, 1.0, 1.0, 0.22],
        0.05,
    ));
    let v_fill = (v_track.w * v_val).max(0.0);
    draw.sdf.push(SdfInstance::button(
        v_track.x,
        v_track.y,
        v_fill,
        v_track.h,
        thumb_r,
        [0.0, 0.50, 1.0, 0.90],
        [0.0; 4],
        0.0,
    ));
    let v_cx = mindset_surface::slider_thumb_center_x(v_track, v_val, thumb_r);
    draw.sdf.push(SdfInstance::button(
        v_cx - thumb_r,
        v_track.y,
        thumb_r * 2.0,
        thumb_r * 2.0,
        thumb_r,
        [1.0, 1.0, 1.0, 1.0],
        [0.20, 0.25, 0.35, 0.40],
        0.15,
    ));

    // --- UI Scale Slider ---
    let s_track = layout.ui_scale_track;
    let s_prog = ((s - 0.80) / (1.50 - 0.80)).clamp(0.0, 1.0);
    draw.queue_text(
        &format!("🔍 UI Scale  {}%", (s * 100.0).round() as u32),
        s_track.x + 2.0 * s,
        s_track.y - 15.0 * s,
        11.0 * s,
        [0.85, 0.89, 0.96, 1.0],
    );
    draw.sdf.push(SdfInstance::button(
        s_track.x,
        s_track.y,
        s_track.w,
        s_track.h,
        thumb_r,
        [0.15, 0.20, 0.30, 0.60],
        [1.0, 1.0, 1.0, 0.22],
        0.05,
    ));
    let s_fill = (s_track.w * s_prog).max(0.0);
    draw.sdf.push(SdfInstance::button(
        s_track.x,
        s_track.y,
        s_fill,
        s_track.h,
        thumb_r,
        [0.15, 0.78, 0.50, 0.90],
        [0.0; 4],
        0.0,
    ));
    let s_cx = mindset_surface::slider_thumb_center_x(s_track, s_prog, thumb_r);
    draw.sdf.push(SdfInstance::button(
        s_cx - thumb_r,
        s_track.y,
        thumb_r * 2.0,
        thumb_r * 2.0,
        thumb_r,
        [1.0, 1.0, 1.0, 1.0],
        [0.20, 0.25, 0.35, 0.40],
        0.15,
    ));
}

/// Hit test action on the modern control panel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlPanelAction {
    Close,
    ToggleWifi,
    ToggleBluetooth,
    ToggleDnd,
    ToggleAirplane,
    SetBrightness(f32),
    SetVolume(f32),
    SetUiScale(f32),
    SetBacklight(f32),
    ToggleNightLight,
    OpenSettings,
    OpenProjection,
}

/// Hit-test against `ControlCenterLayout` (zero discrepancy with rendering geometry).
pub fn hit_test_control_panel(
    layout: &ControlCenterLayout,
    cx: f32,
    cy: f32,
    ui_scale: f32,
) -> Option<ControlPanelAction> {
    if !layout.panel_rect.contains(cx, cy) {
        return None;
    }

    let s = ui_scale.clamp(0.80, 1.50);

    // Header buttons
    if layout.close_btn.contains(cx, cy) {
        return Some(ControlPanelAction::Close);
    }
    if layout.settings_btn.contains(cx, cy) {
        return Some(ControlPanelAction::OpenSettings);
    }

    // Quick tiles
    if layout.wifi_tile.contains(cx, cy) {
        return Some(ControlPanelAction::ToggleWifi);
    }
    if layout.bluetooth_tile.contains(cx, cy) {
        return Some(ControlPanelAction::ToggleBluetooth);
    }
    if layout.dnd_tile.contains(cx, cy) {
        return Some(ControlPanelAction::ToggleDnd);
    }
    if layout.airplane_tile.contains(cx, cy) {
        return Some(ControlPanelAction::ToggleAirplane);
    }
    if layout.night_light_tile.contains(cx, cy) {
        return Some(ControlPanelAction::ToggleNightLight);
    }

    // Continuous Sliders (Drag band: at least track_h, e.g. track.y - 4*s .. track.y + track.h + 4*s)
    let thumb_r = layout.brightness_track.h * 0.5;

    let b_band = LayoutRect::new(
        layout.brightness_track.x - 4.0 * s,
        layout.brightness_track.y - 4.0 * s,
        layout.brightness_track.w + 8.0 * s,
        layout.brightness_track.h + 8.0 * s,
    );
    if b_band.contains(cx, cy) {
        let frac = mindset_surface::normalized_slider_value(layout.brightness_track, cx, thumb_r);
        return Some(ControlPanelAction::SetBrightness(frac));
    }

    let v_band = LayoutRect::new(
        layout.volume_track.x - 4.0 * s,
        layout.volume_track.y - 4.0 * s,
        layout.volume_track.w + 8.0 * s,
        layout.volume_track.h + 8.0 * s,
    );
    if v_band.contains(cx, cy) {
        let frac = mindset_surface::normalized_slider_value(layout.volume_track, cx, thumb_r);
        return Some(ControlPanelAction::SetVolume(frac));
    }

    let s_band = LayoutRect::new(
        layout.ui_scale_track.x - 4.0 * s,
        layout.ui_scale_track.y - 4.0 * s,
        layout.ui_scale_track.w + 8.0 * s,
        layout.ui_scale_track.h + 8.0 * s,
    );
    if s_band.contains(cx, cy) {
        let frac = mindset_surface::normalized_slider_value(layout.ui_scale_track, cx, thumb_r);
        let scale = (0.80 + frac * (1.50 - 0.80)).clamp(0.80, 1.50);
        return Some(ControlPanelAction::SetUiScale(scale));
    }

    None
}
