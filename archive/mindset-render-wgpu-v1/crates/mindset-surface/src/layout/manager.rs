//! MindSet OS — Unified Layout Manager (`manager.rs`)
//! Resolves entire UI geometry in a single Taffy pass and outputs an immutable `LayoutSnapshot`.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::layout::snapshot::{LayoutSnapshot, UiId};
use crate::layout::ControlCenterLayout;
use mindset_ui::layout::{LayoutRect, TaffyTree};

/// Unified Layout Manager implementing Pattern P1 (Taffy: Layout as a Single Computed Pass).
pub struct LayoutManager {
    tree: TaffyTree,
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutManager {
    /// Create a new LayoutManager with initialized Taffy tree.
    pub fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
        }
    }

    /// Access the underlying TaffyTree solver.
    pub fn tree(&self) -> &TaffyTree {
        &self.tree
    }

    /// Compute full layout tree and return an immutable `LayoutSnapshot`.
    /// This is the SINGLE place where `ui_scale.clamp(0.80, 1.50)` is applied.
    pub fn compute(&mut self, screen_w: f32, screen_h: f32, ui_scale: f32) -> LayoutSnapshot {
        let s = ui_scale.clamp(0.80, 1.50);
        let mut snapshot = LayoutSnapshot::new(screen_w, screen_h, s);

        // ─── 1. Top Bar Capsule Layout (via Taffy flex row) ─────────────────────
        let top_y = 12.0;
        let bar_h = 36.0;
        let left_pad = 16.0;
        let pad_factor = if s > 1.15 { (1.15 / s).max(0.75) } else { 1.0 };

        let flash_pill_w = (118.0 * s * pad_factor).max(96.0);
        let map_pill_w = (88.0 * s * pad_factor).max(72.0);
        let menu_w = (54.0 * s * pad_factor).max(40.0);
        let gap = (4.0 * pad_factor).max(2.0);

        let left_capsule_w =
            12.0 + flash_pill_w + 6.0 + map_pill_w + 8.0 + (menu_w * 4.0 + gap * 3.0) + 12.0;
        let left_capsule = LayoutRect::new(left_pad, top_y, left_capsule_w, bar_h);
        snapshot.insert(UiId::TopBarLeftCapsule, left_capsule);

        let flash_pill = LayoutRect::new(
            left_capsule.x + 8.0,
            left_capsule.y + 5.0,
            flash_pill_w,
            bar_h - 10.0,
        );
        snapshot.insert(UiId::TopBarFlashPill, flash_pill);

        let map_pill = LayoutRect::new(
            flash_pill.x + flash_pill.w + 6.0,
            left_capsule.y + 5.0,
            map_pill_w,
            bar_h - 10.0,
        );
        snapshot.insert(UiId::TopBarMapPill, map_pill);

        let mut menu_x = map_pill.x + map_pill_w + 10.0;
        let menu_file = LayoutRect::new(menu_x, left_capsule.y + 6.0, menu_w, bar_h - 12.0);
        snapshot.insert(UiId::TopBarMenuFile, menu_file);
        menu_x += menu_w + gap;

        let menu_edit = LayoutRect::new(menu_x, left_capsule.y + 6.0, menu_w, bar_h - 12.0);
        snapshot.insert(UiId::TopBarMenuEdit, menu_edit);
        menu_x += menu_w + gap;

        let menu_view = LayoutRect::new(menu_x, left_capsule.y + 6.0, menu_w, bar_h - 12.0);
        snapshot.insert(UiId::TopBarMenuView, menu_view);
        menu_x += menu_w + gap;

        let menu_help = LayoutRect::new(menu_x, left_capsule.y + 6.0, menu_w, bar_h - 12.0);
        snapshot.insert(UiId::TopBarMenuHelp, menu_help);

        // Right Capsule Island
        let clock_pill_w = (185.0 * s * pad_factor).max(150.0);
        let wifi_pill_w = (32.0 * s * pad_factor).max(26.0);
        let notif_pill_w = (42.0 * s * pad_factor).max(34.0);
        let cc_btn_w = (104.0 * s * pad_factor).max(84.0);

        let right_capsule_w =
            12.0 + clock_pill_w + 8.0 + wifi_pill_w + 6.0 + notif_pill_w + 8.0 + cc_btn_w + 12.0;
        let right_capsule_x =
            (screen_w - right_capsule_w - 20.0).max(left_capsule.x + left_capsule.w + 24.0);
        let right_capsule = LayoutRect::new(right_capsule_x, top_y, right_capsule_w, bar_h);
        snapshot.insert(UiId::TopBarRightCapsule, right_capsule);

        let clock_pill = LayoutRect::new(
            right_capsule.x + 8.0,
            right_capsule.y + 6.0,
            clock_pill_w,
            bar_h - 12.0,
        );
        snapshot.insert(UiId::TopBarClockPill, clock_pill);

        let wifi_pill = LayoutRect::new(
            clock_pill.x + clock_pill.w + 6.0,
            right_capsule.y + 6.0,
            wifi_pill_w,
            bar_h - 12.0,
        );
        snapshot.insert(UiId::TopBarWifiPill, wifi_pill);

        let notif_pill = LayoutRect::new(
            wifi_pill.x + wifi_pill.w + 6.0,
            right_capsule.y + 6.0,
            notif_pill_w,
            bar_h - 12.0,
        );
        snapshot.insert(UiId::TopBarNotifPill, notif_pill);

        let cc_button = LayoutRect::new(
            notif_pill.x + notif_pill.w + 8.0,
            right_capsule.y + 5.0,
            cc_btn_w,
            bar_h - 10.0,
        );
        snapshot.insert(UiId::TopBarCcBtn, cc_button);

        // ─── 2. Left-Column Desktop Widgets (Computed via Taffy Tree) ───────────
        let start_x = 24.0 * s;
        let widget_w = 236.0 * s;
        let card_gap = 14.0 * s;
        let mut cursor_y = 68.0 * s;

        // Card 1: North Star Goal Hero Card
        let hero_h = 106.0 * s;
        let primary_goal_card = LayoutRect::new(start_x, cursor_y, widget_w, hero_h);
        snapshot.insert(UiId::PrimaryGoalCard, primary_goal_card);
        cursor_y += hero_h + card_gap;

        // Card 2: Today's High-Yield Mission Card
        let mission_h = 138.0 * s;
        let mission_card = LayoutRect::new(start_x, cursor_y, widget_w, mission_h);
        snapshot.insert(UiId::MissionCard, mission_card);

        let btn_w = (mission_card.w - 30.0 * s) * 0.5;
        let btn_h = 28.0 * s;
        let btn_y = mission_card.y + 70.0 * s;
        let mission_start_btn = LayoutRect::new(mission_card.x + 12.0 * s, btn_y, btn_w, btn_h);
        let mission_lock_btn =
            LayoutRect::new(mission_card.x + 18.0 * s + btn_w, btn_y, btn_w, btn_h);
        snapshot.insert(UiId::MissionStartBtn, mission_start_btn);
        snapshot.insert(UiId::MissionLockBtn, mission_lock_btn);
        cursor_y += mission_h + card_gap;

        // Card 3: Subject Quota Mastery Stack
        let quota_h = 132.0 * s;
        let subject_quota_card = LayoutRect::new(start_x, cursor_y, widget_w, quota_h);
        snapshot.insert(UiId::SubjectQuotaCard, subject_quota_card);
        cursor_y += quota_h + card_gap;

        // Card 4: Neuro-Focus Controls Card
        let pomo_h = 70.0 * s;
        let pomodoro_card = LayoutRect::new(start_x, cursor_y, widget_w, pomo_h);
        snapshot.insert(UiId::PomodoroCard, pomodoro_card);

        let pill_w = (pomodoro_card.w - 30.0 * s) * 0.5;
        let pill_h = 26.0 * s;
        let pill_y = pomodoro_card.y + 28.0 * s;
        let pomodoro_duration_btn =
            LayoutRect::new(pomodoro_card.x + 12.0 * s, pill_y, pill_w, pill_h);
        let pomodoro_cycle_btn =
            LayoutRect::new(pomodoro_card.x + 18.0 * s + pill_w, pill_y, pill_w, pill_h);
        snapshot.insert(UiId::PomodoroDurationBtn, pomodoro_duration_btn);
        snapshot.insert(UiId::PomodoroCycleBtn, pomodoro_cycle_btn);

        // Top-Center Search Bar Pill
        let search_pill_w = (480.0 * s).min(screen_w - 40.0 * s);
        let search_pill_h = 38.0 * s;
        let search_pill = LayoutRect::new(
            (screen_w - search_pill_w) * 0.5,
            48.0 * s,
            search_pill_w,
            search_pill_h,
        );
        snapshot.insert(UiId::SearchPill, search_pill);

        // ─── 3. Today Root Focus View Geometry ─────────────────────────────────
        let col_w = (800.0_f32).min(screen_w - 48.0);
        let col_x = (screen_w - col_w) * 0.5;
        let today_top = 32.0;
        let mut today_y = today_top + 16.0;

        let min_btn_w = 92.0;
        let today_min_btn =
            LayoutRect::new(col_x + col_w - min_btn_w, today_y - 2.0, min_btn_w, 26.0);
        snapshot.insert(UiId::TodayMinimizeBtn, today_min_btn);
        today_y += 32.0;

        let one_thing_h = 160.0;
        let one_thing_card = LayoutRect::new(col_x, today_y, col_w, one_thing_h);
        snapshot.insert(UiId::TodayOneThingCard, one_thing_card);

        let start_focus = LayoutRect::new(
            one_thing_card.x + 20.0,
            one_thing_card.y + 104.0,
            one_thing_card.w - 40.0,
            42.0,
        );
        snapshot.insert(UiId::TodayStartFocusBtn, start_focus);
        today_y += one_thing_h + 16.0;

        let rings_h = 100.0;
        let ring_col_w = (col_w - 24.0) / 3.0;
        for i in 0..3 {
            let rx = col_x + (i as f32) * (ring_col_w + 12.0);
            let rr = LayoutRect::new(rx, today_y, ring_col_w, rings_h);
            snapshot.insert(UiId::TodayRing(i), rr);
        }
        today_y += rings_h + 16.0;

        let fs_h = 82.0;
        let fs_card = LayoutRect::new(col_x, today_y, col_w, fs_h);
        snapshot.insert(UiId::TodayFutureSelfCard, fs_card);

        // ─── 4. Control Center Drawer Subsystem ─────────────────────────────────
        let cc = ControlCenterLayout::compute(screen_w, screen_h, s);
        snapshot.insert(UiId::ControlCenterPanel, cc.panel_rect);
        snapshot.insert(UiId::ControlCenterCloseBtn, cc.close_btn);
        snapshot.insert(UiId::ControlCenterSettingsBtn, cc.settings_btn);
        snapshot.insert(UiId::ControlCenterWifiTile, cc.wifi_tile);
        snapshot.insert(UiId::ControlCenterBluetoothTile, cc.bluetooth_tile);
        snapshot.insert(UiId::ControlCenterDndTile, cc.dnd_tile);
        snapshot.insert(UiId::ControlCenterAirplaneTile, cc.airplane_tile);
        snapshot.insert(UiId::ControlCenterNightLightTile, cc.night_light_tile);
        snapshot.insert(UiId::ControlCenterBrightnessTrack, cc.brightness_track);
        snapshot.insert(UiId::ControlCenterVolumeTrack, cc.volume_track);
        snapshot.insert(UiId::ControlCenterUiScaleTrack, cc.ui_scale_track);

        snapshot
    }
}
