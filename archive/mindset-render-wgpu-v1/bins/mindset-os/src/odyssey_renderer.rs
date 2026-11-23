//! MindSet OS — Odyssey Journey Map & High-Yield Learning Game Renderer (`odyssey_renderer.rs`)
//! Renders the full-screen interactive liquidmorphic Odyssey Map:
//! - Sinuous level expedition path leading to Target: MBBS Doctor (NEET 2027)
//! - Active node 3-stage mission cockpit (Concept Flash -> PYQ Drill -> Mastery Check)
//! - Resilience Campfire de-escalation interface
//! - Collectible Trophy Vault deck
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use mindset_render::SdfInstance;
use mindset_ui::{LayoutRect, NodeStatus, OdysseyMapState, StageStatus};

/// Click actions emitted by the Odyssey Map interactive renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OdysseyClickAction {
    None,
    CloseMap,
    OpenTrophyDeck,
    CloseTrophyDeck,
    SelectNode(usize),
    EmbarkMission,
    AdvanceStage,
    ResolveCampfire,
}

/// Hit test regions collected during Odyssey Map rendering.
#[derive(Default)]
pub struct OdysseyHitMap {
    pub close_btn: LayoutRect,
    pub trophy_btn: LayoutRect,
    pub embark_btn: LayoutRect,
    pub advance_stage_btn: LayoutRect,
    pub campfire_btn: LayoutRect,
    pub node_buttons: Vec<(usize, LayoutRect)>,
}

impl OdysseyHitMap {
    pub fn test(&self, x: f32, y: f32) -> OdysseyClickAction {
        if self.close_btn.contains(x, y) {
            return OdysseyClickAction::CloseMap;
        }
        if self.trophy_btn.contains(x, y) {
            return OdysseyClickAction::OpenTrophyDeck;
        }
        if self.embark_btn.contains(x, y) {
            return OdysseyClickAction::EmbarkMission;
        }
        if self.advance_stage_btn.contains(x, y) {
            return OdysseyClickAction::AdvanceStage;
        }
        if self.campfire_btn.contains(x, y) {
            return OdysseyClickAction::ResolveCampfire;
        }
        for (idx, rect) in &self.node_buttons {
            if rect.contains(x, y) {
                return OdysseyClickAction::SelectNode(*idx);
            }
        }
        OdysseyClickAction::None
    }
}

