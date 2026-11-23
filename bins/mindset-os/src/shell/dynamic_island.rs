//! Dynamic Island Notch for MindSet OS
//!
//! Deep obsidian morphing notch anchored at top-center:
//! - Compact: Glanceable time, streak, search shortcut, and audio indicator.
//! - Search: Live global search across syllabus capsules, tools, and notes (Ctrl+K).
//! - ControlCenter: Instant toggle for Lo-Fi audio, Light/Dark theme, 18 FPS motion, wallpaper, and map theme.
//! - ActiveFocus: Live Pomodoro timer, topic title, and session controls during Deep Focus.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length};
use mindset_core::{MindSetCoreState, WindowId};

use crate::storage::db::{CapsuleRecord, UserProfile};
use crate::theme::{h_space, px_space, ThemePalette};
use crate::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IslandMode {
    Compact,
    Search,
    ControlCenter,
    ActiveFocus,
    MaindySpeaking,
}

impl IslandMode {
    pub fn target_dimensions(self, search_results_count: usize) -> (f32, f32) {
        match self {
            Self::Compact => (300.0, 34.0),
            Self::Search => {
                let h = if search_results_count == 0 {
                    52.0
                } else {
                    240.0
                };
                (480.0, h)
            }
            Self::ControlCenter => (440.0, 260.0),
            Self::ActiveFocus => (340.0, 42.0),
            Self::MaindySpeaking => (340.0, 38.0),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn view<'a>(
    mode: IslandMode,
    width: f32,
    height: f32,
    core: &'a MindSetCoreState,
    profile: &'a UserProfile,
    capsules: &'a [CapsuleRecord],
    search_query: &'a str,
    is_audio_playing: bool,
    _palette: ThemePalette,
) -> Element<'a, Message> {
    let is_focused = core.pomodoro.is_running;
    let obsidian_bg = Color::from_rgba(0.012, 0.016, 0.024, 0.98);
    let rim_light = if is_audio_playing {
        Color::from_rgba(0.12, 0.65, 1.0, 0.45)
    } else if is_focused {
        Color::from_rgba(0.98, 0.60, 0.20, 0.45)
    } else {
        Color::from_rgba(1.0, 1.0, 1.0, 0.16)
    };

    let inner_content: Element<'a, Message> = match mode {
        IslandMode::Compact => render_compact(core, profile, is_audio_playing),
        IslandMode::ActiveFocus => render_active_focus(core),
        IslandMode::Search => render_search(search_query, capsules),
        IslandMode::ControlCenter => render_control_center(profile, is_audio_playing),
        IslandMode::MaindySpeaking => render_maindy_speaking(),
    };

    let pill_radius = (height / 2.0).min(20.0);

    container(inner_content)
        .width(width)
        .height(height)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(obsidian_bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(pill_radius),
                color: rim_light,
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: if is_audio_playing {
                    Color::from_rgba(0.12, 0.48, 0.96, 0.35)
                } else if is_focused {
                    Color::from_rgba(0.98, 0.60, 0.20, 0.30)
                } else {
                    Color::from_rgba(0.0, 0.0, 0.0, 0.50)
                },
                offset: iced::Vector::new(0.0, 6.0),
                blur_radius: 18.0,
            },
            ..Default::default()
        })
        .into()
}

