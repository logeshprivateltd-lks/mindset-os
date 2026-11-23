//! Floating Bottom Dock for MindSet OS
//!
//! Official v1.0 8-Item Application Dock:
//! 1. ⊞ Launcher (Toggles App Launcher Modal)
//! 2. ☀️ Deep Focus (Today Dashboard & Capsule Gate)
//! 3. 🧠 Maindy AI (Local Ollama Tutor & Mentor)
//! 4. 📖 Study Reader (NCERT & Derivation Sessions)
//! 5. 📋 Focus Arena (Task Quotas & Syllabus Sprints)
//! 6. 🌐 MindStrict Browser (Distraction-Free Web)
//! 7. 📁 Study Vault (Notes & Formula Sheets)
//! 8. ⚙️ Settings (MindSet OS Preferences & Profile)
//!
//! Features active dot indicators, high-opacity liquid frosted glass,
//! and safe-area clamping above screen edge.

use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Alignment, Color, Element};
use mindset_core::{ActiveOverlay, MindSetCoreState, WindowId, WindowMode};

use crate::theme::neumorphism::{neu_card, NeuPalette};
use crate::theme::{px_space, ThemePalette};
use crate::Message;

pub fn view<'a>(
    core: &'a MindSetCoreState,
    dock_scales: &[f32],
    palette: ThemePalette,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let mut dock_items = row![].spacing(8).align_y(Alignment::End);

    // 0. Home / Launcher Button
    let scale_0 = dock_scales.first().copied().unwrap_or(1.0);
    let box_size_0 = (36.0 * scale_0).round();
    let font_size_0 = (18.0 * scale_0).round();

    let is_launcher_active = core.wm.active_overlay == ActiveOverlay::Launcher;
    let home_badge = container(text("⊞").size(font_size_0).color(Color::WHITE))
        .width(box_size_0)
        .height(box_size_0)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.48, 0.96))),
            border: iced::Border {
                radius: iced::border::Radius::from(9.0 * scale_0),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.40),
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 6.0,
            },
            ..Default::default()
        });

    let home_indicator = if is_launcher_active {
        text("●").size(6).color(palette.accent)
    } else {
        text(" ").size(6).color(Color::TRANSPARENT)
    };

    let home_btn = button(
        column![home_badge, home_indicator]
            .spacing(2)
            .align_x(Alignment::Center),
    )
    .on_press(Message::ToggleOverlay(ActiveOverlay::Launcher))
    .style(move |_t, s| {
        let bg = match s {
            button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.20),
            button::Status::Pressed => Color::from_rgba(1.0, 1.0, 1.0, 0.35),
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(10.0),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .padding([2, 4]);

    let home_area = mouse_area(home_btn)
        .on_enter(Message::DockHovered(Some(0)))
        .on_exit(Message::DockHovered(None));

    dock_items = dock_items.push(home_area);

    // Subtle macOS-style shelf divider between launcher and apps
    let home_divider = container(px_space(1.0, 22.0)).style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.18))),
        border: iced::Border {
            radius: iced::border::Radius::from(0.5),
            ..Default::default()
        },
        ..Default::default()
    });
    dock_items = dock_items.push(home_divider);

    // Official 9 Pinned Core Applications (Indices 1..=9)
    let apps = [
        (
            WindowId::Today,
            "☀️",
            Color::from_rgb(0.00, 0.65, 0.80),
            "Deep Focus",
            Message::OpenWindow(WindowId::Today),
        ),
        (
            WindowId::StudyReader,
            "📖",
            Color::from_rgb(0.12, 0.38, 0.85),
            "Study Reader",
            Message::OpenWindow(WindowId::StudyReader),
        ),
        (
            WindowId::LayaAiChat,
            "🧠",
            Color::from_rgb(0.92, 0.45, 0.25),
            "Maindy AI",
            Message::OpenWindow(WindowId::LayaAiChat),
        ),
        (
            WindowId::Tasks,
            "📋",
            Color::from_rgb(0.12, 0.75, 0.38),
            "Focus Arena",
            Message::OpenWindow(WindowId::Tasks),
        ),
        (
            WindowId::SearchBrowser,
            "🌐",
            Color::from_rgb(0.00, 0.48, 0.95),
            "MindStrict",
            Message::LaunchMindSetBrowser(None),
        ),
        (
            WindowId::SearchBrowser,
            "📺",
            Color::from_rgb(0.88, 0.20, 0.20),
            "YouTube Study",
            Message::LaunchYouTubeApp,
        ),
        (
            WindowId::FileVault,
            "📁",
            Color::from_rgb(0.95, 0.72, 0.12),
            "Study Vault",
            Message::OpenWindow(WindowId::FileVault),
        ),
        (
            WindowId::SystemMonitor,
            "💻",
            Color::from_rgb(0.45, 0.30, 0.85),
            "Sys Monitor",
            Message::OpenWindow(WindowId::SystemMonitor),
        ),
        (
            WindowId::Settings,
            "⚙️",
            Color::from_rgb(0.55, 0.55, 0.60),
            "Settings",
            Message::OpenWindow(WindowId::Settings),
        ),
    ];

    for (idx_offset, (id, icon, badge_bg, _name, press_msg)) in apps.into_iter().enumerate() {
        // macOS-style shelf divider before system utilities (Sys Monitor & Settings)
        if idx_offset == 7 {
            let util_divider = container(px_space(1.0, 22.0)).style(move |_t| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.18))),
                border: iced::Border {
                    radius: iced::border::Radius::from(0.5),
                    ..Default::default()
                },
                ..Default::default()
            });
            dock_items = dock_items.push(util_divider);
        }

        let item_idx = idx_offset + 1;
        let scale = dock_scales.get(item_idx).copied().unwrap_or(1.0);
        let box_size = (36.0 * scale).round();
        let font_size = (16.0 * scale).round();

        let is_open = core.wm.is_open(id);
        let is_active = core.wm.active_window() == Some(id);
        let win = core.wm.get_window(id);
        let is_minimized = win.is_some_and(|w| w.mode == WindowMode::Minimized);

        let indicator = if is_active {
            text("●").size(7).color(palette.accent)
        } else if is_open && !is_minimized {
            text("●").size(6).color(Color::from_rgba(1.0, 1.0, 1.0, 0.70))
        } else if is_open && is_minimized {
            text("○").size(6).color(Color::from_rgba(1.0, 1.0, 1.0, 0.35))
        } else {
            text(" ").size(6).color(Color::TRANSPARENT)
        };

        let badge = container(text(icon).size(font_size))
            .width(box_size)
            .height(box_size)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(badge_bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(9.0 * scale),
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.35),
                    width: 1.0,
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.18),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 6.0,
                },
                ..Default::default()
            });

        let item_content = column![badge, indicator]
            .spacing(2)
            .align_x(Alignment::Center);

        let btn = button(item_content)
            .on_press(press_msg)
            .style(move |_t, s| {
                let bg = match s {
                    button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.20),
                    button::Status::Pressed => Color::from_rgba(1.0, 1.0, 1.0, 0.35),
                    _ => Color::TRANSPARENT,
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .padding([2, 4]);

        let area = mouse_area(btn)
            .on_enter(Message::DockHovered(Some(item_idx)))
            .on_exit(Message::DockHovered(None));

        dock_items = dock_items.push(area);
    }

    let dock_capsule = container(neu_card(dock_items, neu, 22.0, 1.0)).padding([2, 4]);

    dock_capsule.into()
}