/// Renders the full-screen interactive Odyssey Journey Map.
pub fn render_odyssey_map(
    draw: &mut ClippedDraw,
    screen_w: f32,
    screen_h: f32,
    map_state: &OdysseyMapState,
) -> OdysseyHitMap {
    let mut hit_map = OdysseyHitMap::default();

    // 0. Full-screen dimmed backdrop scrim
    draw.sdf.push(SdfInstance::button(
        0.0,
        0.0,
        screen_w,
        screen_h,
        0.0,
        [0.02, 0.04, 0.08, 0.78],
        [0.0; 4],
        0.0,
    ));

    // 1. Centered liquidmorphic modal container
    let modal_w = (screen_w * 0.90).clamp(840.0, 1160.0);
    let modal_h = (screen_h * 0.88).clamp(580.0, 760.0);
    let modal_x = (screen_w - modal_w) * 0.5;
    let modal_y = (screen_h - modal_h) * 0.5;
    let modal_rect = LayoutRect::new(modal_x, modal_y, modal_w, modal_h);

    // Frosted obsidian glass surface
    draw.sdf.push(SdfInstance::glass_panel(
        modal_rect.x,
        modal_rect.y,
        modal_rect.w,
        modal_rect.h,
        24.0,
        [0.07, 0.10, 0.18, 0.94],
        [0.35, 0.65, 1.0, 0.35],
    ));

    // 2. Top Header Bar: North Star Summit Goal & Stats
    let header_h = 68.0;
    let header_r = LayoutRect::new(modal_rect.x, modal_rect.y, modal_rect.w, header_h);
    draw.sdf.push(SdfInstance::glass_panel(
        header_r.x,
        header_r.y,
        header_r.w,
        header_r.h,
        24.0,
        [0.10, 0.15, 0.26, 0.85],
        [1.0, 1.0, 1.0, 0.15],
    ));

    // Summit Badge & Title
    let summit_badge = LayoutRect::new(header_r.x + 20.0, header_r.y + 16.0, 36.0, 36.0);
    draw.push_btn(
        summit_badge,
        [0.98, 0.75, 0.14, 0.25],
        [0.98, 0.75, 0.14, 0.80],
    );
    draw.queue_text_centered_in_rect("🩺", summit_badge, 18.0, [1.0, 1.0, 1.0, 1.0]);

    let title_x = header_r.x + 66.0;
    draw.queue_text(
        &format!(
            "TARGET: {} ({})",
            map_state.target_role.to_uppercase(),
            map_state.target_exam
        ),
        title_x,
        header_r.y + 14.0,
        13.0,
        [0.98, 0.75, 0.14, 1.0],
    );
    draw.queue_text(
        &format!(
            "NEET 2027 Countdown: {} Days Left • Target: {} / {} Marks",
            map_state.days_remaining, map_state.target_score, map_state.max_score
        ),
        title_x,
        header_r.y + 32.0,
        10.5,
        [0.70, 0.78, 0.90, 1.0],
    );

    // Center Stat Pills
    let stat_w = 170.0;
    let stat_h = 36.0;
    let stat_x = modal_rect.x + (modal_rect.w - stat_w) * 0.48;
    let stat_r = LayoutRect::new(stat_x, header_r.y + 16.0, stat_w, stat_h);
    draw.push_btn(stat_r, [0.12, 0.18, 0.30, 0.65], [0.25, 0.45, 0.85, 0.40]);
    draw.queue_text_centered_in_rect(
        &format!(
            "Projected: {} / {} m",
            map_state.projected_score, map_state.max_score
        ),
        stat_r,
        10.5,
        [0.38, 0.82, 1.0, 1.0],
    );

    // Right action buttons: [🏆 Trophy Vault] & [✕ Return]
    let close_btn_w = 84.0;
    let close_r = LayoutRect::new(
        header_r.x + header_r.w - close_btn_w - 18.0,
        header_r.y + 18.0,
        close_btn_w,
        32.0,
    );
    draw.push_btn(close_r, [0.85, 0.20, 0.20, 0.20], [0.85, 0.20, 0.20, 0.60]);
    draw.queue_text_centered_in_rect("✕ Return", close_r, 11.0, [1.0, 0.60, 0.60, 1.0]);
    hit_map.close_btn = close_r;

    let trophy_w = 140.0;
    let trophy_r = LayoutRect::new(
        close_r.x - trophy_w - 12.0,
        header_r.y + 18.0,
        trophy_w,
        32.0,
    );
    draw.push_btn(trophy_r, [0.98, 0.75, 0.14, 0.20], [0.98, 0.75, 0.14, 0.60]);
    draw.queue_text_centered_in_rect(
        &format!("🏆 Trophy Vault ({})", map_state.trophy_deck.len()),
        trophy_r,
        10.5,
        [0.98, 0.75, 0.14, 1.0],
    );
    hit_map.trophy_btn = trophy_r;

    // 3. Middle Canvas: Winding Expedition Path & Level Nodes
    let canvas_top = header_r.y + header_h + 10.0;
    let bottom_panel_h = 196.0;
    let canvas_h = modal_rect.h - header_h - bottom_panel_h - 20.0;
    let canvas_r = LayoutRect::new(
        modal_rect.x + 16.0,
        canvas_top,
        modal_rect.w - 32.0,
        canvas_h,
    );

    // Background terrain grid
    draw.sdf.push(SdfInstance::glass_panel(
        canvas_r.x,
        canvas_r.y,
        canvas_r.w,
        canvas_r.h,
        16.0,
        [0.05, 0.07, 0.13, 0.60],
        [1.0, 1.0, 1.0, 0.08],
    ));

    // Render path trail segments connecting nodes
    for i in 0..map_state.nodes.len().saturating_sub(1) {
        let n1 = &map_state.nodes[i];
        let n2 = &map_state.nodes[i + 1];

        let p1x = canvas_r.x + n1.norm_x * (canvas_r.w - 80.0) + 40.0;
        let p1y = canvas_r.y + n1.norm_y * (canvas_r.h - 60.0) + 30.0;
        let p2x = canvas_r.x + n2.norm_x * (canvas_r.w - 80.0) + 40.0;
        let p2y = canvas_r.y + n2.norm_y * (canvas_r.h - 60.0) + 30.0;

        let seg_center_x = (p1x + p2x) * 0.5;
        let seg_center_y = (p1y + p2y) * 0.5;
        let dx = p2x - p1x;
        let dy = p2y - p1y;
        let len = (dx * dx + dy * dy).sqrt();

        // Path segment glow
        let path_col = if n1.status == NodeStatus::Completed && n2.status == NodeStatus::Completed {
            [0.10, 0.85, 0.55, 0.55] // Emerald trail
        } else if n1.status == NodeStatus::Completed && n2.status == NodeStatus::ActiveToday {
            [0.98, 0.75, 0.14, 0.70] // Golden amber active trail
        } else {
            [0.25, 0.35, 0.50, 0.25] // Muted slate trail
        };

        draw.sdf.push(SdfInstance::button(
            seg_center_x - len * 0.5,
            seg_center_y - 2.0,
            len,
            4.0,
            2.0,
            path_col,
            [0.0; 4],
            0.0,
        ));
    }

    // Render Level Nodes along the path
    for (idx, node) in map_state.nodes.iter().enumerate() {
        let nx = canvas_r.x + node.norm_x * (canvas_r.w - 80.0) + 40.0;
        let ny = canvas_r.y + node.norm_y * (canvas_r.h - 60.0) + 30.0;
        let node_size =
            if node.status == NodeStatus::ActiveToday || node.status == NodeStatus::Campfire {
                44.0
            } else {
                34.0
            };
        let node_r = LayoutRect::new(
            nx - node_size * 0.5,
            ny - node_size * 0.5,
            node_size,
            node_size,
        );

        hit_map.node_buttons.push((idx, node_r));

        let is_selected = map_state.selected_node_idx == Some(idx);

        match node.status {
            NodeStatus::Completed => {
                draw.push_btn(node_r, [0.08, 0.65, 0.40, 0.85], [0.15, 0.90, 0.55, 0.95]);
                let lvl_str = format!("{}", node.level);
                draw.queue_text_centered_in_rect(&lvl_str, node_r, 11.0, [1.0, 1.0, 1.0, 1.0]);
            }
            NodeStatus::ActiveToday => {
                // Pulsing solar amber beacon
                draw.sdf.push(SdfInstance::button(
                    node_r.x - 4.0,
                    node_r.y - 4.0,
                    node_r.w + 8.0,
                    node_r.h + 8.0,
                    (node_size + 8.0) * 0.5,
                    [0.98, 0.75, 0.14, 0.30],
                    [0.98, 0.75, 0.14, 0.80],
                    0.10,
                ));
                draw.push_btn(node_r, [0.98, 0.75, 0.14, 0.90], [1.0, 1.0, 1.0, 0.90]);
                draw.queue_text_centered_in_rect("👨‍⚕️", node_r, 15.0, [1.0, 1.0, 1.0, 1.0]);

                // Active Badge Tag above node
                let tag_r =
                    LayoutRect::new(node_r.x - 24.0, node_r.y - 20.0, node_r.w + 48.0, 16.0);
                draw.push_btn(tag_r, [0.10, 0.15, 0.28, 0.90], [0.98, 0.75, 0.14, 0.80]);
                draw.queue_text_centered_in_rect(
                    "🎯 ACTIVE TODAY",
                    tag_r,
                    8.5,
                    [0.98, 0.75, 0.14, 1.0],
                );
            }
            NodeStatus::Campfire => {
                // Warm campfire recovery node
                draw.sdf.push(SdfInstance::button(
                    node_r.x - 4.0,
                    node_r.y - 4.0,
                    node_r.w + 8.0,
                    node_r.h + 8.0,
                    (node_size + 8.0) * 0.5,
                    [0.90, 0.40, 0.10, 0.35],
                    [0.95, 0.50, 0.15, 0.90],
                    0.10,
                ));
                draw.push_btn(node_r, [0.85, 0.35, 0.10, 0.90], [1.0, 0.70, 0.30, 0.90]);
                draw.queue_text_centered_in_rect("🔥", node_r, 16.0, [1.0, 1.0, 1.0, 1.0]);

                let tag_r =
                    LayoutRect::new(node_r.x - 30.0, node_r.y - 20.0, node_r.w + 60.0, 16.0);
                draw.push_btn(tag_r, [0.15, 0.10, 0.08, 0.90], [0.95, 0.50, 0.15, 0.80]);
                draw.queue_text_centered_in_rect(
                    "🔥 CAMPFIRE PITSTOP",
                    tag_r,
                    8.5,
                    [1.0, 0.70, 0.30, 1.0],
                );
            }
            NodeStatus::Locked => {
                let border_col = if is_selected {
                    [0.38, 0.82, 1.0, 0.80]
                } else {
                    [0.25, 0.35, 0.50, 0.40]
                };
                draw.push_btn(node_r, [0.10, 0.14, 0.22, 0.70], border_col);
                let lvl_str = format!("{}", node.level);
                draw.queue_text_centered_in_rect(&lvl_str, node_r, 10.0, [0.55, 0.65, 0.80, 1.0]);
            }
        }

        // Mini chapter tag below node
        let cap_y = node_r.y + node_r.h + 4.0;
        let cap_text = draw.fit_text_to_px(&node.chapter_title, 9.0, 100.0);
        draw.queue_text(
            &cap_text,
            node_r.x - 24.0,
            cap_y,
            9.0,
            [0.75, 0.82, 0.92, 1.0],
        );
    }

    // 4. Bottom Active Mission Command Deck (3-Stage Mission or Campfire)
    let bottom_y = canvas_top + canvas_h + 10.0;
    let bottom_r = LayoutRect::new(
        modal_rect.x + 16.0,
        bottom_y,
        modal_rect.w - 32.0,
        bottom_panel_h,
    );

    if map_state.campfire.active {
        // ─── RESILIENCE CAMPFIRE VIEW ───
        draw.sdf.push(SdfInstance::glass_panel(
            bottom_r.x,
            bottom_r.y,
            bottom_r.w,
            bottom_r.h,
            18.0,
            [0.18, 0.10, 0.08, 0.92],
            [0.95, 0.50, 0.15, 0.60],
        ));

        let cf_y = bottom_r.y + 16.0;
        draw.queue_text(
            "🔥 RESILIENCE CAMPFIRE — COGNITIVE LOAD REDUCTION",
            bottom_r.x + 20.0,
            cf_y,
            13.0,
            [1.0, 0.70, 0.30, 1.0],
        );
        draw.queue_text(
            &map_state.campfire.supportive_message,
            bottom_r.x + 20.0,
            cf_y + 24.0,
            11.0,
            [0.90, 0.85, 0.80, 1.0],
        );

        // 1 Single 15-Minute Micro-Task Box
        let task_r = LayoutRect::new(bottom_r.x + 20.0, cf_y + 54.0, bottom_r.w - 40.0, 52.0);
        draw.push_btn(task_r, [0.25, 0.14, 0.10, 0.80], [0.95, 0.50, 0.15, 0.50]);
        draw.queue_text(
            "🎯 Today's De-escalation Micro-Mission:",
            task_r.x + 16.0,
            task_r.y + 12.0,
            10.5,
            [1.0, 0.70, 0.30, 1.0],
        );
        draw.queue_text(
            &format!(
                "{} • Just 15 minutes to earn +{} Points & resume expedition!",
                map_state.campfire.micro_task_title, map_state.campfire.points_needed
            ),
            task_r.x + 16.0,
            task_r.y + 30.0,
            11.5,
            [1.0, 1.0, 1.0, 1.0],
        );

        // Action CTA
        let act_btn_w = 280.0;
        let act_btn_r = LayoutRect::new(
            bottom_r.x + (bottom_r.w - act_btn_w) * 0.5,
            task_r.y + task_r.h + 16.0,
            act_btn_w,
            36.0,
        );
        draw.push_btn(act_btn_r, [0.95, 0.50, 0.15, 0.90], [1.0, 0.80, 0.50, 1.0]);
        draw.queue_text_centered_in_rect(
            "✨ Complete 15m Pitstop & Resume",
            act_btn_r,
            11.5,
            [0.10, 0.05, 0.02, 1.0],
        );
        hit_map.campfire_btn = act_btn_r;
    } else {
        // ─── 3-STAGE HIGH-YIELD MISSION VIEW ───
        draw.sdf.push(SdfInstance::glass_panel(
            bottom_r.x,
            bottom_r.y,
            bottom_r.w,
            bottom_r.h,
            18.0,
            [0.08, 0.12, 0.22, 0.92],
            [0.35, 0.65, 1.0, 0.35],
        ));

        let sel_idx = map_state.selected_node_idx.unwrap_or(0);
        let node = map_state.nodes.get(sel_idx).unwrap_or(&map_state.nodes[0]);

        let deck_y = bottom_r.y + 14.0;
        draw.queue_text(
            &format!(
                "⚡ LEVEL {}: {} ({})",
                node.level, node.chapter_title, node.subject
            ),
            bottom_r.x + 20.0,
            deck_y,
            12.5,
            [0.98, 0.75, 0.14, 1.0],
        );
        draw.queue_text(
            &format!(
                "🔒 Guaranteed Exam Yield: +{} Marks locked upon mission completion",
                node.marks_yield
            ),
            bottom_r.x + 20.0,
            deck_y + 20.0,
            10.0,
            [0.65, 0.75, 0.88, 1.0],
        );

        // 3-Stage Progress Bar & Cards
        let stages_y = deck_y + 44.0;
        let stage_gap = 12.0;
        let total_stages_w = bottom_r.w - 40.0 - 180.0; // Reserve 180px on right for CTA
        let stage_card_w = (total_stages_w - stage_gap * 2.0) / 3.0;
        let stage_card_h = 74.0;

        let stages = [
            (&node.mission.stage1_concept, 1),
            (&node.mission.stage2_drill, 2),
            (&node.mission.stage3_mastery, 3),
        ];

        for (i, (stage, num)) in stages.iter().enumerate() {
            let sc_x = bottom_r.x + 20.0 + i as f32 * (stage_card_w + stage_gap);
            let sc_r = LayoutRect::new(sc_x, stages_y, stage_card_w, stage_card_h);

            let (bg_col, border_col, status_text, status_col) = match stage.status {
                StageStatus::Completed => (
                    [0.06, 0.40, 0.25, 0.75],
                    [0.10, 0.85, 0.55, 0.80],
                    "✓ COMPLETED",
                    [0.10, 0.85, 0.55, 1.0],
                ),
                StageStatus::InProgress => (
                    [0.35, 0.25, 0.05, 0.85],
                    [0.98, 0.75, 0.14, 0.90],
                    "▶ IN PROGRESS",
                    [0.98, 0.75, 0.14, 1.0],
                ),
                StageStatus::Pending => (
                    [0.08, 0.12, 0.20, 0.60],
                    [0.25, 0.35, 0.50, 0.35],
                    "PENDING",
                    [0.55, 0.65, 0.80, 1.0],
                ),
            };

            draw.push_btn(sc_r, bg_col, border_col);
            draw.queue_text(
                &format!("Stage {}: {}m", num, stage.duration_mins),
                sc_r.x + 10.0,
                sc_r.y + 10.0,
                9.5,
                [0.70, 0.80, 0.95, 1.0],
            );
            draw.queue_text(
                &stage.name,
                sc_r.x + 10.0,
                sc_r.y + 26.0,
                10.0,
                [1.0, 1.0, 1.0, 1.0],
            );
            draw.queue_text(status_text, sc_r.x + 10.0, sc_r.y + 48.0, 9.0, status_col);
        }

        // Right Action Column: [▶ Complete Stage] & [🚀 Embark Mission]
        let cta_x = bottom_r.x + bottom_r.w - 180.0;
        let cta_y = stages_y;

        let advance_btn = LayoutRect::new(cta_x, cta_y, 160.0, 32.0);
        draw.push_btn(
            advance_btn,
            [0.15, 0.45, 0.85, 0.85],
            [0.38, 0.82, 1.0, 0.90],
        );
        draw.queue_text_centered_in_rect(
            "▶ Complete Stage",
            advance_btn,
            10.5,
            [1.0, 1.0, 1.0, 1.0],
        );
        hit_map.advance_stage_btn = advance_btn;

        let embark_btn = LayoutRect::new(cta_x, cta_y + 40.0, 160.0, 34.0);
        draw.push_btn(embark_btn, [0.98, 0.75, 0.14, 0.90], [1.0, 0.85, 0.35, 1.0]);
        draw.queue_text_centered_in_rect(
            "🚀 Embark in OS",
            embark_btn,
            11.0,
            [0.10, 0.08, 0.02, 1.0],
        );
        hit_map.embark_btn = embark_btn;

        // Bottom sub-caption
        let prog_lbl = format!("Mission Progress: {} / 3 Stages Done • Completing unlocks next node and mints Trophy Card.", node.mission.completed_count());
        draw.queue_text(
            &prog_lbl,
            bottom_r.x + 20.0,
            bottom_r.y + bottom_r.h - 18.0,
            9.0,
            [0.55, 0.65, 0.80, 1.0],
        );
    }

    hit_map
}

