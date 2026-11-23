//! MindSet OS — Study, Maindy AI & Laya AI Window Renderers (`study.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use crate::state::{active_study_topic, ShellState, STUDY_TOPIC_DATA};
use mindset_render::{format_markdown_lines, wrap_text_chars};
use mindset_ui::LayoutRect;

pub fn render_study_reader_content(
    state: &ShellState,
    draw: &mut ClippedDraw,
    area: LayoutRect,
    scroll_y: f32,
) {
    let p = state.theme.palette;

    // Check if session completed (timer reached 0 and was running)
    if state.timer_secs <= 0.0 {
        let card_w = 420.0_f32.min(area.w - 40.0);
        let card_h = 240.0;
        let card_r = LayoutRect::new(
            area.x + (area.w - card_w) * 0.5,
            area.y + 40.0,
            card_w,
            card_h,
        );
        draw.sdf.push(mindset_render::SdfInstance::glass_panel(
            card_r.x,
            card_r.y,
            card_r.w,
            card_r.h,
            16.0,
            p.surface_raised,
            p.border,
        ));

        draw.queue_text_centered_in_rect(
            "Session Complete 🎉",
            LayoutRect::new(card_r.x, card_r.y + 20.0, card_r.w, 24.0),
            17.0,
            p.accent_primary,
        );
        draw.queue_text_centered_in_rect(
            "25 Minutes Focus Logged • Wave Optics",
            LayoutRect::new(card_r.x, card_r.y + 54.0, card_r.w, 20.0),
            13.5,
            p.text_secondary,
        );
        draw.queue_text_centered_in_rect(
            "Mastery: 42% -> 58% (+16%)",
            LayoutRect::new(card_r.x, card_r.y + 80.0, card_r.w, 20.0),
            13.5,
            p.success,
        );

        let back_btn = LayoutRect::new(
            card_r.x + (card_r.w - 180.0) * 0.5,
            card_r.y + 140.0,
            180.0,
            42.0,
        );
        draw.sdf.push(mindset_render::SdfInstance::button(
            back_btn.x,
            back_btn.y,
            back_btn.w,
            back_btn.h,
            10.0,
            p.accent_primary,
            [1.0, 1.0, 1.0, 0.40],
            0.15,
        ));
        draw.queue_text_centered_in_rect("Back to Today", back_btn, 15.5, [0.08, 0.08, 0.08, 1.0]);
        return;
    }

    let (topic_bar, rest) = area.split_top(34.0);

    for (i, topic_item) in STUDY_TOPIC_DATA.iter().enumerate() {
        let tr = topic_bar.child_row(i, STUDY_TOPIC_DATA.len(), 8.0);
        let is_sel = state.study_topic == i;
        let fill = if is_sel {
            p.accent_primary
        } else {
            p.surface_raised
        };
        let tc = if is_sel {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            p.text_primary
        };
        draw.push_btn(tr, fill, p.border);
        draw.queue_text(topic_item.title, tr.x + 8.0, tr.y + 7.0, 11.0, tc);
    }

    let topic = match active_study_topic(&STUDY_TOPIC_DATA, state.study_topic) {
        Some(t) => t,
        None => return,
    };

    let mut y = rest.y + 8.0 - scroll_y;

    // Header Question
    draw.queue_text(topic.question, rest.x, y, 12.5, p.text_secondary);
    y += 24.0;

    // Formula Box
    let f_box = LayoutRect::new(rest.x, y, (rest.w - 16.0).max(40.0), 36.0);
    draw.push_btn(f_box, p.surface_raised, p.border);
    draw.queue_text(
        &format!("Formula: {}", topic.formula),
        f_box.x + 12.0,
        f_box.y + 10.0,
        13.0,
        p.accent_primary,
    );
    y += 44.0;

    // Derivation Steps
    draw.queue_text("Derivation Steps:", rest.x, y, 12.0, p.text_secondary);
    y += 18.0;
    let wrapped_derivation = wrap_text_chars(topic.derivation, 68);
    for line in wrapped_derivation.lines() {
        draw.queue_text(line, rest.x, y, 12.5, p.text_primary);
        y += 18.0;
    }
    y += 8.0;

    // Board Exam Note
    draw.queue_text(topic.exam_note, rest.x, y, 12.0, p.warning);
}

