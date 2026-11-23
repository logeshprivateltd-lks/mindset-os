//! MindSet Search Browser & Internet Shield Renderer
//! Multi-screen browser UI, Arc sidebar, Omnibox, Shield Dashboard,
//! YouTube screen-time counter, and 7-stage troubleshooter.

use crate::draw_helpers::ClippedDraw;
use crate::state::{BrowserPageKind, ShellState};
use mindset_net::{NetRepairAction, StageStatus};
use mindset_ui::layout::LayoutRect;
use mindset_ui::widgets::browser::*;

fn format_seconds(secs: u32) -> String {
    let m = secs / 60;
    let s = secs % 60;
    format!("{:02}:{:02}", m, s)
}

fn render_browser_screen_new_tab(
    state: &ShellState,
    draw: &mut ClippedDraw,
    viewport: LayoutRect,
    scroll_y: f32,
    s: f32,
) {
    let p = state.theme.palette;
    let mut y = viewport.y + 16.0 - scroll_y;

    // 1. HERO BRANDING & IDENTITY
    let logo_r = LayoutRect::new(viewport.x + (viewport.w - 44.0) * 0.5, y, 44.0, 44.0);
    draw.push_squircle_badge(logo_r, BROWSER_SOLAR_AMBER, "🌐", [0.05, 0.06, 0.08, 1.0]);
    y += 50.0;

    let title_r = LayoutRect::new(viewport.x + 10.0, y, viewport.w - 20.0, 24.0);
    draw.queue_text_centered_in_rect(
        "MINDSET BROWSER",
        title_r,
        16.0 * s.clamp(0.9, 1.2),
        BROWSER_SOLAR_AMBER,
    );
    y += 26.0;

    let sub_r = LayoutRect::new(viewport.x + 10.0, y, viewport.w - 20.0, 18.0);
    draw.queue_text_centered_in_rect(
        "Sovereign Academic Workspace • Zero Telemetry • Deep Focus",
        sub_r,
        11.0,
        BROWSER_TEXT_SEC,
    );
    y += 28.0;

    // 2. CENTRAL FLOATING SEARCH / OMNIBOX BAR
    let search_w = (viewport.w * 0.76).clamp(340.0, 680.0);
    let search_x = viewport.x + (viewport.w - search_w) * 0.5;
    let search_pill = LayoutRect::new(search_x, y, search_w, 42.0);
    draw.push_btn(search_pill, BROWSER_SURFACE_SEC, BROWSER_BORDER_FOCUS);

    let search_txt = if state.search_input.text.is_empty() {
        "Search encyclopedia, academic papers, notes, or ask Maindy AI..."
    } else {
        &state.search_input.text
    };
    let search_tc = if state.search_input.text.is_empty() {
        BROWSER_TEXT_SEC
    } else {
        BROWSER_TEXT_PRI
    };
    draw.queue_text(
        "🔍",
        search_pill.x + 14.0,
        search_pill.y + 13.0,
        14.0,
        BROWSER_SOLAR_AMBER,
    );
    draw.queue_text_bounded(
        search_txt,
        search_pill.x + 40.0,
        search_pill.y + 14.0,
        search_pill.w - 180.0,
        11.5,
        search_tc,
    );

    // Action buttons inside search pill
    let btn_search_r = LayoutRect::new(
        search_pill.x + search_pill.w - 146.0,
        search_pill.y + 6.0,
        138.0,
        30.0,
    );
    draw.push_btn(btn_search_r, BROWSER_SOLAR_AMBER, BROWSER_SOLAR_AMBER);
    draw.queue_text_centered_in_rect(
        "Search / Browse",
        btn_search_r,
        10.5,
        [0.05, 0.06, 0.08, 1.0],
    );
    y += 50.0;

    // Search Engine Switcher Pills
    let eng_w = 110.0;
    let eng_gap = 8.0;
    let eng_total = 4.0 * eng_w + 3.0 * eng_gap;
    let eng_start_x = viewport.x + (viewport.w - eng_total) * 0.5;
    let engines = [
        "🏛️ Academic Wiki",
        "📄 arXiv Preprints",
        "📚 Wikipedia STEM",
        "🦆 DuckDuckGo",
    ];
    for (i, eng) in engines.iter().enumerate() {
        let er = LayoutRect::new(eng_start_x + i as f32 * (eng_w + eng_gap), y, eng_w, 24.0);
        let is_sel = i == 0;
        let fill = if is_sel {
            BROWSER_AMBER_MUTED
        } else {
            BROWSER_SURFACE_SEC
        };
        let border = if is_sel {
            BROWSER_SOLAR_AMBER
        } else {
            BROWSER_BORDER
        };
        draw.push_btn(er, fill, border);
        let tc = if is_sel {
            BROWSER_SOLAR_AMBER
        } else {
            BROWSER_TEXT_SEC
        };
        draw.queue_text_centered_in_rect(eng, er, 9.5, tc);
    }
    y += 36.0;

    // Blocked Banner (if active)
    if let Some(ref reason) = state.browser_blocked_reason {
        let banner_r = LayoutRect::new(viewport.x + 12.0, y, viewport.w - 24.0, 40.0);
        draw.push_panel(banner_r, [p.error[0], p.error[1], p.error[2], 0.18], 8.0);
        let badge_r = LayoutRect::new(banner_r.x + 8.0, banner_r.y + 7.0, 140.0, 26.0);
        draw.push_labeled_btn(
            badge_r,
            "[BLOCKED BY SHIELD]",
            9.5,
            [p.error[0], p.error[1], p.error[2], 0.85],
            p.error,
        );

        let dis_btn = LayoutRect::new(banner_r.x + banner_r.w - 74.0, banner_r.y + 8.0, 66.0, 24.0);
        draw.push_labeled_btn(
            dis_btn,
            "Dismiss",
            10.0,
            BROWSER_SURFACE_SEC,
            BROWSER_BORDER,
        );

        let max_msg_w = (dis_btn.x - (badge_r.x + badge_r.w + 12.0) - 8.0).max(40.0);
        draw.queue_text_bounded(
            reason,
            badge_r.x + badge_r.w + 10.0,
            banner_r.y + 12.0,
            max_msg_w,
            11.0,
            p.text_primary,
        );
        y += 48.0;
    }

    // 3. QUICK ACCESS PORTALS GRID (3 Columns x 2 Rows)
    let qa_hdr = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 18.0);
    draw.queue_text_bounded(
        "QUICK ACCESS ACADEMIC PORTALS",
        qa_hdr.x,
        qa_hdr.y,
        qa_hdr.w,
        10.5,
        BROWSER_SOLAR_AMBER,
    );
    y += 22.0;

    let portal_cards = [
        (
            "🏛️ NPTEL (IIT & IISc)",
            "Engineering Physics, Math & CS Lectures",
        ),
        (
            "🎓 Khan Academy",
            "Interactive STEM Mastery & Calculus Paths",
        ),
        (
            "📚 Wikipedia STEM",
            "Verified Peer-Reviewed Science Articles",
        ),
        (
            "📺 YouTube Study",
            "Distraction-Free MIT & NPTEL Video Feeds",
        ),
        (
            "📄 arXiv Preprints",
            "Quantum Physics & Machine Learning Research",
        ),
        (
            "🔬 MIT OpenCourseWare",
            "Electricity & Magnetism, Classical Mechanics",
        ),
    ];

    let cols = 3.0;
    let gap = 10.0;
    let card_w = (viewport.w - 32.0 - (cols - 1.0) * gap) / cols;
    let card_h = 56.0;

    for (idx, (p_title, p_desc)) in portal_cards.iter().enumerate() {
        let col = (idx % 3) as f32;
        let row = (idx / 3) as f32;
        let cx = viewport.x + 16.0 + col * (card_w + gap);
        let cy = y + row * (card_h + gap);
        let card_r = LayoutRect::new(cx, cy, card_w, card_h);

        draw.push_btn(card_r, BROWSER_SURFACE_SEC, BROWSER_BORDER);
        draw.queue_text_bounded(
            p_title,
            card_r.x + 12.0,
            card_r.y + 10.0,
            card_r.w - 24.0,
            11.5,
            BROWSER_TEXT_PRI,
        );
        draw.queue_text_bounded(
            p_desc,
            card_r.x + 12.0,
            card_r.y + 32.0,
            card_r.w - 24.0,
            10.0,
            BROWSER_TEXT_SEC,
        );
    }
    y += 2.0 * card_h + gap + 16.0;

    // 4. RECENT WEBSITES & STUDY HISTORY (3 Columns)
    let rec_hdr = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 18.0);
    draw.queue_text_bounded(
        "RECENT ACADEMIC SESSIONS",
        rec_hdr.x,
        rec_hdr.y,
        rec_hdr.w,
        10.5,
        BROWSER_SOLAR_AMBER,
    );
    y += 22.0;

    let recents = [
        (
            "📺 MIT 8.02 Electricity & Magnetism",
            "youtube.com • 15m ago • Video Lecture",
            "▶ 1080p",
        ),
        (
            "🏛️ NPTEL Wave Optics: Huygens Principle",
            "nptel.ac.in • 42m ago • Study Notes",
            "🏛️ Course",
        ),
        (
            "📚 Wikipedia STEM: Wave Interference",
            "en.wikipedia.org • 1h ago • Reader Mode",
            "📖 Read",
        ),
    ];

    let rcard_w = (viewport.w - 32.0 - 2.0 * gap) / 3.0;
    let rcard_h = 62.0;
    for (idx, (rtitle, rsub, rbadge)) in recents.iter().enumerate() {
        let rc_x = viewport.x + 16.0 + idx as f32 * (rcard_w + gap);
        let rc_r = LayoutRect::new(rc_x, y, rcard_w, rcard_h);
        draw.push_btn(rc_r, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        let badge_r = LayoutRect::new(rc_r.x + rc_r.w - 68.0, rc_r.y + 10.0, 58.0, 20.0);
        draw.push_btn(badge_r, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect(rbadge, badge_r, 9.0, BROWSER_SOLAR_AMBER);

        draw.queue_text_bounded(
            rtitle,
            rc_r.x + 12.0,
            rc_r.y + 10.0,
            rc_r.w - 84.0,
            11.0,
            BROWSER_TEXT_PRI,
        );
        draw.queue_text_bounded(
            rsub,
            rc_r.x + 12.0,
            rc_r.y + 34.0,
            rc_r.w - 24.0,
            10.0,
            BROWSER_TEXT_SEC,
        );
    }
    y += rcard_h + 16.0;

    // 5. READING LIST PREVIEW & DAILY FOCUS QUOTE (2 Columns)
    let half_w = (viewport.w - 32.0 - gap) * 0.5;

    // Left: Reading List Preview Card
    let rl_card = LayoutRect::new(viewport.x + 16.0, y, half_w, 82.0);
    draw.push_btn(rl_card, BROWSER_SURFACE_SEC, BROWSER_BORDER);
    draw.queue_text(
        "📖 OFFLINE READING LIST",
        rl_card.x + 12.0,
        rl_card.y + 10.0,
        10.5,
        BROWSER_SOLAR_AMBER,
    );
    draw.queue_text_bounded(
        "Huygens-Fresnel Principle: Wavefront Superposition",
        rl_card.x + 12.0,
        rl_card.y + 30.0,
        rl_card.w - 24.0,
        11.0,
        BROWSER_TEXT_PRI,
    );
    draw.queue_split_row(
        "8 min read • 85% completed",
        "Cached Offline",
        LayoutRect::new(rl_card.x + 12.0, rl_card.y + 54.0, rl_card.w - 24.0, 18.0),
        10.0,
        BROWSER_TEXT_SEC,
        BROWSER_EMERALD,
    );

    // Right: Daily Focus Inspiration Quote Card
    let quote_card = LayoutRect::new(viewport.x + 16.0 + half_w + gap, y, half_w, 82.0);
    draw.push_btn(quote_card, BROWSER_SURFACE_SEC, BROWSER_BORDER);
    draw.queue_text(
        "✨ SOVEREIGN FOCUS PRINCIPLE",
        quote_card.x + 12.0,
        quote_card.y + 10.0,
        10.5,
        BROWSER_SOLAR_AMBER,
    );
    draw.queue_text_bounded(
        "\"Study without distraction is the gateway to sovereign intellect.\"",
        quote_card.x + 12.0,
        quote_card.y + 30.0,
        quote_card.w - 24.0,
        11.0,
        BROWSER_TEXT_PRI,
    );
    draw.queue_text_bounded(
        "— MindSet Academic Protocol • Zero Notifications",
        quote_card.x + 12.0,
        quote_card.y + 54.0,
        quote_card.w - 24.0,
        10.0,
        BROWSER_TEXT_SEC,
    );
    y += 82.0 + 16.0;

    // 6. HIGH-CONTRAST PRIVACY & SOVEREIGN STATUS STRIP
    let strip_r = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 36.0);
    draw.push_panel(strip_r, BROWSER_GLASS_RAISED, 6.0);
    let col_w = (strip_r.w - 24.0) / 4.0;
    let c1 = LayoutRect::new(strip_r.x + 8.0, strip_r.y + 8.0, col_w, 20.0);
    let c2 = LayoutRect::new(strip_r.x + 8.0 + col_w + 4.0, strip_r.y + 8.0, col_w, 20.0);
    let c3 = LayoutRect::new(
        strip_r.x + 8.0 + (col_w + 4.0) * 2.0,
        strip_r.y + 8.0,
        col_w,
        20.0,
    );
    let c4 = LayoutRect::new(
        strip_r.x + 8.0 + (col_w + 4.0) * 3.0,
        strip_r.y + 8.0,
        col_w,
        20.0,
    );

    draw.queue_text_bounded(
        "🛡️ Trackers: 14 Blocked",
        c1.x,
        c1.y + 2.0,
        c1.w,
        10.0,
        BROWSER_EMERALD,
    );
    draw.queue_text_bounded(
        "🔒 Incognito: LOCKED OFF",
        c2.x,
        c2.y + 2.0,
        c2.w,
        10.0,
        BROWSER_EMERALD,
    );
    draw.queue_text_bounded(
        "⚡ AdBlock: Manifest V3 Active",
        c3.x,
        c3.y + 2.0,
        c3.w,
        10.0,
        [0.30, 0.75, 1.0, 1.0],
    );
    let yt_left = format!(
        "⏱️ YT Quota: {}",
        state.net_hub.youtube_guard.remaining_formatted()
    );
    draw.queue_text_bounded(&yt_left, c4.x, c4.y + 2.0, c4.w, 10.0, BROWSER_SOLAR_AMBER);
}