/// Renders the Collectible Trophy Vault Deck (unlocked flashcards).
pub fn render_trophy_deck(
    draw: &mut ClippedDraw,
    screen_w: f32,
    screen_h: f32,
    map_state: &OdysseyMapState,
) -> LayoutRect {
    // 0. Dimmed backdrop
    draw.sdf.push(SdfInstance::button(
        0.0,
        0.0,
        screen_w,
        screen_h,
        0.0,
        [0.02, 0.04, 0.08, 0.82],
        [0.0; 4],
        0.0,
    ));

    // Centered modal
    let modal_w = (screen_w * 0.85).clamp(780.0, 1040.0);
    let modal_h = (screen_h * 0.82).clamp(520.0, 680.0);
    let modal_x = (screen_w - modal_w) * 0.5;
    let modal_y = (screen_h - modal_h) * 0.5;
    let modal_rect = LayoutRect::new(modal_x, modal_y, modal_w, modal_h);

    draw.sdf.push(SdfInstance::glass_panel(
        modal_rect.x,
        modal_rect.y,
        modal_rect.w,
        modal_rect.h,
        24.0,
        [0.08, 0.11, 0.20, 0.95],
        [0.98, 0.75, 0.14, 0.50],
    ));

    // Header
    let mut y = modal_rect.y + 20.0;
    draw.queue_text(
        "🏆 TROPHY VAULT — COLLECTIBLE MASTERY DECK",
        modal_rect.x + 24.0,
        y,
        14.0,
        [0.98, 0.75, 0.14, 1.0],
    );
    draw.queue_text(
        &format!(
            "{} Mastered Cards • High-Yield Revision Deck for NEET 2027",
            map_state.trophy_deck.len()
        ),
        modal_rect.x + 24.0,
        y + 22.0,
        10.5,
        [0.70, 0.78, 0.90, 1.0],
    );

    // Close Button
    let close_btn = LayoutRect::new(modal_rect.x + modal_rect.w - 94.0, y, 70.0, 28.0);
    draw.push_btn(
        close_btn,
        [0.85, 0.20, 0.20, 0.25],
        [0.85, 0.20, 0.20, 0.65],
    );
    draw.queue_text_centered_in_rect("✕ Close", close_btn, 10.5, [1.0, 0.65, 0.65, 1.0]);

    // Cards Grid
    y += 56.0;
    let card_w = (modal_rect.w - 48.0 - 24.0) / 3.0;
    let card_h = 168.0;

    for (i, card) in map_state.trophy_deck.iter().take(6).enumerate() {
        let col = i % 3;
        let row = i / 3;
        let cx = modal_rect.x + 24.0 + col as f32 * (card_w + 12.0);
        let cy = y + row as f32 * (card_h + 12.0);
        let cr = LayoutRect::new(cx, cy, card_w, card_h);

        let subj_color = match card.subject.as_str() {
            "Physics" => [0.20, 0.60, 0.95, 0.85],
            "Chemistry" => [0.65, 0.35, 0.95, 0.85],
            _ => [0.10, 0.85, 0.55, 0.85],
        };

        // Card body
        draw.sdf.push(SdfInstance::glass_panel(
            cr.x,
            cr.y,
            cr.w,
            cr.h,
            14.0,
            [0.10, 0.14, 0.24, 0.85],
            subj_color,
        ));

        // Subject Tag
        draw.queue_text(
            &format!("Card #{}: {}", card.id, card.subject),
            cr.x + 12.0,
            cr.y + 12.0,
            9.5,
            subj_color,
        );
        draw.queue_text(
            &format!("+{} Marks", card.marks_yield),
            cr.x + cr.w - 60.0,
            cr.y + 12.0,
            9.0,
            [0.98, 0.75, 0.14, 1.0],
        );

        // Title
        let fit_title = draw.fit_text_to_px(&card.title, 11.0, cr.w - 24.0);
        draw.queue_text(
            &fit_title,
            cr.x + 12.0,
            cr.y + 28.0,
            11.0,
            [1.0, 1.0, 1.0, 1.0],
        );

        // Formula Box
        let fb_r = LayoutRect::new(cr.x + 10.0, cr.y + 48.0, cr.w - 20.0, 52.0);
        draw.push_btn(fb_r, [0.06, 0.08, 0.14, 0.80], [1.0, 1.0, 1.0, 0.10]);
        draw.queue_text_bounded(
            &card.formula_summary,
            fb_r.x + 8.0,
            fb_r.y + 6.0,
            fb_r.w - 16.0,
            9.0,
            [0.98, 0.75, 0.14, 1.0],
        );

        // Pro-tip
        let pt_y = cr.y + 108.0;
        draw.queue_text(
            "💡 Pro-Tip:",
            cr.x + 12.0,
            pt_y,
            9.0,
            [0.40, 0.85, 1.0, 1.0],
        );
        draw.queue_text_bounded(
            &card.pro_tip,
            cr.x + 12.0,
            pt_y + 14.0,
            cr.w - 24.0,
            8.5,
            [0.70, 0.80, 0.90, 1.0],
        );
    }

    close_btn
}
