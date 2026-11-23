//! Application & Syllabus Launcher Modal (Ctrl+Space)
//!
//! Pixel-faithful reproduction of the central Start Menu / Launcher matching reference:
//! - Top Search Bar: "🔍 Search for apps, files, and settings" in rounded translucent input
//! - AI Hub Hero Card: Vibrant silk gradient wave with "AI Hub" + "Ask" CTA, and mini weather/date on right
//! - Pinned Section: MindSet OS Official 8-App Suite
//! - Recommended Section: Recent study materials & capsules
//! - Bottom Action Bar: "⊞ All Apps", "📁 Documents", "🖼️ Pictures", and right-aligned "⏻ Power".
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length};
use mindset_core::{ActiveOverlay, MindSetCoreState, WindowId};

use crate::theme::{
    glass_elevated, h_space, modern_text_input, primary_button, px_space, secondary_button,
    ThemePalette, TYPE_BODY, TYPE_CAPTION, TYPE_METRIC, TYPE_SUBHEAD,
};
use crate::Message;

pub fn view<'a>(
    _core: &'a MindSetCoreState,
    palette: ThemePalette,
    search_query: &'a str,
) -> Element<'a, Message> {
    // 1. Search Bar Input in a Rounded Translucent Pill
    let search_input = text_input(
        "Search for apps, study capsules, and settings...",
        search_query,
    )
    .on_input(Message::SearchInputChanged)
    .style(modern_text_input(palette))
    .padding(8)
    .width(Length::Fill);

    let search_bar = container(
        row![
            text("🔍").size(TYPE_SUBHEAD).color(palette.text_secondary),
            search_input,
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([2, 8])
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(
            1.0, 1.0, 1.0, 0.25,
        ))),
        border: iced::Border {
            radius: iced::border::Radius::from(10.0),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.40),
            width: 1.0,
        },
        ..Default::default()
    });

    // 2. Search Query Filtering: If user typed a search, show matching apps, capsules, and files
    let q = search_query.trim().to_lowercase();
    if !q.is_empty() {
        let all_searchable = [
            // Student Apps
            (
                "App",
                "🌅",
                "Today's Capsule",
                "Daily study goals, milestones & focus progress",
                Message::OpenWindow(WindowId::Today),
            ),
            (
                "App",
                "🧠",
                "Maindy AI",
                "Local Ollama Feynman tutor & academic mentor",
                Message::OpenWindow(WindowId::LayaAiChat),
            ),
            (
                "App",
                "🌐",
                "MindSet Browser",
                "Real Chromium with dedicated student profile and academic study bookmarks bar",
                Message::LaunchMindSetBrowser(None),
            ),
            (
                "App",
                "📺",
                "YouTube Study",
                "Real standalone YouTube desktop player app with zero browser tabs",
                Message::LaunchYouTubeApp,
            ),
            (
                "App",
                "🛡️",
                "FreeTube (Ad-Free)",
                "Pre-installed ad-free and distraction-free private YouTube player",
                Message::LaunchFreeTubeApp,
            ),
            (
                "App",
                "📖",
                "Study Reader",
                "NCERT textbooks, notes & capsule reader",
                Message::OpenWindow(WindowId::StudyReader),
            ),
            (
                "App",
                "📁",
                "File Vault",
                "Student notes, formula sheets & local capsules",
                Message::OpenWindow(WindowId::FileVault),
            ),
            (
                "App",
                "📋",
                "Syllabus Quotas",
                "Topic completion sprint quotas & exam targets",
                Message::OpenWindow(WindowId::Tasks),
            ),
            (
                "App",
                "⚔️",
                "Verification Boss",
                "3-Stage Feynman mastery boss challenge",
                Message::OpenWindow(WindowId::ProgressStats),
            ),
            (
                "App",
                "📊",
                "System Monitor",
                "CPU, 4-8 GB RAM discipline & system doctor",
                Message::OpenWindow(WindowId::SystemMonitor),
            ),
            (
                "App",
                "⚙️",
                "Preferences",
                "Appearance, themes, audio & hardware settings",
                Message::OpenWindow(WindowId::Settings),
            ),
            // Study Capsules
            (
                "Capsule",
                "📑",
                "Electrochemistry — Daniell Cell & Nernst Equation",
                "Study Capsule • Standard EMF, Nernst equation concentration quotients",
                Message::OpenWindow(WindowId::StudyReader),
            ),
            (
                "Capsule",
                "⚡",
                "Gauss's Law & Electric Flux Derivation",
                "Study Capsule • High-yield electrostatics derivations & Gaussian surfaces",
                Message::OpenWindow(WindowId::StudyReader),
            ),
            (
                "Capsule",
                "🔬",
                "Ray Optics & Thin Lens Formula",
                "Study Capsule • Focal length, magnification & refraction indices",
                Message::OpenWindow(WindowId::StudyReader),
            ),
            (
                "Capsule",
                "🧬",
                "Genetics & Mendelian Dihybrid Cross",
                "Study Capsule • Punnett squares, 9:3:3:1 phenotypic ratios",
                Message::OpenWindow(WindowId::StudyReader),
            ),
            (
                "Capsule",
                "📐",
                "Matrices & Determinants Matrix Inversion",
                "Study Capsule • Adjoint methods, Cramers rule & linear systems",
                Message::OpenWindow(WindowId::StudyReader),
            ),
            // Vault Files
            (
                "Vault",
                "📝",
                "Physics_HighYield_Formulas.txt",
                "File Vault • High-yield Physics equations and constants",
                Message::OpenWindow(WindowId::FileVault),
            ),
            (
                "Vault",
                "🎯",
                "Syllabus_Mastery_Plan.txt",
                "File Vault • Academic goals, TN Class 12 Boards & JEE plan",
                Message::OpenWindow(WindowId::FileVault),
            ),
            (
                "Vault",
                "📘",
                "README_STUDY_VAULT.txt",
                "File Vault • Local offline vault guide and sync details",
                Message::OpenWindow(WindowId::FileVault),
            ),
            (
                "Vault",
                "📚",
                "NCERT_Sample_Problems.txt",
                "File Vault • Worked textbook exercises and solutions",
                Message::OpenWindow(WindowId::FileVault),
            ),
        ];

        let mut results = column![].spacing(6);
        let mut count = 0;
        for (tag, icon, name, desc, msg) in all_searchable {
            if name.to_lowercase().contains(&q)
                || desc.to_lowercase().contains(&q)
                || tag.to_lowercase().contains(&q)
            {
                count += 1;
                let item = button(
                    row![
                        text(icon).size(TYPE_SUBHEAD),
                        column![
                            row![
                                text(name).size(TYPE_BODY).color(palette.text_primary),
                                text(format!(" • {}", tag)).size(TYPE_CAPTION).color(palette.accent),
                            ]
                            .spacing(4)
                            .align_y(Alignment::Center),
                            text(desc).size(TYPE_CAPTION).color(palette.text_secondary),
                        ]
                        .spacing(2),
                        h_space(),
                        text("Open →").size(TYPE_CAPTION).color(palette.accent),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                )
                .on_press(msg)
                .style(secondary_button(palette))
                .padding([6, 12])
                .width(Length::Fill);

                results = results.push(item);
            }
        }

        if count == 0 {
            results = results.push(
                container(
                    text(format!(
                        "No applications or capsules matching \"{}\"",
                        search_query
                    ))
                    .size(TYPE_BODY)
                    .color(palette.text_muted),
                )
                .padding(16),
            );
        }

        let search_results_card = container(
            column![
                search_bar,
                px_space(0.0, 8.0),
                text(format!("Search Results ({})", count))
                    .size(TYPE_BODY)
                    .color(palette.text_secondary),
                scrollable(results).height(280),
            ]
            .spacing(8)
            .padding(16),
        )
        .width(460)
        .style(move |_t| glass_elevated(palette));

        return search_results_card.into();
    }

    // 3. AI Hub Banner + Dynamic Weather & Date
    let now = chrono::Local::now();
    let date_str = now.format("%A, %B %-d").to_string();

    let ai_hub_card = container(
        row![
            // Left gradient pill: AI Hub with Ask button on smooth silk wave
            container(
                row![
                    column![
                        text("Maindy AI").size(TYPE_BODY).color(Color::WHITE),
                        button(text("Ask Coach").size(TYPE_CAPTION).color(Color::WHITE))
                            .on_press(Message::OpenWindow(WindowId::LayaAiChat))
                            .style(primary_button(palette))
                            .padding([2, 10]),
                    ]
                    .spacing(2),
                    h_space(),
                    text("🧠").size(TYPE_METRIC),
                ]
                .align_y(Alignment::Center),
            )
            .width(180)
            .padding([8, 12])
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.35, 0.82))),
                border: iced::Border {
                    radius: iced::border::Radius::from(10.0),
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.45),
                    width: 1.0,
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.08, 0.24, 0.65, 0.30),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                },
                ..Default::default()
            }),
            h_space(),
            // Right mini weather & date
            row![
                text("☀️").size(TYPE_METRIC),
                column![
                    text(date_str).size(TYPE_CAPTION).color(palette.text_primary),
                    text("22° Study Mode Active")
                        .size(TYPE_CAPTION)
                        .color(palette.text_secondary),
                ]
                .spacing(1),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .align_y(Alignment::Center)
        .padding(4),
    )
    .width(Length::Fill);

    // 4. Pinned Section: 10-App MindSet OS Academic Suite
    let pinned_title = text("Pinned Applications").size(TYPE_BODY).color(palette.text_secondary);

    let row1 = row![
        app_pill(
            WindowId::Today,
            "🌅",
            "Today's Capsule",
            Color::from_rgb(0.00, 0.65, 0.80),
            palette
        ),
        app_pill(
            WindowId::LayaAiChat,
            "🧠",
            "Maindy AI",
            Color::from_rgb(0.42, 0.38, 0.98),
            palette
        ),
        app_pill_msg(
            Message::LaunchMindSetBrowser(None),
            "🌐",
            "Browser",
            Color::from_rgb(0.08, 0.56, 0.78),
            palette,
        ),
        app_pill_msg(
            Message::LaunchYouTubeApp,
            "📺",
            "YouTube",
            Color::from_rgb(0.88, 0.20, 0.20),
            palette,
        ),
        app_pill_msg(
            Message::LaunchFreeTubeApp,
            "🛡️",
            "FreeTube",
            Color::from_rgb(0.85, 0.35, 0.12),
            palette,
        ),
        app_pill(
            WindowId::StudyReader,
            "📖",
            "Reader",
            Color::from_rgb(0.12, 0.38, 0.85),
            palette
        ),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let row2 = row![
        app_pill(
            WindowId::FileVault,
            "📁",
            "Vault",
            Color::from_rgb(0.95, 0.72, 0.12),
            palette
        ),
        app_pill(
            WindowId::Tasks,
            "📋",
            "Quotas",
            Color::from_rgb(0.16, 0.72, 0.45),
            palette
        ),
        app_pill(
            WindowId::ProgressStats,
            "⚔️",
            "Boss Fight",
            Color::from_rgb(0.95, 0.52, 0.14),
            palette
        ),
        app_pill(
            WindowId::SystemMonitor,
            "📊",
            "Monitor",
            Color::from_rgb(0.18, 0.55, 0.92),
            palette
        ),
        app_pill(
            WindowId::Settings,
            "⚙️",
            "Preferences",
            Color::from_rgb(0.45, 0.52, 0.62),
            palette
        ),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let pinned_section = column![pinned_title, row1, row2].spacing(8);

    // 5. Recommended Study Materials & Capsules
    let recommended_title = text("Recommended Study Materials").size(TYPE_BODY).color(palette.text_secondary);
    let rec1 = recent_item_row(
        "📝",
        "Physics_HighYield_Formulas.txt",
        "Study Vault • High-Yield Physics Formulas",
        palette,
        Message::OpenWindow(WindowId::FileVault),
    );
    let rec2 = recent_item_row(
        "📑",
        "Electrochemistry_Daniell_Cell.capsule",
        "Active Capsule • Standard EMF & Nernst Equation",
        palette,
        Message::OpenWindow(WindowId::StudyReader),
    );
    let rec3 = recent_item_row(
        "⚡",
        "Gauss_Law_and_Flux.capsule",
        "Odyssey Trail • Stage 1 Derivations & Flux",
        palette,
        Message::OpenWindow(WindowId::StudyReader),
    );
    let rec4 = recent_item_row(
        "🎯",
        "Syllabus_Mastery_Plan.txt",
        "Roadmap • TN Class 12 Boards & JEE 590/600",
        palette,
        Message::OpenWindow(WindowId::FileVault),
    );

    let recommended_section = column![recommended_title, rec1, rec2, rec3, rec4].spacing(4);

    // 6. Footer Action Bar with all buttons wired
    let footer = container(
        row![
            button(
                row![text("⊞").size(TYPE_BODY), text("All Apps").size(TYPE_CAPTION)]
                    .spacing(4)
                    .align_y(Alignment::Center),
            )
            .on_press(Message::OpenWindow(WindowId::Today))
            .style(secondary_button(palette))
            .padding([4, 8]),
            button(
                row![text("📁").size(TYPE_BODY), text("Vault").size(TYPE_CAPTION)]
                    .spacing(4)
                    .align_y(Alignment::Center),
            )
            .on_press(Message::OpenWindow(WindowId::FileVault))
            .style(secondary_button(palette))
            .padding([4, 8]),
            button(
                row![text("🎯").size(TYPE_BODY), text("Focus Mode").size(TYPE_CAPTION)]
                    .spacing(4)
                    .align_y(Alignment::Center),
            )
            .on_press(Message::ToggleTunnelVision)
            .style(secondary_button(palette))
            .padding([4, 8]),
            h_space(),
            button(
                row![text("⏻").size(TYPE_BODY), text("Power").size(TYPE_CAPTION)]
                    .spacing(4)
                    .align_y(Alignment::Center),
            )
            .on_press(Message::ToggleOverlay(ActiveOverlay::PowerMenu))
            .style(secondary_button(palette))
            .padding([4, 8]),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([6, 0]);

    let dialog_content = column![
        search_bar,
        ai_hub_card,
        pinned_section,
        recommended_section,
        footer,
    ]
    .spacing(12)
    .padding(16);

    container(dialog_content)
        .width(460)
        .style(move |_t| glass_elevated(palette))
        .into()
}

// ----------------------------------------------------------------------------
// Helpers: App Pill & Recent Item Row
// ----------------------------------------------------------------------------
fn app_pill_msg<'a>(
    msg: Message,
    icon: &'static str,
    name: &'static str,
    bg_tint: Color,
    palette: ThemePalette,
) -> Element<'a, Message> {
    button(
        column![
            container(text(icon).size(TYPE_METRIC))
                .width(44)
                .height(44)
                .center_x(44)
                .center_y(44)
                .style(move |_t| container::Style {
                    background: Some(iced::Background::Color(bg_tint)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(9.0),
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.40),
                        width: 1.0,
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                        offset: iced::Vector::new(0.0, 3.0),
                        blur_radius: 8.0,
                    },
                    ..Default::default()
                }),
            text(name).size(TYPE_CAPTION).color(palette.text_primary),
        ]
        .spacing(4)
        .align_x(Alignment::Center),
    )
    .on_press(msg)
    .style(move |_t, s| {
        let bg = match s {
            button::Status::Hovered => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.20),
            _ => iced::Color::TRANSPARENT,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(8.0),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .padding(6)
    .into()
}

fn app_pill<'a>(
    win_id: WindowId,
    icon: &'static str,
    name: &'static str,
    bg_tint: Color,
    palette: ThemePalette,
) -> Element<'a, Message> {
    button(
        column![
            container(text(icon).size(TYPE_METRIC))
                .width(44)
                .height(44)
                .center_x(44)
                .center_y(44)
                .style(move |_t| container::Style {
                    background: Some(iced::Background::Color(bg_tint)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(9.0),
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.40),
                        width: 1.0,
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                        offset: iced::Vector::new(0.0, 3.0),
                        blur_radius: 8.0,
                    },
                    ..Default::default()
                }),
            text(name).size(TYPE_CAPTION).color(palette.text_primary),
        ]
        .spacing(4)
        .align_x(Alignment::Center),
    )
    .on_press(Message::OpenWindow(win_id))
    .style(move |_t, s| {
        let bg = match s {
            button::Status::Hovered => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.20),
            _ => iced::Color::TRANSPARENT,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(8.0),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .padding([4, 6])
    .into()
}

fn recent_item_row<'a>(
    icon: &'static str,
    title: &'static str,
    subtitle: &'static str,
    palette: ThemePalette,
    on_press: Message,
) -> Element<'a, Message> {
    button(
        row![
            container(text(icon).size(TYPE_SUBHEAD))
                .width(26)
                .height(26)
                .center_x(26)
                .center_y(26)
                .style(move |_t| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(
                        1.0, 1.0, 1.0, 0.25
                    ))),
                    border: iced::Border {
                        radius: iced::border::Radius::from(5.0),
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.35),
                        width: 1.0,
                    },
                    ..Default::default()
                }),
            column![
                text(title).size(TYPE_BODY).color(palette.text_primary),
                text(subtitle).size(TYPE_CAPTION).color(palette.text_muted),
            ]
            .spacing(1),
            h_space(),
            text("↗").size(TYPE_CAPTION).color(palette.text_muted),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .on_press(on_press)
    .style(move |_t, s| {
        let bg = match s {
            button::Status::Hovered => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.18),
            _ => iced::Color::TRANSPARENT,
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
    .padding([4, 6])
    .width(Length::Fill)
    .into()
}