fn render_compact<'a>(
    core: &'a MindSetCoreState,
    _profile: &'a UserProfile,
    is_audio_playing: bool,
) -> Element<'a, Message> {
    // Left Activity Pill: Live Pomodoro timer or Audio wave or Search trigger
    let left_activity: Element<'a, Message> = if is_audio_playing {
        button(
            row![
                text("🎵").size(11),
                px_space(2.0, 0.0),
                text("Lo-Fi")
                    .size(10)
                    .color(Color::from_rgb(0.20, 0.75, 1.0)),
            ]
            .align_y(Alignment::Center),
        )
        .on_press(Message::ToggleAudio)
        .style(|_t, s| {
            let bg = match s {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .padding([2, 5])
        .into()
    } else if core.pomodoro.is_running {
        let timer_str = core.pomodoro.format_time();
        button(
            row![
                text("🎯").size(11),
                px_space(2.0, 0.0),
                text(timer_str)
                    .size(10)
                    .color(Color::from_rgb(0.98, 0.60, 0.20)),
            ]
            .align_y(Alignment::Center),
        )
        .on_press(Message::SetIslandMode(IslandMode::ActiveFocus))
        .style(|_t, s| {
            let bg = match s {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .padding([2, 5])
        .into()
    } else {
        button(
            row![
                text("🔍")
                    .size(11)
                    .color(Color::from_rgba(1.0, 1.0, 1.0, 0.70)),
                text("Search").size(11).color(Color::WHITE),
                text("Ctrl+K")
                    .size(9)
                    .color(Color::from_rgba(1.0, 1.0, 1.0, 0.40)),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        )
        .on_press(Message::SetIslandMode(IslandMode::Search))
        .style(|_t, s| {
            let bg = match s {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.14),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .padding([2, 6])
        .into()
    };

    // Center: Hardware Camera / Sensor Pinhole Dot
    let hardware_camera_dot = container(row![])
        .width(7)
        .height(7)
        .style(|_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.01, 0.01, 0.02))),
            border: iced::Border {
                radius: iced::border::Radius::from(3.5),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                width: 0.5,
            },
            ..Default::default()
        });

    // Right Activity Pill: Streak + Quick Settings
    let streak_text = format!("🔥 {}d", core.today.streak.count.max(1));
    let streak_badge = text(streak_text)
        .size(10)
        .color(Color::from_rgb(0.98, 0.57, 0.24));

    let control_btn = button(text("⚙️").size(11).color(Color::WHITE))
        .on_press(Message::SetIslandMode(IslandMode::ControlCenter))
        .style(|_t, s| {
            let bg = match s {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .padding([2, 4]);

    row![
        left_activity,
        h_space(),
        hardware_camera_dot,
        h_space(),
        streak_badge,
        control_btn,
    ]
    .spacing(6)
    .padding([2, 10])
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .into()
}

fn render_active_focus<'a>(core: &'a MindSetCoreState) -> Element<'a, Message> {
    let timer_str = core.pomodoro.format_time();
    let is_paused = !core.pomodoro.is_running;

    let pulse_badge = text("🎯").size(13);
    let time_badge = text(timer_str)
        .size(13)
        .color(Color::from_rgb(0.16, 0.72, 0.45)); // Emerald Green

    let pause_btn = button(
        text(if is_paused { "▶ Resume" } else { "⏸ Pause" })
            .size(10)
            .color(Color::WHITE),
    )
    .on_press(Message::TogglePomodoro)
    .style(|_t, s| {
        let bg = match s {
            button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.20),
            _ => Color::from_rgba(1.0, 1.0, 1.0, 0.10),
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(6.0),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .padding([2, 8]);

    let exit_btn = button(
        text("✕ Exit Focus")
            .size(10)
            .color(Color::from_rgb(0.95, 0.40, 0.40)),
    )
    .on_press(Message::ToggleTunnelVision)
    .style(|_t, s| {
        let bg = match s {
            button::Status::Hovered => Color::from_rgba(0.95, 0.40, 0.40, 0.25),
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(6.0),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .padding([2, 8]);

    row![
        pulse_badge,
        time_badge,
        text("Deep Focus")
            .size(10)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.70)),
        h_space(),
        pause_btn,
        exit_btn,
    ]
    .spacing(8)
    .padding([2, 12])
    .align_y(Alignment::Center)
    .into()
}

fn render_search<'a>(search_query: &'a str, capsules: &'a [CapsuleRecord]) -> Element<'a, Message> {
    let input = text_input("Search syllabus, formulas, notes, or apps...", search_query)
        .on_input(Message::IslandSearchChanged)
        .style(|_t, _s| text_input::Style {
            background: iced::Background::Color(Color::TRANSPARENT),
            border: iced::Border::default(),
            icon: Color::from_rgba(1.0, 1.0, 1.0, 0.50),
            placeholder: Color::from_rgba(1.0, 1.0, 1.0, 0.40),
            value: Color::WHITE,
            selection: Color::from_rgba(0.16, 0.71, 0.91, 0.40),
        })
        .padding(6)
        .width(Length::Fill);

    let close_btn = button(text("✕").size(11).color(Color::WHITE))
        .on_press(Message::SetIslandMode(IslandMode::Compact))
        .style(|_t, s| {
            let bg = match s {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.20),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .padding([2, 6]);

    let search_bar = row![text("🔍").size(13).color(Color::WHITE), input, close_btn]
        .spacing(6)
        .align_y(Alignment::Center);

    let q = search_query.trim().to_lowercase();
    let mut results_col = column![].spacing(4);

    if !q.is_empty() {
        let matching_caps = capsules
            .iter()
            .filter(|c| c.unit.to_lowercase().contains(&q) || c.subject.to_lowercase().contains(&q))
            .take(3);

        for cap in matching_caps {
            let cap_btn = button(
                row![
                    text("⚡").size(11).color(Color::from_rgb(0.98, 0.80, 0.08)),
                    column![
                        text(&cap.unit).size(10).color(Color::WHITE),
                        text(format!("{} • {} mins", cap.subject, cap.estimated_minutes))
                            .size(8)
                            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.60)),
                    ]
                    .spacing(1),
                    h_space(),
                    text("Launch →")
                        .size(9)
                        .color(Color::from_rgb(0.16, 0.71, 0.91)),
                ]
                .align_y(Alignment::Center),
            )
            .on_press(Message::OpenWindow(WindowId::StudyReader))
            .style(|_t, s| {
                let bg = match s {
                    button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                    _ => Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(6.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .padding([4, 8])
            .width(Length::Fill);

            results_col = results_col.push(cap_btn);
        }

        let app_shortcuts = [
            ("Maindy AI Tutor", WindowId::LayaAiChat, "🧠"),
            ("Study Session Reader", WindowId::StudyReader, "📖"),
            ("Focus Quotas & Tasks", WindowId::Tasks, "📋"),
            ("Student File Vault", WindowId::FileVault, "📁"),
        ];

        for (name, wid, icon) in app_shortcuts {
            if name.to_lowercase().contains(&q) {
                let app_btn = button(
                    row![
                        text(icon).size(11),
                        text(name).size(10).color(Color::WHITE),
                        h_space(),
                        text("Open")
                            .size(9)
                            .color(Color::from_rgb(0.16, 0.71, 0.91)),
                    ]
                    .align_y(Alignment::Center),
                )
                .on_press(Message::OpenWindow(wid))
                .style(|_t, s| {
                    let bg = match s {
                        button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                        _ => Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                    };
                    button::Style {
                        background: Some(iced::Background::Color(bg)),
                        border: iced::Border {
                            radius: iced::border::Radius::from(6.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .padding([4, 8])
                .width(Length::Fill);

                results_col = results_col.push(app_btn);
            }
        }
    }

    let scroll_results = scrollable(results_col).height(160).width(Length::Fill);

    column![search_bar, px_space(0.0, 4.0), scroll_results]
        .spacing(4)
        .padding(10)
        .into()
}

fn render_control_center<'a>(
    profile: &'a UserProfile,
    is_audio_playing: bool,
) -> Element<'a, Message> {
    // 1. Audio Row
    let (audio_lbl, audio_color) = if is_audio_playing {
        (
            "🎧 Rainfall Lo-Fi: Active",
            Color::from_rgb(0.16, 0.72, 0.45),
        )
    } else {
        (
            "🔇 Lo-Fi Ambient: Muted",
            Color::from_rgba(1.0, 1.0, 1.0, 0.70),
        )
    };
    let audio_toggle_btn = button(text(audio_lbl).size(10).color(audio_color))
        .on_press(Message::ToggleAudio)
        .style(|_t, s| {
            let bg = match s {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                _ => Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .padding([4, 10]);

    // 2. Theme Mode Row
    let is_light = profile.theme_mode == "AlpineMatteLight";
    let light_btn = button(text("☀️ Light").size(9).color(if is_light {
        Color::WHITE
    } else {
        Color::from_rgba(1.0, 1.0, 1.0, 0.50)
    }))
    .on_press(Message::SetThemeMode("AlpineMatteLight".to_string()))
    .style(move |_t, _s| button::Style {
        background: Some(iced::Background::Color(if is_light {
            Color::from_rgb(0.08, 0.56, 0.78)
        } else {
            Color::TRANSPARENT
        })),
        border: iced::Border {
            radius: iced::border::Radius::from(5.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .padding([3, 8]);

    let dark_btn = button(text("🌙 Dark").size(9).color(if !is_light {
        Color::WHITE
    } else {
        Color::from_rgba(1.0, 1.0, 1.0, 0.50)
    }))
    .on_press(Message::SetThemeMode("ObsidianMatteDark".to_string()))
    .style(move |_t, _s| button::Style {
        background: Some(iced::Background::Color(if !is_light {
            Color::from_rgb(0.08, 0.56, 0.78)
        } else {
            Color::TRANSPARENT
        })),
        border: iced::Border {
            radius: iced::border::Radius::from(5.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .padding([3, 8]);

    let theme_row = row![
        text("Theme:")
            .size(10)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.70)),
        light_btn,
        dark_btn,
        h_space(),
        text("Motion:")
            .size(10)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.70)),
        button(
            text(if profile.mild_motion_enabled {
                "🌊 Dynamic (18 FPS)"
            } else {
                "🌿 Eco Static"
            })
            .size(9)
            .color(Color::WHITE)
        )
        .on_press(Message::SetMildMotion(!profile.mild_motion_enabled))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                1.0, 1.0, 1.0, 0.12
            ))),
            border: iced::Border {
                radius: iced::border::Radius::from(5.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .padding([3, 8]),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    // 3. Wallpaper Presets + Custom Button
    let wp_lake = button(text("🏔️ Alpine").size(9).color(Color::WHITE))
        .on_press(Message::SetWallpaperPreset(
            "assets/wallpaper.jpg".to_string(),
        ))
        .padding([3, 6]);
    let wp_custom = button(
        text("📁 Custom...")
            .size(9)
            .color(Color::from_rgb(0.16, 0.71, 0.91)),
    )
    .on_press(Message::SelectCustomWallpaper)
    .padding([3, 6]);

    let wallpaper_row = row![
        text("Wallpaper:")
            .size(10)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.70)),
        wp_lake,
        wp_custom,
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    // 4. Map Theme Picker
    let map_themes = [
        ("Alpine", "AlpineTrail"),
        ("Golden", "GoldenValley"),
        ("Midnight", "MidnightStarpath"),
        ("Minimal", "MinimalMatte"),
    ];

    let mut map_theme_row = row![text("Odyssey:")
        .size(10)
        .color(Color::from_rgba(1.0, 1.0, 1.0, 0.70))]
    .spacing(4)
    .align_y(Alignment::Center);
    for (label, val) in map_themes {
        let is_sel = profile.map_theme == val;
        let btn = button(text(label).size(8).color(if is_sel {
            Color::WHITE
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.50)
        }))
        .on_press(Message::SetMapTheme(val.to_string()))
        .style(move |_t, _s| button::Style {
            background: Some(iced::Background::Color(if is_sel {
                Color::from_rgb(0.08, 0.56, 0.78)
            } else {
                Color::TRANSPARENT
            })),
            border: iced::Border {
                radius: iced::border::Radius::from(4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .padding([2, 5]);
        map_theme_row = map_theme_row.push(btn);
    }

    // Done button
    let close_btn = button(text("▲ Collapse").size(10).color(Color::WHITE))
        .on_press(Message::SetIslandMode(IslandMode::Compact))
        .style(|_t, s| {
            let bg = match s {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.20),
                _ => Color::from_rgba(1.0, 1.0, 1.0, 0.10),
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .padding([4, 12]);

    column![
        row![
            text("🎛️ Control Center").size(11).color(Color::WHITE),
            h_space(),
            close_btn
        ]
        .align_y(Alignment::Center),
        px_space(0.0, 2.0),
        audio_toggle_btn,
        theme_row,
        wallpaper_row,
        map_theme_row,
    ]
    .spacing(8)
    .padding(12)
    .into()
}

fn render_maindy_speaking<'a>() -> Element<'a, Message> {
    let bar = |h: f32| {
        container(px_space(3.0, h)).style(|_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.38, 0.72, 1.0))),
            border: iced::Border {
                radius: iced::border::Radius::from(2.0),
                ..Default::default()
            },
            ..Default::default()
        })
    };

    let bars = row![bar(8.0), bar(16.0), bar(12.0), bar(20.0), bar(14.0), bar(10.0)]
        .spacing(3)
        .align_y(Alignment::Center);

    let stop_btn = button(
        row![text("⏹").size(10).color(Color::WHITE), text("Stop").size(10).color(Color::WHITE)]
            .spacing(4)
            .align_y(Alignment::Center),
    )
    .on_press(Message::StopTts)
    .style(|_t, s| {
        let bg = match s {
            button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.22),
            _ => Color::from_rgba(1.0, 1.0, 1.0, 0.12),
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(6.0),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.25),
                width: 1.0,
            },
            ..Default::default()
        }
    })
    .padding([2, 8]);

    row![
        bars,
        px_space(6.0, 0.0),
        text("Maindy Speaking...").size(11).color(Color::WHITE),
        h_space(),
        stop_btn,
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .padding(iced::Padding {
        top: 0.0,
        right: 12.0,
        bottom: 0.0,
        left: 12.0,
    })
    .width(Length::Fill)
    .into()
}
