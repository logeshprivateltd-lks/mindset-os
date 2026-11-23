//! MindSet OS — Tasks, Reverse Goal Breakdown & What-If Stats Window (`tasks_stats.rs`)
//! Features:
//! 1. Reverse Goal Engineering & Chapter Yield table sorted by ROI (Marks / Hour)
//! 2. Direct Micro-Attribution (+16 Marks for Modern Physics / Optics)
//! 3. Interactive "What-If" Simulation Text (Target 680/720 Govt College Cut-Off)
//! 4. Primary (90% AI cognition) vs Secondary (20% passive) Goal Switcher
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use crate::state::ShellState;
use mindset_ui::{ChapterYieldStatus, LayoutRect};

pub fn render_tasks_content(
    state: &ShellState,
    draw: &mut ClippedDraw,
    area: LayoutRect,
    scroll_y: f32,
) {
    let p = state.theme.palette;
    let goal = &state.exam_goal;
    let (header_row, list_area) = area.split_top(48.0);

    // Goal Banner & Priority Caution
    let banner_r = LayoutRect::new(header_row.x, header_row.y, header_row.w, 42.0);
    draw.push_panel(banner_r, p.surface_raised, 10.0);
    draw.queue_text(
        &format!(
            "🎯 PRIMARY GOAL: {} • Target: {}/{}m",
            goal.title, goal.target_score, goal.max_score
        ),
        banner_r.x + 12.0,
        banner_r.y + 8.0,
        11.5,
        p.accent_primary,
    );
    draw.queue_text(
        "⚡ Maindy AI Focus Policy: 90% cognition on Primary • 20% on Secondary goal.",
        banner_r.x + 12.0,
        banner_r.y + 24.0,
        9.5,
        p.warning,
    );

    // Chapter Yield Reverse Breakdown
    let btn_w = 88.0;
    let font_body = 12.0;
    for (i, ch) in goal.chapters.iter().enumerate() {
        let row_r = LayoutRect::new(
            list_area.x,
            list_area.y + 6.0 + i as f32 * 58.0 - scroll_y,
            list_area.w,
            52.0,
        );
        draw.push_panel(row_r, p.surface_raised, 10.0);

        // Header line: title on left, marks weight & subject on right
        let subject_badge = format!("[{}] {}m", ch.subject, ch.weightage_marks);
        let split_rect = LayoutRect::new(row_r.x + 12.0, row_r.y + 8.0, row_r.w - 24.0, 20.0);
        draw.queue_split_row(
            &ch.title,
            &subject_badge,
            split_rect,
            font_body,
            p.text_primary,
            p.accent_primary,
        );

        // Bottom line: ROI metric, progress, and Action button
        let roi_lbl = format!(
            "ROI: {:.1} Marks/hr • Est: {:.1}h",
            ch.roi_marks_per_hour(),
            ch.estimated_hours
        );
        draw.queue_text(
            &roi_lbl,
            row_r.x + 12.0,
            row_r.y + 32.0,
            10.0,
            p.text_secondary,
        );

        let action_btn = LayoutRect::new(
            row_r.x + row_r.w - btn_w - 12.0,
            row_r.y + 24.0,
            btn_w,
            24.0,
        );
        match ch.status {
            ChapterYieldStatus::Locked => {
                draw.push_btn(
                    action_btn,
                    [p.success[0], p.success[1], p.success[2], 0.20],
                    p.success,
                );
                draw.queue_text_centered_in_rect("🔒 Locked", action_btn, 10.0, p.success);
            }
            ChapterYieldStatus::InProgress | ChapterYieldStatus::NextUp => {
                let is_today = i == goal.today_mission_idx;
                let (bg, tc, lbl) = if is_today {
                    (p.accent_primary, [1.0, 1.0, 1.0, 1.0], "⚡ Today (+16)")
                } else {
                    (p.surface, p.text_primary, "Lock Marks")
                };
                draw.push_btn(action_btn, bg, p.border);
                draw.queue_text_centered_in_rect(lbl, action_btn, 10.0, tc);
            }
            ChapterYieldStatus::StrategicSkip => {
                draw.push_btn(
                    action_btn,
                    [p.text_muted[0], p.text_muted[1], p.text_muted[2], 0.15],
                    p.border,
                );
                draw.queue_text_centered_in_rect("⚡ Skipped", action_btn, 10.0, p.text_muted);
            }
        }
    }
}

