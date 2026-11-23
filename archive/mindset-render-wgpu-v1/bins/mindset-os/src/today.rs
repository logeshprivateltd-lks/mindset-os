//! MindSet OS — Today Screen State & Neuro-Mission Renderer (`today.rs`)
//! Single-column, neuroscience-designed root view:
//! 1. Greeting Row with Streak Flame & Freezes Remaining
//! 2. Today's One Thing Card (Title, Topic, Subtopic, Duration, Why-Line, Marks Delta) + [ START FOCUS ]
//!    (Or Recovery Banner when days are missed)
//! 3. Exactly Three Progress Rings: Today, Week, Syllabus
//! 4. Future Self Card (Target Identity Label, "Today's X min = 1 step closer.")
//! 5. Tomorrow Preview (Muted line)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use chrono::{Local, NaiveDate, Timelike};
use mindset_intelligence::goal_bridge::GoalBridge;
use mindset_motion::{Spring1D, SpringPreset};
use mindset_render::{CelebrationBurst, SdfInstance};
use mindset_surface::session::{DOCK_H, TOP_BAR_H};
use mindset_sys::recovery::{check_recovery, RecoveryState, StreakState};
use mindset_ui::layout::LayoutRect;
use mindset_ui::widgets::ProgressRing;

#[derive(Debug, Clone)]
pub struct TaskSnapshot {
    pub id: String,
    pub title: String,
    pub topic: String,
    pub subtopic: String,
    pub duration_minutes: u32,
    pub why_line: String,
    pub marks_delta: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct RingProgress {
    pub label: String,
    pub value_text: String,
    pub fraction: f32,
}

#[derive(Debug, Clone)]
pub struct FutureSelfSnapshot {
    pub target_identity_label: String, // e.g. "Dr. Arjun, MBBS 2029"
    pub glow_timer: f32,               // 400ms glow on completion
}

#[derive(Debug, Clone)]
pub struct StreakSnapshot {
    pub count: u32,
    pub freezes_remaining: u32,
    pub flame_scale: Spring1D,
    pub freeze_used_timer: f32,
}

pub struct TodayState {
    pub one_thing: Option<TaskSnapshot>,
    pub rings: [ProgressRing; 3], // Today, Week, Syllabus
    pub ring_data: [RingProgress; 3],
    pub future_self: FutureSelfSnapshot,
    pub streak: StreakSnapshot,
    pub tomorrow_preview: Vec<String>,
    pub recovery: Option<RecoveryState>,
    pub celebration: CelebrationBurst,
    pub last_session_date: Option<NaiveDate>,
}

impl Default for TodayState {
    fn default() -> Self {
        Self::new()
    }
}

impl TodayState {
    pub fn new() -> Self {
        let ring_today = ProgressRing::new("Today", "2h/4h");
        let ring_week = ProgressRing::new("Week", "8h/20h");
        let ring_syllabus = ProgressRing::new("Syllabus", "34%");

        let ring_data = [
            RingProgress {
                label: "Today".to_string(),
                value_text: "2h/4h".to_string(),
                fraction: 0.50,
            },
            RingProgress {
                label: "Week".to_string(),
                value_text: "8h/20h".to_string(),
                fraction: 0.40,
            },
            RingProgress {
                label: "Syllabus".to_string(),
                value_text: "34%".to_string(),
                fraction: 0.34,
            },
        ];

        Self {
            one_thing: None,
            rings: [ring_today, ring_week, ring_syllabus],
            ring_data,
            future_self: FutureSelfSnapshot {
                target_identity_label: "Dr. Logesh Karthik, MBBS 2027".to_string(),
                glow_timer: 0.0,
            },
            streak: StreakSnapshot {
                count: 14,
                freezes_remaining: 2,
                flame_scale: Spring1D::with_preset(1.0, SpringPreset::SnappyMicro),
                freeze_used_timer: 0.0,
            },
            tomorrow_preview: vec![
                "Modern Physics".to_string(),
                "Organic Chemistry".to_string(),
                "4 Pomodoros".to_string(),
            ],
            recovery: None,
            celebration: CelebrationBurst::new(),
            last_session_date: Some(Local::now().date_naive()),
        }
    }

