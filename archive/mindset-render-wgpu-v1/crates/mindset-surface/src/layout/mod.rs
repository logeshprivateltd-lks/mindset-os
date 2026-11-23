//! MindSet OS — Surface Layout Subsystem (`layout/mod.rs`)
//! Single source of truth for Control Center, window chrome buttons, and Taffy layout.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod chrome;
pub mod manager;
pub mod overlay;
pub mod snapshot;

pub use chrome::{WindowChromeHitTargets, WindowChromeManager};
pub use manager::LayoutManager;
pub use overlay::{OverlayDispatchResult, OverlayManager};
pub use snapshot::{LayoutSnapshot, LayoutViolation, UiId};

use mindset_ui::LayoutRect;

/// Normalized continuous slider value calculation (clamped [0.0, 1.0], zero-panic).
#[inline]
pub fn normalized_slider_value(track: LayoutRect, cursor_x: f32, thumb_radius: f32) -> f32 {
    let usable_w = track.w - 2.0 * thumb_radius;
    if usable_w <= 0.0 {
        return 0.0;
    }
    let rel_x = cursor_x - (track.x + thumb_radius);
    (rel_x / usable_w).clamp(0.0, 1.0)
}

/// Calculate thumb center X position given progress [0.0, 1.0] and thumb radius.
#[inline]
pub fn slider_thumb_center_x(track: LayoutRect, progress: f32, thumb_radius: f32) -> f32 {
    let usable_w = (track.w - 2.0 * thumb_radius).max(0.0);
    track.x + thumb_radius + progress.clamp(0.0, 1.0) * usable_w
}

/// Complete geometric layout and hit targets for the modern Control Center overlay.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlCenterLayout {
    pub panel_rect: LayoutRect,
    pub brightness_track: LayoutRect,
    pub brightness_thumb: LayoutRect,
    pub volume_track: LayoutRect,
    pub volume_thumb: LayoutRect,
    pub ui_scale_track: LayoutRect,
    pub ui_scale_thumb: LayoutRect,

    pub wifi_tile: LayoutRect,
    pub bluetooth_tile: LayoutRect,
    pub dnd_tile: LayoutRect,
    pub airplane_tile: LayoutRect,
    pub night_light_tile: LayoutRect,

    pub close_btn: LayoutRect,
    pub settings_btn: LayoutRect,
}

