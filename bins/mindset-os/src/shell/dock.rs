//! Floating Bottom Dock for MindSet OS
//!
//! Frosted Glass Bottom Dock with concentric rounded corners (outer 22 = inner 12 + padding 10),
//! spring hover magnification, and active window indicator dots:
//! 0. ⊞ Launcher (App Launcher Overlay)
//! 1. ☀️ Deep Focus (Today Dashboard)
//! 2. 📖 Study Reader (NCERT & Derivation Sessions)
//! 3. 🧠 Maindy AI (Local Tutor & Mentor)
//! 4. 🗺️ 3D Odyssey (Syllabus Odyssey Map)
//! 5. 📝 Scratchpad (Quick Formula & Study Scratchpad)
//! 6. 📋 Focus Arena (Task Quotas & Syllabus Sprints)
//! 7. 🌐 MindStrict (Distraction-Free Web)
//! 8. 📺 YouTube Study (Video Lectures)
//! 9. 📁 Study Vault (Notes & Formula Sheets)
//! 10. 💻 Sys Monitor (System Diagnostics)
//! 11. ⚙️ Settings (Preferences & Profile)

use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Alignment, Color, Element};
use mindset_core::{ActiveOverlay, MindSetCoreState, WindowId, WindowMode};

use crate::theme::neumorphism::NeuPalette;
use crate::theme::{frosted_glass_container, px_space, ThemePalette};
use crate::Message;

const DOCK_OUTER_RADIUS: f32 = 22.0;
const DOCK_SHELL_PADDING: f32 = 10.0;
const DOCK_INNER_ICON_RADIUS: f32 = DOCK_OUTER_RADIUS - DOCK_SHELL_PADDING; // Exact concentric 12.0px

pub fn view<'a>(
    core: &'a MindSetCoreState,
    dock_scales: &[f32],
    hovered_index: Option<usize>,
    palette: ThemePalette,
    _neu: &NeuPalette,
) -> Element<'a, Message> {
    let mut dock_items = row![].spacing(6).align_y(Alignment::End);

    // 0. Home / Launcher Button
    let scale_0 = dock_scales.first().copied().unwrap_or(1.0);
    let box_size_0 = (36.0 * scale_0).round();
    let font_size_0 = (17.0 * scale_0).round();

    let is_launcher_active = core.wm.active_overlay == ActiveOverlay::Launcher;
    let home_badge = container(text("⊞").size(font_size_0).color(Color::WHITE))
        .width(box_size_0)
        .height(box_size_0)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.48, 0.96))),
            border: iced::Border {
                radius: iced::border::Radius::from(DOCK_INNER_ICON_RADIUS * scale_0),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.42),
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.22),
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
            button::Status::Pressed => Color::from_rgba(1.0, 1.0, 1.0, 0.32),
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(DOCK_INNER_ICON_RADIUS),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .padding(0);

    let home_area = mouse_area(home_btn)
        .on_enter(Message::DockHovered(Some(0)))
        .on_exit(Message::DockHovered(None));

    dock_items = dock_items.push(home_area);

    // Subtle frosted divider between launcher and workspace apps
    let divider_color = palette.glass_border;
    let home_divider = container(px_space(1.0, 22.0)).style(move |_t| container::Style {
        background: Some(iced::Background::Color(divider_color)),
        border: iced::Border {
            radius: iced::border::Radius::from(0.5),
            ..Default::default()
        },
        ..Default::default()
    });
    dock_items = dock_items.push(home_divider);

    // Pinned Core Applications (Indices 1..=11) — includes OdysseyMap & Scratchpad as floating windows
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
            WindowId::OdysseyMap,
            "🗺️",
            Color::from_rgb(0.28, 0.44, 0.88),
            "3D Odyssey",
            Message::OpenWindow(WindowId::OdysseyMap),
        ),
        (
            WindowId::Scratchpad,
            "📝",
            Color::from_rgb(0.86, 0.56, 0.14),
            "Scratchpad",
            Message::OpenWindow(WindowId::Scratchpad),
        ),
        (
            WindowId::Tasks,
            "📋",
            Color::from_rgb(0.12, 0.72, 0.38),
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
            WindowId::YouTubeStudy,
            "📺",
            Color::from_rgb(0.88, 0.20, 0.20),
            "YouTube Study",
            Message::OpenWindow(WindowId::YouTubeStudy),
        ),
        (
            WindowId::MindStore,
            "🛍️",
            Color::from_rgb(0.16, 0.62, 0.88),
            "MindStore",
            Message::OpenWindow(WindowId::MindStore),
        ),
        (
            WindowId::FileVault,
            "📁",
            Color::from_rgb(0.90, 0.68, 0.12),
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
            Color::from_rgb(0.46, 0.50, 0.58),
            "Settings",
            Message::OpenWindow(WindowId::Settings),
        ),
    ];

    for (idx_offset, (id, icon, badge_bg, name, press_msg)) in apps.into_iter().enumerate() {
        // Subtle shelf divider before system utilities (Sys Monitor & Settings)
        if idx_offset == 9 {
            let util_divider = container(px_space(1.0, 22.0)).style(move |_t| container::Style {
                background: Some(iced::Background::Color(divider_color)),
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
            text("●").size(6).color(palette.text_secondary)
        } else if is_open && is_minimized {
            text("○").size(6).color(palette.text_muted)
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
                    radius: iced::border::Radius::from(DOCK_INNER_ICON_RADIUS * scale),
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.36),
                    width: 1.0,
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.18),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 6.0,
                },
                ..Default::default()
            });

        let tooltip: Element<'a, Message> = if hovered_index == Some(item_idx) {
            container(text(name).size(10).color(palette.text_primary))
                .padding([2, 8])
                .style(move |_t| crate::theme::frosted_glass_style(palette, 8.0, true))
                .into()
        } else {
            px_space(0.0, 0.0).into()
        };

        let item_content = column![tooltip, badge, indicator]
            .spacing(2)
            .align_x(Alignment::Center);

        let btn = button(item_content)
            .on_press(press_msg)
            .style(move |_t, s| {
                let bg = match s {
                    button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                    button::Status::Pressed => Color::from_rgba(1.0, 1.0, 1.0, 0.30),
                    _ => Color::TRANSPARENT,
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(DOCK_INNER_ICON_RADIUS),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .padding(0);

        let area = mouse_area(btn)
            .on_enter(Message::DockHovered(Some(item_idx)))
            .on_exit(Message::DockHovered(None));

        dock_items = dock_items.push(area);
    }

    let dock_inner = container(dock_items).padding(DOCK_SHELL_PADDING);
    frosted_glass_container(palette, DOCK_OUTER_RADIUS, true, dock_inner)
}