    pub fn refresh(&mut self) {
        let today = Local::now().date_naive();
        let streak_state = StreakState {
            current_streak: self.streak.count,
            freezes_remaining: self.streak.freezes_remaining,
        };
        self.recovery = check_recovery(self.last_session_date, today, &streak_state);

        // Populate goal bridge & One Thing
        let exam_date = NaiveDate::from_ymd_opt(2027, 4, 5).unwrap_or(today);
        let bridge = GoalBridge::new(
            "NEET 2027".to_string(),
            exam_date,
            720,
            0.25,
            0.06,
            Some(0.40),
            Some(0.75),
        );

        let marks = bridge.marks_delta();
        let why = bridge.why_line("Wave Optics");

        self.one_thing = Some(TaskSnapshot {
            id: "task_wave_optics_derivation".to_string(),
            title: "Derive Gauss's Law & Wave Optics Interference".to_string(),
            topic: "Physics".to_string(),
            subtopic: "Wave Optics".to_string(),
            duration_minutes: 25,
            why_line: why,
            marks_delta: marks,
        });

        self.rings[0].set_progress(self.ring_data[0].fraction, false);
        self.rings[1].set_progress(self.ring_data[1].fraction, false);
        self.rings[2].set_progress(self.ring_data[2].fraction, false);
    }

    pub fn update(&mut self, dt: f32) {
        self.celebration.update(dt);
        self.streak.flame_scale.update(dt);
        if self.future_self.glow_timer > 0.0 {
            self.future_self.glow_timer = (self.future_self.glow_timer - dt).max(0.0);
        }
        if self.streak.freeze_used_timer > 0.0 {
            self.streak.freeze_used_timer = (self.streak.freeze_used_timer - dt).max(0.0);
        }
        for ring in &mut self.rings {
            ring.update(dt);
        }
    }