pub fn render_stats_content(
    state: &ShellState,
    draw: &mut ClippedDraw,
    area: LayoutRect,
    scroll_y: f32,
) {
    let p = state.theme.palette;
    let goal = &state.exam_goal;
    let (chart_area, score_area) = area.split_top(150.0);

    // Weekly Study Hours Chart
    draw.queue_text(
        "Weekly High-Yield Study Time (Hours)",
        chart_area.x,
        chart_area.y - scroll_y,
        12.5,
        p.text_primary,
    );
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let max_val = state
        .weekly_bars
        .iter()
        .cloned()
        .fold(0.0_f32, f32::max)
        .max(1.0);

    for (i, &hrs) in state.weekly_bars.iter().enumerate() {
        let col = chart_area.child_row(i, 7, 8.0);
        let bar_h = (hrs / max_val * 70.0).max(4.0);
        let bar_r = LayoutRect::new(
            col.x + 4.0,
            chart_area.y + 110.0 - bar_h - scroll_y,
            col.w - 8.0,
            bar_h,
        );
        draw.push_btn(bar_r, p.accent_primary, [0.0; 4]);
        draw.queue_text(
            days[i],
            col.x + 2.0,
            chart_area.y + 118.0 - scroll_y,
            9.5,
            p.text_secondary,
        );
    }

    // ─── Reverse Goal Score Tracker & What-If Simulator ───────────────────────
    let proj = goal.projected_score();
    let score_txt = format!("Projected Score: {} / {} Marks", proj, goal.max_score);
    draw.queue_text(
        &score_txt,
        score_area.x,
        score_area.y + 10.0 - scroll_y,
        15.0,
        p.accent_primary,
    );

    let threshold_lbl = format!(
        "Govt Medical College Cut-off: {}/{} Marks",
        goal.target_score, goal.max_score
    );
    draw.queue_text(
        &threshold_lbl,
        score_area.x,
        score_area.y + 30.0 - scroll_y,
        10.5,
        p.text_secondary,
    );

    let pb = LayoutRect::new(
        score_area.x,
        score_area.y + 48.0 - scroll_y,
        score_area.w - 180.0,
        10.0,
    );
    draw.push_progress_bar(
        pb,
        (proj as f32 / goal.max_score as f32).clamp(0.0, 1.0),
        p.border,
        p.success,
    );

    // Incremental adjustment buttons
    let p5_btn = LayoutRect::new(
        score_area.x + score_area.w - 160.0,
        score_area.y + 38.0 - scroll_y,
        72.0,
        26.0,
    );
    let m5_btn = LayoutRect::new(
        score_area.x + score_area.w - 80.0,
        score_area.y + 38.0 - scroll_y,
        72.0,
        26.0,
    );
    draw.push_btn(p5_btn, p.success, [0.0; 4]);
    draw.queue_text_centered_in_rect("+12 Marks", p5_btn, 10.5, [1.0, 1.0, 1.0, 1.0]);
    draw.push_btn(m5_btn, p.surface_raised, p.border);
    draw.queue_text_centered_in_rect("-12 Marks", m5_btn, 10.5, p.text_primary);

    // Dynamic What-If Simulation Box
    let sim_box_y = score_area.y + 70.0 - scroll_y;
    let sim_r = LayoutRect::new(score_area.x, sim_box_y, score_area.w, 64.0);
    draw.push_panel(sim_r, p.surface_raised, 10.0);

    draw.queue_text(
        "💡 NEUROSCIENCE REVERSE WHAT-IF SIMULATOR:",
        sim_r.x + 12.0,
        sim_r.y + 8.0,
        10.5,
        p.accent_secondary,
    );
    let gap = goal.target_score.saturating_sub(proj);
    let sim_text = if gap > 0 {
        format!("You only need {} more marks to clear the Government Medical College cut-off! Mastering Modern Physics (+16m) and Optics (+12m) guarantees your doctor seat.", gap)
    } else {
        "🎯 CUT-OFF SECURED! Your projected score qualifies you for AIIMS / Top Govt Medical College. Maintain revision cadence.".to_string()
    };
    let fitted = draw.fit_text_to_px(&sim_text, 10.0, sim_r.w - 24.0);
    draw.queue_text(
        &fitted,
        sim_r.x + 12.0,
        sim_r.y + 26.0,
        10.0,
        p.text_primary,
    );
    draw.queue_text(
        "Strategic High-Yield Focus beats 12-hour chaotic brute force.",
        sim_r.x + 12.0,
        sim_r.y + 42.0,
        9.0,
        p.text_muted,
    );
}
