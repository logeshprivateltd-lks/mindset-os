//! MindSet OS — File Vault & Storage Window Renderer (`vault.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use crate::state::{ExplorerViewMode, ShellState};
use mindset_render::{optimal_text_color_for_fill, safe_utf8_prefix};
use mindset_ui::LayoutRect;

pub fn render_file_vault_content(
    state: &ShellState,
    draw: &mut ClippedDraw,
    area: LayoutRect,
    scroll_y: f32,
) {
    let p = state.theme.palette;
    let (top_bar, main_area) = area.split_top(42.0);

    // ── Top Toolbar ────────────────────────────────────────────────────────
    let back_btn = LayoutRect::new(top_bar.x + 4.0, top_bar.y + 6.0, 28.0, 28.0);
    let fwd_btn = LayoutRect::new(top_bar.x + 36.0, top_bar.y + 6.0, 28.0, 28.0);
    let up_btn = LayoutRect::new(top_bar.x + 68.0, top_bar.y + 6.0, 28.0, 28.0);
    draw.push_btn(back_btn, p.surface_raised, p.border);
    draw.queue_text(
        "[<]",
        back_btn.x + 5.0,
        back_btn.y + 6.0,
        10.0,
        p.text_primary,
    );
    draw.push_btn(fwd_btn, p.surface_raised, p.border);
    draw.queue_text(
        "[>]",
        fwd_btn.x + 5.0,
        fwd_btn.y + 6.0,
        10.0,
        p.text_primary,
    );
    draw.push_btn(up_btn, p.surface_raised, p.border);
    draw.queue_text("[^]", up_btn.x + 5.0, up_btn.y + 6.0, 10.0, p.text_primary);

    let path_str = match state.explorer_selected_sidebar {
        0 => "Vault / Home",
        1 => "Vault / Class 12 / Physics",
        2 => "Vault / Class 12 / Chemistry",
        3 => "Vault / Study Notes",
        4 => "Vault / Sovereign Vault",
        _ => "Vault / Trash",
    };
    let breadcrumb_w = (top_bar.w - 380.0).max(120.0);
    let breadcrumb_r = LayoutRect::new(top_bar.x + 102.0, top_bar.y + 6.0, breadcrumb_w, 28.0);
    draw.push_panel(breadcrumb_r, p.surface_raised, 6.0);
    let path_disp = format!("[DIR] {}", path_str);
    draw.queue_text(
        &path_disp,
        breadcrumb_r.x + 10.0,
        breadcrumb_r.y + 6.0,
        11.0,
        p.text_primary,
    );

    let search_r = LayoutRect::new(top_bar.x + top_bar.w - 264.0, top_bar.y + 6.0, 160.0, 28.0);
    draw.push_panel(search_r, p.surface_raised, 6.0);
    let search_disp = if state.explorer_search.is_empty() {
        "Search filter..."
    } else {
        &state.explorer_search
    };
    draw.queue_text(
        search_disp,
        search_r.x + 8.0,
        search_r.y + 6.0,
        10.5,
        if state.explorer_search.is_empty() {
            p.text_secondary
        } else {
            p.text_primary
        },
    );

    let view_toggle_r = LayoutRect::new(top_bar.x + top_bar.w - 96.0, top_bar.y + 6.0, 92.0, 28.0);
    draw.push_btn(view_toggle_r, p.accent_primary, p.accent_primary);
    let view_lbl = if state.explorer_view_mode == ExplorerViewMode::Grid {
        "[List View]"
    } else {
        "[Grid View]"
    };
    draw.queue_text(
        view_lbl,
        view_toggle_r.x + 8.0,
        view_toggle_r.y + 6.0,
        10.5,
        [1.0, 1.0, 1.0, 1.0],
    );

    // ── Left Sidebar ───────────────────────────────────────────────────────
    let s = state.personalization_store.appearance.ui_scale;
    let sidebar_w = (210.0 * s).max(190.0);
    let sidebar_r = LayoutRect::new(main_area.x, main_area.y + 2.0, sidebar_w, main_area.h - 4.0);
    draw.push_panel(
        sidebar_r,
        [p.surface[0], p.surface[1], p.surface[2], 0.40],
        6.0,
    );

    let dirs = [
        ("[HME]", "Home"),
        ("[PHY]", "Class 12 Physics"),
        ("[CHM]", "Chemistry"),
        ("[NTE]", "Study Notes"),
        ("[VLT]", "Sovereign Vault"),
        ("[TRH]", "Trash"),
    ];
    let font_side_badge = (10.0 * s).clamp(9.0, 13.0);
    let font_side_name = (11.0 * s).clamp(10.0, 14.5);
    for (i, (badge, name)) in dirs.iter().enumerate() {
        let item_r = LayoutRect::new(
            sidebar_r.x + 4.0 * s,
            sidebar_r.y + 8.0 * s + i as f32 * (36.0 * s),
            sidebar_r.w - 8.0 * s,
            32.0 * s,
        );
        let is_sel = state.explorer_selected_sidebar == i;
        let fill = if is_sel {
            [0.851, 0.608, 0.149, 1.0]
        } else {
            p.surface_raised
        };
        let tc = if is_sel {
            optimal_text_color_for_fill(fill)
        } else {
            p.text_secondary
        };
        draw.push_btn(
            item_r,
            fill,
            if is_sel {
                [0.851, 0.608, 0.149, 1.0]
            } else {
                p.border
            },
        );
        draw.queue_text(
            badge,
            item_r.x + 8.0 * s,
            item_r.y + 8.0 * s,
            font_side_badge,
            tc,
        );
        let name_x = item_r.x + 48.0 * s;
        draw.queue_text_bounded(
            name,
            name_x,
            item_r.y + 8.0 * s,
            item_r.w - (name_x - item_r.x) - 6.0 * s,
            font_side_name,
            tc,
        );
    }

    let sep_r = LayoutRect::new(
        main_area.x + sidebar_w + 2.0,
        main_area.y + 2.0,
        1.0,
        main_area.h - 4.0,
    );
    draw.push_panel(sep_r, p.border, 0.0);

    // ── Main File View ─────────────────────────────────────────────────────
    let files_r = LayoutRect::new(
        main_area.x + sidebar_w + 10.0,
        main_area.y + 2.0,
        main_area.w - sidebar_w - 12.0,
        main_area.h - 4.0,
    );

    let mut display_files: Vec<(String, usize, usize, &'static str, [f32; 4])> = Vec::new();
    match state.explorer_selected_sidebar {
        0 => {
            display_files.push((
                "Class 12 Physics Notes".to_string(),
                124,
                45800,
                "[DIR]",
                [0.145, 0.388, 0.922, 1.0],
            ));
            display_files.push((
                "Chemistry Formula Cheatsheet".to_string(),
                85,
                32400,
                "[DIR]",
                [0.10, 0.73, 0.51, 1.0],
            ));
            display_files.push((
                "CBSE Model Exam Papers.pdf".to_string(),
                240,
                89200,
                "[PDF]",
                [0.984, 0.749, 0.141, 1.0],
            ));
            for note in state.vault_notes.iter().take(3) {
                display_files.push((
                    note.name.clone(),
                    note.line_count,
                    note.size_bytes as usize,
                    "[MD]",
                    [0.851, 0.608, 0.149, 1.0],
                ));
            }
        }
        1 => {
            display_files.push((
                "Faraday_Law_Induction.md".to_string(),
                92,
                18400,
                "[MD]",
                [0.145, 0.388, 0.922, 1.0],
            ));
            display_files.push((
                "Wave_Optics_Interference.md".to_string(),
                110,
                24500,
                "[MD]",
                [0.145, 0.388, 0.922, 1.0],
            ));
            display_files.push((
                "Semiconductor_Diodes.pdf".to_string(),
                65,
                48200,
                "[PDF]",
                [0.984, 0.749, 0.141, 1.0],
            ));
        }
        2 => {
            display_files.push((
                "Coordination_Compounds.md".to_string(),
                145,
                38100,
                "[MD]",
                [0.10, 0.73, 0.51, 1.0],
            ));
            display_files.push((
                "Electrochemistry_Nernst.md".to_string(),
                88,
                19200,
                "[MD]",
                [0.10, 0.73, 0.51, 1.0],
            ));
            display_files.push((
                "Organic_Reaction_Mechanisms.pdf".to_string(),
                180,
                72000,
                "[PDF]",
                [0.984, 0.749, 0.141, 1.0],
            ));
        }
        3 => {
            for note in &state.vault_notes {
                display_files.push((
                    note.name.clone(),
                    note.line_count,
                    note.size_bytes as usize,
                    "[MD]",
                    [0.851, 0.608, 0.149, 1.0],
                ));
            }
        }
        4 => {
            display_files.push((
                "Encrypted_Seed_Recovery.dat".to_string(),
                32,
                4096,
                "[VLT]",
                [0.851, 0.608, 0.149, 1.0],
            ));
            display_files.push((
                "Personal_Journal_2026.enc".to_string(),
                420,
                95400,
                "[VLT]",
                [0.851, 0.608, 0.149, 1.0],
            ));
        }
        _ => {
            display_files.push((
                "Old_Draft_Calculus_Limits.md".to_string(),
                15,
                3200,
                "[TRH]",
                [0.6, 0.6, 0.6, 1.0],
            ));
        }
    }

    if !state.explorer_search.is_empty() {
        let q = state.explorer_search.to_lowercase();
        display_files.retain(|(name, _, _, _, _)| name.to_lowercase().contains(&q));
    }

    if state.explorer_view_mode == ExplorerViewMode::Grid {
        let card_w = (210.0 * s).max(195.0);
        let card_h = (128.0 * s).max(118.0);
        let gap = 16.0 * s;
        let cols = ((files_r.w - 10.0 * s) / (card_w + gap)).floor().max(1.0) as usize;
        let font_card_name = (11.0 * s).clamp(10.0, 14.5);
        let font_card_meta = (9.5 * s).clamp(8.5, 12.5);
        let font_btn = (9.5 * s).clamp(8.5, 12.5);

        for (i, (name, lines, size_bytes, badge, badge_col)) in display_files.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let card_r = LayoutRect::new(
                files_r.x + 8.0 * s + col as f32 * (card_w + gap),
                files_r.y + 8.0 * s + row as f32 * (card_h + gap) - scroll_y,
                card_w,
                card_h,
            );
            draw.push_panel(card_r, p.surface_raised, 8.0 * s);

            let icon_size = (32.0 * s).clamp(28.0, 40.0);
            let icon_r = LayoutRect::new(
                card_r.x + (card_r.w - icon_size) * 0.5,
                card_r.y + 8.0 * s,
                icon_size,
                icon_size,
            );
            draw.push_squircle_badge(icon_r, *badge_col, badge, [1.0, 1.0, 1.0, 1.0]);

            let (nw, _) = draw.measure_line_px(name, font_card_name);
            let nx = (card_r.x + (card_r.w - nw) * 0.5).max(card_r.x + 6.0 * s);
            draw.queue_text_bounded(
                name,
                nx,
                card_r.y + 8.0 * s + icon_size + 6.0 * s,
                card_r.w - 12.0 * s,
                font_card_name,
                p.text_primary,
            );

            let meta_disp = format!("{:.1} KB • {}L", *size_bytes as f32 / 1024.0, lines);
            let (mw, _) = draw.measure_line_px(&meta_disp, font_card_meta);
            let mx = (card_r.x + (card_r.w - mw) * 0.5).max(card_r.x + 6.0 * s);
            draw.queue_text(
                &meta_disp,
                mx,
                card_r.y + 8.0 * s + icon_size + 24.0 * s,
                font_card_meta,
                p.text_secondary,
            );

            let btn_w = (card_r.w - 24.0 * s) * 0.5;
            let btn_h = 24.0 * s;
            let open_btn = LayoutRect::new(
                card_r.x + 8.0 * s,
                card_r.y + card_r.h - btn_h - 8.0 * s,
                btn_w,
                btn_h,
            );
            let ask_btn = LayoutRect::new(
                card_r.x + 16.0 * s + btn_w,
                card_r.y + card_r.h - btn_h - 8.0 * s,
                btn_w,
                btn_h,
            );

            draw.push_btn(open_btn, p.surface, p.border);
            draw.queue_text_centered_in_rect("Open", open_btn, font_btn, p.accent_primary);

            let ask_tc = optimal_text_color_for_fill(p.accent_primary);
            draw.push_btn(ask_btn, p.accent_primary, p.accent_primary);
            draw.queue_text_centered_in_rect("Ask AI", ask_btn, font_btn, ask_tc);
        }
    } else {
        // List View
        draw.queue_text(
            "File Name",
            files_r.x + 36.0,
            files_r.y + 8.0,
            10.5,
            p.text_secondary,
        );
        draw.queue_text(
            "Size",
            files_r.x + files_r.w - 240.0,
            files_r.y + 8.0,
            10.5,
            p.text_secondary,
        );
        draw.queue_text(
            "Actions",
            files_r.x + files_r.w - 120.0,
            files_r.y + 8.0,
            10.5,
            p.text_secondary,
        );

        for (i, (name, lines, size_bytes, badge, badge_col)) in display_files.iter().enumerate() {
            let row_r = LayoutRect::new(
                files_r.x + 4.0,
                files_r.y + 28.0 + i as f32 * 42.0 - scroll_y,
                files_r.w - 8.0,
                36.0,
            );
            draw.push_panel(row_r, p.surface_raised, 6.0);

            let badge_r = LayoutRect::new(row_r.x + 6.0, row_r.y + 7.0, 24.0, 22.0);
            draw.push_squircle_badge(badge_r, *badge_col, badge, [1.0, 1.0, 1.0, 1.0]);

            let name_disp = safe_utf8_prefix(name, 32);
            draw.queue_text(
                name_disp,
                row_r.x + 36.0,
                row_r.y + 10.0,
                11.0,
                p.text_primary,
            );

            let meta_disp = format!("{:.1} KB • {}L", *size_bytes as f32 / 1024.0, lines);
            draw.queue_text(
                &meta_disp,
                row_r.x + row_r.w - 240.0,
                row_r.y + 10.0,
                10.0,
                p.text_secondary,
            );

            let open_btn = LayoutRect::new(row_r.x + row_r.w - 140.0, row_r.y + 6.0, 64.0, 24.0);
            let ask_btn = LayoutRect::new(row_r.x + row_r.w - 70.0, row_r.y + 6.0, 64.0, 24.0);
            draw.push_btn(open_btn, p.surface, p.border);
            draw.queue_text(
                "Open",
                open_btn.x + 18.0,
                open_btn.y + 5.0,
                9.5,
                p.accent_primary,
            );
            draw.push_btn(ask_btn, p.accent_primary, p.accent_primary);
            draw.queue_text(
                "Ask AI",
                ask_btn.x + 14.0,
                ask_btn.y + 5.0,
                9.5,
                [1.0, 1.0, 1.0, 1.0],
            );
        }
    }
}