fn render_browser_screen_active_browsing(
    state: &ShellState,
    draw: &mut ClippedDraw,
    viewport: LayoutRect,
    scroll_y: f32,
    s: f32,
) {
    let p = state.theme.palette;
    let mut y = viewport.y + 12.0 - scroll_y;

    match &state.browser_page_kind {
        BrowserPageKind::WebPage {
            url: _,
            title,
            domain,
            content_lines,
            is_video,
        } => {
            // 1. PAGE HEADER CARD
            let header_card_r = LayoutRect::new(viewport.x + 12.0, y, viewport.w - 24.0, 68.0);
            draw.push_btn(header_card_r, BROWSER_SURFACE_SEC, BROWSER_BORDER);

            let badge_w = 110.0;
            let badge_r = LayoutRect::new(
                header_card_r.x + 12.0,
                header_card_r.y + 12.0,
                badge_w,
                24.0,
            );
            let badge_txt = if *is_video {
                "📺 VIDEO MODE"
            } else {
                "🌐 IN-OS WEB"
            };
            draw.push_btn(badge_r, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
            draw.queue_text_centered_in_rect(badge_txt, badge_r, 9.5, BROWSER_SOLAR_AMBER);

            draw.queue_text_bounded(
                title,
                header_card_r.x + badge_w + 18.0,
                header_card_r.y + 14.0,
                header_card_r.w - badge_w - 30.0,
                13.0,
                BROWSER_TEXT_PRI,
            );

            let meta_txt = format!(
                "Origin: {} • TLS 1.3 Verified • Zero Telemetry • Sovereign Sandboxed Tab",
                domain
            );
            draw.queue_text_bounded(
                &meta_txt,
                header_card_r.x + 14.0,
                header_card_r.y + 42.0,
                header_card_r.w - 28.0,
                10.5,
                BROWSER_TEXT_SEC,
            );
            y += 78.0;

            // 2. VIDEO CANVAS (if is_video is true)
            if *is_video {
                let video_frame_r = LayoutRect::new(viewport.x + 12.0, y, viewport.w - 24.0, 204.0);
                draw.push_btn(video_frame_r, [0.03, 0.04, 0.06, 1.0], BROWSER_BORDER);

                let vtitle_r = LayoutRect::new(
                    video_frame_r.x + 14.0,
                    video_frame_r.y + 12.0,
                    video_frame_r.w - 28.0,
                    20.0,
                );
                draw.queue_text_bounded(
                    "▶ 1080p60 • MIT 8.02: Electricity & Magnetism (Prof. Walter Lewin)",
                    vtitle_r.x,
                    vtitle_r.y,
                    vtitle_r.w,
                    11.5,
                    BROWSER_TEXT_PRI,
                );

                let play_icon_r = LayoutRect::new(
                    video_frame_r.x + (video_frame_r.w - 56.0) * 0.5,
                    video_frame_r.y + 56.0,
                    56.0,
                    56.0,
                );
                draw.push_btn(play_icon_r, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
                draw.queue_text_centered_in_rect("▶", play_icon_r, 22.0, BROWSER_SOLAR_AMBER);

                let scrub_bg = LayoutRect::new(
                    video_frame_r.x + 16.0,
                    video_frame_r.y + 144.0,
                    video_frame_r.w - 32.0,
                    6.0,
                );
                draw.push_panel(scrub_bg, [0.18, 0.20, 0.25, 1.0], 3.0);
                let scrub_fill = LayoutRect::new(scrub_bg.x, scrub_bg.y, scrub_bg.w * 0.35, 6.0);
                draw.push_panel(scrub_fill, BROWSER_SOLAR_AMBER, 3.0);

                let ctrl_row = LayoutRect::new(
                    video_frame_r.x + 16.0,
                    video_frame_r.y + 162.0,
                    video_frame_r.w - 32.0,
                    22.0,
                );
                draw.queue_split_row(
                    "⏱️ 15:45 / 45:00 • Speed: 1.0x • Audio: 48kHz Stereo",
                    "🛡️ Video Ads Blocked: 2 • Sidebars Hidden",
                    ctrl_row,
                    10.5,
                    BROWSER_TEXT_SEC,
                    BROWSER_EMERALD,
                );
                y += 214.0;
            }

            // 3. ARTICLE BODY CONTENT CARD
            let line_count = content_lines.len();
            let body_card_h = (line_count as f32 * 26.0 + 36.0).max(180.0);
            let body_card_r = LayoutRect::new(viewport.x + 12.0, y, viewport.w - 24.0, body_card_h);
            draw.push_btn(body_card_r, BROWSER_SURFACE_SEC, BROWSER_BORDER);

            for (i, line) in content_lines.iter().enumerate() {
                let ly = body_card_r.y + 14.0 + i as f32 * 25.0;
                if line == "─" {
                    let div_r =
                        LayoutRect::new(body_card_r.x + 12.0, ly + 10.0, body_card_r.w - 24.0, 1.0);
                    draw.push_panel(div_r, BROWSER_BORDER, 0.0);
                } else {
                    let is_header = line.starts_with("• Lecture")
                        || line.starts_with("• Unit")
                        || line.starts_with("Article:")
                        || line.starts_with("Instructor:")
                        || line.starts_with("Overview:");
                    let col = if is_header {
                        BROWSER_TEXT_PRI
                    } else {
                        BROWSER_TEXT_SEC
                    };
                    let font_sz = if is_header { 11.5 } else { 11.0 };
                    draw.queue_text_bounded(
                        line,
                        body_card_r.x + 16.0,
                        ly,
                        body_card_r.w - 32.0,
                        font_sz,
                        col,
                    );
                }
            }
            y += body_card_h + 16.0;

            // 4. BOTTOM ACTION BUTTONS
            let btn_save_w = 170.0;
            let btn_read_w = 170.0;
            let btn_ai_w = 150.0;
            let btn_home_w = 160.0;
            let save_vault_btn = LayoutRect::new(viewport.x + 12.0, y, btn_save_w, 34.0);
            let read_btn =
                LayoutRect::new(save_vault_btn.x + btn_save_w + 10.0, y, btn_read_w, 34.0);
            let ai_btn = LayoutRect::new(read_btn.x + btn_read_w + 10.0, y, btn_ai_w, 34.0);
            let home_btn = LayoutRect::new(ai_btn.x + btn_ai_w + 10.0, y, btn_home_w, 34.0);

            draw.push_labeled_btn(
                save_vault_btn,
                "➕ Save to Study Vault",
                10.5,
                BROWSER_AMBER_MUTED,
                BROWSER_SOLAR_AMBER,
            );
            draw.push_labeled_btn(
                read_btn,
                "📖 Send to Study Reader",
                10.5,
                BROWSER_SURFACE_SEC,
                BROWSER_BORDER,
            );
            draw.push_labeled_btn(
                ai_btn,
                "🤖 Ask Maindy AI",
                10.5,
                BROWSER_SOLAR_AMBER,
                BROWSER_SOLAR_AMBER,
            );
            draw.push_labeled_btn(
                home_btn,
                "⬅ Academic Search Home",
                10.5,
                BROWSER_SURFACE_SEC,
                BROWSER_BORDER,
            );
        }
        BrowserPageKind::BlockedShield {
            url: _,
            domain,
            reason,
            category,
        } => {
            // HERO SECURITY BLOCKED CONTAINER
            let hero_r = LayoutRect::new(viewport.x + 14.0, y, viewport.w - 28.0, 300.0);
            draw.push_panel(hero_r, [p.error[0], p.error[1], p.error[2], 0.12], 10.0);
            draw.push_btn(
                LayoutRect::new(hero_r.x, hero_r.y, hero_r.w, 4.0),
                p.error,
                p.error,
            );

            let badge_w = 260.0;
            let badge_r = LayoutRect::new(
                hero_r.x + (hero_r.w - badge_w) * 0.5,
                hero_r.y + 20.0,
                badge_w,
                28.0,
            );
            draw.push_btn(badge_r, [p.error[0], p.error[1], p.error[2], 0.85], p.error);
            draw.queue_text_centered_in_rect(
                "🚫 ACCESS RESTRICTED BY SHIELD",
                badge_r,
                10.5,
                [1.0, 1.0, 1.0, 1.0],
            );

            let blocked_title = format!("Target Blocked: {}", domain);
            draw.queue_text_centered_in_rect(
                &blocked_title,
                LayoutRect::new(hero_r.x + 14.0, hero_r.y + 58.0, hero_r.w - 28.0, 24.0),
                14.5,
                BROWSER_TEXT_PRI,
            );

            let cat_pill_r = LayoutRect::new(
                hero_r.x + (hero_r.w - 240.0) * 0.5,
                hero_r.y + 88.0,
                240.0,
                24.0,
            );
            draw.push_panel(cat_pill_r, BROWSER_SURFACE_SEC, 4.0);
            let cat_text = format!("Category: {}", category);
            draw.queue_text_centered_in_rect(&cat_text, cat_pill_r, 10.0, BROWSER_TEXT_SEC);

            let reason_box_r =
                LayoutRect::new(hero_r.x + 20.0, hero_r.y + 122.0, hero_r.w - 40.0, 88.0);
            draw.push_panel(reason_box_r, BROWSER_SURFACE_SEC, 6.0);
            draw.queue_text_bounded(
                reason,
                reason_box_r.x + 14.0,
                reason_box_r.y + 12.0,
                reason_box_r.w - 28.0,
                11.5,
                [p.error[0], p.error[1], p.error[2], 1.0],
            );
            draw.queue_text_bounded("MindSet OS Focus Guard and Internet Integrity policy enforce sovereign distraction-free study.", reason_box_r.x + 14.0, reason_box_r.y + 36.0, reason_box_r.w - 28.0, 10.5, BROWSER_TEXT_SEC);
            draw.queue_text_bounded("Entertainment streaming, social feeds, and short-form algorithms are prohibited during study hours.", reason_box_r.x + 14.0, reason_box_r.y + 56.0, reason_box_r.w - 28.0, 10.5, BROWSER_TEXT_SEC);

            let btn_home_w = 210.0;
            let btn_quota_w = 190.0;
            let ret_home_btn = LayoutRect::new(
                hero_r.x + (hero_r.w - btn_home_w - btn_quota_w - 12.0) * 0.5,
                hero_r.y + 230.0,
                btn_home_w,
                36.0,
            );
            let check_quota_btn = LayoutRect::new(
                ret_home_btn.x + btn_home_w + 12.0,
                hero_r.y + 230.0,
                btn_quota_w,
                36.0,
            );

            draw.push_labeled_btn(
                ret_home_btn,
                "🏠 Return to Academic Portal",
                11.0,
                BROWSER_SOLAR_AMBER,
                BROWSER_SOLAR_AMBER,
            );
            draw.push_labeled_btn(
                check_quota_btn,
                "⏱️ View YouTube Timer Tab",
                11.0,
                BROWSER_SURFACE_SEC,
                BROWSER_BORDER,
            );
        }
        BrowserPageKind::SearchHome => {
            render_browser_screen_new_tab(state, draw, viewport, scroll_y, s);
        }
    }
}

fn render_browser_screen_bookmarks(
    _state: &ShellState,
    draw: &mut ClippedDraw,
    viewport: LayoutRect,
    scroll_y: f32,
    _s: f32,
) {
    let mut y = viewport.y + 16.0 - scroll_y;

    // Header
    let hdr_r = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 28.0);
    draw.queue_split_row(
        "⭐ Bookmarks & Pinned Tabs",
        "18 Saved Bookmarks • Local SQLite",
        hdr_r,
        14.0,
        BROWSER_SOLAR_AMBER,
        BROWSER_TEXT_SEC,
    );
    y += 34.0;

    // Categories
    let cats = [
        "All (18)",
        "🏛️ Academic Portals (6)",
        "🔬 Research (5)",
        "⚡ Physics & Math (4)",
        "💻 CS (3)",
    ];
    let cat_w = 135.0;
    let cat_gap = 8.0;
    for (i, cat) in cats.iter().enumerate() {
        let cr = LayoutRect::new(
            viewport.x + 16.0 + i as f32 * (cat_w + cat_gap),
            y,
            cat_w,
            26.0,
        );
        let is_sel = i == 0;
        let fill = if is_sel {
            BROWSER_AMBER_MUTED
        } else {
            BROWSER_SURFACE_SEC
        };
        let border = if is_sel {
            BROWSER_SOLAR_AMBER
        } else {
            BROWSER_BORDER
        };
        draw.push_btn(cr, fill, border);
        let tc = if is_sel {
            BROWSER_SOLAR_AMBER
        } else {
            BROWSER_TEXT_SEC
        };
        draw.queue_text_centered_in_rect(cat, cr, 10.0, tc);
    }
    y += 38.0;

    // Bookmarks Cards (2 Columns)
    let bookmarks = [
        (
            "🏛️ NPTEL Wave Optics & Quantum Mechanics",
            "https://nptel.ac.in",
            "Academic",
        ),
        (
            "📺 MIT 8.02 Electricity & Magnetism (Lewin)",
            "https://youtube.com/watch?v=physics",
            "Video",
        ),
        (
            "📚 Wikipedia Quantum Electrodynamics (QED)",
            "https://en.wikipedia.org",
            "STEM",
        ),
        (
            "📄 arXiv:2403.11928 - Sovereign Visual Compositor",
            "https://arxiv.org/abs/2403.11928",
            "Research",
        ),
        (
            "🎓 Khan Academy Multivariable Calculus",
            "https://khanacademy.org",
            "Math",
        ),
        (
            "💻 Stanford CS106B Programming Abstractions",
            "https://cs106b.stanford.edu",
            "CS",
        ),
        (
            "🧪 LibreTexts Chemistry: Thermodynamics",
            "https://chem.libretexts.org",
            "Chemistry",
        ),
        (
            "📄 Nature Physics: High-Energy Superconductors",
            "https://nature.com/articles/physics",
            "Research",
        ),
    ];

    let cols = 2.0;
    let gap = 12.0;
    let card_w = (viewport.w - 32.0 - gap) / cols;
    let card_h = 74.0;

    for (idx, (btitle, burl, btag)) in bookmarks.iter().enumerate() {
        let col = (idx % 2) as f32;
        let row = (idx / 2) as f32;
        let bx = viewport.x + 16.0 + col * (card_w + gap);
        let by = y + row * (card_h + gap);
        let card_r = LayoutRect::new(bx, by, card_w, card_h);
        draw.push_btn(card_r, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        let icon_r = LayoutRect::new(card_r.x + 12.0, card_r.y + 12.0, 32.0, 32.0);
        draw.push_btn(icon_r, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect("⭐", icon_r, 12.0, BROWSER_SOLAR_AMBER);

        draw.queue_text_bounded(
            btitle,
            card_r.x + 52.0,
            card_r.y + 12.0,
            card_r.w - 140.0,
            11.5,
            BROWSER_TEXT_PRI,
        );
        draw.queue_text_bounded(
            burl,
            card_r.x + 52.0,
            card_r.y + 34.0,
            card_r.w - 140.0,
            10.0,
            BROWSER_TEXT_SEC,
        );

        let tag_r = LayoutRect::new(card_r.x + 52.0, card_r.y + 52.0, 60.0, 16.0);
        draw.push_btn(tag_r, [1.0, 1.0, 1.0, 0.06], BROWSER_BORDER);
        draw.queue_text_centered_in_rect(btag, tag_r, 9.0, BROWSER_TEXT_SEC);

        let open_btn = LayoutRect::new(card_r.x + card_r.w - 86.0, card_r.y + 22.0, 76.0, 28.0);
        draw.push_btn(open_btn, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect("Open Tab", open_btn, 10.0, BROWSER_SOLAR_AMBER);
    }
}

fn render_browser_screen_reading_list(
    _state: &ShellState,
    draw: &mut ClippedDraw,
    viewport: LayoutRect,
    scroll_y: f32,
    _s: f32,
) {
    let mut y = viewport.y + 16.0 - scroll_y;

    // Header
    let hdr_r = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 28.0);
    draw.queue_split_row(
        "📖 Reading List & Offline Reader",
        "4 Articles Saved • 44 Min Total Read Time • 100% Cached Locally",
        hdr_r,
        14.0,
        BROWSER_SOLAR_AMBER,
        BROWSER_TEXT_SEC,
    );
    y += 36.0;

    let articles = [
        (
            "Huygens-Fresnel Principle: Wavefront Superposition Analysis",
            "nptel.ac.in",
            "8 min read",
            0.85,
            "Cached Offline",
        ),
        (
            "Maxwell's Equations in Differential & Integral Forms",
            "ocw.mit.edu",
            "14 min read",
            0.0,
            "Cached Offline",
        ),
        (
            "Zero-Copy GPU Buffer Streaming in Pure Rust wgpu",
            "rust-lang.org",
            "12 min read",
            0.40,
            "Cached Offline",
        ),
        (
            "Chemical Thermodynamics: Gibbs Free Energy & Phase Transitions",
            "chem.libretexts.org",
            "10 min read",
            0.0,
            "Cached Offline",
        ),
    ];

    let card_w = viewport.w - 32.0;
    let card_h = 76.0;
    let gap = 12.0;

    for (title, domain, read_time, progress, cache_st) in articles.iter() {
        let cr = LayoutRect::new(viewport.x + 16.0, y, card_w, card_h);
        draw.push_btn(cr, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        let icon_r = LayoutRect::new(cr.x + 12.0, cr.y + 14.0, 36.0, 36.0);
        draw.push_btn(icon_r, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect("📖", icon_r, 14.0, BROWSER_SOLAR_AMBER);

        draw.queue_text_bounded(
            title,
            cr.x + 58.0,
            cr.y + 12.0,
            cr.w - 200.0,
            12.0,
            BROWSER_TEXT_PRI,
        );

        let meta = format!("{} • {} • {}", domain, read_time, cache_st);
        draw.queue_text_bounded(
            &meta,
            cr.x + 58.0,
            cr.y + 34.0,
            cr.w - 200.0,
            10.5,
            BROWSER_TEXT_SEC,
        );

        // Progress bar
        let pb_r = LayoutRect::new(cr.x + 58.0, cr.y + 54.0, 180.0, 6.0);
        draw.push_progress_bar(
            pb_r,
            *progress,
            [0.18, 0.20, 0.25, 1.0],
            BROWSER_SOLAR_AMBER,
        );

        let pct_txt = format!("{}%", (*progress * 100.0) as i32);
        draw.queue_text(
            &pct_txt,
            pb_r.x + pb_r.w + 8.0,
            cr.y + 50.0,
            9.5,
            BROWSER_TEXT_SEC,
        );

        let btn_r = LayoutRect::new(cr.x + cr.w - 120.0, cr.y + 22.0, 108.0, 30.0);
        let btn_txt = if *progress > 0.0 {
            "Resume Read"
        } else {
            "Start Reading"
        };
        draw.push_btn(btn_r, BROWSER_SOLAR_AMBER, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect(btn_txt, btn_r, 10.5, [0.05, 0.06, 0.08, 1.0]);

        y += card_h + gap;
    }
}

fn render_browser_screen_history(
    _state: &ShellState,
    draw: &mut ClippedDraw,
    viewport: LayoutRect,
    scroll_y: f32,
    _s: f32,
) {
    let mut y = viewport.y + 16.0 - scroll_y;

    // Header
    let hdr_r = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 28.0);
    draw.queue_split_row(
        "🕒 Sovereign Browsing History",
        "🔒 Encrypted SQLite Local Audit Log • Zero Cloud Syncing",
        hdr_r,
        14.0,
        BROWSER_SOLAR_AMBER,
        BROWSER_TEXT_SEC,
    );
    y += 34.0;

    let today_items = [
        (
            "09:10 AM",
            "MIT 8.02 Electricity & Magnetism (Prof. Lewin)",
            "youtube.com",
            "📺 Video Lecture • Ads Blocked: 2",
        ),
        (
            "08:55 AM",
            "NPTEL Wave Optics Course Notes",
            "nptel.ac.in",
            "🏛️ In-OS Web • TLS 1.3 Verified",
        ),
        (
            "08:42 AM",
            "Wikipedia STEM — Wave Interference & Diffraction",
            "en.wikipedia.org",
            "📚 Sovereign Reader Mode",
        ),
        (
            "08:30 AM",
            "Blocked Attempt: Netflix Streaming",
            "netflix.com",
            "🚫 Blocked by Focus Shield",
        ),
    ];

    let date_hdr = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 20.0);
    draw.queue_text_bounded(
        "TODAY — Sunday, November 15, 2026",
        date_hdr.x,
        date_hdr.y,
        date_hdr.w,
        11.0,
        BROWSER_SOLAR_AMBER,
    );
    y += 24.0;

    for (time, title, domain, meta) in today_items.iter() {
        let hr = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 48.0);
        draw.push_btn(hr, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        draw.queue_text(time, hr.x + 12.0, hr.y + 15.0, 11.0, BROWSER_SOLAR_AMBER);
        draw.queue_text_bounded(
            title,
            hr.x + 84.0,
            hr.y + 8.0,
            hr.w - 200.0,
            11.5,
            BROWSER_TEXT_PRI,
        );

        let sub = format!("{} • {}", domain, meta);
        let sub_col = if meta.contains("Blocked") {
            [0.937, 0.325, 0.314, 1.0]
        } else {
            BROWSER_TEXT_SEC
        };
        draw.queue_text_bounded(&sub, hr.x + 84.0, hr.y + 28.0, hr.w - 200.0, 10.0, sub_col);

        let reopen_btn = LayoutRect::new(hr.x + hr.w - 86.0, hr.y + 10.0, 74.0, 28.0);
        draw.push_btn(reopen_btn, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect("Revisit", reopen_btn, 10.0, BROWSER_SOLAR_AMBER);

        y += 54.0;
    }
    y += 12.0;

    let yest_items = [
        (
            "09:45 PM",
            "Rust wgpu 24 Memory Management Guide",
            "wgpu.rs",
            "💻 Technical Documentation",
        ),
        (
            "08:15 PM",
            "Khan Academy Vector Calculus Matrix Operations",
            "khanacademy.org",
            "🎓 Completed Practice",
        ),
        (
            "04:20 PM",
            "arXiv:2403.11928 Preprints on Visual Compositors",
            "arxiv.org",
            "📄 Research Paper",
        ),
    ];

    let yest_hdr = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 20.0);
    draw.queue_text_bounded(
        "YESTERDAY — Saturday, November 14, 2026",
        yest_hdr.x,
        yest_hdr.y,
        yest_hdr.w,
        11.0,
        BROWSER_SOLAR_AMBER,
    );
    y += 24.0;

    for (time, title, domain, meta) in yest_items.iter() {
        let hr = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 48.0);
        draw.push_btn(hr, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        draw.queue_text(time, hr.x + 12.0, hr.y + 15.0, 11.0, BROWSER_SOLAR_AMBER);
        draw.queue_text_bounded(
            title,
            hr.x + 84.0,
            hr.y + 8.0,
            hr.w - 200.0,
            11.5,
            BROWSER_TEXT_PRI,
        );

        let sub = format!("{} • {}", domain, meta);
        draw.queue_text_bounded(
            &sub,
            hr.x + 84.0,
            hr.y + 28.0,
            hr.w - 200.0,
            10.0,
            BROWSER_TEXT_SEC,
        );

        let reopen_btn = LayoutRect::new(hr.x + hr.w - 86.0, hr.y + 10.0, 74.0, 28.0);
        draw.push_btn(reopen_btn, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect("Revisit", reopen_btn, 10.0, BROWSER_SOLAR_AMBER);

        y += 54.0;
    }
}

fn render_browser_screen_downloads(
    _state: &ShellState,
    draw: &mut ClippedDraw,
    viewport: LayoutRect,
    scroll_y: f32,
    _s: f32,
) {
    let mut y = viewport.y + 16.0 - scroll_y;

    // Header
    let hdr_r = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 28.0);
    draw.queue_split_row(
        "📥 Downloads & Academic Assets",
        "Vault Storage: 2.1 GB / 32 GB Used • 4 Files Downloaded",
        hdr_r,
        14.0,
        BROWSER_SOLAR_AMBER,
        BROWSER_TEXT_SEC,
    );
    y += 36.0;

    let downloads = [
        (
            "📄 MIT_802_Lecture11_Notes.pdf",
            "14.2 MB / 14.2 MB",
            1.0,
            "Completed • Saved to Vault/Physics",
            true,
        ),
        (
            "📦 NPTEL_WaveOptics_Assignments.zip",
            "28.5 MB / 28.5 MB",
            1.0,
            "Completed • Saved to Vault/Assignments",
            true,
        ),
        (
            "📄 Quantum_Electrodynamics_Feynman.pdf",
            "8.4 MB / 12.0 MB",
            0.70,
            "Downloading: 70% • 3.2 MB/s",
            false,
        ),
        (
            "📄 MindSet_OS_Architecture_Whitepaper.pdf",
            "4.1 MB / 4.1 MB",
            1.0,
            "Completed • Saved to Vault",
            true,
        ),
    ];

    let card_w = viewport.w - 32.0;
    let card_h = 72.0;
    let gap = 12.0;

    for (filename, size_str, progress, status, is_done) in downloads.iter() {
        let dr = LayoutRect::new(viewport.x + 16.0, y, card_w, card_h);
        draw.push_btn(dr, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        let icon = if filename.contains(".zip") {
            "📦"
        } else {
            "📄"
        };
        let icon_r = LayoutRect::new(dr.x + 12.0, dr.y + 14.0, 36.0, 36.0);
        draw.push_btn(icon_r, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect(icon, icon_r, 14.0, BROWSER_SOLAR_AMBER);

        draw.queue_text_bounded(
            filename,
            dr.x + 58.0,
            dr.y + 12.0,
            dr.w - 200.0,
            12.0,
            BROWSER_TEXT_PRI,
        );

        let meta = format!("{} • {}", size_str, status);
        let meta_col = if *is_done {
            BROWSER_EMERALD
        } else {
            BROWSER_SOLAR_AMBER
        };
        draw.queue_text_bounded(
            &meta,
            dr.x + 58.0,
            dr.y + 32.0,
            dr.w - 200.0,
            10.5,
            meta_col,
        );

        let pb_r = LayoutRect::new(dr.x + 58.0, dr.y + 52.0, 200.0, 6.0);
        let bar_col = if *is_done {
            BROWSER_EMERALD
        } else {
            BROWSER_SOLAR_AMBER
        };
        draw.push_progress_bar(pb_r, *progress, [0.18, 0.20, 0.25, 1.0], bar_col);

        let act_btn = LayoutRect::new(dr.x + dr.w - 130.0, dr.y + 20.0, 118.0, 32.0);
        let act_txt = if *is_done { "Open in Vault" } else { "Pause" };
        draw.push_btn(act_btn, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect(act_txt, act_btn, 10.5, BROWSER_SOLAR_AMBER);

        y += card_h + gap;
    }
}

fn render_browser_screen_settings(
    _state: &ShellState,
    draw: &mut ClippedDraw,
    viewport: LayoutRect,
    scroll_y: f32,
    _s: f32,
) {
    let mut y = viewport.y + 16.0 - scroll_y;

    // Header
    let hdr_r = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 28.0);
    draw.queue_split_row(
        "⚙️ MindSet Browser Preferences & Security",
        "Sovereign Web Configuration • Manifest V3 • Zero Telemetry",
        hdr_r,
        14.0,
        BROWSER_SOLAR_AMBER,
        BROWSER_TEXT_SEC,
    );
    y += 36.0;

    let sections = [
        (
            "1. Visual & Spatial Interface",
            vec![
                (
                    "Theme Appearance",
                    "Solar Amber Deep Glass (Active Sovereign Theme)",
                ),
                ("Tab Bar Layout", "Horizontal Squircle Tabs (Active)"),
                (
                    "Viewport Corner Radius",
                    "8.0px Smooth Anti-Aliased Squircle",
                ),
            ],
        ),
        (
            "2. Academic Search & AI Synthesis",
            vec![
                (
                    "Default Search Engine",
                    "MindSet Sovereign Academic Engine (Offline Index)",
                ),
                ("Fallback Engine", "DuckDuckGo (Strict Privacy Mode)"),
                (
                    "Maindy AI Summarization",
                    "Automatic study synthesis generation enabled",
                ),
            ],
        ),
        (
            "3. MindSet Shield & Privacy Policy",
            vec![
                (
                    "Strict Zero-Incognito Lock",
                    "Enforced by OS Kernel (Immutable — Cannot be disabled)",
                ),
                (
                    "Third-Party Tracker Blocker",
                    "Strict Blocking (4,821 Manifest V3 rules)",
                ),
                (
                    "Intercept YouTube Shorts",
                    "Active (All /shorts/ routes 403 Forbidden)",
                ),
            ],
        ),
        (
            "4. Performance & Memory Optimization",
            vec![
                (
                    "Background Tab Sleeping",
                    "Enabled (Suspends inactive tabs after 10m, saving ~45% RAM)",
                ),
                (
                    "Hardware Accelerated Compositor",
                    "wgpu Vulkan / DX12 Native Active (60 FPS VSync)",
                ),
            ],
        ),
    ];

    let card_w = viewport.w - 32.0;
    for (sec_title, rows) in sections.iter() {
        let sec_h = 32.0 + rows.len() as f32 * 34.0;
        let sr = LayoutRect::new(viewport.x + 16.0, y, card_w, sec_h);
        draw.push_btn(sr, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        draw.queue_text(
            sec_title,
            sr.x + 14.0,
            sr.y + 10.0,
            11.5,
            BROWSER_SOLAR_AMBER,
        );

        for (i, (opt_lbl, opt_val)) in rows.iter().enumerate() {
            let ry = sr.y + 32.0 + i as f32 * 34.0;
            let opt_r = LayoutRect::new(sr.x + 14.0, ry, sr.w - 28.0, 24.0);
            draw.queue_split_row(
                opt_lbl,
                opt_val,
                opt_r,
                10.5,
                BROWSER_TEXT_PRI,
                BROWSER_TEXT_SEC,
            );
        }

        y += sec_h + 14.0;
    }
}

fn render_browser_screen_shield_dashboard(
    _state: &ShellState,
    draw: &mut ClippedDraw,
    viewport: LayoutRect,
    scroll_y: f32,
    _s: f32,
) {
    let mut y = viewport.y + 16.0 - scroll_y;

    // Header
    let hdr_r = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 28.0);
    draw.queue_split_row(
        "🛡️ MindSet Shield & Privacy Dashboard",
        "Live Protection Matrix • Zero Telemetry • Strict Sandboxing",
        hdr_r,
        14.0,
        BROWSER_SOLAR_AMBER,
        BROWSER_TEXT_SEC,
    );
    y += 36.0;

    // 4 KPI Stat Cards
    let kpis = [
        ("14", "Trackers Blocked Today", BROWSER_EMERALD),
        ("100%", "HTTPS TLS Enforced", BROWSER_EMERALD),
        ("LOCKED", "Zero-Incognito Policy", BROWSER_SOLAR_AMBER),
        ("0 KB", "Telemetry Sent to Cloud", [0.30, 0.75, 1.0, 1.0]),
    ];

    let cols = 4.0;
    let gap = 10.0;
    let card_w = (viewport.w - 32.0 - (cols - 1.0) * gap) / cols;
    let card_h = 76.0;

    for (i, (val, lbl, col)) in kpis.iter().enumerate() {
        let cx = viewport.x + 16.0 + i as f32 * (card_w + gap);
        let cr = LayoutRect::new(cx, y, card_w, card_h);
        draw.push_btn(cr, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        draw.queue_text_centered_in_rect(
            val,
            LayoutRect::new(cr.x + 8.0, cr.y + 12.0, cr.w - 16.0, 26.0),
            18.0,
            *col,
        );
        draw.queue_text_centered_in_rect(
            lbl,
            LayoutRect::new(cr.x + 6.0, cr.y + 44.0, cr.w - 12.0, 18.0),
            10.0,
            BROWSER_TEXT_SEC,
        );
    }
    y += card_h + 16.0;

    // Category Shields Grid (2 Columns x 2 Rows)
    let sh_hdr = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 18.0);
    draw.queue_text_bounded(
        "ACTIVE SECURITY FILTERS & CATEGORY SHIELDS",
        sh_hdr.x,
        sh_hdr.y,
        sh_hdr.w,
        10.5,
        BROWSER_SOLAR_AMBER,
    );
    y += 24.0;

    let shields = [
        (
            "🛡️ Social Media Shield",
            "Instagram, TikTok, Twitter/X, Reddit blocked during focus sessions.",
            "BLOCKED (Strict)",
        ),
        (
            "🎬 OTT Streaming Shield",
            "Netflix, Prime Video, Disney+ prevented from background execution.",
            "BLOCKED (Strict)",
        ),
        (
            "🚫 YouTube Shorts Blocker",
            "All short-form dopamine algorithm routes intercepted with 403 Forbidden.",
            "ACTIVE",
        ),
        (
            "⚡ Ad & Tracker Rules",
            "4,821 DeclarativeNetRequest declarative rules loaded in Manifest V3.",
            "4,821 ACTIVE",
        ),
    ];

    let scols = 2.0;
    let scard_w = (viewport.w - 32.0 - gap) / scols;
    let scard_h = 74.0;

    for (idx, (stitle, sdesc, sbadge)) in shields.iter().enumerate() {
        let col = (idx % 2) as f32;
        let row = (idx / 2) as f32;
        let sx = viewport.x + 16.0 + col * (scard_w + gap);
        let sy = y + row * (scard_h + gap);
        let sr = LayoutRect::new(sx, sy, scard_w, scard_h);
        draw.push_btn(sr, BROWSER_SURFACE_SEC, BROWSER_BORDER);

        let badge_r = LayoutRect::new(sr.x + sr.w - 100.0, sr.y + 10.0, 90.0, 20.0);
        draw.push_btn(badge_r, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
        draw.queue_text_centered_in_rect(sbadge, badge_r, 9.5, BROWSER_SOLAR_AMBER);

        draw.queue_text_bounded(
            stitle,
            sr.x + 12.0,
            sr.y + 10.0,
            sr.w - 120.0,
            11.5,
            BROWSER_TEXT_PRI,
        );
        draw.queue_text_bounded(
            sdesc,
            sr.x + 12.0,
            sr.y + 34.0,
            sr.w - 24.0,
            10.0,
            BROWSER_TEXT_SEC,
        );
    }
    y += 2.0 * scard_h + gap + 16.0;

    // Action Buttons Bar
    let bar_r = LayoutRect::new(viewport.x + 16.0, y, viewport.w - 32.0, 42.0);
    draw.push_panel(bar_r, BROWSER_GLASS_RAISED, 6.0);

    let b1 = LayoutRect::new(bar_r.x + 10.0, bar_r.y + 6.0, 230.0, 30.0);
    let b2 = LayoutRect::new(b1.x + 240.0, bar_r.y + 6.0, 210.0, 30.0);
    draw.push_labeled_btn(
        b1,
        "🩺 Run 7-Stage Troubleshooter",
        10.5,
        BROWSER_AMBER_MUTED,
        BROWSER_SOLAR_AMBER,
    );
    draw.push_labeled_btn(
        b2,
        "📋 SQLite Audit Logs (0 Leak)",
        10.5,
        BROWSER_SURFACE_SEC,
        BROWSER_BORDER,
    );
}

pub fn render_search_browser_content(
    state: &ShellState,
    draw: &mut ClippedDraw,
    area: LayoutRect,
    scroll_y: f32,
) {
    let p = state.theme.palette;
    let s = state.personalization_store.appearance.ui_scale;

    // 1. OPAQUE DEEP CHARCOAL CANVAS (#0B0D12) TO PREVENT WALLPAPER BLEED-THROUGH
    draw.push_panel(area, BROWSER_BG_DEEP, 8.0);
    draw.push_btn(area, [0.0, 0.0, 0.0, 0.0], BROWSER_BORDER);

    // 2. SUB-TAB BAR (34px) - Allows toggling between Browser View, YouTube Timer, AdBlock Rules, Net Doctor
    let (sub_tabs_bar, body) = area.split_top(34.0);
    let tab_titles = [
        "🌐 MindSet Sovereign Browser",
        "⏱️ YouTube Timer & Focus Shield",
        "🛡️ Ad-Blocker Rules",
        "🩺 Net Troubleshooter",
    ];
    let tab_count = 4.0;
    let tab_gap = 6.0;
    let tab_w = (sub_tabs_bar.w - (tab_count - 1.0) * tab_gap) / tab_count;

    for (i, title) in tab_titles.iter().enumerate() {
        let tab_r = LayoutRect::new(
            sub_tabs_bar.x + i as f32 * (tab_w + tab_gap),
            sub_tabs_bar.y + 2.0,
            tab_w,
            28.0,
        );
        let is_sel = state.browser_active_tab == i;
        let fill = if is_sel {
            BROWSER_SOLAR_AMBER
        } else {
            BROWSER_SURFACE_SEC
        };
        let border = if is_sel {
            BROWSER_SOLAR_AMBER
        } else {
            BROWSER_BORDER
        };
        draw.push_btn(tab_r, fill, border);
        let tc = if is_sel {
            [0.05, 0.06, 0.08, 1.0]
        } else {
            BROWSER_TEXT_SEC
        };
        draw.queue_text_centered_in_rect(title, tab_r, 10.5 * s.clamp(0.9, 1.15), tc);
    }

    match state.browser_active_tab {
        0 => {
            // Tab 0: Premium Native Arc/Safari MindSet Browser
            // A. HORIZONTAL TAB BAR (32px)
            let (tab_strip, body_after_tabs) = body.split_top(32.0);
            draw.push_panel(tab_strip, BROWSER_GLASS_RAISED, 0.0);

            let tab_h = 26.0;
            let max_tabs_w = tab_strip.w - 180.0;
            let rendered_tab_count = state.browser_tabs.len().max(1) as f32;
            let single_tab_w = ((max_tabs_w - (rendered_tab_count + 1.0) * 6.0)
                / rendered_tab_count)
                .clamp(110.0, 200.0);

            for (idx, tab) in state.browser_tabs.iter().enumerate() {
                let tab_x = tab_strip.x + 8.0 + idx as f32 * (single_tab_w + 6.0);
                let tab_r = LayoutRect::new(tab_x, tab_strip.y + 3.0, single_tab_w, tab_h);
                let is_active = state.browser_active_tab_idx == idx
                    && state.browser_screen == BrowserScreen::ActiveBrowsing;
                let fill = if is_active {
                    BROWSER_SURFACE_SEC
                } else {
                    BROWSER_BG_DEEP
                };
                let border = if is_active {
                    BROWSER_BORDER_FOCUS
                } else {
                    BROWSER_BORDER
                };
                draw.push_btn(tab_r, fill, border);

                // Favicon
                draw.queue_text(
                    &tab.icon,
                    tab_r.x + 8.0,
                    tab_r.y + 5.0,
                    11.0,
                    BROWSER_TEXT_PRI,
                );

                // Title
                let title_max_w = (tab_r.w - 44.0).max(40.0);
                let tc = if is_active {
                    BROWSER_TEXT_PRI
                } else {
                    BROWSER_TEXT_SEC
                };
                draw.queue_text_bounded(
                    &tab.title,
                    tab_r.x + 26.0,
                    tab_r.y + 6.0,
                    title_max_w,
                    10.5,
                    tc,
                );

                // Close tab '×'
                let close_r = LayoutRect::new(tab_r.x + tab_r.w - 16.0, tab_r.y + 6.0, 12.0, 14.0);
                draw.queue_text("×", close_r.x, close_r.y, 11.0, BROWSER_TEXT_SEC);

                // Solar Amber active line indicator at bottom of active tab
                if is_active {
                    let ind_r =
                        LayoutRect::new(tab_r.x + 4.0, tab_r.y + tab_r.h - 2.0, tab_r.w - 8.0, 2.0);
                    draw.push_btn(ind_r, BROWSER_SOLAR_AMBER, BROWSER_SOLAR_AMBER);
                }
            }

            // New Tab (+) button
            let plus_x = tab_strip.x + 8.0 + state.browser_tabs.len() as f32 * (single_tab_w + 6.0);
            let plus_r = LayoutRect::new(plus_x, tab_strip.y + 4.0, 26.0, 24.0);
            draw.push_btn(plus_r, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            draw.queue_text_centered_in_rect("+", plus_r, 13.0, BROWSER_TEXT_PRI);

            // Right Workspace Switcher Pill
            let ws_r = LayoutRect::new(
                tab_strip.x + tab_strip.w - 170.0,
                tab_strip.y + 4.0,
                162.0,
                24.0,
            );
            draw.push_btn(ws_r, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            let ws_title = match state.browser_workspace_idx {
                1 => "🔬 Research Space ▾",
                2 => "✨ Personal Space ▾",
                _ => "🏛️ Study Space ▾",
            };
            draw.queue_text_centered_in_rect(ws_title, ws_r, 10.0, BROWSER_SOLAR_AMBER);

            // B. FLOATING PILL NAVIGATION TOOLBAR (40px)
            let (nav_bar, main_area) = body_after_tabs.split_top(40.0);
            draw.push_panel(nav_bar, BROWSER_GLASS_RAISED, 0.0);

            let nav_btn_h = 28.0;
            let btn_back = LayoutRect::new(nav_bar.x + 8.0, nav_bar.y + 6.0, 30.0, nav_btn_h);
            let btn_fwd = LayoutRect::new(btn_back.x + 34.0, nav_bar.y + 6.0, 30.0, nav_btn_h);
            let btn_rel = LayoutRect::new(btn_fwd.x + 34.0, nav_bar.y + 6.0, 30.0, nav_btn_h);
            let btn_home = LayoutRect::new(btn_rel.x + 34.0, nav_bar.y + 6.0, 34.0, nav_btn_h);

            draw.push_labeled_btn(btn_back, "←", 12.0, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            draw.push_labeled_btn(btn_fwd, "→", 12.0, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            draw.push_labeled_btn(btn_rel, "⟳", 12.0, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            draw.push_labeled_btn(btn_home, "🏠", 11.0, BROWSER_SURFACE_SEC, BROWSER_BORDER);

            // Right Utility Cluster
            let right_w = 260.0;
            let right_x = nav_bar.x + nav_bar.w - right_w - 8.0;
            let btn_bm = LayoutRect::new(right_x, nav_bar.y + 6.0, 32.0, nav_btn_h);
            let btn_dl = LayoutRect::new(btn_bm.x + 36.0, nav_bar.y + 6.0, 32.0, nav_btn_h);
            let btn_sh = LayoutRect::new(btn_dl.x + 36.0, nav_bar.y + 6.0, 32.0, nav_btn_h);
            let btn_st = LayoutRect::new(btn_sh.x + 36.0, nav_bar.y + 6.0, 32.0, nav_btn_h);
            let btn_prof = LayoutRect::new(btn_st.x + 36.0, nav_bar.y + 6.0, 80.0, nav_btn_h);

            draw.push_labeled_btn(btn_bm, "⭐", 11.0, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            draw.push_labeled_btn(btn_dl, "📥", 11.0, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            draw.push_labeled_btn(btn_sh, "🛡️", 11.0, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            draw.push_labeled_btn(btn_st, "⚙️", 11.0, BROWSER_SURFACE_SEC, BROWSER_BORDER);
            draw.push_labeled_btn(
                btn_prof,
                "👤 LKS Pro",
                10.0,
                BROWSER_SURFACE_SEC,
                BROWSER_BORDER,
            );

            // Center Omnibox / Floating Pill Address Bar
            let omni_x = btn_home.x + 40.0;
            let omni_w = (right_x - omni_x - 10.0).max(120.0);
            let omni_r = LayoutRect::new(omni_x, nav_bar.y + 5.0, omni_w, 30.0);
            let is_omni_foc = state.search_input.is_focused;
            let omni_border = if is_omni_foc {
                BROWSER_BORDER_FOCUS
            } else {
                BROWSER_BORDER
            };
            draw.push_btn(omni_r, BROWSER_SURFACE_SEC, omni_border);

            // Lock & Security Badge inside omnibox
            let lock_badge_w = 88.0;
            let lock_r = LayoutRect::new(omni_r.x + 4.0, omni_r.y + 4.0, lock_badge_w, 22.0);
            draw.push_btn(
                lock_r,
                [0.063, 0.725, 0.506, 0.15],
                [0.063, 0.725, 0.506, 0.4],
            );
            draw.queue_text_centered_in_rect("🔒 TLS 1.3", lock_r, 9.5, BROWSER_EMERALD);

            // Shield tracker badge on right side of Omnibox
            let shield_badge_w = 90.0;
            let sh_badge_r = LayoutRect::new(
                omni_r.x + omni_r.w - shield_badge_w - 4.0,
                omni_r.y + 4.0,
                shield_badge_w,
                22.0,
            );
            draw.push_btn(sh_badge_r, BROWSER_AMBER_MUTED, BROWSER_SOLAR_AMBER);
            draw.queue_text_centered_in_rect("🛡️ 14 Blocked", sh_badge_r, 9.5, BROWSER_SOLAR_AMBER);

            // URL / Search text
            let url_disp_x = lock_r.x + lock_badge_w + 6.0;
            let url_disp_w = (sh_badge_r.x - url_disp_x - 6.0).max(40.0);
            let disp_txt = match state.browser_screen {
                BrowserScreen::ActiveBrowsing => match &state.browser_page_kind {
                    BrowserPageKind::WebPage { url, .. } => url.as_str(),
                    BrowserPageKind::BlockedShield { url, .. } => url.as_str(),
                    BrowserPageKind::SearchHome => {
                        if state.search_input.text.is_empty() {
                            "Search web, Wikipedia, or enter URL..."
                        } else {
                            state.search_input.text.as_str()
                        }
                    }
                },
                BrowserScreen::NewTab => {
                    if state.search_input.text.is_empty() {
                        "Search web, notes, or enter URL..."
                    } else {
                        state.search_input.text.as_str()
                    }
                }
                BrowserScreen::Bookmarks => "mindset://bookmarks",
                BrowserScreen::ReadingList => "mindset://reading-list",
                BrowserScreen::History => "mindset://history",
                BrowserScreen::Downloads => "mindset://downloads",
                BrowserScreen::Settings => "mindset://settings",
                BrowserScreen::ShieldDashboard => "mindset://shield-dashboard",
            };
            let txt_col = if state.search_input.text.is_empty()
                && state.browser_screen == BrowserScreen::NewTab
            {
                BROWSER_TEXT_SEC
            } else {
                BROWSER_TEXT_PRI
            };
            draw.queue_text_bounded(
                disp_txt,
                url_disp_x,
                omni_r.y + 8.0,
                url_disp_w,
                11.0,
                txt_col,
            );

            // C. MAIN BODY: LEFT ARC SIDEBAR + RIGHT CONTENT VIEWPORT
            let sb_w = if state.browser_sidebar_expanded {
                (160.0 * s).clamp(140.0, 190.0)
            } else {
                52.0
            };
            let (sidebar_r, viewport_outer) = main_area.split_left(sb_w);

            // 1. Arc-inspired Collapsible Sidebar
            draw.push_panel(sidebar_r, BROWSER_GLASS_RAISED, 0.0);
            draw.push_btn(
                LayoutRect::new(
                    sidebar_r.x + sidebar_r.w - 1.0,
                    sidebar_r.y,
                    1.0,
                    sidebar_r.h,
                ),
                BROWSER_BORDER,
                BROWSER_BORDER,
            );

            let mut sby = sidebar_r.y + 8.0;
            if state.browser_sidebar_expanded {
                draw.queue_text_bounded(
                    "SPACES",
                    sidebar_r.x + 12.0,
                    sby,
                    sidebar_r.w - 24.0,
                    9.5,
                    BROWSER_TEXT_SEC,
                );
                sby += 18.0;

                let spaces = [("🏛️ Study", 0), ("🔬 Research", 1), ("✨ Personal", 2)];
                for (sp_lbl, sp_idx) in spaces.iter() {
                    let sp_r = LayoutRect::new(sidebar_r.x + 6.0, sby, sidebar_r.w - 12.0, 24.0);
                    let is_cur = state.browser_workspace_idx == *sp_idx;
                    let sfill = if is_cur {
                        BROWSER_SURFACE_SEC
                    } else {
                        [0.0; 4]
                    };
                    let sbor = if is_cur {
                        BROWSER_SOLAR_AMBER
                    } else {
                        [0.0; 4]
                    };
                    draw.push_btn(sp_r, sfill, sbor);
                    let stc = if is_cur {
                        BROWSER_SOLAR_AMBER
                    } else {
                        BROWSER_TEXT_SEC
                    };
                    draw.queue_text_bounded(
                        sp_lbl,
                        sp_r.x + 6.0,
                        sp_r.y + 5.0,
                        sp_r.w - 12.0,
                        10.0,
                        stc,
                    );
                    sby += 26.0;
                }
                sby += 8.0;
                draw.queue_text_bounded(
                    "NAVIGATION",
                    sidebar_r.x + 12.0,
                    sby,
                    sidebar_r.w - 24.0,
                    9.5,
                    BROWSER_TEXT_SEC,
                );
                sby += 18.0;
            }

            let nav_items = [
                (BrowserScreen::NewTab, "🧭", "New Tab"),
                (BrowserScreen::ActiveBrowsing, "🌐", "Active Tab"),
                (BrowserScreen::Bookmarks, "⭐", "Bookmarks"),
                (BrowserScreen::ReadingList, "📖", "Reading List"),
                (BrowserScreen::History, "🕒", "History"),
                (BrowserScreen::Downloads, "📥", "Downloads"),
                (BrowserScreen::ShieldDashboard, "🛡️", "Shield Hub"),
                (BrowserScreen::Settings, "⚙️", "Settings"),
            ];

            for (screen_target, icon, label) in nav_items.iter() {
                let item_r = LayoutRect::new(sidebar_r.x + 4.0, sby, sidebar_r.w - 8.0, 30.0);
                let is_active_screen = state.browser_screen == *screen_target;
                let ifill = if is_active_screen {
                    BROWSER_SURFACE_SEC
                } else {
                    [0.0; 4]
                };
                let iborder = if is_active_screen {
                    BROWSER_BORDER_FOCUS
                } else {
                    [0.0; 4]
                };
                draw.push_btn(item_r, ifill, iborder);

                if state.browser_sidebar_expanded {
                    draw.queue_text(icon, item_r.x + 8.0, item_r.y + 7.0, 11.5, BROWSER_TEXT_PRI);
                    let itc = if is_active_screen {
                        BROWSER_SOLAR_AMBER
                    } else {
                        BROWSER_TEXT_SEC
                    };
                    draw.queue_text_bounded(
                        label,
                        item_r.x + 28.0,
                        item_r.y + 8.0,
                        item_r.w - 34.0,
                        10.5,
                        itc,
                    );
                } else {
                    draw.queue_text_centered_in_rect(
                        icon,
                        item_r,
                        12.0,
                        if is_active_screen {
                            BROWSER_SOLAR_AMBER
                        } else {
                            BROWSER_TEXT_PRI
                        },
                    );
                }
                sby += 34.0;
            }

            // Sidebar Bottom Controls (Collapse Toggle & Security Pill)
            let toggle_icon = if state.browser_sidebar_expanded {
                "« Collapse"
            } else {
                "»"
            };
            let toggle_r = LayoutRect::new(
                sidebar_r.x + 6.0,
                sidebar_r.y + sidebar_r.h - 32.0,
                sidebar_r.w - 12.0,
                24.0,
            );
            draw.push_labeled_btn(
                toggle_r,
                toggle_icon,
                10.0,
                BROWSER_SURFACE_SEC,
                BROWSER_BORDER,
            );

            // 2. Right Content Viewport Frame
            let (viewport_content, status_bar) = viewport_outer.split_bottom(22.0);
            draw.push_panel(viewport_content, BROWSER_BG_DEEP, 6.0);

            // Minimalist Status Bar at bottom
            draw.push_panel(status_bar, BROWSER_GLASS_RAISED, 0.0);
            draw.queue_split_row(
                "🛡️ MindSet Shield: 14 Trackers Blocked • TLS 1.3 Strict",
                "🏛️ Academic Space • 100% Zoom • 🔒 Local SQLite Only",
                LayoutRect::new(
                    status_bar.x + 8.0,
                    status_bar.y + 3.0,
                    status_bar.w - 16.0,
                    16.0,
                ),
                9.5,
                BROWSER_TEXT_SEC,
                BROWSER_TEXT_SEC,
            );

            // Render the selected visual screen in viewport_content!
            match state.browser_screen {
                BrowserScreen::NewTab => {
                    render_browser_screen_new_tab(state, draw, viewport_content, scroll_y, s);
                }
                BrowserScreen::ActiveBrowsing => {
                    render_browser_screen_active_browsing(
                        state,
                        draw,
                        viewport_content,
                        scroll_y,
                        s,
                    );
                }
                BrowserScreen::Bookmarks => {
                    render_browser_screen_bookmarks(state, draw, viewport_content, scroll_y, s);
                }
                BrowserScreen::ReadingList => {
                    render_browser_screen_reading_list(state, draw, viewport_content, scroll_y, s);
                }
                BrowserScreen::History => {
                    render_browser_screen_history(state, draw, viewport_content, scroll_y, s);
                }
                BrowserScreen::Downloads => {
                    render_browser_screen_downloads(state, draw, viewport_content, scroll_y, s);
                }
                BrowserScreen::Settings => {
                    render_browser_screen_settings(state, draw, viewport_content, scroll_y, s);
                }
                BrowserScreen::ShieldDashboard => {
                    render_browser_screen_shield_dashboard(
                        state,
                        draw,
                        viewport_content,
                        scroll_y,
                        s,
                    );
                }
            }
        }
        1 => {
            // Tab 1: YouTube Timer & App Blocker
            let mut y = body.y + 8.0 - scroll_y;

            // Card 1: YouTube Screen-Time Quota Card
            let yt_card = LayoutRect::new(body.x, y, body.w, 180.0);
            draw.push_panel(yt_card, p.surface_raised, 10.0);

            let header_row =
                LayoutRect::new(yt_card.x + 14.0, yt_card.y + 12.0, yt_card.w - 28.0, 22.0);
            let timer_st = if state.net_hub.youtube_guard.timer_running {
                "TIMER RUNNING"
            } else {
                "TIMER PAUSED"
            };
            let st_color = if state.net_hub.youtube_guard.timer_running {
                [0.063, 0.725, 0.506, 1.0]
            } else {
                p.text_secondary
            };
            draw.queue_split_row(
                "YouTube Daily Screen-Time Guard",
                timer_st,
                header_row,
                13.0,
                p.text_primary,
                st_color,
            );

            let quota_info = format!(
                "Used: {} / Limit: {} (Remaining: {})",
                format_seconds(state.net_hub.youtube_guard.used_today_secs),
                format_seconds(state.net_hub.youtube_guard.daily_limit_secs),
                state.net_hub.youtube_guard.remaining_formatted(),
            );
            draw.queue_text_bounded(
                &quota_info,
                yt_card.x + 14.0,
                yt_card.y + 38.0,
                yt_card.w - 28.0,
                11.0,
                p.text_secondary,
            );

            // Progress Bar
            let pb_r = LayoutRect::new(yt_card.x + 14.0, yt_card.y + 58.0, yt_card.w - 28.0, 10.0);
            let ratio = (state.net_hub.youtube_guard.used_today_secs as f32
                / state.net_hub.youtube_guard.daily_limit_secs.max(1) as f32)
                .clamp(0.0, 1.0);
            let pb_col = if ratio >= 1.0 {
                p.error
            } else {
                p.accent_primary
            };
            draw.push_progress_bar(
                pb_r,
                ratio,
                [p.border[0], p.border[1], p.border[2], 0.3],
                pb_col,
            );

            // Controls row
            let btn_play_label = if state.net_hub.youtube_guard.timer_running {
                "[Pause YT Timer]"
            } else {
                "[Start YT Timer]"
            };
            let play_btn_r = LayoutRect::new(yt_card.x + 14.0, yt_card.y + 78.0, 130.0, 28.0);
            draw.push_labeled_btn(
                play_btn_r,
                btn_play_label,
                10.5,
                p.accent_primary,
                p.accent_primary,
            );

            let presets = [15, 30, 45, 60];
            for (pi, &m) in presets.iter().enumerate() {
                let pr_r = LayoutRect::new(
                    play_btn_r.x + 130.0 + 8.0 + pi as f32 * 56.0,
                    yt_card.y + 78.0,
                    50.0,
                    28.0,
                );
                let is_cur = state.net_hub.youtube_guard.daily_limit_secs == m * 60;
                let (fill, bcol) = if is_cur {
                    (p.accent_primary, p.accent_primary)
                } else {
                    (p.surface, p.border)
                };
                draw.push_labeled_btn(pr_r, &format!("{}m", m), 10.5, fill, bcol);
            }

            let sim_btn_r =
                LayoutRect::new(yt_card.x + yt_card.w - 174.0, yt_card.y + 78.0, 160.0, 28.0);
            draw.push_labeled_btn(
                sim_btn_r,
                "[Simulate Limit Reached]",
                10.0,
                [p.error[0], p.error[1], p.error[2], 0.20],
                p.error,
            );

            // Badges row
            let badge1_r = LayoutRect::new(
                yt_card.x + 14.0,
                yt_card.y + 120.0,
                (yt_card.w - 38.0) * 0.5,
                26.0,
            );
            let badge2_r = LayoutRect::new(
                badge1_r.x + badge1_r.w + 10.0,
                yt_card.y + 120.0,
                (yt_card.w - 38.0) * 0.5,
                26.0,
            );
            draw.push_panel(badge1_r, [p.error[0], p.error[1], p.error[2], 0.15], 4.0);
            draw.push_panel(badge2_r, [0.063, 0.725, 0.506, 0.12], 4.0);
            let shorts_msg = format!(
                "🚫 Shorts Blocked: /shorts/ ({} Intercepted)",
                state.net_hub.youtube_guard.shorts_blocked_count
            );
            draw.queue_text_centered_in_rect(&shorts_msg, badge1_r, 10.0, p.error);
            draw.queue_text_centered_in_rect(
                "✨ Feed Shield: Distractions & Recommendations Hidden",
                badge2_r,
                10.0,
                [0.063, 0.725, 0.506, 1.0],
            );

            let quota_hint =
                "Quota resets automatically each morning. Zero-bypass enforcement active.";
            draw.queue_text_bounded(
                quota_hint,
                yt_card.x + 14.0,
                yt_card.y + 154.0,
                yt_card.w - 28.0,
                10.0,
                p.text_secondary,
            );

            y += 194.0;

            // Card 2: Entertainment Category Shields
            let ent_card = LayoutRect::new(body.x, y, body.w, 240.0);
            draw.push_panel(ent_card, p.surface_raised, 10.0);

            let ent_hdr = LayoutRect::new(
                ent_card.x + 14.0,
                ent_card.y + 12.0,
                ent_card.w - 28.0,
                20.0,
            );
            let blocked_cnt_str = format!(
                "Total Attempts Shielded: {}",
                state.net_hub.entertainment_shield.blocked_attempts
            );
            draw.queue_split_row(
                "Entertainment & Distraction Category Shields",
                &blocked_cnt_str,
                ent_hdr,
                13.0,
                p.text_primary,
                p.accent_primary,
            );
            draw.queue_text_bounded(
                    "Enforces sovereign deep-work integrity across browser navigations and desktop network traffic.",
                    ent_card.x + 14.0, ent_card.y + 34.0, ent_card.w - 28.0, 10.5, p.text_secondary,
                );

            let categories = [
                (
                    "1. OTT Streaming Services (Netflix, Prime Video, Disney+, Hotstar, Twitch)",
                    state.net_hub.entertainment_shield.block_ott,
                ),
                (
                    "2. Social Media Feeds (Instagram, Reddit, X/Twitter, TikTok, Facebook)",
                    state.net_hub.entertainment_shield.block_social,
                ),
                (
                    "3. Web Gaming Portals (CrazyGames, Poki, Roblox, Miniclip)",
                    state.net_hub.entertainment_shield.block_games,
                ),
                (
                    "4. Focus Chat Apps (WhatsApp Web, Discord, Telegram Web)",
                    state.net_hub.entertainment_shield.block_chat_during_focus,
                ),
            ];

            for (ci, (cat_label, is_blocked)) in categories.iter().enumerate() {
                let cat_row_r = LayoutRect::new(
                    ent_card.x + 14.0,
                    ent_card.y + 60.0 + ci as f32 * 42.0,
                    ent_card.w - 28.0,
                    36.0,
                );
                draw.push_btn(cat_row_r, p.surface, p.border);

                let pill_r = LayoutRect::new(
                    cat_row_r.x + cat_row_r.w - 110.0,
                    cat_row_r.y + 5.0,
                    102.0,
                    26.0,
                );
                let (btn_lbl, btn_fill, btn_col) = if *is_blocked {
                    (
                        "[BLOCKED]",
                        [p.error[0], p.error[1], p.error[2], 0.25],
                        p.error,
                    )
                } else {
                    (
                        "[ALLOWED]",
                        [0.063, 0.725, 0.506, 0.20],
                        [0.063, 0.725, 0.506, 1.0],
                    )
                };
                draw.push_labeled_btn(pill_r, btn_lbl, 10.5, btn_fill, btn_col);

                let max_w = (pill_r.x - cat_row_r.x - 20.0).max(40.0);
                draw.queue_text_bounded(
                    cat_label,
                    cat_row_r.x + 12.0,
                    cat_row_r.y + 11.0,
                    max_w,
                    11.0,
                    p.text_primary,
                );
            }
        }
        2 => {
            // Tab 2: Ad-Blocker Scripts
            let mut y = body.y + 8.0 - scroll_y;

            // Telemetry Counters Grid (4 metric cards)
            let card_gap = 10.0;
            let card_w = (body.w - 3.0 * card_gap) / 4.0;
            let telemetry_data = [
                (
                    "Trackers Blocked",
                    state.net_hub.adblock.stats.ads_blocked_count.to_string(),
                    [0.063, 0.725, 0.506, 1.0],
                ),
                (
                    "Video Ads Skipped",
                    state.net_hub.adblock.stats.video_ads_skipped.to_string(),
                    p.accent_primary,
                ),
                (
                    "Bandwidth Saved",
                    format!(
                        "{:.1} MB",
                        state.net_hub.adblock.stats.estimated_kb_saved as f32 / 1024.0
                    ),
                    [0.984, 0.749, 0.141, 1.0],
                ),
                (
                    "Shorts Intercepted",
                    state.net_hub.youtube_guard.shorts_blocked_count.to_string(),
                    p.error,
                ),
            ];

            for (i, (label, val, col)) in telemetry_data.iter().enumerate() {
                let tc_r =
                    LayoutRect::new(body.x + i as f32 * (card_w + card_gap), y, card_w, 74.0);
                draw.push_panel(tc_r, p.surface_raised, 8.0);
                draw.queue_text_centered_in_rect(
                    val,
                    LayoutRect::new(tc_r.x, tc_r.y + 10.0, tc_r.w, 30.0),
                    20.0,
                    *col,
                );
                draw.queue_text_centered_in_rect(
                    label,
                    LayoutRect::new(tc_r.x, tc_r.y + 44.0, tc_r.w, 20.0),
                    10.5,
                    p.text_secondary,
                );
            }
            y += 86.0;

            // Active Shield Scripts Card
            let scripts_card = LayoutRect::new(body.x, y, body.w, 200.0);
            draw.push_panel(scripts_card, p.surface_raised, 10.0);

            let sc_hdr = LayoutRect::new(
                scripts_card.x + 14.0,
                scripts_card.y + 12.0,
                scripts_card.w - 28.0,
                20.0,
            );
            draw.queue_split_row(
                "Chromium Manifest V3 Shield Extension Scripts",
                "Extension Status: FORCE-INJECTED",
                sc_hdr,
                13.0,
                p.text_primary,
                [0.063, 0.725, 0.506, 1.0],
            );
            draw.queue_text_bounded(
                    "Shield scripts are automatically verified on disk in ~/.mindset_data/browser_shield_ext/",
                    scripts_card.x + 14.0, scripts_card.y + 34.0, scripts_card.w - 28.0, 10.5, p.text_secondary,
                );

            let modules = [
                    ("manifest.json", "Manifest V3 declaration registering DeclarativeNetRequest rules & Content Scripts."),
                    ("rules.json", "DeclarativeNetRequest rule set blocking 45+ advertising hosts & telemetry domains."),
                    ("mindset_shield_inject.js", "DOM content script removing ad banners, promo popups, and redirecting Shorts."),
                ];
            for (mi, (fname, desc)) in modules.iter().enumerate() {
                let mod_r = LayoutRect::new(
                    scripts_card.x + 14.0,
                    scripts_card.y + 58.0 + mi as f32 * 36.0,
                    scripts_card.w - 28.0,
                    30.0,
                );
                draw.push_btn(mod_r, p.surface, p.border);
                let badge_mod = LayoutRect::new(mod_r.x + 6.0, mod_r.y + 4.0, 170.0, 22.0);
                draw.push_labeled_btn(
                    badge_mod,
                    fname,
                    10.0,
                    [
                        p.accent_primary[0],
                        p.accent_primary[1],
                        p.accent_primary[2],
                        0.20,
                    ],
                    p.accent_primary,
                );
                let max_d_w =
                    (mod_r.x + mod_r.w - (badge_mod.x + badge_mod.w + 12.0) - 8.0).max(40.0);
                draw.queue_text_bounded(
                    desc,
                    badge_mod.x + badge_mod.w + 10.0,
                    mod_r.y + 8.0,
                    max_d_w,
                    10.5,
                    p.text_secondary,
                );
            }

            // Verify & Reload Button
            let reload_btn_r = LayoutRect::new(
                scripts_card.x + 14.0,
                scripts_card.y + 164.0,
                scripts_card.w - 28.0,
                26.0,
            );
            draw.push_labeled_btn(
                reload_btn_r,
                "[Verify & Reload Shield Extension Scripts on Disk]",
                10.5,
                p.surface,
                p.border,
            );
        }
        3 => {
            // Tab 3: Internet Troubleshooter
            let mut y = body.y + 8.0 - scroll_y;

            // Master Diagnostic Button
            let diag_btn_r = LayoutRect::new(body.x, y, body.w, 36.0);
            draw.push_labeled_btn(
                diag_btn_r,
                "[Run Full 7-Stage Internet Diagnosis]",
                12.0,
                p.accent_primary,
                p.accent_primary,
            );
            y += 44.0;

            // 4 Quick Repair Buttons
            let rep_gap = 8.0;
            let rep_w = (body.w - 3.0 * rep_gap) / 4.0;
            let repair_btns = [
                (
                    "[Flush DNS Cache]",
                    NetRepairAction::FlushDnsAndResetSockets,
                ),
                (
                    "[Safe Academic DNS]",
                    NetRepairAction::SwitchToSafeAcademicDns,
                ),
                (
                    "[Clear Proxy Traps]",
                    NetRepairAction::ClearProxyAndUnblockPorts,
                ),
                (
                    "[Rebuild Shield]",
                    NetRepairAction::RebuildBrowserShieldScripts,
                ),
            ];
            for (ri, (lbl, _)) in repair_btns.iter().enumerate() {
                let r_btn = LayoutRect::new(body.x + ri as f32 * (rep_w + rep_gap), y, rep_w, 30.0);
                draw.push_labeled_btn(r_btn, lbl, 10.5, p.surface_raised, p.border);
            }
            y += 38.0;

            // 7 Diagnostic Stage Cards
            for (si, stage) in state.net_hub.troubleshooter.stages.iter().enumerate() {
                let st_card = LayoutRect::new(body.x, y + si as f32 * 56.0, body.w, 50.0);
                draw.push_panel(st_card, p.surface_raised, 6.0);

                // Badge status
                let (st_lbl, st_bg, st_fg) = match stage.status {
                    StageStatus::Passed => (
                        "[PASS]",
                        [0.063, 0.725, 0.506, 0.25],
                        [0.063, 0.725, 0.506, 1.0],
                    ),
                    StageStatus::Warning => (
                        "[WARN]",
                        [0.984, 0.749, 0.141, 0.25],
                        [0.984, 0.749, 0.141, 1.0],
                    ),
                    StageStatus::Failed => (
                        "[FAIL]",
                        [p.error[0], p.error[1], p.error[2], 0.25],
                        p.error,
                    ),
                    StageStatus::Pending => ("[IDLE]", [0.3, 0.3, 0.4, 0.25], p.text_secondary),
                };
                let badge_r = LayoutRect::new(st_card.x + 8.0, st_card.y + 12.0, 56.0, 26.0);
                draw.push_labeled_btn(badge_r, st_lbl, 10.0, st_bg, st_fg);

                // Repair / Test button
                let rep_btn_r =
                    LayoutRect::new(st_card.x + st_card.w - 110.0, st_card.y + 12.0, 102.0, 26.0);
                draw.push_labeled_btn(rep_btn_r, "[Diagnose]", 10.0, p.surface, p.border);

                // Latency Pill
                let lat_str = format!("{} ms", stage.latency_ms);
                let lat_r = LayoutRect::new(rep_btn_r.x - 70.0, st_card.y + 15.0, 62.0, 20.0);
                draw.queue_text_bounded(
                    &lat_str,
                    lat_r.x,
                    lat_r.y + 2.0,
                    lat_r.w,
                    10.0,
                    p.text_secondary,
                );

                // Stage Name & Summary
                let text_start_x = badge_r.x + badge_r.w + 10.0;
                let max_text_w = (lat_r.x - text_start_x - 8.0).max(40.0);
                let title = format!("Stage {}: {}", stage.id, stage.name);
                draw.queue_text_bounded(
                    &title,
                    text_start_x,
                    st_card.y + 8.0,
                    max_text_w,
                    11.5,
                    p.text_primary,
                );
                draw.queue_text_bounded(
                    &stage.summary,
                    text_start_x,
                    st_card.y + 26.0,
                    max_text_w,
                    10.0,
                    p.text_secondary,
                );
            }
        }
        _ => {}
    }
}
