//! MindSet OS — Desktop Neuro-Mission Rail (`desktop_widgets.rs`)
//! Left-side floating visionOS glass rail built on neuroscience principles:
//! 1. North Star Goal Countdown Hero (NEET 2027 Doctor Aspirant)
//! 2. Today's High-Yield Mission with direct marks attribution (+12 Marks for 1 hr physics)
//! 3. 3-Subject Quota Stack with dual-track locked vs pending bars
//! 4. Ultradian Rhythm (90m/25m) & Pomodoro Session Control
//! 5. Maindy AI Priority Guard (90% Focus on Primary Goal, 20% on Secondary)
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use crate::frame::DesktopLayout;
use crate::state::ShellState;
use mindset_render::SdfInstance;
use mindset_ui::LayoutRect;

pub fn render_desktop_widgets(state: &ShellState, draw: &mut ClippedDraw, layout: &DesktopLayout) {
    let p = state.theme.palette;
    let goal = &state.exam_goal;

    // ─── 1. North Star Goal Countdown Hero Card ───────────────────────────────
    let hero_r = layout.primary_goal_card;
    draw.sdf.push(SdfInstance::glass_panel(
        hero_r.x,
        hero_r.y,
        hero_r.w,
        hero_r.h,
        18.0,
        [
            p.surface_raised[0],
            p.surface_raised[1],
            p.surface_raised[2],
            0.88,
        ],
        p.border,
    ));

    // Stethoscope / Goal Icon Badge
    let icon_r = LayoutRect::new(hero_r.x + 12.0, hero_r.y + 12.0, 32.0, 32.0);
    draw.sdf.push(SdfInstance::button(
        icon_r.x,
        icon_r.y,
        icon_r.w,
        icon_r.h,
        10.0,
        [
            p.accent_primary[0],
            p.accent_primary[1],
            p.accent_primary[2],
            0.22,
        ],
        p.accent_primary,
        0.0,
    ));
    draw.queue_text_centered_in_rect("🩺", icon_r, 16.0, p.accent_primary);

    let mut cursor_y = hero_r.y + 11.0;
    draw.queue_text(
        "TARGET: MBBS DOCTOR",
        hero_r.x + 50.0,
        cursor_y,
        11.5,
        p.accent_primary,
    );
    cursor_y += 15.0;
    draw.queue_text(
        &format!("NEET 2027 • {} Days Left", goal.days_remaining),
        hero_r.x + 50.0,
        cursor_y,
        10.0,
        p.text_secondary,
    );

    // Projected Score Bar
    cursor_y = hero_r.y + 50.0;
    let proj_score = goal.projected_score();
    let score_lbl = format!("Projected Score: {} / {} Marks", proj_score, goal.max_score);
    draw.queue_text(&score_lbl, hero_r.x + 12.0, cursor_y, 11.0, p.text_primary);

    let rank_pct = if goal.target_score > 0 {
        ((proj_score as f32 / goal.target_score as f32) * 100.0).min(100.0)
    } else {
        0.0
    };
    let rank_lbl = format!("{:.0}% Target", rank_pct);
    let (rw, _) = draw.measure_line_px(&rank_lbl, 10.0);
    draw.queue_text(
        &rank_lbl,
        hero_r.x + hero_r.w - rw - 12.0,
        cursor_y,
        10.0,
        p.success,
    );

    cursor_y += 18.0;
    let pb_r = LayoutRect::new(hero_r.x + 12.0, cursor_y, hero_r.w - 24.0, 6.0);
    draw.push_progress_bar(
        pb_r,
        (proj_score as f32 / goal.max_score as f32).clamp(0.0, 1.0),
        p.border,
        p.accent_primary,
    );

    // AI Focus Caution Badge
    cursor_y += 14.0;
    draw.queue_text(
        "⚡ Maindy AI Focus: 90% Primary • 20% Secondary",
        hero_r.x + 12.0,
        cursor_y,
        8.5,
        p.text_muted,
    );

    // ─── 2. Today's High-Yield Mission Card (Dopamine Attribution) ────────────
    let mission_r = layout.mission_card;
    draw.sdf.push(SdfInstance::glass_panel(
        mission_r.x,
        mission_r.y,
        mission_r.w,
        mission_r.h,
        18.0,
        [
            p.surface_raised[0],
            p.surface_raised[1],
            p.surface_raised[2],
            0.92,
        ],
        [
            p.accent_secondary[0],
            p.accent_secondary[1],
            p.accent_secondary[2],
            0.45,
        ],
    ));

    let mut my = mission_r.y + 10.0;
    draw.queue_text(
        "⚡ TODAY'S HIGH-YIELD MISSION",
        mission_r.x + 12.0,
        my,
        10.5,
        p.accent_secondary,
    );

    let roi_badge = LayoutRect::new(mission_r.x + mission_r.w - 68.0, my - 2.0, 56.0, 16.0);
    draw.push_btn(
        roi_badge,
        [p.warning[0], p.warning[1], p.warning[2], 0.20],
        p.warning,
    );
    draw.queue_text_centered_in_rect("🔥 4.0 m/h", roi_badge, 9.5, p.warning);

    my += 20.0;
    let mission_title = if let Some(node) = state.odyssey_map.active_node() {
        format!("Lvl {}: {}", node.level, node.chapter_title)
    } else {
        goal.today_mission()
            .map(|m| m.title.clone())
            .unwrap_or_else(|| "Physics: Modern Physics".to_string())
    };
    let fitted_title = draw.fit_text_to_px(&mission_title, 11.5, mission_r.w - 24.0);
    draw.queue_text(&fitted_title, mission_r.x + 12.0, my, 11.5, p.text_primary);

    my += 18.0;
    let marks_gain = state
        .odyssey_map
        .active_node()
        .map(|n| n.marks_yield)
        .unwrap_or(16);
    let stage_info = state
        .odyssey_map
        .active_node()
        .map(|n| format!("Stage {}/3 Done", n.mission.completed_count()))
        .unwrap_or_else(|| "3-Stage Flow".to_string());
    let attr_lbl = format!("⏱ {} ──> 🔒 Locks +{} Marks", stage_info, marks_gain);
    draw.queue_text(&attr_lbl, mission_r.x + 12.0, my, 9.5, p.text_secondary);

    // Interactive Trigger Buttons: [▶ Start Flow] and [🔒 Lock Marks]
    let start_flow_btn = layout.hits.mission_start_btn;
    let lock_marks_btn = layout.hits.mission_lock_btn;

    draw.push_btn(start_flow_btn, p.accent_primary, [0.0; 4]);
    draw.queue_text_centered_in_rect("▶ Start Flow", start_flow_btn, 10.5, [1.0, 1.0, 1.0, 1.0]);

    draw.push_btn(
        lock_marks_btn,
        [p.success[0], p.success[1], p.success[2], 0.25],
        p.success,
    );
    draw.queue_text_centered_in_rect("🔒 Lock Marks", lock_marks_btn, 10.5, p.success);

    my = start_flow_btn.y + start_flow_btn.h + 6.0;
    let caption_avail_w = (mission_r.w - 24.0).max(0.0);
    let avg_char_w = 5.8;
    let max_chars = (caption_avail_w / avg_char_w).floor() as usize;
    let wrapped_caption = mindset_render::wrap_text_chars(
        "Micro-attribution unlocks dopamine upon completion.",
        max_chars,
    );
    for subline in wrapped_caption.lines() {
        let fitted = draw.fit_text_to_px(subline, 8.5, caption_avail_w);
        draw.queue_text(&fitted, mission_r.x + 12.0, my, 8.5, p.text_muted);
        my += 11.0;
    }

    // ─── 3. Subject Quota Mastery Stack ───────────────────────────────────────
    let quota_r = layout.subject_quota_card;
    draw.sdf.push(SdfInstance::glass_panel(
        quota_r.x,
        quota_r.y,
        quota_r.w,
        quota_r.h,
        18.0,
        [
            p.surface_raised[0],
            p.surface_raised[1],
            p.surface_raised[2],
            0.85,
        ],
        p.border,
    ));

    let mut qy = quota_r.y + 10.0;
    draw.queue_text(
        "SUBJECT QUOTA TARGETS",
        quota_r.x + 12.0,
        qy,
        10.5,
        p.text_primary,
    );
    draw.queue_text(
        "Breakdown",
        quota_r.x + quota_r.w - 64.0,
        qy,
        9.5,
        p.text_secondary,
    );

    for subj in &goal.subjects {
        qy += 22.0;
        let line_lbl = format!(
            "{}: {} / {}m",
            subj.name, subj.locked_marks, subj.target_marks
        );
        draw.queue_text(&line_lbl, quota_r.x + 12.0, qy, 10.0, p.text_secondary);

        let pct_lbl = format!("{:.0}%", subj.progress_ratio() * 100.0);
        let (pw, _) = draw.measure_line_px(&pct_lbl, 9.5);
        draw.queue_text(
            &pct_lbl,
            quota_r.x + quota_r.w - pw - 12.0,
            qy,
            9.5,
            p.text_primary,
        );

        qy += 12.0;
        let spb = LayoutRect::new(quota_r.x + 12.0, qy, quota_r.w - 24.0, 4.0);
        let fill_col = match subj.name.as_str() {
            "Biology" => p.success,
            "Chemistry" => p.info,
            _ => p.accent_primary,
        };
        draw.push_progress_bar(spb, subj.progress_ratio(), p.border, fill_col);
    }

    // ─── 4. Ultradian Rhythm (90m/25m) & Pomodoro Session Control ─────────────
    let timer_r = layout.pomodoro_card;
    draw.sdf.push(SdfInstance::glass_panel(
        timer_r.x,
        timer_r.y,
        timer_r.w,
        timer_r.h,
        16.0,
        [
            p.surface_raised[0],
            p.surface_raised[1],
            p.surface_raised[2],
            0.88,
        ],
        p.border,
    ));

    let ty = timer_r.y + 10.0;
    draw.queue_text(
        "🧠 NEURO-FOCUS CONTROLS",
        timer_r.x + 12.0,
        ty,
        9.5,
        p.accent_primary,
    );

    let timer_pill = layout.hits.pomodoro_duration_btn;
    let sound_pill = layout.hits.pomodoro_cycle_btn;

    // Timer Mode Pill (Clicking cycles duration 25m -> 30m -> 45m -> 60m -> 25m)
    draw.push_btn(timer_pill, p.surface, p.border);
    let t_lbl = format!("⏱ {}m", state.pomodoro_duration_mins);
    draw.queue_text_centered_in_rect(&t_lbl, timer_pill, 9.5, p.text_primary);

    // Session Counter Pill (Clicking cycles 1/4 -> 2/4 -> 3/4 -> 4/4 -> 1/4)
    draw.push_btn(sound_pill, p.surface, p.border);
    let s_lbl = format!("🎯 Session {}/4", state.pomodoro_completed_sessions);
    draw.queue_text_centered_in_rect(&s_lbl, sound_pill, 9.5, p.text_primary);
}