pub fn render_maindy_chat_content(
    state: &ShellState,
    draw: &mut ClippedDraw,
    area: LayoutRect,
    scroll_y: f32,
) {
    let p = state.theme.palette;
    let (header_bar, rest) = area.split_top(42.0);
    let (chat_area, input_bar) = rest.split_top(rest.h - 48.0);

    // Header bar with custom glowing MindSet AI Vector Orb face
    draw.push_maindy_ai_orb(header_bar.x + 18.0, header_bar.y + 18.0);
    draw.queue_text(
        "Maindy AI Study Companion",
        header_bar.x + 42.0,
        header_bar.y + 6.0,
        13.0,
        p.accent_primary,
    );
    draw.queue_text(
        "Local Sovereign Tutor • Active Stream Ready",
        header_bar.x + 42.0,
        header_bar.y + 24.0,
        10.5,
        p.text_secondary,
    );

    let pad_x = 12.0;
    let scrollbar_w = 14.0;
    let body_w = (chat_area.w - pad_x * 2.0 - scrollbar_w).max(80.0);
    let max_chars = ((body_w / 6.8).floor() as usize).max(12);

    let lines = format_markdown_lines(&state.ai_response_text);
    let mut y = chat_area.y + 10.0 - scroll_y;

    for line in lines {
        if y > chat_area.y + chat_area.h {
            break;
        }
        let (font_sz, col) = if line.is_header {
            (14.0, p.accent_primary)
        } else {
            (11.5, p.text_primary)
        };

        let wrapped = wrap_text_chars(&line.text, max_chars);
        for sub in wrapped.lines() {
            if y >= chat_area.y - 10.0 && y <= chat_area.y + chat_area.h + 5.0 {
                let bullet_prefix = if line.is_bullet { "• " } else { "" };
                let full_line = format!("{}{}", bullet_prefix, sub);
                draw.queue_text(&full_line, chat_area.x + pad_x, y, font_sz, col);
            }
            y += if line.is_header { 22.0 } else { 16.5 };
        }
    }

    // Input Bar (Bottom)
    let s = state
        .personalization_store
        .appearance
        .ui_scale
        .clamp(0.80, 1.50);
    let send_btn_w = 78.0 * s;
    let send_btn_h = 32.0 * s;
    let send_btn = LayoutRect::new(
        input_bar.x + input_bar.w - send_btn_w - 8.0 * s,
        input_bar.y + 8.0 * s,
        send_btn_w,
        send_btn_h,
    );
    let inp_r = LayoutRect::new(
        input_bar.x + 8.0 * s,
        input_bar.y + 8.0 * s,
        (send_btn.x - input_bar.x - 16.0 * s).max(40.0),
        send_btn_h,
    );

    draw.push_btn(inp_r, p.surface_raised, p.border);
    let disp = if state.maindy_query_input.text.is_empty() {
        &state.maindy_query_input.placeholder
    } else {
        &state.maindy_query_input.text
    };
    let input_text_max_w = (send_btn.x - inp_r.x - 16.0 * s).max(40.0);
    draw.queue_text_bounded(
        disp,
        inp_r.x + 8.0 * s,
        inp_r.y + (inp_r.h - 11.0 * s) * 0.5,
        input_text_max_w,
        11.0 * s,
        p.text_secondary,
    );

    draw.push_btn(send_btn, p.accent_primary, [0.0; 4]);
    draw.queue_text_centered_in_rect("Ask AI", send_btn, 11.5 * s, [1.0, 1.0, 1.0, 1.0]);
}

pub fn render_laya_chat_content(
    state: &ShellState,
    draw: &mut ClippedDraw,
    area: LayoutRect,
    scroll_y: f32,
) {
    render_maindy_chat_content(state, draw, area, scroll_y);
}
