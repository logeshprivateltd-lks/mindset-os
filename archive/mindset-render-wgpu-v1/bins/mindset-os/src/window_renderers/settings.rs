//! MindSet OS — Settings & Personalization Window Renderer (`settings.rs`)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::draw_helpers::ClippedDraw;
use crate::state::ShellState;
use mindset_render::{optimal_text_color_for_fill, safe_utf8_prefix, WallpaperPreset};
use mindset_ui::theme::ThemeMode;
use mindset_ui::LayoutRect;

pub fn render_settings_content(
    state: &ShellState,
    draw: &mut ClippedDraw,
    area: LayoutRect,
    scroll_y: f32,
) {
    let p = state.theme.palette;

    static SETTINGS_NAV: &[(&str, &str)] = &[
        ("[USR]", "Profile & Identity"),
        ("[THM]", "Appearance & Glass"),
        ("[DSK]", "Desktop & Workspaces"),
        ("[AI]", "AI Personalization"),
        ("[RTN]", "Daily Routine"),
        ("[NTF]", "Notifications"),
        ("[PRD]", "Productivity"),
        ("[A11]", "Accessibility"),
        ("[SND]", "Sound & Bluetooth"),
        ("[SEC]", "Privacy & Adaptive"),
    ];

    // 1. Left Navigation Rail (230px)
    let rail_w = 230.0;
    let rail_r = LayoutRect::new(area.x, area.y, rail_w, area.h);
    draw.push_panel(
        rail_r,
        [p.surface[0], p.surface[1], p.surface[2], 0.45],
        8.0,
    );

    for (i, (badge, title)) in SETTINGS_NAV.iter().enumerate() {
        let item_r = LayoutRect::new(
            area.x + 4.0,
            area.y + 6.0 + i as f32 * 46.0,
            rail_w - 8.0,
            40.0,
        );
        let is_sel = state.settings_active_tab == i;
        let fill = if is_sel {
            p.accent_primary
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
            if is_sel { p.accent_primary } else { p.border },
        );
        draw.queue_text(badge, item_r.x + 8.0, item_r.y + 12.0, 10.5, tc);
        draw.queue_text(title, item_r.x + 52.0, item_r.y + 12.0, 11.5, tc);
    }

    // Vertical Separator
    let sep_r = LayoutRect::new(area.x + rail_w + 2.0, area.y, 1.0, area.h);
    draw.push_panel(sep_r, p.border, 0.0);

    // 2. Right Content Area (scrollable, clamped to 820px max-width container)
    let available_w = (area.w - rail_w - 28.0).max(100.0);
    let form_w = available_w.min(820.0);
    let content_x = area.x + rail_w + 14.0 + ((available_w - form_w) * 0.5).max(0.0);
    let content_w = form_w;
    let mut y = area.y + 8.0 - scroll_y;

    match state.settings_active_tab {
        0 => {
            // Tab 0: Profile & Identity
            draw.queue_text(
                "Profile & Identity Settings",
                content_x,
                y,
                14.0,
                p.text_primary,
            );
            draw.queue_text(
                "Configure your academic persona, daily target hours, and avatar styling.",
                content_x,
                y + 20.0,
                10.5,
                p.text_secondary,
            );
            y += 50.0;

            let card1 = LayoutRect::new(content_x, y, content_w, 48.0);
            draw.push_panel(card1, p.surface_raised, 8.0);
            draw.queue_text(
                "Full Name",
                card1.x + 12.0,
                card1.y + 8.0,
                10.5,
                p.text_secondary,
            );
            let full_name = format!(
                "{} [LKS Scholar]",
                state.personalization_store.profile.preferred_name
            );
            draw.queue_text(
                &full_name,
                card1.x + 12.0,
                card1.y + 24.0,
                12.5,
                p.text_primary,
            );
            y += 56.0;

            let card2 = LayoutRect::new(content_x, y, content_w, 48.0);
            draw.push_panel(card2, p.surface_raised, 8.0);
            draw.queue_text(
                "Preferred Call Name",
                card2.x + 12.0,
                card2.y + 8.0,
                10.5,
                p.text_secondary,
            );
            draw.queue_text(
                &state.personalization_store.profile.preferred_name,
                card2.x + 12.0,
                card2.y + 24.0,
                12.5,
                p.accent_primary,
            );
            y += 56.0;

            draw.queue_text("Academic Stage & Focus", content_x, y, 11.5, p.text_primary);
            let stages = [
                "School / Class 12",
                "Undergraduate",
                "Postgraduate",
                "Competitive (JEE/NEET)",
            ];
            let pill_w = (content_w - 18.0) / 4.0;
            for (i, st) in stages.iter().enumerate() {
                let pr = LayoutRect::new(
                    content_x + i as f32 * (pill_w + 6.0),
                    y + 18.0,
                    pill_w,
                    28.0,
                );
                let is_active = state.active_academic_stage == i;
                let fill = if is_active {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.surface_raised
                };
                let tc = if is_active {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    p.text_secondary
                };
                draw.push_btn(
                    pr,
                    fill,
                    if is_active {
                        [0.851, 0.608, 0.149, 1.0]
                    } else {
                        p.border
                    },
                );
                let tw = st.chars().count() as f32 * 4.8;
                draw.queue_text(st, pr.x + (pr.w - tw) * 0.5, pr.y + 7.0, 9.5, tc);
            }
            y += 56.0;

            draw.queue_text(
                "Daily Study Target Hours",
                content_x,
                y,
                11.5,
                p.text_primary,
            );
            let hours = [
                "3.0 hrs",
                "4.5 hrs (Target)",
                "6.0 hrs",
                "8.0 hrs (Intense)",
            ];
            for (i, hr) in hours.iter().enumerate() {
                let hr_r = LayoutRect::new(
                    content_x + i as f32 * (pill_w + 6.0),
                    y + 18.0,
                    pill_w,
                    28.0,
                );
                let is_active = state.active_study_target_hours == i;
                let fill = if is_active {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.surface_raised
                };
                let tc = if is_active {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    p.text_secondary
                };
                draw.push_btn(
                    hr_r,
                    fill,
                    if is_active {
                        [0.851, 0.608, 0.149, 1.0]
                    } else {
                        p.border
                    },
                );
                let tw = hr.chars().count() as f32 * 5.0;
                draw.queue_text(hr, hr_r.x + (hr_r.w - tw) * 0.5, hr_r.y + 7.0, 10.0, tc);
            }
            y += 58.0;

            let wiz_btn = LayoutRect::new(content_x, y, content_w, 36.0);
            draw.push_btn(wiz_btn, p.accent_primary, p.accent_primary);
            draw.queue_text(
                "[Launch 11-Step Initial Setup Wizard]",
                wiz_btn.x + (wiz_btn.w - 230.0) * 0.5,
                wiz_btn.y + 10.0,
                12.0,
                [1.0, 1.0, 1.0, 1.0],
            );
        }

        1 => {
            // Tab 1: Appearance & Glass
            draw.queue_text(
                "Appearance, Glass & Themes",
                content_x,
                y,
                14.0,
                p.text_primary,
            );
            draw.queue_text(
                "Customize dual-Kawase blur, wallpaper presets, color temperature, and UI scaling.",
                content_x,
                y + 20.0,
                10.5,
                p.text_secondary,
            );
            y += 48.0;

            draw.queue_text("Theme Mode", content_x, y, 11.5, p.text_primary);
            let btn_l = LayoutRect::new(content_x, y + 18.0, 100.0, 28.0);
            let btn_d = LayoutRect::new(content_x + 110.0, y + 18.0, 100.0, 28.0);
            let is_l = state.theme.mode == ThemeMode::Light;
            let is_d = state.theme.mode == ThemeMode::Dark;
            let fill_l = if is_l {
                [0.851, 0.608, 0.149, 1.0]
            } else {
                p.surface_raised
            };
            let fill_d = if is_d {
                [0.851, 0.608, 0.149, 1.0]
            } else {
                p.surface_raised
            };
            let tc_l = if is_l {
                [1.0, 1.0, 1.0, 1.0]
            } else {
                p.text_primary
            };
            let tc_d = if is_d {
                [1.0, 1.0, 1.0, 1.0]
            } else {
                p.text_primary
            };
            draw.push_btn(
                btn_l,
                fill_l,
                if is_l {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.border
                },
            );
            draw.queue_text("Light Mode", btn_l.x + 18.0, btn_l.y + 7.0, 11.0, tc_l);
            draw.push_btn(
                btn_d,
                fill_d,
                if is_d {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.border
                },
            );
            draw.queue_text("Dark Mode", btn_d.x + 18.0, btn_d.y + 7.0, 11.0, tc_d);
            y += 56.0;

            draw.queue_text(
                "Scenic Wallpaper Atmosphere",
                content_x,
                y,
                11.5,
                p.text_primary,
            );
            let presets = WallpaperPreset::all();
            for (i, &wp) in presets.iter().enumerate() {
                let row = (i / 3) as f32;
                let col = (i % 3) as f32;
                let btn_w = (content_w - 16.0) / 3.0;
                let wp_btn = LayoutRect::new(
                    content_x + col * (btn_w + 8.0),
                    y + 18.0 + row * 34.0,
                    btn_w,
                    28.0,
                );
                let is_sel = state.wallpaper_preset == wp;
                let fill = if is_sel {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.surface_raised
                };
                let tc = if is_sel {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    p.text_primary
                };
                draw.push_btn(
                    wp_btn,
                    fill,
                    if is_sel {
                        [0.851, 0.608, 0.149, 1.0]
                    } else {
                        p.border
                    },
                );
                let lw = wp.label().chars().count() as f32 * 5.6;
                draw.queue_text(
                    wp.label(),
                    wp_btn.x + (wp_btn.w - lw) * 0.5,
                    wp_btn.y + 7.0,
                    10.5,
                    tc,
                );
            }
            y += 94.0;

            let warmth_pct = (state.warmth_slider.value * 100.0).round() as i32;
            let slider_w = (content_w * 0.45).min(240.0);
            draw.queue_text(
                &format!("Night Warmth Amber Filter: {}%", warmth_pct),
                content_x,
                y + 6.0,
                11.5,
                p.text_primary,
            );
            let sl_r = LayoutRect::new(content_x + content_w - slider_w, y + 6.0, slider_w, 10.0);
            draw.push_slider(
                sl_r,
                state.warmth_slider.value,
                p.border,
                [0.851, 0.608, 0.149, 1.0],
            );
            y += 42.0;

            let scale_pct = (state.theme.ui_scale * 100.0).round() as i32;
            draw.queue_text(
                &format!("UI & Font Scaling: {}% (Default: 115%)", scale_pct),
                content_x,
                y + 6.0,
                11.5,
                p.text_primary,
            );
            let sc_val = ((state.theme.ui_scale - 0.85) / 0.50).clamp(0.0, 1.0);
            let sc_r = LayoutRect::new(content_x + content_w - slider_w, y + 6.0, slider_w, 10.0);
            draw.push_slider(sc_r, sc_val, p.border, p.accent_primary);
            y += 42.0;

            draw.queue_text("Glass Blur Quality", content_x, y, 11.5, p.text_primary);
            let b1 = LayoutRect::new(content_x, y + 18.0, 160.0, 28.0);
            let b2 = LayoutRect::new(content_x + 170.0, y + 18.0, 150.0, 28.0);
            let q0 = state.glass_blur_quality == 0;
            draw.push_btn(
                b1,
                if q0 {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.surface_raised
                },
                if q0 {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.border
                },
            );
            draw.queue_text(
                "Dual-Kawase 4-Pass",
                b1.x + 18.0,
                b1.y + 7.0,
                10.5,
                if q0 {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    p.text_primary
                },
            );
            draw.push_btn(
                b2,
                if !q0 {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.surface_raised
                },
                if !q0 {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.border
                },
            );
            draw.queue_text(
                "Standard 2-Pass",
                b2.x + 22.0,
                b2.y + 7.0,
                10.5,
                if !q0 {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    p.text_secondary
                },
            );
        }

        2 => {
            // Tab 2: Desktop & Workspaces
            draw.queue_text(
                "Desktop Shell & Workspaces",
                content_x,
                y,
                14.0,
                p.text_primary,
            );
            draw.queue_text(
                "Workspace assignments, desktop icon density, and interactive widgets.",
                content_x,
                y + 20.0,
                10.5,
                p.text_secondary,
            );
            y += 48.0;

            draw.queue_text(
                "Virtual Workspaces Configuration",
                content_x,
                y,
                11.5,
                p.text_primary,
            );
            let ws_names = [
                "Workspace 1: Study & Reader",
                "Workspace 2: Maindy AI Lab",
                "Workspace 3: Vault & Files",
                "Workspace 4: Sys Monitor",
            ];
            for (i, name) in ws_names.iter().enumerate() {
                let wr = LayoutRect::new(content_x, y + 20.0 + i as f32 * 36.0, content_w, 30.0);
                let is_active = state.active_workspace == i as u8;
                let fill = if is_active {
                    p.surface_raised
                } else {
                    p.surface
                };
                draw.push_panel(wr, fill, 6.0);
                draw.queue_text(
                    name,
                    wr.x + 12.0,
                    wr.y + 8.0,
                    11.0,
                    if is_active {
                        p.accent_primary
                    } else {
                        p.text_primary
                    },
                );
                let tag_r = LayoutRect::new(wr.x + wr.w - 134.0, wr.y + 4.0, 68.0, 22.0);
                draw.push_btn(
                    tag_r,
                    if is_active {
                        p.accent_primary
                    } else {
                        p.surface_raised
                    },
                    p.border,
                );
                draw.queue_text(
                    if is_active { "Active" } else { "Switch" },
                    tag_r.x + 14.0,
                    tag_r.y + 4.0,
                    9.5,
                    if is_active {
                        [1.0, 1.0, 1.0, 1.0]
                    } else {
                        p.text_secondary
                    },
                );
                let pill_r = LayoutRect::new(wr.x + wr.w - 56.0, wr.y + 3.0, 48.0, 24.0);
                draw.push_pill_toggle(pill_r, state.workspaces_enabled[i]);
            }
            y += 4.0 * 36.0 + 28.0;

            draw.queue_text(
                "Desktop Interactive Widgets",
                content_x,
                y,
                11.5,
                p.text_primary,
            );
            let widgets = [
                ("Focus Ring / Daily Progress", 0),
                ("Current Class 12 Goal & Target", 1),
                ("Pomodoro Timer Shelf", 2),
                ("System Resource Glance Bar", 3),
            ];
            for (lbl, idx) in widgets {
                let wr = LayoutRect::new(content_x, y + 20.0 + idx as f32 * 34.0, content_w, 28.0);
                draw.push_panel(wr, p.surface_raised, 6.0);
                draw.queue_text(lbl, wr.x + 12.0, wr.y + 6.0, 11.0, p.text_primary);
                let pill_r = LayoutRect::new(wr.x + wr.w - 56.0, wr.y + 2.0, 48.0, 24.0);
                draw.push_pill_toggle(pill_r, state.widgets_enabled[idx]);
            }
        }

        8 => {
            // Tab 8: Soundscapes & Bluetooth Peripherals
            draw.queue_text(
                "Study Soundscapes & Bluetooth Peripherals",
                content_x,
                y,
                14.0,
                p.text_primary,
            );
            draw.queue_text(
                "Manage background binaural audio streams and scan for wireless study accessories.",
                content_x,
                y + 20.0,
                10.5,
                p.text_secondary,
            );
            y += 50.0;

            draw.queue_text(
                "Active Study Soundscape",
                content_x,
                y,
                11.5,
                p.text_primary,
            );
            let sounds = [
                ("Off", 0),
                ("Lo-Fi Study Beats", 1),
                ("Rain & Thunder", 2),
                ("Binaural 40Hz Gamma", 3),
                ("Forest Brook", 4),
            ];
            let s_pill_w = (content_w - 24.0) / 5.0;
            for (name, idx) in sounds {
                let sr = LayoutRect::new(
                    content_x + idx as f32 * (s_pill_w + 6.0),
                    y + 18.0,
                    s_pill_w,
                    28.0,
                );
                let is_sel = state.ambient_soundscape == idx;
                let fill = if is_sel {
                    [0.851, 0.608, 0.149, 1.0]
                } else {
                    p.surface_raised
                };
                draw.push_btn(
                    sr,
                    fill,
                    if is_sel {
                        [0.851, 0.608, 0.149, 1.0]
                    } else {
                        p.border
                    },
                );
                let tw = name.chars().count() as f32 * 4.6;
                draw.queue_text(
                    name,
                    sr.x + (sr.w - tw) * 0.5,
                    sr.y + 7.0,
                    9.0,
                    if is_sel {
                        [1.0, 1.0, 1.0, 1.0]
                    } else {
                        p.text_secondary
                    },
                );
            }
            y += 58.0;

            let vol_pct = (state.master_volume * 100.0).round() as i32;
            let slider_w = (content_w * 0.45).min(240.0);
            draw.queue_text(
                &format!("Master Volume: {}%", vol_pct),
                content_x,
                y + 6.0,
                11.5,
                p.text_primary,
            );
            let vol_r = LayoutRect::new(content_x + content_w - slider_w, y + 6.0, slider_w, 10.0);
            draw.push_slider(vol_r, state.master_volume, p.border, p.accent_primary);
            y += 48.0;

            let scan_w = 140.0;
            draw.queue_text(
                "Discovered Bluetooth Peripherals",
                content_x,
                y + 6.0,
                11.5,
                p.text_primary,
            );
            let scan_btn = LayoutRect::new(content_x + content_w - scan_w, y, scan_w, 28.0);
            draw.push_btn(scan_btn, p.accent_primary, p.accent_primary);
            let scan_lbl = if state.bluetooth_mgr.is_scanning {
                "[Scanning...]"
            } else {
                "[Scan for Devices]"
            };
            draw.queue_text_centered_in_rect(scan_lbl, scan_btn, 10.5, [1.0, 1.0, 1.0, 1.0]);
            y += 40.0;

            if state.bluetooth_mgr.nearby_devices.is_empty() {
                let empty_r = LayoutRect::new(content_x, y, content_w, 48.0);
                draw.push_panel(empty_r, p.surface_raised, 8.0);
                draw.queue_text("No Bluetooth devices paired. Click [Scan for Devices] to discover nearby peripherals.", empty_r.x + 14.0, empty_r.y + 16.0, 11.0, p.text_secondary);
            } else {
                for (i, dev) in state.bluetooth_mgr.nearby_devices.iter().enumerate() {
                    let dev_r = LayoutRect::new(content_x, y + i as f32 * 66.0, content_w, 58.0);
                    draw.push_panel(dev_r, p.surface, 6.0);

                    let icon_r = LayoutRect::new(dev_r.x + 8.0, dev_r.y + 10.0, 36.0, 36.0);
                    draw.push_btn(icon_r, p.surface_raised, p.border);
                    draw.queue_text(
                        dev.device_type.icon_badge(),
                        icon_r.x + 6.0,
                        icon_r.y + 11.0,
                        10.0,
                        p.text_secondary,
                    );

                    draw.queue_text(
                        &dev.name,
                        dev_r.x + 52.0,
                        dev_r.y + 10.0,
                        11.0,
                        p.text_primary,
                    );
                    draw.queue_text(
                        safe_utf8_prefix(&dev.profile_info, 48),
                        dev_r.x + 52.0,
                        dev_r.y + 30.0,
                        9.5,
                        p.text_secondary,
                    );

                    let pair_btn =
                        LayoutRect::new(dev_r.x + dev_r.w - 110.0, dev_r.y + 14.0, 102.0, 28.0);
                    draw.push_btn(pair_btn, p.accent_primary, p.accent_primary);
                    draw.queue_text(
                        "Pair & Connect",
                        pair_btn.x + 10.0,
                        pair_btn.y + 7.0,
                        9.5,
                        [1.0, 1.0, 1.0, 1.0],
                    );
                }
            }
        }

        _ => {
            // Tab 9: Privacy & Adaptive AI
            draw.queue_text(
                "Sovereign Local Privacy & Adaptive Engine",
                content_x,
                y,
                14.0,
                p.text_primary,
            );
            draw.queue_text("Audit local storage, manage encryption, and approve AI personalization suggestions.", content_x, y + 20.0, 10.5, p.text_secondary);
            y += 48.0;

            let card1 = LayoutRect::new(content_x, y, content_w, 46.0);
            draw.push_panel(card1, p.surface_raised, 8.0);
            draw.queue_text(
                "Sovereign Privacy Status",
                card1.x + 12.0,
                card1.y + 6.0,
                10.5,
                p.text_secondary,
            );
            draw.queue_text(
                "100% Offline • Zero Analytics • Zero Cloud Telemetry",
                card1.x + 12.0,
                card1.y + 22.0,
                11.5,
                [0.10, 0.73, 0.51, 1.0],
            );
            y += 54.0;

            draw.queue_text(
                "Adaptive AI Personalization Suggestions",
                content_x,
                y,
                11.5,
                p.text_primary,
            );
            let sug_r = LayoutRect::new(content_x, y + 18.0, content_w, 54.0);
            draw.push_panel(sug_r, p.surface_raised, 8.0);
            draw.queue_text(
                "Suggestion: Shift evening study block 30m earlier based on past week focus.",
                sug_r.x + 12.0,
                sug_r.y + 8.0,
                10.5,
                p.text_primary,
            );
            draw.queue_text(
                "Confidence: 94% • High academic retention correlation",
                sug_r.x + 12.0,
                sug_r.y + 24.0,
                9.5,
                p.text_secondary,
            );

            let app_btn = LayoutRect::new(sug_r.x + sug_r.w - 146.0, sug_r.y + 12.0, 68.0, 26.0);
            let dis_btn = LayoutRect::new(sug_r.x + sug_r.w - 74.0, sug_r.y + 12.0, 66.0, 26.0);
            draw.push_btn(app_btn, p.accent_primary, p.accent_primary);
            draw.queue_text(
                "Approve",
                app_btn.x + 12.0,
                app_btn.y + 6.0,
                10.0,
                [1.0, 1.0, 1.0, 1.0],
            );
            draw.push_btn(dis_btn, p.surface, p.border);
            draw.queue_text(
                "Dismiss",
                dis_btn.x + 12.0,
                dis_btn.y + 6.0,
                10.0,
                p.text_secondary,
            );
            y += 82.0;

            draw.queue_text(
                "Data Management & Safety",
                content_x,
                y,
                11.5,
                p.text_primary,
            );
            let exp_btn = LayoutRect::new(content_x, y + 18.0, 180.0, 30.0);
            let rst_btn = LayoutRect::new(content_x + 192.0, y + 18.0, 180.0, 30.0);
            draw.push_btn(exp_btn, p.surface_raised, p.border);
            draw.queue_text(
                "Export Data (JSON)",
                exp_btn.x + 28.0,
                exp_btn.y + 7.0,
                11.0,
                p.accent_primary,
            );
            draw.push_btn(rst_btn, [p.error[0], p.error[1], p.error[2], 0.20], p.error);
            draw.queue_text(
                "Factory Reset Defaults",
                rst_btn.x + 24.0,
                rst_btn.y + 7.0,
                11.0,
                p.error,
            );
        }
    }
}
