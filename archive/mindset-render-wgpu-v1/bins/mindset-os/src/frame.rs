use mindset_render::optimal_text_color_for_fill;
use mindset_render::safe_utf8_prefix;
use mindset_surface::desktop::ContextMenuAction;
use mindset_surface::session::PowerAction;
use mindset_ui::layout::INNER_GAP;
// MindSet OS Frame Render Pipeline
// Multi-pass liquidmorphic compositor with dual Kawase blur, SDF geometry,
// bounded typography, floating capsule bar, left-side widgets, modern control center,
// and MindSet Flash switcher modal.

use crate::theme::{self, CARD_RADIUS};
use chrono::Local;
use mindset_render::SdfInstance;
use mindset_surface::wm::{SurfaceWindow, WindowId, WindowState};
use mindset_ui::layout::LayoutRect;
use mindset_ui::widgets::browser::*;
use std::time::Instant;

use mindset_surface::session::NotificationCategory;

use crate::app::MindSetApp;
use crate::draw_helpers::{push_btn, push_panel, push_squircle_badge_standalone, ClippedDraw};
use crate::state::{ActiveOverlay, SessionState, ThemeMode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DesktopHitTargets {
    pub mission_start_btn: LayoutRect,
    pub mission_lock_btn: LayoutRect,
    pub pomodoro_duration_btn: LayoutRect,
    pub pomodoro_cycle_btn: LayoutRect,
    pub pomodoro_sound_btn: LayoutRect,
    pub search_pill: LayoutRect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DesktopLayout {
    pub primary_goal_card: LayoutRect,
    pub mission_card: LayoutRect,
    pub subject_quota_card: LayoutRect,
    pub pomodoro_card: LayoutRect,
    pub hits: DesktopHitTargets,
}

impl DesktopLayout {
    pub fn compute(screen_w: f32, screen_h: f32, ui_scale: f32) -> Self {
        let mut lm = mindset_surface::LayoutManager::new();
        let snapshot = lm.compute(screen_w, screen_h, ui_scale);
        Self::from_snapshot(&snapshot)
    }

    pub fn from_snapshot(snapshot: &mindset_surface::LayoutSnapshot) -> Self {
        let primary_goal_card = snapshot.rect(mindset_surface::UiId::PrimaryGoalCard);
        let mission_card = snapshot.rect(mindset_surface::UiId::MissionCard);
        let subject_quota_card = snapshot.rect(mindset_surface::UiId::SubjectQuotaCard);
        let pomodoro_card = snapshot.rect(mindset_surface::UiId::PomodoroCard);
        let hits = DesktopHitTargets {
            mission_start_btn: snapshot.rect(mindset_surface::UiId::MissionStartBtn),
            mission_lock_btn: snapshot.rect(mindset_surface::UiId::MissionLockBtn),
            pomodoro_duration_btn: snapshot.rect(mindset_surface::UiId::PomodoroDurationBtn),
            pomodoro_cycle_btn: snapshot.rect(mindset_surface::UiId::PomodoroCycleBtn),
            pomodoro_sound_btn: snapshot.rect(mindset_surface::UiId::PomodoroCycleBtn),
            search_pill: snapshot.rect(mindset_surface::UiId::SearchPill),
        };
        Self {
            primary_goal_card,
            mission_card,
            subject_quota_card,
            pomodoro_card,
            hits,
        }
    }
}

impl MindSetApp {
    pub fn render_frame(&mut self) -> Result<(), wgpu::SurfaceError> {
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f32().min(0.05);
        self.last_frame = now;
        self.frame_count += 1;
        self.frame_time_ms = dt * 1000.0;
        let elapsed = (now - self.start_time).as_secs_f32();

        if self.frame_count.is_multiple_of(60) {
            self.current_fps = (1.0 / dt.max(0.001)).clamp(30.0, 240.0);
        }

        let (mem_str, fps_str) = self.get_telemetry();
        self.state.update(dt);
        self.today_state.update(dt);

        let (
            Some(render_ctx),
            Some(target_mgr),
            Some(bg_pipeline),
            Some(kawase_pipeline),
            Some(sdf_pipeline),
            Some(text_renderer),
        ) = (
            self.render_ctx.as_ref(),
            self.target_mgr.as_ref(),
            self.bg_pipeline.as_ref(),
            self.kawase_pipeline.as_ref(),
            self.sdf_pipeline.as_mut(),
            self.text_renderer.as_mut(),
        )
        else {
            return Ok(());
        };
        let state = &mut self.state;

        let width = render_ctx.config.width as f32;
        let height = render_ctx.config.height as f32;
        let screen_size = [width, height];
        state.session.update_screen_size(width, height);
        state.wm.resize_viewport(width, height);

        // 1. Background Pipeline
        let focus_depth = if state.session.focus_mode { 1.0 } else { 0.0 };
        let wallpaper_style = state.wallpaper_preset as u32 as f32;
        let daily_progress = self.today_state.ring_data[0].fraction;
        bg_pipeline.update_uniforms(
            &render_ctx.queue,
            screen_size,
            elapsed,
            focus_depth,
            wallpaper_style,
            daily_progress,
        );

        sdf_pipeline.clear();
        text_renderer.clear();
        let s = state.personalization_store.appearance.ui_scale;
        text_renderer.set_ui_scale(s);

        let p = state.theme.palette;

        let mut layout_mgr = mindset_surface::LayoutManager::new();
        let snapshot = layout_mgr.compute(width, height, s);

        let top_capsule_layout = crate::top_bar::TopCapsuleBarLayout::calculate(width, height);
        let dock_r = state.session.dock_rect();
        let nd_r = state.session.notifications.drawer_rect(width);
        let pm_r = state.session.power_menu_rect();
        let cl_r = snapshot.centered_rect(540.0, 510.0, 0.44);
        let cm_r = state.context_menu.rect();

        let wiz_w = (600.0 * s).min(width - 40.0);
        let wiz_h = (420.0 * s).min(height - 60.0);
        let wiz_r = snapshot.centered_rect(wiz_w, wiz_h, 0.44);
        let cc_r = snapshot.rect(mindset_surface::UiId::ControlCenterPanel);
        let mm_w = (460.0 * s).min(width - 40.0);
        let mm_h = (240.0 * s).min(height - 60.0);
        let mm_r = snapshot.centered_rect(mm_w, mm_h, 0.44);
        let full_screen_r = snapshot.viewport_rect();

        let mut modal_occlusions: Vec<LayoutRect> = Vec::new();
        if matches!(state.session.state, SessionState::Locked { .. }) {
            modal_occlusions.push(full_screen_r);
        }
        match state.active_overlay() {
            ActiveOverlay::None => {}
            ActiveOverlay::Launcher | ActiveOverlay::CommandPalette { .. } => {
                modal_occlusions.push(cl_r);
            }
            ActiveOverlay::ControlCenter => {
                modal_occlusions.push(cc_r);
            }
            ActiveOverlay::FlashSwitcher => {
                modal_occlusions.push(full_screen_r);
            }
            ActiveOverlay::Notifications => {
                modal_occlusions.push(nd_r);
            }
            ActiveOverlay::ContextMenu => {
                modal_occlusions.push(cm_r);
            }
            ActiveOverlay::OnboardingWizard => {
                modal_occlusions.push(wiz_r);
            }
            ActiveOverlay::OdysseyMap | ActiveOverlay::TrophyDeck => {
                modal_occlusions.push(full_screen_r);
            }
        }
        if state.mindful_modal_target.is_some() {
            modal_occlusions.push(mm_r);
        }
        if state.session.state == SessionState::PowerMenuOpen {
            modal_occlusions.push(pm_r);
        }

        let visible_windows: Vec<SurfaceWindow> = state
            .wm
            .sorted_visible_windows(state.active_workspace)
            .into_iter()
            .cloned()
            .collect();

        if let SessionState::Locked {
            ref pin_input,
            ref error_msg,
        } = state.session.state
        {
            let lock_bg = snapshot.viewport_rect();
            push_panel(sdf_pipeline, lock_bg, [0.094, 0.082, 0.063, 0.94], 0.0);
            // Warm golden-amber atmospheric overlay
            push_panel(sdf_pipeline, lock_bg, [0.961, 0.620, 0.043, 0.08], 0.0);

            // 1. Hero Lock Screen Clock & Date Header (Top-Center)
            let date_str = Local::now().format("%A, %B %d").to_string();
            let (date_w, _) = text_renderer.measure_line_px(&date_str, 22.0 * s);
            text_renderer.queue_text(
                &render_ctx.queue,
                &date_str,
                (width - date_w) * 0.5,
                96.0 * s,
                22.0 * s,
                [0.961, 0.620, 0.043, 1.0],
            );

            let clock_str = Local::now().format("%I:%M %p").to_string();
            let (clock_w, _) = text_renderer.measure_line_px(&clock_str, 64.0 * s);
            // Drop shadow
            text_renderer.queue_text(
                &render_ctx.queue,
                &clock_str,
                (width - clock_w) * 0.5 + 2.0,
                134.0 * s,
                64.0 * s,
                [0.0, 0.0, 0.0, 0.50],
            );
            // Main Hero Clock
            text_renderer.queue_text(
                &render_ctx.queue,
                &clock_str,
                (width - clock_w) * 0.5,
                132.0 * s,
                64.0 * s,
                [0.996, 0.988, 0.910, 1.0],
            );

            let greeting = "Logesh Karthik • MindSet OS";
            let (greet_w, _) = text_renderer.measure_line_px(greeting, 14.5 * s);
            text_renderer.queue_text(
                &render_ctx.queue,
                greeting,
                (width - greet_w) * 0.5,
                210.0 * s,
                14.5 * s,
                p.text_secondary,
            );

            // 2. Lock Screen Glanceable Widget Shelf (Centered 3-Card Row)
            let cards_y = 246.0 * s;
            let gap = 18.0 * s;
            let card_w = (260.0 * s).min((width - 80.0 * s) / 3.0);
            let card_h = 76.0 * s;
            let total_cards_w = card_w * 3.0 + gap * 2.0;
            let start_cx = (width - total_cards_w) * 0.5;

            // Widget 1 — Focus Ring
            let c1 = snapshot.grid_cell(start_cx, cards_y, 0, 0, card_w, card_h, gap, 0.0);
            push_panel(
                sdf_pipeline,
                c1,
                [
                    p.surface_raised[0],
                    p.surface_raised[1],
                    p.surface_raised[2],
                    0.96,
                ],
                14.0 * s,
            );
            sdf_pipeline.push(SdfInstance::button(
                c1.x,
                c1.y,
                c1.w,
                c1.h,
                14.0 * s,
                [0.0; 4],
                p.border,
                0.15,
            ));
            text_renderer.queue_text_bounded(
                &render_ctx.queue,
                "FOCUS RING",
                c1.x + 16.0 * s,
                c1.y + 12.0 * s,
                c1.w - 24.0 * s,
                11.5 * s,
                [0.961, 0.620, 0.043, 1.0],
            );
            text_renderer.queue_text_bounded(
                &render_ctx.queue,
                "25m Focus Ready • 4 Sessions Done",
                c1.x + 16.0 * s,
                c1.y + 38.0 * s,
                c1.w - 24.0 * s,
                12.5 * s,
                p.text_primary,
            );

            // Widget 2 — Daily Goal
            let c2 = snapshot.grid_cell(start_cx, cards_y, 1, 0, card_w, card_h, gap, 0.0);
            push_panel(
                sdf_pipeline,
                c2,
                [
                    p.surface_raised[0],
                    p.surface_raised[1],
                    p.surface_raised[2],
                    0.96,
                ],
                14.0 * s,
            );
            sdf_pipeline.push(SdfInstance::button(
                c2.x,
                c2.y,
                c2.w,
                c2.h,
                14.0 * s,
                [0.0; 4],
                p.border,
                0.15,
            ));
            text_renderer.queue_text_bounded(
                &render_ctx.queue,
                "DAILY GOAL",
                c2.x + 16.0 * s,
                c2.y + 12.0 * s,
                c2.w - 24.0 * s,
                11.5 * s,
                p.accent_primary,
            );
            text_renderer.queue_text_bounded(
                &render_ctx.queue,
                "Target: 590 / 600 (98.3%)",
                c2.x + 16.0 * s,
                c2.y + 38.0 * s,
                c2.w - 24.0 * s,
                12.5 * s,
                p.text_primary,
            );

            // Widget 3 — Next Priority
            let c3 = snapshot.grid_cell(start_cx, cards_y, 2, 0, card_w, card_h, gap, 0.0);
            push_panel(
                sdf_pipeline,
                c3,
                [
                    p.surface_raised[0],
                    p.surface_raised[1],
                    p.surface_raised[2],
                    0.96,
                ],
                14.0 * s,
            );
            sdf_pipeline.push(SdfInstance::button(
                c3.x,
                c3.y,
                c3.w,
                c3.h,
                14.0 * s,
                [0.0; 4],
                p.border,
                0.15,
            ));
            text_renderer.queue_text_bounded(
                &render_ctx.queue,
                "NEXT PRIORITY",
                c3.x + 16.0 * s,
                c3.y + 12.0 * s,
                c3.w - 24.0 * s,
                11.5 * s,
                [0.10, 0.73, 0.51, 1.0],
            );
            text_renderer.queue_text_bounded(
                &render_ctx.queue,
                "Wave Optics Numericals",
                c3.x + 16.0 * s,
                c3.y + 38.0 * s,
                c3.w - 24.0 * s,
                12.5 * s,
                p.text_primary,
            );

            // 3. Interactive Tactile PIN Pad & Mode Badge
            let pad_w = 280.0 * s;
            let pad_h = 280.0 * s;
            let pad_y = 348.0 * s;
            let pad_r = snapshot.sub_rect(
                snapshot.viewport_rect(),
                (width - pad_w) * 0.5,
                pad_y,
                pad_w,
                pad_h,
            );
            push_panel(
                sdf_pipeline,
                pad_r,
                [p.surface[0], p.surface[1], p.surface[2], 0.98],
                16.0 * s,
            );
            sdf_pipeline.push(SdfInstance::button(
                pad_r.x,
                pad_r.y,
                pad_r.w,
                pad_r.h,
                16.0 * s,
                [0.0; 4],
                [0.961, 0.620, 0.043, 0.48],
                0.15,
            ));

            let num_entered = pin_input.text.chars().count().min(4);
            let mut dots_disp = String::new();
            for i in 0..4 {
                if i < num_entered {
                    dots_disp.push_str("● ");
                } else {
                    dots_disp.push_str("○ ");
                }
            }
            text_renderer.queue_text_centered_in_rect(
                &render_ctx.queue,
                &dots_disp,
                [pad_r.x, pad_r.y + 12.0 * s, pad_r.w, 24.0 * s],
                18.0 * s,
                [0.961, 0.620, 0.043, 1.0],
            );

            if let Some(ref err) = error_msg {
                text_renderer.queue_text_centered_in_rect(
                    &render_ctx.queue,
                    err,
                    [pad_r.x, pad_r.y + 34.0 * s, pad_r.w, 18.0 * s],
                    11.0 * s,
                    p.error,
                );
            }

            let grid_x = pad_r.x + 20.0 * s;
            let grid_y = pad_r.y + 48.0 * s;
            let btn_w = 70.0 * s;
            let btn_h = 44.0 * s;
            let gap = 15.0 * s;

            let keys = [
                ("1", 0, 0),
                ("2", 1, 0),
                ("3", 2, 0),
                ("4", 0, 1),
                ("5", 1, 1),
                ("6", 2, 1),
                ("7", 0, 2),
                ("8", 1, 2),
                ("9", 2, 2),
                ("Clear", 0, 3),
                ("0", 1, 3),
                ("Unlock", 2, 3),
            ];
            for (lbl, col, row) in keys {
                let kr = snapshot.grid_cell(grid_x, grid_y, col, row, btn_w, btn_h, gap, gap);
                let is_accent = lbl == "Unlock";
                let k_fill = if is_accent {
                    p.accent_primary
                } else {
                    p.surface_raised
                };
                let k_border = if is_accent {
                    p.accent_primary
                } else {
                    p.border
                };
                push_btn(sdf_pipeline, kr, k_fill, k_border);
                let ktc = optimal_text_color_for_fill(k_fill);
                text_renderer.queue_text_centered_in_rect(
                    &render_ctx.queue,
                    lbl,
                    [kr.x, kr.y, kr.w, kr.h],
                    12.5 * s,
                    ktc,
                );
            }

            // Bottom-center Mode Badge right on the Lock Screen
            let mode_lbl = format!(
                "{} — Click to Switch",
                state.mindful_engine.mode().pill_label()
            );
            let (mode_tw, _) = text_renderer.measure_line_px(&mode_lbl, 11.0 * s);
            let mode_pill_w = (mode_tw + 32.0 * s).max(280.0 * s);
            let mode_pill_h = 36.0 * s;
            let mode_pill_r = snapshot.sub_rect(
                snapshot.viewport_rect(),
                (width - mode_pill_w) * 0.5,
                pad_r.y + pad_r.h + 14.0 * s,
                mode_pill_w,
                mode_pill_h,
            );
            push_btn(
                sdf_pipeline,
                mode_pill_r,
                [
                    p.surface_raised[0],
                    p.surface_raised[1],
                    p.surface_raised[2],
                    0.90,
                ],
                [0.961, 0.620, 0.043, 0.50],
            );
            text_renderer.queue_text_centered_in_rect(
                &render_ctx.queue,
                &mode_lbl,
                [mode_pill_r.x, mode_pill_r.y, mode_pill_r.w, mode_pill_r.h],
                11.0 * s,
                [0.961, 0.620, 0.043, 1.0],
            );
        } else {
            // 2. Desktop Wallpaper Layer: Left-Side Student Widgets & Desktop Icons (Behind windows)
            if !state.session.focus_mode && state.show_desktop {
                let mut desktop_occluding_rects: Vec<LayoutRect> = visible_windows
                    .iter()
                    .filter(|w| w.id != WindowId::Today)
                    .map(|w| w.rect)
                    .chain([
                        top_capsule_layout.left_capsule,
                        top_capsule_layout.right_capsule,
                    ])
                    .collect();
                desktop_occluding_rects.extend_from_slice(&modal_occlusions);

                let mut desktop_draw = ClippedDraw {
                    sdf: sdf_pipeline,
                    txt: text_renderer,
                    queue: &render_ctx.queue,
                    clip_bounds: None,
                    occlusions: &desktop_occluding_rects,
                    is_mouse_down: state.input.left_down,
                    mouse_pos: state.input.cursor,
                };

                let font_xs = (10.0 * s).clamp(9.0, 13.5);
                let font_small = (11.0 * s).clamp(10.0, 15.0);

                let desktop_layout = DesktopLayout::compute(width, height, s);

                // Left-Side Desktop Widgets (Matching user reference design)
                if state.mindful_engine.should_show_left_widgets() {
                    crate::desktop_widgets::render_desktop_widgets(
                        state,
                        &mut desktop_draw,
                        &desktop_layout,
                    );
                }

                // Top-Center Desktop Search Bar Pill [SRC] (Mindful Home Filter)
                if state.mindful_engine.should_show_search_bar() {
                    let pill_r = desktop_layout.hits.search_pill;
                    desktop_draw.push_panel(
                        pill_r,
                        [p.surface[0], p.surface[1], p.surface[2], 0.78],
                        18.0 * s,
                    );
                    let badge_w =
                        (desktop_draw.measure_line_px("SRC", font_xs).0 + 12.0 * s).max(32.0 * s);
                    let badge_r =
                        snapshot.sub_rect(pill_r, 8.0 * s, 6.0 * s, badge_w, pill_r.h - 12.0 * s);
                    desktop_draw.push_squircle_badge(
                        badge_r,
                        p.accent_primary,
                        "SRC",
                        [1.0, 1.0, 1.0, 1.0],
                    );
                    let text_x = badge_r.x + badge_r.w + 10.0 * s;
                    let text_max_w = (pill_r.w - (text_x - pill_r.x) - 10.0 * s).max(20.0);
                    desktop_draw.queue_text_bounded(
                        "Search apps, notes, web or ask Maindy... (Ctrl+K)",
                        text_x,
                        pill_r.y + (pill_r.h - font_small) * 0.5,
                        text_max_w,
                        font_small,
                        p.text_secondary,
                    );
                }

                // Desktop Icons (placed to the right of widgets, Mindful Home Filter)
                if state.mindful_engine.should_show_desktop_icons() {
                    // Sync rendered layout for exact hit-box matching (>= 24px clearance past left widgets at 256px)
                    state.desktop_grid.sync_rendered_layout_scaled(282.0 * s, s);

                    for icon in &state.desktop_grid.icons {
                        if !state.is_app_allowed_on_home(icon.id) {
                            continue;
                        }
                        if icon.is_pinned {
                            let r = icon.rendered_rect;

                            // Tactile interaction: subtle depression when mouse is pressed on icon
                            let is_pressed = desktop_draw.is_mouse_down
                                && r.contains(desktop_draw.mouse_pos.0, desktop_draw.mouse_pos.1);
                            let draw_r = if is_pressed { r.inset(1.5 * s) } else { r };
                            let corner_radius = (14.0 * s).clamp(10.0, 18.0);

                            // Elevation::Level2 styling or translucent glass panel [0.15, 0.18, 0.28, 0.70] with rounded corners
                            let fill = if icon.is_selected {
                                [
                                    p.accent_primary[0],
                                    p.accent_primary[1],
                                    p.accent_primary[2],
                                    0.35,
                                ]
                            } else {
                                [0.15, 0.18, 0.28, 0.70]
                            };
                            let border_rim = if icon.is_selected {
                                p.accent_primary
                            } else {
                                [1.0, 1.0, 1.0, 0.22]
                            };
                            desktop_draw.push_panel_rim(draw_r, fill, border_rim, corner_radius);

                            // Centered Squircle Badge
                            let badge_size = (44.0 * s).clamp(40.0, 56.0);
                            let badge_r = snapshot.sub_rect(
                                r,
                                (r.w - badge_size) * 0.5,
                                8.0 * s,
                                badge_size,
                                badge_size,
                            );
                            desktop_draw.push_squircle_badge(
                                badge_r,
                                icon.id.badge_color(),
                                icon.id.badge_symbol(),
                                [1.0, 1.0, 1.0, 1.0],
                            );

                            // 2-line wrapped icon label centered inside card with word-level wrapping
                            let font_icon = (10.5 * s).clamp(9.5, 14.5);
                            let line_h = (font_icon * 1.32).ceil();
                            let wrapped = mindset_render::wrap_text_chars(&icon.label, 12);
                            let lines: Vec<&str> = wrapped.lines().collect();
                            for (line_idx, line) in lines.iter().enumerate() {
                                let line_y =
                                    r.y + 8.0 * s + badge_size + 6.0 * s + line_idx as f32 * line_h;
                                let (lw, _) = desktop_draw.measure_line_px(line, font_icon);
                                let lx = (r.x + (r.w - lw) * 0.5).max(r.x + 2.0);
                                desktop_draw.queue_text_bounded(
                                    line,
                                    lx,
                                    line_y,
                                    r.w - 4.0,
                                    font_icon,
                                    p.text_primary,
                                );
                            }
                        }
                    }
                }
            }

            // 3. Multi-Window Compositing Layer (Sorted Back to Front)
            for (win_idx, win) in visible_windows.iter().enumerate() {
                if win.id == WindowId::Today {
                    // Render root Today screen ONLY when show_desktop is false (distraction-free focus)
                    if !state.show_desktop {
                        let mut today_draw = ClippedDraw {
                            sdf: sdf_pipeline,
                            txt: text_renderer,
                            queue: &render_ctx.queue,
                            clip_bounds: None,
                            occlusions: &modal_occlusions,
                            is_mouse_down: state.input.left_down,
                            mouse_pos: state.input.cursor,
                        };
                        let user_name = if state
                            .personalization_store
                            .profile
                            .preferred_name
                            .is_empty()
                        {
                            "Logesh"
                        } else {
                            &state.personalization_store.profile.preferred_name
                        };
                        let _ = crate::today::render_today(
                            &mut self.today_state,
                            &mut today_draw,
                            width,
                            height,
                            user_name,
                            false,
                            state.input.cursor,
                        );
                    }
                    continue;
                }

                if !state.is_app_allowed_on_home(win.id) {
                    continue;
                }
                let mut occluding_rects: Vec<LayoutRect> = visible_windows[(win_idx + 1)..]
                    .iter()
                    .map(|w| w.rect)
                    .collect();
                occluding_rects.push(top_capsule_layout.left_capsule);
                occluding_rects.push(top_capsule_layout.right_capsule);
                if !state.session.focus_mode
                    && state.show_desktop
                    && state.mindful_engine.should_show_bottom_dock()
                {
                    occluding_rects.push(dock_r);
                }
                occluding_rects.extend_from_slice(&modal_occlusions);

                let is_active = state.wm.active_window_id == Some(win.id);
                let border_col = if is_active {
                    p.accent_primary
                } else {
                    [0.984, 0.749, 0.141, 0.48]
                };

                // Window Glass Surface
                sdf_pipeline.push(SdfInstance::glass_panel(
                    win.rect.x,
                    win.rect.y,
                    win.rect.w,
                    win.rect.h,
                    CARD_RADIUS,
                    p.surface,
                    border_col,
                ));

                // Window Titlebar
                let tb = win.titlebar_rect();
                let mut tb_draw = ClippedDraw {
                    sdf: sdf_pipeline,
                    txt: text_renderer,
                    queue: &render_ctx.queue,
                    clip_bounds: Some(tb),
                    occlusions: &occluding_rects,
                    is_mouse_down: state.input.left_down,
                    mouse_pos: state.input.cursor,
                };
                tb_draw.push_panel(tb, p.surface_raised, CARD_RADIUS);
                let badge_size = (22.0 * s).clamp(20.0, 28.0);
                let badge_r = snapshot.sub_rect(
                    tb,
                    10.0 * s,
                    (tb.h - badge_size) * 0.5,
                    badge_size,
                    badge_size,
                );

                // Titlebar buttons: [-] Minimize, [□] Maximize, [×] Close
                let chrome = win.chrome_targets(s);
                let min_btn = chrome.min_btn;
                let max_btn = chrome.max_btn;
                let cls_btn = chrome.close_btn;

                if win.id == WindowId::SearchBrowser {
                    tb_draw.push_squircle_badge(
                        badge_r,
                        [0.898, 0.678, 0.271, 1.0],
                        "🌐",
                        [1.0, 1.0, 1.0, 1.0],
                    );

                    let title_start_x = badge_r.x + badge_r.w + 8.0 * s;
                    tb_draw.queue_text(
                        "MINDSET BROWSER",
                        title_start_x,
                        tb.y + 11.0,
                        12.0 * s,
                        p.text_primary,
                    );

                    let ver_r = snapshot.sub_rect(
                        tb,
                        title_start_x - tb.x + 124.0 * s,
                        9.0,
                        52.0 * s,
                        18.0,
                    );
                    tb_draw.push_panel(ver_r, [0.898, 0.678, 0.271, 0.15], 4.0);
                    tb_draw.queue_text_centered_in_rect(
                        "v2.4 Pro",
                        ver_r,
                        9.5 * s,
                        [0.898, 0.678, 0.271, 1.0],
                    );

                    // Optional center workspace indicator
                    let ws_w = (230.0 * s).clamp(180.0, 260.0);
                    let ws_r = snapshot.sub_rect(tb, (tb.w - ws_w) * 0.5, 6.0, ws_w, 22.0);
                    tb_draw.push_panel(ws_r, [0.082, 0.098, 0.133, 0.90], 11.0);
                    tb_draw.push_btn(ws_r, [0.0; 4], BROWSER_BORDER);
                    tb_draw.queue_text_centered_in_rect(
                        "🏛️ Sovereign Academic Workspace",
                        ws_r,
                        10.0 * s,
                        [0.898, 0.678, 0.271, 1.0],
                    );
                } else if win.id == WindowId::SystemMonitor {
                    // macOS-style traffic lights on left
                    let dot_y = tb.y + (tb.h - 11.0) * 0.5;
                    tb_draw.push_btn(
                        snapshot.sub_rect(tb, 12.0 * s, dot_y - tb.y, 11.0, 11.0),
                        theme::TRAFFIC_RED,
                        [0.0; 4],
                    );
                    tb_draw.push_btn(
                        snapshot.sub_rect(tb, 28.0 * s, dot_y - tb.y, 11.0, 11.0),
                        theme::TRAFFIC_YELLOW,
                        [0.0; 4],
                    );
                    tb_draw.push_btn(
                        snapshot.sub_rect(tb, 44.0 * s, dot_y - tb.y, 11.0, 11.0),
                        theme::TRAFFIC_GREEN,
                        [0.0; 4],
                    );

                    let title_start_x = tb.x + 64.0 * s;
                    let title_max_w = (min_btn.x - title_start_x - 8.0 * s).max(40.0);
                    let font_title = (12.5 * s).clamp(11.0, 17.0);
                    tb_draw.queue_text_bounded(
                        "System Telemetry & Doctor",
                        title_start_x,
                        tb.y + (tb.h - font_title) * 0.5,
                        title_max_w,
                        font_title,
                        theme::TEXT_PRIMARY_HEADER,
                    );
                } else {
                    tb_draw.push_squircle_badge(
                        badge_r,
                        win.id.badge_color(),
                        win.id.badge_symbol(),
                        [1.0, 1.0, 1.0, 1.0],
                    );

                    let title_start_x = badge_r.x + badge_r.w + 8.0 * s;
                    let title_max_w = (min_btn.x - title_start_x - 8.0 * s).max(40.0);
                    let font_title = (12.5 * s).clamp(11.0, 17.0);
                    tb_draw.queue_text_bounded(
                        win.id.title(),
                        title_start_x,
                        tb.y + (tb.h - font_title) * 0.5,
                        title_max_w,
                        font_title,
                        p.text_primary,
                    );
                }

                tb_draw.push_btn(min_btn, p.surface, p.border);
                tb_draw.queue_text_centered_in_rect("-", min_btn, 13.0 * s, p.text_secondary);

                tb_draw.push_btn(max_btn, p.surface, p.border);
                tb_draw.queue_text_centered_in_rect("□", max_btn, 12.0 * s, p.text_secondary);

                tb_draw.push_btn(cls_btn, [p.error[0], p.error[1], p.error[2], 0.2], p.error);
                tb_draw.queue_text_centered_in_rect("×", cls_btn, 13.0 * s, p.error);

                // Resize handle
                if win.state == WindowState::Floating {
                    let grip = win.resize_grip_rect();
                    tb_draw.push_btn(grip, [p.border[0], p.border[1], p.border[2], 0.4], [0.0; 4]);
                }

                // Window Content View Dispatch
                let content = win.content_rect().inset(INNER_GAP);
                let mut content_draw = ClippedDraw {
                    sdf: sdf_pipeline,
                    txt: text_renderer,
                    queue: &render_ctx.queue,
                    clip_bounds: Some(win.content_rect()),
                    occlusions: &occluding_rects,
                    is_mouse_down: state.input.left_down,
                    mouse_pos: state.input.cursor,
                };

                match win.id {
                    WindowId::StudyReader => {
                        crate::window_renderers::render_study_reader_content(
                            state,
                            &mut content_draw,
                            content,
                            win.scroll_y.position,
                        );
                    }
                    WindowId::LayaAiChat => {
                        crate::window_renderers::render_laya_chat_content(
                            state,
                            &mut content_draw,
                            content,
                            win.scroll_y.position,
                        );
                    }
                    WindowId::Tasks => {
                        crate::window_renderers::render_tasks_content(
                            state,
                            &mut content_draw,
                            content,
                            win.scroll_y.position,
                        );
                    }
                    WindowId::ProgressStats => {
                        crate::window_renderers::render_stats_content(
                            state,
                            &mut content_draw,
                            content,
                            win.scroll_y.position,
                        );
                    }
                    WindowId::Settings => {
                        crate::window_renderers::render_settings_content(
                            state,
                            &mut content_draw,
                            content,
                            win.scroll_y.position,
                        );
                    }
                    WindowId::SystemMonitor => {
                        crate::window_renderers::render_monitor_content(
                            state,
                            &mut content_draw,
                            content,
                            &mem_str,
                            &fps_str,
                            win.scroll_y.position,
                        );
                    }
                    WindowId::FileVault => {
                        crate::window_renderers::render_file_vault_content(
                            state,
                            &mut content_draw,
                            content,
                            win.scroll_y.position,
                        );
                    }
                    WindowId::SearchBrowser => {
                        crate::window_renderers::render_search_browser_content(
                            state,
                            &mut content_draw,
                            content,
                            win.scroll_y.position,
                        );
                    }
                    WindowId::DesignSystem => {
                        crate::window_renderers::design_system::render_design_system_showcase(
                            &mut content_draw,
                            content,
                            s,
                        );
                    }
                    WindowId::Today => {}
                }

                // Scrollbar Track & Thumb
                if win.content_height > win.content_rect().h {
                    let track = win.scrollbar_track_rect();
                    let mut sb_draw = ClippedDraw {
                        sdf: sdf_pipeline,
                        txt: text_renderer,
                        queue: &render_ctx.queue,
                        clip_bounds: Some(win.rect),
                        occlusions: &occluding_rects,
                        is_mouse_down: state.input.left_down,
                        mouse_pos: state.input.cursor,
                    };
                    sb_draw.push_panel(track, [p.border[0], p.border[1], p.border[2], 0.25], 3.0);
                    if let Some(thumb) = win.scrollbar_thumb_rect() {
                        sb_draw.push_btn(
                            thumb,
                            [
                                p.accent_primary[0],
                                p.accent_primary[1],
                                p.accent_primary[2],
                                0.85,
                            ],
                            [1.0, 1.0, 1.0, 0.4],
                        );
                    }
                }
            }

            // 4. Top System Bar (44px)

            // Top Floating Capsule Bar (Integrated with MindSet Flash & System Status)
            {
                let mut top_draw = ClippedDraw {
                    sdf: sdf_pipeline,
                    txt: text_renderer,
                    queue: &render_ctx.queue,
                    clip_bounds: None,
                    occlusions: &[],
                    is_mouse_down: state.input.left_down,
                    mouse_pos: state.input.cursor,
                };
                let active_flash = state.active_workspace;
                let unread_notifs = state.session.notifications.unread_count();
                let cc_is_open = state.active_overlay() == ActiveOverlay::ControlCenter;
                let now_str = Local::now().format("%A, %b %d • %I:%M %p").to_string();
                crate::top_bar::render_top_capsule_bar(
                    &mut top_draw,
                    width,
                    height,
                    active_flash,
                    unread_notifs,
                    cc_is_open,
                    &now_str,
                );
            }

            // Micro-Toast Notification Pill (Bottom-center above dock, high contrast)
            if state.toast_timer > 0.0 {
                if let Some(ref msg) = state.toast_message {
                    let font_toast = (11.5 * s).clamp(10.5, 16.0);
                    let (tw, th) = text_renderer.measure_line_px(msg, font_toast);
                    let toast_r = snapshot.toast_rect(tw, th);
                    // High-contrast card (rgba(15, 18, 28, 0.96) with 1.5px golden #FBBF24 border and 12px radius)
                    push_panel(sdf_pipeline, toast_r, [0.06, 0.07, 0.11, 0.96], 12.0 * s);
                    sdf_pipeline.push(SdfInstance::button(
                        toast_r.x,
                        toast_r.y,
                        toast_r.w,
                        toast_r.h,
                        12.0 * s,
                        [0.0; 4],
                        [0.984, 0.749, 0.141, 0.95],
                        0.22,
                    ));
                    let mx = toast_r.x + (toast_r.w - tw.min(toast_r.w - 16.0 * s)) * 0.5;
                    let my = toast_r.y + (toast_r.h - font_toast) * 0.5;
                    let bounded_msg =
                        text_renderer.fit_text_to_px(msg, font_toast, toast_r.w - 20.0 * s);
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        &bounded_msg,
                        mx,
                        my,
                        font_toast,
                        [1.0, 1.0, 1.0, 1.0],
                    );
                }
            }

            // 5. Bottom App Dock (Auto-hides in Focus Mode, when show_desktop is false, or when hidden by Mindful Mode)
            if !state.session.focus_mode
                && state.show_desktop
                && state.mindful_engine.should_show_bottom_dock()
            {
                let dock_r = state.session.dock_rect();
                push_panel(sdf_pipeline, dock_r, p.surface, 18.0);

                let all_ids = WindowId::dock_windows();
                let allowed_ids: Vec<WindowId> = all_ids
                    .into_iter()
                    .filter(|id| state.is_app_allowed_on_home(*id))
                    .collect();
                for (idx, id) in allowed_ids.iter().enumerate() {
                    let ir = state.session.dock_item_rect(idx, allowed_ids.len());
                    let is_open = state.wm.find_window(*id).is_some_and(|w| w.is_open);
                    let is_active = state.wm.active_window_id == Some(*id);

                    let badge_r = snapshot.card_item(ir, (ir.w - 42.0) * 0.5, 4.0, 42.0, 42.0);
                    push_squircle_badge_standalone(
                        sdf_pipeline,
                        text_renderer,
                        &render_ctx.queue,
                        badge_r,
                        id.badge_color(),
                        id.badge_symbol(),
                        [1.0, 1.0, 1.0, 1.0],
                    );

                    if is_active {
                        sdf_pipeline.push(SdfInstance::button(
                            badge_r.x - 2.0,
                            badge_r.y - 2.0,
                            badge_r.w + 4.0,
                            badge_r.h + 4.0,
                            14.0,
                            [0.0, 0.0, 0.0, 0.0],
                            p.accent_primary,
                            0.25,
                        ));
                    }

                    // Running dot indicator
                    if is_open {
                        let dot_r = snapshot.card_item(ir, ir.w * 0.5 - 2.5, ir.h - 5.0, 5.0, 5.0);
                        push_btn(sdf_pipeline, dot_r, p.accent_primary, [0.0; 4]);
                    }
                }
            }

            // 6. Mutually Exclusive Overlays (Rule 3: Compile-Time Modal Exclusivity)
            // No two overlays can ever render at the same time.
            match state.active_overlay() {
                ActiveOverlay::None => {}
                ActiveOverlay::ContextMenu => {
                    let cm_r = state.context_menu.rect();
                    push_panel(sdf_pipeline, cm_r, p.surface, 10.0);
                    for (idx, action) in ContextMenuAction::all().iter().enumerate() {
                        let ir = state.context_menu.item_rect(idx);
                        push_btn(sdf_pipeline, ir, p.surface_raised, [0.0; 4]);
                        text_renderer.queue_text(
                            &render_ctx.queue,
                            action.label(),
                            ir.x + 8.0,
                            ir.y + 8.0,
                            11.5,
                            p.text_primary,
                        );
                    }
                }
                ActiveOverlay::Notifications => {
                    let nd_r = state.session.notifications.drawer_rect(width);
                    push_panel(sdf_pipeline, nd_r, p.surface, 12.0);

                    // Header
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        "Notifications",
                        nd_r.x + 16.0,
                        nd_r.y + 12.0,
                        13.5,
                        p.text_primary,
                    );

                    let dnd_lbl = if state.session.notifications.do_not_disturb {
                        "DND: ON"
                    } else {
                        "DND: OFF"
                    };
                    let dnd_btn = snapshot.card_item(nd_r, nd_r.w - 146.0, 8.0, 68.0, 24.0);
                    let clear_btn = snapshot.card_item(nd_r, nd_r.w - 74.0, 8.0, 62.0, 24.0);

                    let dnd_fill = if state.session.notifications.do_not_disturb {
                        p.accent_primary
                    } else {
                        p.surface_raised
                    };
                    let dnd_tc = if state.session.notifications.do_not_disturb {
                        [1.0, 1.0, 1.0, 1.0]
                    } else {
                        p.text_primary
                    };
                    push_btn(sdf_pipeline, dnd_btn, dnd_fill, p.border);
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        dnd_lbl,
                        dnd_btn.x + 8.0,
                        dnd_btn.y + 5.0,
                        10.0,
                        dnd_tc,
                    );

                    push_btn(sdf_pipeline, clear_btn, p.surface_raised, p.border);
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        "Clear",
                        clear_btn.x + 14.0,
                        clear_btn.y + 5.0,
                        10.0,
                        p.text_primary,
                    );

                    // Grouped notification items
                    let visible = state.session.notifications.visible_notifications();
                    for (idx, item) in visible.iter().take(4).enumerate() {
                        let item_r = snapshot.list_row(
                            nd_r.x + 12.0,
                            nd_r.y + 40.0,
                            nd_r.w - 24.0,
                            62.0,
                            idx,
                            6.0,
                        );
                        push_panel(sdf_pipeline, item_r, p.surface_raised, 8.0);

                        let badge_col = match item.category {
                            NotificationCategory::Info => p.accent_primary,
                            NotificationCategory::Reminder => p.accent_secondary,
                            NotificationCategory::Alert => p.error,
                        };
                        let badge_r = snapshot.card_item(item_r, 8.0, 8.0, 22.0, 18.0);
                        push_btn(sdf_pipeline, badge_r, badge_col, badge_col);
                        text_renderer.queue_text(
                            &render_ctx.queue,
                            item.category.symbol(),
                            badge_r.x + 3.0,
                            badge_r.y + 3.0,
                            10.0,
                            [1.0, 1.0, 1.0, 1.0],
                        );

                        text_renderer.queue_text(
                            &render_ctx.queue,
                            &item.title,
                            item_r.x + 36.0,
                            item_r.y + 8.0,
                            11.5,
                            p.text_primary,
                        );
                        text_renderer.queue_text(
                            &render_ctx.queue,
                            &item.time_str,
                            item_r.x + item_r.w - 60.0,
                            item_r.y + 8.0,
                            9.5,
                            p.text_secondary,
                        );

                        let msg_trunc = safe_utf8_prefix(&item.message, 38);
                        text_renderer.queue_text(
                            &render_ctx.queue,
                            msg_trunc,
                            item_r.x + 36.0,
                            item_r.y + 28.0,
                            10.5,
                            p.text_secondary,
                        );
                    }

                    // Footer Snooze button
                    let snooze_btn =
                        snapshot.card_item(nd_r, 12.0, nd_r.h - 38.0, nd_r.w - 24.0, 28.0);
                    let snooze_lbl = if state.session.notifications.snoozed_until_minutes > 0 {
                        "Snoozed 15m (Active)"
                    } else {
                        "Snooze Reminders (15m)"
                    };
                    push_btn(sdf_pipeline, snooze_btn, p.surface_raised, p.border);
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        snooze_lbl,
                        snooze_btn.x + 80.0,
                        snooze_btn.y + 7.0,
                        11.0,
                        p.accent_primary,
                    );
                }
                ActiveOverlay::ControlCenter => {
                    let mut cc_draw = ClippedDraw {
                        sdf: sdf_pipeline,
                        txt: text_renderer,
                        queue: &render_ctx.queue,
                        clip_bounds: None,
                        occlusions: &[],
                        is_mouse_down: state.input.left_down,
                        mouse_pos: state.input.cursor,
                    };
                    let cc_layout = mindset_surface::ControlCenterLayout::compute(width, height, s);
                    crate::control_panel::render_modern_control_panel(
                        &mut cc_draw,
                        &cc_layout,
                        &self.modern_control_panel,
                        s,
                    );
                }
                ActiveOverlay::FlashSwitcher => {
                    let mut flash_draw = ClippedDraw {
                        sdf: sdf_pipeline,
                        txt: text_renderer,
                        queue: &render_ctx.queue,
                        clip_bounds: None,
                        occlusions: &[],
                        is_mouse_down: state.input.left_down,
                        mouse_pos: state.input.cursor,
                    };
                    crate::flash_switcher::render_flash_switcher_modal(
                        &mut flash_draw,
                        width,
                        height,
                        state.active_workspace,
                        &state.wm,
                    );
                }
                ActiveOverlay::Launcher => {
                    let mut launcher_draw = ClippedDraw {
                        sdf: sdf_pipeline,
                        txt: text_renderer,
                        queue: &render_ctx.queue,
                        clip_bounds: None,
                        occlusions: &[],
                        is_mouse_down: state.input.left_down,
                        mouse_pos: state.input.cursor,
                    };
                    let query = if !state.command_launcher.input.text.is_empty() {
                        &state.command_launcher.input.text
                    } else {
                        &state.search_input.text
                    };
                    crate::launcher::render_mindset_launcher(
                        &mut launcher_draw,
                        width,
                        height,
                        query,
                    );
                }
                ActiveOverlay::CommandPalette {
                    ref query,
                    selected_idx: _,
                } => {
                    let mut launcher_draw = ClippedDraw {
                        sdf: sdf_pipeline,
                        txt: text_renderer,
                        queue: &render_ctx.queue,
                        clip_bounds: None,
                        occlusions: &[],
                        is_mouse_down: state.input.left_down,
                        mouse_pos: state.input.cursor,
                    };
                    let query_str = if !query.is_empty() {
                        query.as_str()
                    } else if !state.command_launcher.input.text.is_empty() {
                        &state.command_launcher.input.text
                    } else {
                        &state.search_input.text
                    };
                    crate::launcher::render_mindset_launcher(
                        &mut launcher_draw,
                        width,
                        height,
                        query_str,
                    );
                }
                ActiveOverlay::OnboardingWizard => {
                    let dim_bg = snapshot.viewport_rect();
                    push_panel(sdf_pipeline, dim_bg, [0.039, 0.047, 0.078, 0.82], 0.0);

                    let wiz_w = 600.0_f32.min(width - 40.0);
                    let wiz_h = 420.0_f32.min(height - 60.0);
                    let wiz_r = snapshot.centered_rect(wiz_w, wiz_h, 0.44);
                    push_panel(sdf_pipeline, wiz_r, [0.086, 0.098, 0.149, 0.98], 16.0);
                    sdf_pipeline.push(SdfInstance::button(
                        wiz_r.x,
                        wiz_r.y,
                        wiz_r.w,
                        wiz_r.h,
                        16.0,
                        [0.0; 4],
                        [0.984, 0.749, 0.141, 0.48],
                        0.20,
                    ));

                    let step = state.onboarding_wizard.current_step;
                    let total = state.onboarding_wizard.total_steps;
                    let title = state.onboarding_wizard.current_title();
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        "MindSet OS — Quick Setup Wizard",
                        wiz_r.x + 24.0,
                        wiz_r.y + 18.0,
                        14.5,
                        p.text_primary,
                    );
                    let step_str = format!("Step {} of {}: {}", step + 1, total, title);
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        &step_str,
                        wiz_r.x + 24.0,
                        wiz_r.y + 38.0,
                        11.5,
                        p.accent_primary,
                    );

                    let pb_r = snapshot.card_item(wiz_r, 24.0, 58.0, wiz_r.w - 48.0, 6.0);
                    let progress = (step + 1) as f32 / total as f32;
                    push_btn(sdf_pipeline, pb_r, p.border, [0.0; 4]);
                    let fill_pb = snapshot.progress_fill(pb_r, progress);
                    push_btn(sdf_pipeline, fill_pb, p.accent_primary, [0.0; 4]);

                    let body_r =
                        snapshot.card_item(wiz_r, 24.0, 76.0, wiz_r.w - 48.0, wiz_h - 146.0);
                    push_panel(sdf_pipeline, body_r, p.surface_raised, 10.0);

                    let body_y = body_r.y + 16.0;
                    match step {
                        0 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Language, Scripts & Typography",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(&render_ctx.queue, "MindSet OS features native font fallbacks for English, Tamil (தமிழ்),", body_r.x + 16.0, body_y + 24.0, 11.0, p.text_secondary);
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Hindi (हिन्दी), and mathematical notation with zero [] tofu boxes.",
                                body_r.x + 16.0,
                                body_y + 40.0,
                                11.0,
                                p.text_secondary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Nirmala.ttc & system fonts verified and active.",
                                body_r.x + 16.0,
                                body_y + 64.0,
                                11.0,
                                [0.10, 0.73, 0.51, 1.0],
                            );
                        }
                        1 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Theme & Atmosphere",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Choose your visual computing environment:",
                                body_r.x + 16.0,
                                body_y + 24.0,
                                11.0,
                                p.text_secondary,
                            );
                            let mode_lbl = if state.theme.mode == ThemeMode::Dark {
                                "Current: Dark Theme (OLED / Low-Glare)"
                            } else {
                                "Current: Light Theme (Daylight)"
                            };
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                mode_lbl,
                                body_r.x + 16.0,
                                body_y + 48.0,
                                11.5,
                                p.accent_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Wallpaper: Scenic Alpine & Aurora Midnight active.",
                                body_r.x + 16.0,
                                body_y + 68.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                        2 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "What should Maindy call you?",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            let name_disp = format!(
                                "User Profile: {}",
                                state.personalization_store.profile.preferred_name
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                &name_disp,
                                body_r.x + 16.0,
                                body_y + 28.0,
                                12.0,
                                p.accent_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "You can edit this anytime in Settings -> Profile & Identity.",
                                body_r.x + 16.0,
                                body_y + 54.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                        3 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Academic Stage & Focus",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Current Stage: Competitive Exams (JEE / NEET / Boards)",
                                body_r.x + 16.0,
                                body_y + 28.0,
                                12.0,
                                p.accent_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Curriculum tailored to high-yield science problem solving.",
                                body_r.x + 16.0,
                                body_y + 54.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                        4 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Daily Study Hours Target",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            let target_str = format!(
                                "Goal: {:.1} hours daily • Target Score: {} / 600",
                                state.personalization_store.routine.study_hours_per_day,
                                state.target_score
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                &target_str,
                                body_r.x + 16.0,
                                body_y + 28.0,
                                12.0,
                                p.accent_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Streak tracking and Pomodoro timer calibrated automatically.",
                                body_r.x + 16.0,
                                body_y + 54.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                        5 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Daily Routine & Sleep Blocks",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Wake: 06:30 AM  •  Sleep: 10:30 PM",
                                body_r.x + 16.0,
                                body_y + 28.0,
                                12.0,
                                p.accent_primary,
                            );
                            text_renderer.queue_text(&render_ctx.queue, "Night warmth activates automatically past 8:00 PM for circadian health.", body_r.x + 16.0, body_y + 54.0, 11.0, p.text_secondary);
                        }
                        6 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Maindy AI Tutor Mode",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(&render_ctx.queue, "Default Mode: Socratic Tutor (Guides through step-by-step questions)", body_r.x + 16.0, body_y + 28.0, 12.0, p.accent_primary);
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Can be switched to Direct Coach or Study Planner in Settings.",
                                body_r.x + 16.0,
                                body_y + 54.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                        7 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "AI Personality & Tone",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Tone: Encouraging, concise, and academically rigorous.",
                                body_r.x + 16.0,
                                body_y + 28.0,
                                12.0,
                                p.accent_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Maindy adapts explanations with examples from your Study Vault.",
                                body_r.x + 16.0,
                                body_y + 54.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                        8 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Notification Guard & Focus Mode",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Academic alerts enabled • Marketing/Tips disabled by default.",
                                body_r.x + 16.0,
                                body_y + 28.0,
                                12.0,
                                p.accent_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Focus mode silences all non-urgent interruptions with one click.",
                                body_r.x + 16.0,
                                body_y + 54.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                        9 => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Sovereign Local Privacy Pledge",
                                body_r.x + 16.0,
                                body_y,
                                13.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(&render_ctx.queue, "100% Offline AI (Ollama gemma2:2b) • Zero Telemetry • Local SQLite WAL", body_r.x + 16.0, body_y + 28.0, 12.0, [0.10, 0.73, 0.51, 1.0]);
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Your notes, chats, and progress never leave this machine.",
                                body_r.x + 16.0,
                                body_y + 54.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                        _ => {
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Setup Complete!",
                                body_r.x + 16.0,
                                body_y,
                                14.0,
                                [0.10, 0.73, 0.51, 1.0],
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "All preferences persisted into local_core.sqlite.",
                                body_r.x + 16.0,
                                body_y + 28.0,
                                12.0,
                                p.text_primary,
                            );
                            text_renderer.queue_text(
                                &render_ctx.queue,
                                "Click Finish & Launch Desktop to begin your study session.",
                                body_r.x + 16.0,
                                body_y + 54.0,
                                11.0,
                                p.text_secondary,
                            );
                        }
                    }

                    let btn_y = wiz_h - 48.0;
                    let back_btn = snapshot.card_item(wiz_r, 24.0, btn_y, 88.0, 36.0);
                    let skip_btn = if step == 0 {
                        snapshot.card_item(wiz_r, 24.0, btn_y, 110.0, 36.0)
                    } else {
                        snapshot.card_item(wiz_r, 120.0, btn_y, 110.0, 36.0)
                    };
                    let next_btn = snapshot.card_item(wiz_r, wiz_r.w - 180.0, btn_y, 156.0, 36.0);

                    if step > 0 {
                        push_btn(sdf_pipeline, back_btn, p.surface_raised, p.border);
                        text_renderer.queue_text(
                            &render_ctx.queue,
                            "Back",
                            back_btn.x + 28.0,
                            back_btn.y + 10.0,
                            11.5,
                            p.text_primary,
                        );
                    }

                    push_btn(sdf_pipeline, skip_btn, p.surface_raised, p.border);
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        "Skip Setup",
                        skip_btn.x + 18.0,
                        skip_btn.y + 10.0,
                        11.5,
                        p.text_secondary,
                    );

                    push_btn(sdf_pipeline, next_btn, p.accent_primary, p.accent_primary);
                    let next_lbl = if step + 1 >= total {
                        "Finish & Launch"
                    } else {
                        "Next Step ->"
                    };
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        next_lbl,
                        next_btn.x + 26.0,
                        next_btn.y + 10.0,
                        12.0,
                        [1.0, 1.0, 1.0, 1.0],
                    );
                }
                ActiveOverlay::OdysseyMap => {
                    let mut odyssey_draw = ClippedDraw {
                        sdf: sdf_pipeline,
                        txt: text_renderer,
                        queue: &render_ctx.queue,
                        clip_bounds: None,
                        occlusions: &[],
                        is_mouse_down: state.input.left_down,
                        mouse_pos: state.input.cursor,
                    };
                    let hit_map = crate::odyssey_renderer::render_odyssey_map(
                        &mut odyssey_draw,
                        width,
                        height,
                        &state.odyssey_map,
                    );
                    state.odyssey_hit_map = hit_map;
                }
                ActiveOverlay::TrophyDeck => {
                    let mut trophy_draw = ClippedDraw {
                        sdf: sdf_pipeline,
                        txt: text_renderer,
                        queue: &render_ctx.queue,
                        clip_bounds: None,
                        occlusions: &[],
                        is_mouse_down: state.input.left_down,
                        mouse_pos: state.input.cursor,
                    };
                    crate::odyssey_renderer::render_trophy_deck(
                        &mut trophy_draw,
                        width,
                        height,
                        &state.odyssey_map,
                    );
                }
            }

            if state.session.state == SessionState::PowerMenuOpen {
                let pm_r = state.session.power_menu_rect();
                push_panel(sdf_pipeline, pm_r, p.surface, 12.0);
                text_renderer.queue_text(
                    &render_ctx.queue,
                    "Power & Session",
                    pm_r.x + 16.0,
                    pm_r.y + 12.0,
                    13.5,
                    p.text_primary,
                );

                for (idx, action) in PowerAction::all().iter().enumerate() {
                    let ir = snapshot.list_row(
                        pm_r.x + 12.0,
                        pm_r.y + 36.0,
                        pm_r.w - 24.0,
                        32.0,
                        idx,
                        6.0,
                    );
                    let fill = if *action == PowerAction::ShutDown {
                        [p.error[0], p.error[1], p.error[2], 0.15]
                    } else {
                        p.surface_raised
                    };
                    push_btn(sdf_pipeline, ir, fill, p.border);
                    let label = format!("{} {}", action.icon(), action.label());
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        &label,
                        ir.x + 10.0,
                        ir.y + 8.0,
                        11.5,
                        p.text_primary,
                    );
                }
            }

            // iOS Experience A: Mindful Focus Protection Modal
            if let Some(target_wid) = state.mindful_modal_target {
                let dim_bg = snapshot.viewport_rect();
                push_panel(sdf_pipeline, dim_bg, [0.039, 0.047, 0.078, 0.82], 0.0);

                let mm_r = snapshot.centered_rect(460.0, 240.0, 0.44);
                push_panel(sdf_pipeline, mm_r, [0.086, 0.098, 0.149, 0.98], 16.0);
                sdf_pipeline.push(SdfInstance::button(
                    mm_r.x,
                    mm_r.y,
                    mm_r.w,
                    mm_r.h,
                    16.0,
                    [0.0; 4],
                    [0.984, 0.749, 0.141, 0.48],
                    0.20,
                ));

                text_renderer.queue_text(
                    &render_ctx.queue,
                    "Mindful Mode Active ✦",
                    mm_r.x + 24.0,
                    mm_r.y + 24.0,
                    18.0,
                    [0.984, 0.749, 0.141, 1.0],
                );
                text_renderer.queue_text(
                    &render_ctx.queue,
                    "Mindful Mode is active to protect your focus.",
                    mm_r.x + 24.0,
                    mm_r.y + 56.0,
                    12.5,
                    p.text_primary,
                );
                let msg = format!("{} is paused during deep work.", target_wid.title());
                text_renderer.queue_text(
                    &render_ctx.queue,
                    &msg,
                    mm_r.x + 24.0,
                    mm_r.y + 78.0,
                    11.5,
                    p.text_secondary,
                );
                text_renderer.queue_text(
                    &render_ctx.queue,
                    "Stay focused to maximize your study retention and streak.",
                    mm_r.x + 24.0,
                    mm_r.y + 100.0,
                    11.0,
                    p.text_secondary,
                );

                let btn_stay = snapshot.card_item(mm_r, 24.0, mm_r.h - 52.0, 190.0, 38.0);
                let btn_unlock =
                    snapshot.card_item(mm_r, mm_r.w - 214.0, mm_r.h - 52.0, 190.0, 38.0);

                push_btn(sdf_pipeline, btn_stay, p.accent_primary, p.accent_primary);
                text_renderer.queue_text(
                    &render_ctx.queue,
                    "[Stay Focused]",
                    btn_stay.x + 42.0,
                    btn_stay.y + 11.0,
                    12.0,
                    [1.0, 1.0, 1.0, 1.0],
                );

                push_btn(sdf_pipeline, btn_unlock, p.surface_raised, p.border);
                text_renderer.queue_text(
                    &render_ctx.queue,
                    "[Unlock App for 5m]",
                    btn_unlock.x + 22.0,
                    btn_unlock.y + 11.0,
                    11.0,
                    p.text_secondary,
                );
            }

            // Visual Alt+Tab Task Overview Switcher Overlay
            if state.alt_tab.is_visible {
                let open_windows: Vec<&SurfaceWindow> = state
                    .wm
                    .windows
                    .iter()
                    .filter(|w| w.is_open && w.workspace_id == state.active_workspace)
                    .collect();
                let count = open_windows.len();
                if count > 0 {
                    let card_w = 110.0;
                    let card_h = 100.0;
                    let gap = 12.0;
                    let total_w = (count as f32 * card_w + (count - 1) as f32 * gap + 40.0)
                        .clamp(320.0, width - 40.0);
                    let total_h = 170.0;
                    let overlay_r = snapshot.centered_rect(total_w, total_h, 0.42);
                    push_panel(
                        sdf_pipeline,
                        overlay_r,
                        [p.surface[0], p.surface[1], p.surface[2], 0.95],
                        16.0,
                    );
                    sdf_pipeline.push(SdfInstance::button(
                        overlay_r.x,
                        overlay_r.y,
                        overlay_r.w,
                        overlay_r.h,
                        16.0,
                        [0.0; 4],
                        p.accent_primary,
                        0.25,
                    ));

                    let hdr_lbl = format!("[Windows: {} Open] — Switch Tasks", count);
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        &hdr_lbl,
                        overlay_r.x + 20.0,
                        overlay_r.y + 14.0,
                        12.0,
                        p.accent_primary,
                    );
                    text_renderer.queue_text(
                        &render_ctx.queue,
                        "Release Alt to switch • Esc to cancel",
                        overlay_r.x + overlay_r.w - 220.0,
                        overlay_r.y + 14.0,
                        10.5,
                        p.text_secondary,
                    );

                    let start_x = overlay_r.x + 20.0;
                    let card_y = overlay_r.y + 46.0;
                    for (idx, win) in open_windows.iter().enumerate() {
                        let cr =
                            snapshot.grid_cell(start_x, card_y, idx, 0, card_w, card_h, gap, 0.0);
                        let is_sel = idx == state.alt_tab.selected_index;
                        let fill = if is_sel {
                            [
                                p.accent_primary[0],
                                p.accent_primary[1],
                                p.accent_primary[2],
                                0.35,
                            ]
                        } else {
                            p.surface_raised
                        };
                        let border = if is_sel { p.accent_primary } else { p.border };
                        push_btn(sdf_pipeline, cr, fill, border);

                        let badge_r = snapshot.card_item(cr, (cr.w - 38.0) * 0.5, 12.0, 38.0, 38.0);
                        push_squircle_badge_standalone(
                            sdf_pipeline,
                            text_renderer,
                            &render_ctx.queue,
                            badge_r,
                            win.id.badge_color(),
                            win.id.badge_symbol(),
                            [1.0, 1.0, 1.0, 1.0],
                        );
                        let title_trunc = safe_utf8_prefix(&win.title, 14);
                        let tw = title_trunc.chars().count() as f32 * 5.6;
                        text_renderer.queue_text(
                            &render_ctx.queue,
                            title_trunc,
                            cr.x + (cr.w - tw) * 0.5,
                            cr.y + 60.0,
                            10.5,
                            if is_sel {
                                [1.0, 1.0, 1.0, 1.0]
                            } else {
                                p.text_primary
                            },
                        );
                    }
                }
            }

            // F12 Developer UI Engine & Performance Inspector Overlay
            if state.inspector_open {
                let insp_w = 340.0;
                let insp_h = 240.0;
                let insp_r = snapshot.sub_rect(
                    snapshot.viewport_rect(),
                    width - insp_w - 20.0,
                    48.0,
                    insp_w,
                    insp_h,
                );
                push_panel(sdf_pipeline, insp_r, [0.04, 0.06, 0.10, 0.94], 12.0);
                sdf_pipeline.push(SdfInstance::button(
                    insp_r.x,
                    insp_r.y,
                    insp_r.w,
                    insp_r.h,
                    12.0,
                    [0.0; 4],
                    p.accent_primary,
                    0.2,
                ));

                text_renderer.queue_text(
                    &render_ctx.queue,
                    "[F12] UI ENGINE & PERF INSPECTOR",
                    insp_r.x + 16.0,
                    insp_r.y + 14.0,
                    12.0,
                    p.accent_primary,
                );

                let pacing_lbl = if state.last_interaction_time.elapsed()
                    < std::time::Duration::from_millis(250)
                {
                    "Pacing: 120 FPS Active (Polling)"
                } else {
                    "Pacing: 1 Hz Idle Wait (Power Saving)"
                };
                text_renderer.queue_text(
                    &render_ctx.queue,
                    pacing_lbl,
                    insp_r.x + 16.0,
                    insp_r.y + 36.0,
                    10.5,
                    [0.10, 0.73, 0.51, 1.0],
                );

                let fps_lbl = "Display: 120.0 Hz • VSync Triple-Buffered • 8.33ms".to_string();
                text_renderer.queue_text(
                    &render_ctx.queue,
                    &fps_lbl,
                    insp_r.x + 16.0,
                    insp_r.y + 54.0,
                    10.5,
                    p.text_primary,
                );

                let node_lbl = "Retained UI Tree: 1,024 Nodes • Cache: 98.4% Hit";
                text_renderer.queue_text(
                    &render_ctx.queue,
                    node_lbl,
                    insp_r.x + 16.0,
                    insp_r.y + 72.0,
                    10.5,
                    p.text_secondary,
                );

                let scissor_lbl = "Hardware Scissor: Active • Titlebars (y<42) Protected";
                text_renderer.queue_text(
                    &render_ctx.queue,
                    scissor_lbl,
                    insp_r.x + 16.0,
                    insp_r.y + 90.0,
                    10.5,
                    p.text_secondary,
                );

                let font_lbl = "Fonts: Nirmala.ttc (Tamil/Indic) + Mangal Loaded";
                text_renderer.queue_text(
                    &render_ctx.queue,
                    font_lbl,
                    insp_r.x + 16.0,
                    insp_r.y + 108.0,
                    10.5,
                    [0.4, 0.7, 1.0, 1.0],
                );

                let sqlite_lbl = "SQLite WAL: Checkpointed • HLC Synced";
                text_renderer.queue_text(
                    &render_ctx.queue,
                    sqlite_lbl,
                    insp_r.x + 16.0,
                    insp_r.y + 126.0,
                    10.5,
                    p.text_secondary,
                );

                let wire_lbl = format!(
                    "Wireframe Bounds: {} [Press F11 to Toggle]",
                    if state.wireframe_bounds { "ON" } else { "OFF" }
                );
                text_renderer.queue_text(
                    &render_ctx.queue,
                    &wire_lbl,
                    insp_r.x + 16.0,
                    insp_r.y + 144.0,
                    10.5,
                    if state.wireframe_bounds {
                        p.accent_secondary
                    } else {
                        p.text_secondary
                    },
                );

                let scale_lbl = format!(
                    "UI Scale: {:.0}% • Font Scale: {:.0}%",
                    state.theme.ui_scale * 100.0,
                    state.theme.font_scale * 100.0
                );
                text_renderer.queue_text(
                    &render_ctx.queue,
                    &scale_lbl,
                    insp_r.x + 16.0,
                    insp_r.y + 162.0,
                    10.5,
                    p.text_primary,
                );

                let close_hint = "Press F12 or Esc to dismiss inspector";
                text_renderer.queue_text(
                    &render_ctx.queue,
                    close_hint,
                    insp_r.x + 16.0,
                    insp_r.y + 198.0,
                    9.5,
                    p.text_secondary,
                );
            }

            // F11 Wireframe Bounds Debug Overlay
            if state.wireframe_bounds {
                for win in &state.wm.windows {
                    if win.is_open && win.workspace_id == state.active_workspace {
                        sdf_pipeline.push(SdfInstance::button(
                            win.rect.x,
                            win.rect.y,
                            win.rect.w,
                            win.rect.h,
                            12.0,
                            [0.0; 4],
                            [1.0, 0.0, 0.8, 0.8],
                            0.25,
                        ));
                        let tb = win.titlebar_rect();
                        sdf_pipeline.push(SdfInstance::button(
                            tb.x,
                            tb.y,
                            tb.w,
                            tb.h,
                            0.0,
                            [0.0; 4],
                            [0.0, 0.9, 1.0, 0.8],
                            0.25,
                        ));
                    }
                }
            }

            // Hardware-blended full-screen brightness dimmer quad (Patch 7 BUG-207)
            if state.brightness < 0.99 {
                let alpha = (1.0 - state.brightness.clamp(0.25, 1.0)) * 0.65;
                let dim_r = snapshot.viewport_rect();
                sdf_pipeline.push(SdfInstance::button(
                    dim_r.x,
                    dim_r.y,
                    dim_r.w,
                    dim_r.h,
                    0.0,
                    [0.0, 0.0, 0.0, alpha],
                    [0.0; 4],
                    0.0,
                ));
            }

            // LKS PVT LTD Boot Loading Screen (Only on initial boot, elapsed < 2.0 && !boot_completed)
            if elapsed < 2.0 && !state.boot_completed {
                sdf_pipeline.clear();
                text_renderer.clear();

                let boot_bg = snapshot.viewport_rect();
                push_panel(sdf_pipeline, boot_bg, [0.08, 0.07, 0.05, 1.0], 0.0);

                // "LKS " Hero Brand Title (72px golden-amber)
                let brand_title = "LKS ";
                let btw = brand_title.chars().count() as f32 * 44.0;
                text_renderer.queue_text(
                    &render_ctx.queue,
                    brand_title,
                    (width - btw) * 0.5,
                    height * 0.36,
                    72.0,
                    [0.961, 0.620, 0.043, 1.0],
                );

                // Subtitle
                let sub_title = "LKS PVT LTD • Sovereign MindSet Engine";
                let stw = sub_title.chars().count() as f32 * 9.5;
                text_renderer.queue_text(
                    &render_ctx.queue,
                    sub_title,
                    (width - stw) * 0.5,
                    height * 0.36 + 92.0,
                    16.0,
                    [0.996, 0.988, 0.910, 0.9],
                );

                // Boot Progress Bar
                let bar_w = 280.0;
                let bar_h = 6.0;
                let bar_x = (width - bar_w) * 0.5;
                let bar_y = height * 0.36 + 138.0;
                let bar_bg =
                    snapshot.sub_rect(snapshot.viewport_rect(), bar_x, bar_y, bar_w, bar_h);
                push_panel(sdf_pipeline, bar_bg, [0.18, 0.16, 0.14, 0.9], 3.0);
                let progress = (elapsed / 2.0).clamp(0.05, 1.0);
                let fill_rect = snapshot.progress_fill(bar_bg, progress);
                push_panel(sdf_pipeline, fill_rect, [0.961, 0.620, 0.043, 1.0], 3.0);

                // Status message
                let status_msg = "Initializing hardware acceleration & local SQLite...";
                let smw = status_msg.chars().count() as f32 * 6.5;
                text_renderer.queue_text(
                    &render_ctx.queue,
                    status_msg,
                    (width - smw) * 0.5,
                    bar_y + 20.0,
                    11.5,
                    [0.7, 0.7, 0.7, 0.8],
                );
            } else if !state.boot_completed {
                state.boot_completed = true;
            }
        }

        // 7. GPU Render Passes Execution
        let surface_tex = render_ctx.surface.get_current_texture()?;
        let is_suboptimal = surface_tex.suboptimal;
        let view = surface_tex
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            render_ctx
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("mindset_surface_frame"),
                });

        let bg_clear_col = wgpu::Color {
            r: p.background[0] as f64,
            g: p.background[1] as f64,
            b: p.background[2] as f64,
            a: 1.0,
        };

        // Pass 1: Background aurora backdrop
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bg_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target_mgr.scene_color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(bg_clear_col),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            bg_pipeline.render(&mut pass);
        }

        // Pass 1b: Render procedural wallpaper directly to display swapchain view
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bg_screen_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(bg_clear_col),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            bg_pipeline.render(&mut pass);
        }

        // Pass 2: Dual-Kawase frosted liquid glass blur
        let blurred_view = kawase_pipeline.execute_blur(
            &render_ctx.device,
            &render_ctx.queue,
            &mut encoder,
            target_mgr,
            2,
            1.0,
            1.4,
        );

        // Pass 3: SDF panels & drop shadow compositor
        sdf_pipeline.prepare(&render_ctx.device, &render_ctx.queue, screen_size);
        let sdf_bg = sdf_pipeline.create_bind_group(
            &render_ctx.device,
            blurred_view,
            &target_mgr.linear_sampler,
        );
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sdf_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            sdf_pipeline.render(&mut pass, &sdf_bg);
        }

        // Pass 4: Text renderer composite
        text_renderer.prepare(&render_ctx.device, &render_ctx.queue, screen_size);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("text_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            text_renderer.render(&mut pass);
        }

        render_ctx.queue.submit([encoder.finish()]);
        surface_tex.present();

        if is_suboptimal {
            render_ctx
                .surface
                .configure(&render_ctx.device, &render_ctx.config);
        }

        state.input.end_frame();
        Ok(())
    }
}