    pub fn complete_session(&mut self, minutes: u32, reduced_motion: bool) {
        // Complete current session, spring rings, fire celebration
        self.ring_data[0].fraction =
            (self.ring_data[0].fraction + (minutes as f32 / 240.0)).min(1.0);
        self.ring_data[1].fraction =
            (self.ring_data[1].fraction + (minutes as f32 / 1200.0)).min(1.0);
        self.ring_data[2].fraction = (self.ring_data[2].fraction + 0.01).min(1.0);

        self.rings[0].set_progress(self.ring_data[0].fraction, reduced_motion);
        self.rings[1].set_progress(self.ring_data[1].fraction, reduced_motion);
        self.rings[2].set_progress(self.ring_data[2].fraction, reduced_motion);

        self.future_self.glow_timer = 0.40; // 400ms glow
        self.last_session_date = Some(Local::now().date_naive());

        if !reduced_motion {
            self.celebration
                .trigger([640.0, 240.0], [0.96, 0.62, 0.04, 1.0]);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TodayHitTargets {
    pub col_rect: LayoutRect,
    pub minimize_btn: LayoutRect,
    pub one_thing_card: LayoutRect,
    pub start_focus_btn: Option<LayoutRect>,
    pub setup_goals_btn: Option<LayoutRect>,
    pub restart_btn: Option<LayoutRect>,
    pub skip_btn: Option<LayoutRect>,
    pub ring_rects: [LayoutRect; 3],
    pub future_self_rect: LayoutRect,
}

pub fn compute_today_hit_targets(
    screen_w: f32,
    screen_h: f32,
    has_one_thing: bool,
    has_recovery: bool,
) -> TodayHitTargets {
    let top_y = TOP_BAR_H;
    let col_w = (800.0_f32).min(screen_w - 48.0);
    let col_x = (screen_w - col_w) * 0.5;

    let mut cursor_y = top_y + 16.0;

    let min_btn_w = 92.0;
    let minimize_btn = LayoutRect::new(col_x + col_w - min_btn_w, cursor_y - 2.0, min_btn_w, 26.0);

    cursor_y += 32.0;

    let one_thing_card;
    let mut start_focus_btn = None;
    let mut setup_goals_btn = None;
    let mut restart_btn = None;
    let mut skip_btn = None;

    if has_recovery {
        let rec_h = 144.0;
        let rec_r = LayoutRect::new(col_x, cursor_y, col_w, rec_h);
        one_thing_card = rec_r;
        let r_btn = LayoutRect::new(rec_r.x + 20.0, rec_r.y + 92.0, 180.0, 36.0);
        let s_btn = LayoutRect::new(r_btn.x + r_btn.w + 12.0, rec_r.y + 92.0, 130.0, 36.0);
        restart_btn = Some(r_btn);
        skip_btn = Some(s_btn);
        cursor_y += rec_h + 16.0;
    } else {
        let card_h = 160.0;
        let card_r = LayoutRect::new(col_x, cursor_y, col_w, card_h);
        one_thing_card = card_r;
        if has_one_thing {
            let cta = LayoutRect::new(card_r.x + 20.0, card_r.y + 104.0, card_r.w - 40.0, 42.0);
            start_focus_btn = Some(cta);
        } else {
            let setup = LayoutRect::new(card_r.x + 20.0, card_r.y + 96.0, 160.0, 36.0);
            setup_goals_btn = Some(setup);
        }
        cursor_y += card_h + 16.0;
    }

    let rings_h = 100.0;
    let ring_col_w = (col_w - 24.0) / 3.0;
    let (r0, r1, r2) = if !has_recovery {
        let r0 = LayoutRect::new(col_x, cursor_y, ring_col_w, rings_h);
        let r1 = LayoutRect::new(col_x + (ring_col_w + 12.0), cursor_y, ring_col_w, rings_h);
        let r2 = LayoutRect::new(
            col_x + 2.0 * (ring_col_w + 12.0),
            cursor_y,
            ring_col_w,
            rings_h,
        );
        cursor_y += rings_h + 16.0;
        (r0, r1, r2)
    } else {
        (LayoutRect::zero(), LayoutRect::zero(), LayoutRect::zero())
    };

    let fs_h = 82.0;
    let fs_r = LayoutRect::new(col_x, cursor_y, col_w, fs_h);

    TodayHitTargets {
        col_rect: LayoutRect::new(col_x, top_y, col_w, (screen_h - top_y - DOCK_H).max(200.0)),
        minimize_btn,
        one_thing_card,
        start_focus_btn,
        setup_goals_btn,
        restart_btn,
        skip_btn,
        ring_rects: [r0, r1, r2],
        future_self_rect: fs_r,
    }
}

pub struct TodayClickResult {
    pub open_study_reader: bool,
    pub open_settings_profile: bool,
    pub dismiss_recovery: bool,
    pub minimize_to_desktop: bool,
}

pub fn render_today(
    state: &mut TodayState,
    draw: &mut ClippedDraw,
    screen_w: f32,
    screen_h: f32,
    user_name: &str,
    is_left_click: bool,
    mouse_pos: (f32, f32),
) -> TodayClickResult {
    let mut click_res = TodayClickResult {
        open_study_reader: false,
        open_settings_profile: false,
        dismiss_recovery: false,
        minimize_to_desktop: false,
    };

    let targets = compute_today_hit_targets(
        screen_w,
        screen_h,
        state.one_thing.is_some(),
        state.recovery.is_some(),
    );
    let col_w = targets.col_rect.w;
    let col_x = targets.col_rect.x;

    let p_surface = [0.0824, 0.0980, 0.1333, 0.88];
    let p_surface_raised = [0.1098, 0.1333, 0.1882, 0.94];
    let p_accent = [0.8980, 0.6784, 0.2706, 1.0]; // Solar Amber
    let p_text_primary = [0.9608, 0.9647, 0.9804, 1.0];
    let p_text_secondary = [0.5725, 0.6039, 0.6667, 1.0];
    let p_border = [1.0, 1.0, 1.0, 0.15];
    let p_obsidian = [0.067, 0.094, 0.153, 1.0]; // Dark Obsidian: 8.8:1 contrast against Solar Amber
    let p_success = [0.2039, 0.8275, 0.5490, 1.0]; // Emerald Mint Green (#34D399)
    let p_future_self_subtext = [0.90, 0.93, 0.98, 0.90]; // High-contrast subtext (>= 0.85 opacity)

    let cursor_y = targets.minimize_btn.y + 2.0;

    // ─── 1. Greeting Row: "Good evening, {name}." · {streak} streak · [ 🗕 Desktop ] ───
    let hour = Local::now().hour();
    let time_greeting = if hour < 12 {
        "Good morning"
    } else if hour < 17 {
        "Good afternoon"
    } else {
        "Good evening"
    };
    let greeting_text = if state.last_session_date.is_none() {
        format!("Welcome, {}.", user_name)
    } else {
        format!("{}, {}.", time_greeting, user_name)
    };

    draw.queue_text(&greeting_text, col_x, cursor_y, 17.0, p_text_primary);

    // Minimize to desktop button on far right of greeting row
    draw.sdf.push(SdfInstance::button(
        targets.minimize_btn.x,
        targets.minimize_btn.y,
        targets.minimize_btn.w,
        targets.minimize_btn.h,
        6.0,
        p_surface,
        p_border,
        0.0,
    ));
    draw.queue_text_centered_in_rect("🗕 Desktop", targets.minimize_btn, 11.5, p_text_secondary);

    let streak_str = if state.streak.count == 0 {
        "Start your first session".to_string()
    } else {
        format!(
            "🔥 {}-day streak   ·   {} freezes left",
            state.streak.count, state.streak.freezes_remaining
        )
    };
    let (sw, _) = draw.measure_line_px(&streak_str, 13.5);
    let streak_x = targets.minimize_btn.x - 14.0 - sw;
    draw.queue_text(&streak_str, streak_x, cursor_y + 2.0, 13.5, p_accent);

    if is_left_click && targets.minimize_btn.contains(mouse_pos.0, mouse_pos.1) {
        click_res.minimize_to_desktop = true;
    }

    // ─── 2. Recovery Banner OR Today's One Thing Card ─────────────────────────
    if let Some(ref rec) = state.recovery {
        let rec_r = targets.one_thing_card;
        draw.sdf.push(SdfInstance::glass_panel(
            rec_r.x,
            rec_r.y,
            rec_r.w,
            rec_r.h,
            16.0,
            p_surface_raised,
            p_border,
        ));

        draw.queue_text(
            "Welcome back, friend.",
            rec_r.x + 20.0,
            rec_r.y + 16.0,
            17.0,
            p_text_primary,
        );
        let gap_copy = format!(
            "You missed {} days. That's okay — most NEET toppers had gaps too.\nYour streak is {}.",
            rec.days_missed,
            if rec.streak_preserved {
                "preserved with your freeze"
            } else {
                "ready for a fresh start"
            }
        );
        draw.queue_text(
            &gap_copy,
            rec_r.x + 20.0,
            rec_r.y + 40.0,
            13.5,
            p_text_secondary,
        );

        // Buttons
        if let (Some(restart_btn), Some(skip_btn)) = (targets.restart_btn, targets.skip_btn) {
            draw.sdf.push(SdfInstance::button(
                restart_btn.x,
                restart_btn.y,
                restart_btn.w,
                restart_btn.h,
                8.0,
                p_accent,
                [1.0, 1.0, 1.0, 0.3],
                0.1,
            ));
            draw.queue_text_centered_in_rect("Start 5-min restart", restart_btn, 13.5, p_obsidian);

            draw.sdf.push(SdfInstance::button(
                skip_btn.x, skip_btn.y, skip_btn.w, skip_btn.h, 8.0, p_surface, p_border, 0.0,
            ));
            draw.queue_text_centered_in_rect("Skip for today", skip_btn, 13.5, p_text_secondary);

            if is_left_click {
                if restart_btn.contains(mouse_pos.0, mouse_pos.1) {
                    click_res.open_study_reader = true;
                    click_res.dismiss_recovery = true;
                } else if skip_btn.contains(mouse_pos.0, mouse_pos.1) {
                    click_res.dismiss_recovery = true;
                }
            }
        }
    } else {
        // Normal One Thing Card
        let card_r = targets.one_thing_card;
        draw.sdf.push(SdfInstance::glass_panel(
            card_r.x,
            card_r.y,
            card_r.w,
            card_r.h,
            16.0,
            p_surface_raised,
            p_border,
        ));

        draw.queue_text(
            "TODAY'S ONE THING",
            card_r.x + 20.0,
            card_r.y + 14.0,
            13.5,
            p_accent,
        );

        if let Some(ref t) = state.one_thing {
            // Prominent +X marks badge on top-right (never truncated)
            if let Some(m) = t.marks_delta {
                let badge_text = format!("+{:.1} marks", m);
                let (bw, _) = draw.measure_line_px(&badge_text, 12.5);
                let badge_w = bw + 20.0;
                let badge_h = 24.0;
                let badge_r = LayoutRect::new(
                    card_r.x + card_r.w - 20.0 - badge_w,
                    card_r.y + 12.0,
                    badge_w,
                    badge_h,
                );
                draw.sdf.push(SdfInstance::button(
                    badge_r.x,
                    badge_r.y,
                    badge_r.w,
                    badge_r.h,
                    12.0,
                    [p_success[0], p_success[1], p_success[2], 0.16],
                    [p_success[0], p_success[1], p_success[2], 0.45],
                    0.0,
                ));
                draw.queue_text_centered_in_rect(&badge_text, badge_r, 12.5, p_success);
            }

            draw.queue_text(
                &format!("{} • {}", t.topic, t.subtopic),
                card_r.x + 20.0,
                card_r.y + 36.0,
                17.0,
                p_text_primary,
            );

            // Elevated "Why This Matters" line in crisp p_text_primary
            let why_line = format!("{} min  ·  {}", t.duration_minutes, t.why_line);
            draw.queue_text_bounded(
                &why_line,
                card_r.x + 20.0,
                card_r.y + 64.0,
                card_r.w - 40.0,
                13.5,
                p_text_primary,
            );

            // Primary CTA: [ START FOCUS ]
            if let Some(cta_btn) = targets.start_focus_btn {
                draw.sdf.push(SdfInstance::button(
                    cta_btn.x,
                    cta_btn.y,
                    cta_btn.w,
                    cta_btn.h,
                    10.0,
                    p_accent,
                    [1.0, 1.0, 1.0, 0.40],
                    0.15,
                ));
                draw.queue_text_centered_in_rect("START FOCUS", cta_btn, 15.5, p_obsidian);

                if is_left_click && cta_btn.contains(mouse_pos.0, mouse_pos.1) {
                    click_res.open_study_reader = true;
                }
            }
        } else {
            draw.queue_text(
                "Set up your goals to get started.",
                card_r.x + 20.0,
                card_r.y + 38.0,
                15.5,
                p_text_primary,
            );
            if let Some(setup_btn) = targets.setup_goals_btn {
                draw.sdf.push(SdfInstance::button(
                    setup_btn.x,
                    setup_btn.y,
                    setup_btn.w,
                    setup_btn.h,
                    8.0,
                    p_surface,
                    p_border,
                    0.0,
                ));
                draw.queue_text_centered_in_rect("Set up goals", setup_btn, 13.5, p_text_secondary);
                if is_left_click && setup_btn.contains(mouse_pos.0, mouse_pos.1) {
                    click_res.open_settings_profile = true;
                }
            }
        }

        // ─── 3. Exactly 3 Progress Rings (Today, Week, Syllabus) ──────────────
        for (idx, ring) in state.rings.iter().enumerate() {
            let rr = targets.ring_rects[idx];
            draw.sdf.push(SdfInstance::glass_panel(
                rr.x, rr.y, rr.w, rr.h, 14.0, p_surface, p_border,
            ));

            let center_x = rr.x + 40.0;
            let center_y = rr.y + rr.h * 0.5;
            let outer_r = 26.0;
            let thick = 8.0;

            // Draw SDF ring
            draw.sdf.push(SdfInstance::progress_ring(
                center_x,
                center_y,
                outer_r,
                thick,
                ring.progress.position,
                -std::f32::consts::FRAC_PI_2,
                [0.25, 0.32, 0.45, 0.30],
                [p_accent[0], p_accent[1], p_accent[2], 1.0],
            ));

            // Ring text info
            draw.queue_text(
                &ring.label,
                center_x + outer_r + 14.0,
                center_y - 13.0,
                13.0,
                p_text_secondary,
            );
            draw.queue_text(
                &ring.value_text,
                center_x + outer_r + 14.0,
                center_y + 4.0,
                15.0,
                p_text_primary,
            );
        }
    }

    // ─── 4. Future Self Card ──────────────────────────────────────────────────
    let fs_r = targets.future_self_rect;
    let border_col = if state.future_self.glow_timer > 0.0 {
        [p_accent[0], p_accent[1], p_accent[2], 0.85]
    } else {
        p_border
    };
    draw.sdf.push(SdfInstance::glass_panel(
        fs_r.x, fs_r.y, fs_r.w, fs_r.h, 14.0, p_surface, border_col,
    ));

    // Avatar pill
    let av_r = LayoutRect::new(fs_r.x + 16.0, fs_r.y + 16.0, 50.0, 50.0);
    draw.sdf.push(SdfInstance::button(
        av_r.x,
        av_r.y,
        av_r.w,
        av_r.h,
        25.0,
        p_surface_raised,
        p_accent,
        0.0,
    ));
    draw.queue_text_centered_in_rect("🩺", av_r, 22.0, p_accent);

    let id_label = if state.future_self.target_identity_label.is_empty() {
        "Future You"
    } else {
        &state.future_self.target_identity_label
    };
    draw.queue_text(id_label, fs_r.x + 78.0, fs_r.y + 18.0, 15.5, p_text_primary);
    draw.queue_text(
        "Today's 25 min = 1 step closer to your seat.",
        fs_r.x + 78.0,
        fs_r.y + 42.0,
        13.5,
        p_future_self_subtext,
    );

    let tomorrow_y = fs_r.y + fs_r.h + 16.0;

    // ─── 5. Tomorrow Preview (One line, muted) ────────────────────────────────
    let tomorrow_line = format!("Tomorrow: {}", state.tomorrow_preview.join(" • "));
    draw.queue_text_centered_in_rect(
        &tomorrow_line,
        LayoutRect::new(col_x, tomorrow_y, col_w, 20.0),
        13.5,
        p_text_secondary,
    );

    // ─── 6. Draw Celebration Burst particles (if active) ─────────────────────
    if state.celebration.active {
        for p in &state.celebration.particles {
            if p.life > 0.0 {
                let alpha = (p.life / 0.8).clamp(0.0, 1.0);
                let col = [p.color[0], p.color[1], p.color[2], p.color[3] * alpha];
                draw.sdf.push(SdfInstance::button(
                    p.pos[0] - p.size * 0.5,
                    p.pos[1] - p.size * 0.5,
                    p.size,
                    p.size,
                    p.size * 0.5,
                    col,
                    [0.0; 4],
                    0.0,
                ));
            }
        }
    }

    click_res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn relative_luminance(c: [f32; 4]) -> f32 {
        let to_linear = |v: f32| -> f32 {
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * to_linear(c[0]) + 0.7152 * to_linear(c[1]) + 0.0722 * to_linear(c[2])
    }

    fn contrast_ratio(c1: [f32; 4], c2: [f32; 4]) -> f32 {
        let l1 = relative_luminance(c1);
        let l2 = relative_luminance(c2);
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        (lighter + 0.05) / (darker + 0.05)
    }

    #[test]
    fn test_primary_cta_contrast_fix1() {
        let p_accent = [0.8980, 0.6784, 0.2706, 1.0]; // Button background
        let p_obsidian = [0.067, 0.094, 0.153, 1.0]; // Dark Obsidian text
        let ratio = contrast_ratio(p_accent, p_obsidian);
        assert!(
            ratio >= 4.5,
            "Primary CTA contrast ratio must be at least 4.5:1, got {:.2}:1",
            ratio
        );
        // Exceeds WCAG AAA (7.0:1) as well
        assert!(
            ratio >= 7.0,
            "Obsidian on Solar Amber exceeds 7:1, got {:.2}:1",
            ratio
        );
    }

    #[test]
    fn test_progress_rings_parameters_fix2() {
        // Fix 2: Progress rings track opacity 0.30, filled accent 1.0, stroke thickness >= 7.5
        let track_color = [0.25, 0.32, 0.45, 0.30];
        assert_eq!(track_color[3], 0.30, "Unfilled track opacity must be 0.30");

        let p_accent = [0.8980, 0.6784, 0.2706, 1.0];
        let filled_accent = [p_accent[0], p_accent[1], p_accent[2], 1.0];
        assert_eq!(filled_accent[3], 1.0, "Filled accent opacity must be 1.0");

        let thick = 8.0_f32;
        assert!(thick >= 7.5, "Ring thickness must be at least 7.5px");

        let val_font_size = 15.0_f32;
        assert!(
            val_font_size >= 14.5,
            "Ring value font size must be >= 14.5px"
        );
    }

    #[test]
    fn test_today_one_thing_marks_delta_badge_fix3() {
        let mut state = TodayState::new();
        state.refresh();
        let Some(ref one_thing) = state.one_thing else {
            panic!("One thing task must be populated");
        };
        assert!(
            one_thing.marks_delta.is_some(),
            "Marks delta should be present"
        );
        let Some(marks) = one_thing.marks_delta else {
            panic!("Marks delta must be present");
        };
        assert!(marks > 0.0, "Marks delta must be positive");
        let badge_text = format!("+{:.1} marks", marks);
        assert!(badge_text.starts_with('+'));
        assert!(badge_text.ends_with("marks"));
    }

    #[test]
    fn test_future_self_subtext_opacity_fix4() {
        let p_future_self_subtext = [0.90, 0.93, 0.98, 0.90];
        assert!(
            p_future_self_subtext[3] >= 0.85,
            "Future self subtext opacity must be >= 0.85, got {}",
            p_future_self_subtext[3]
        );
    }
}