impl ControlCenterLayout {
    /// Compute Control Center layout strictly from screen dimensions and clamped UI scale.
    pub fn compute(screen_w: f32, screen_h: f32, ui_scale: f32) -> Self {
        let s = ui_scale.clamp(0.80, 1.50);

        let panel_w = (340.0 * s).min(screen_w - 20.0 * s);
        let panel_h = (360.0 * s).min(screen_h - 60.0 * s);
        let panel_x = screen_w - panel_w - 16.0 * s;
        let panel_y = 48.0 * s;
        let panel_rect = LayoutRect::new(panel_x, panel_y, panel_w, panel_h);

        // Header action buttons
        let btn_sz = 24.0 * s;
        let close_btn = LayoutRect::new(
            panel_x + panel_w - 16.0 * s - btn_sz,
            panel_y + 12.0 * s,
            btn_sz,
            btn_sz,
        );
        let settings_btn = LayoutRect::new(
            close_btn.x - 8.0 * s - btn_sz,
            panel_y + 12.0 * s,
            btn_sz,
            btn_sz,
        );

        // Quick tiles (2 columns)
        let tile_gap = 8.0 * s;
        let pad_x = 18.0 * s;
        let tile_w = (panel_w - pad_x * 2.0 - tile_gap) * 0.5;
        let tile_h = 36.0 * s;

        let row1_y = panel_y + 46.0 * s;
        let wifi_tile = LayoutRect::new(panel_x + pad_x, row1_y, tile_w, tile_h);
        let bluetooth_tile =
            LayoutRect::new(wifi_tile.x + tile_w + tile_gap, row1_y, tile_w, tile_h);

        let row2_y = row1_y + tile_h + 6.0 * s;
        let dnd_tile = LayoutRect::new(panel_x + pad_x, row2_y, tile_w, tile_h);
        let airplane_tile = LayoutRect::new(dnd_tile.x + tile_w + tile_gap, row2_y, tile_w, tile_h);

        let row3_y = row2_y + tile_h + 6.0 * s;
        let night_light_tile =
            LayoutRect::new(panel_x + pad_x, row3_y, panel_w - pad_x * 2.0, tile_h);

        // Continuous Slider tracks
        let track_w = panel_w - pad_x * 2.0;
        let track_h = 22.0 * s;
        let track_x = panel_x + pad_x;
        let thumb_r = track_h * 0.5;

        let brightness_track = LayoutRect::new(track_x, panel_y + 210.0 * s, track_w, track_h);
        let volume_track = LayoutRect::new(track_x, panel_y + 252.0 * s, track_w, track_h);
        let ui_scale_track = LayoutRect::new(track_x, panel_y + 294.0 * s, track_w, track_h);

        // Default initial thumbs (0.85 brightness, 0.70 volume, current ui_scale)
        let b_cx = slider_thumb_center_x(brightness_track, 0.85, thumb_r);
        let brightness_thumb =
            LayoutRect::new(b_cx - thumb_r, brightness_track.y, thumb_r * 2.0, track_h);

        let v_cx = slider_thumb_center_x(volume_track, 0.70, thumb_r);
        let volume_thumb = LayoutRect::new(v_cx - thumb_r, volume_track.y, thumb_r * 2.0, track_h);

        let s_prog = ((s - 0.80) / (1.50 - 0.80)).clamp(0.0, 1.0);
        let s_cx = slider_thumb_center_x(ui_scale_track, s_prog, thumb_r);
        let ui_scale_thumb =
            LayoutRect::new(s_cx - thumb_r, ui_scale_track.y, thumb_r * 2.0, track_h);

        Self {
            panel_rect,
            brightness_track,
            brightness_thumb,
            volume_track,
            volume_thumb,
            ui_scale_track,
            ui_scale_thumb,
            wifi_tile,
            bluetooth_tile,
            dnd_tile,
            airplane_tile,
            night_light_tile,
            close_btn,
            settings_btn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_slider_value_bounds() {
        let track = LayoutRect::new(100.0, 200.0, 200.0, 22.0);
        let r = 11.0;
        assert_eq!(normalized_slider_value(track, 100.0, r), 0.0);
        assert_eq!(normalized_slider_value(track, 111.0, r), 0.0);
        assert_eq!(normalized_slider_value(track, 300.0, r), 1.0);
        assert_eq!(normalized_slider_value(track, 289.0, r), 1.0);
        let mid = normalized_slider_value(track, 200.0, r);
        assert!((mid - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_window_chrome_hit_targets_scaling() {
        let win = LayoutRect::new(100.0, 100.0, 600.0, 400.0);
        let targets_100 = WindowChromeHitTargets::from_rect(win, 1.0);
        let targets_145 = WindowChromeHitTargets::from_rect(win, 1.45);

        assert!(targets_145.close_btn.w > targets_100.close_btn.w);
        assert!(targets_145.max_btn.w > targets_100.max_btn.w);
        assert!(targets_145.min_btn.w > targets_100.min_btn.w);
        assert!(win.contains(targets_100.close_btn.x, targets_100.close_btn.y));
        assert!(win.contains(targets_145.close_btn.x, targets_145.close_btn.y));
    }

    #[test]
    fn test_layout_manager_snapshot_and_taffy_resolution() {
        let mut lm = LayoutManager::new();
        let snapshot = lm.compute(1920.0, 1080.0, 1.0);

        assert!(snapshot.get(UiId::TopBarLeftCapsule).is_some());
        assert!(snapshot.get(UiId::PrimaryGoalCard).is_some());
        assert!(snapshot.get(UiId::MissionCard).is_some());
        assert!(snapshot.get(UiId::SubjectQuotaCard).is_some());
        assert!(snapshot.get(UiId::PomodoroCard).is_some());
        assert!(snapshot.get(UiId::SearchPill).is_some());
        assert!(snapshot.get(UiId::TodayMinimizeBtn).is_some());

        // Test hit test
        let top_pill = snapshot.rect(UiId::TopBarFlashPill);
        let hit = snapshot.hit_test((top_pill.x + 5.0, top_pill.y + 5.0));
        assert!(hit.is_some());

        // Invariant check: cards don't overlap
        let cards = [
            UiId::PrimaryGoalCard,
            UiId::MissionCard,
            UiId::SubjectQuotaCard,
            UiId::PomodoroCard,
        ];
        assert!(snapshot.assert_no_overlaps(&cards).is_ok());
    }

    #[test]
    fn test_overlay_manager_strict_dispatch_order() {
        let mut lm = LayoutManager::new();
        let snapshot = lm.compute(1280.0, 800.0, 1.0);

        // 1. Click on close button inside Control Center panel
        let close_btn = snapshot.rect(UiId::ControlCenterCloseBtn);
        let res_item = OverlayManager::dispatch(
            mindset_ui::ActiveOverlay::ControlCenter,
            (close_btn.x + 2.0, close_btn.y + 2.0),
            &snapshot,
        );
        assert_eq!(
            res_item,
            OverlayDispatchResult::Item(UiId::ControlCenterCloseBtn)
        );

        // 2. Click on empty space inside panel body
        let panel = snapshot.rect(UiId::ControlCenterPanel);
        let res_trap = OverlayManager::dispatch(
            mindset_ui::ActiveOverlay::ControlCenter,
            (panel.x + 10.0, panel.y + 10.0),
            &snapshot,
        );
        assert_eq!(res_trap, OverlayDispatchResult::PanelTrapped);

        // 3. Click outside panel
        let res_dismiss = OverlayManager::dispatch(
            mindset_ui::ActiveOverlay::ControlCenter,
            (50.0, 50.0),
            &snapshot,
        );
        assert_eq!(res_dismiss, OverlayDispatchResult::BackdropDismiss);
    }
}
