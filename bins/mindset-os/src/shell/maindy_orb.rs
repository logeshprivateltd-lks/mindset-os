//! Floating Claymorphic Maindy AI Orb & Quick Command Bar (Screen 3)
//!
//! Clicking the floating Claymorphic Maindy orb from anywhere on MindSet OS opens
//! a compact command & quick-tutor overlay without forcing the full Studio window open.
//! Dismisses cleanly with `Escape` or outside click.

use iced::widget::{button, column, container, mouse_area, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length};
use mindset_core::WindowId;

use crate::storage::db::ChatMessageRecord;
use crate::theme::claymorphism::{clay_card, clay_orb, clay_tutor_card, ClayPalette};
use crate::theme::neumorphism::{neu_card, neu_inset, NeuPalette};
use crate::theme::{h_space, px_space};
use crate::Message;

pub fn view<'a>(
    scale: f32,
    is_drawer_open: bool,
    chat_input: &'a str,
    messages: &'a [ChatMessageRecord],
    is_generating: bool,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let clay = ClayPalette::default();
    let orb_size = (56.0 * scale).clamp(42.0, 72.0);

    // 1. Living Clay Floating Orb
    let orb_icon = text(if is_generating { "⚡" } else { "🧠" })
        .size((22.0 * scale).round())
        .color(Color::WHITE);
    let orb_badge = clay_orb(orb_icon, orb_size);

    let orb_btn = button(orb_badge)
        .on_press(Message::ToggleMaindyDrawer)
        .style(|_t, _s| button::Style {
            background: None,
            border: iced::Border::default(),
            ..Default::default()
        })
        .padding(0);

    let interactive_orb = mouse_area(orb_btn)
        .on_enter(Message::MaindyOrbHovered(true))
        .on_exit(Message::MaindyOrbHovered(false));

    // 2. Screen 3: System-Wide Quick Orb & Command Bar Overlay
    if is_drawer_open {
        let header = row![
            text("🧠").size(16),
            column![
                text("Maindy Quick Command").size(12).color(Color::WHITE),
                text(if is_generating {
                    "Thinking & preparing study steps..."
                } else {
                    "Ask a concept or control MindSet OS"
                })
                .size(9)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.88)),
            ]
            .spacing(1),
            h_space(),
            button(text("✕").size(11).color(Color::WHITE))
                .on_press(Message::ToggleMaindyDrawer)
                .style(|_t, s| {
                    let bg = match s {
                        button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.25),
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
                .padding([2, 6]),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let header_clay = clay_card(header, 20.0);

        // Quick OS & Capsule command chips
        let quick_chip = |label: &'a str, cmd: &'a str| {
            let txt_col = neu.text_primary;
            let bg_norm = neu.surface_elevated;
            let border_col = Color {
                a: 0.25,
                ..neu.shadow_dark
            };
            button(text(label).size(9).color(txt_col))
                .on_press(Message::RunQuickOrbCommand(cmd.to_string()))
                .style(move |_t, s| button::Style {
                    background: Some(iced::Background::Color(if s == button::Status::Hovered {
                        Color { a: 0.95, ..bg_norm }
                    } else {
                        bg_norm
                    })),
                    border: iced::Border {
                        radius: iced::border::Radius::from(8.0),
                        color: border_col,
                        width: 1.0,
                    },
                    ..Default::default()
                })
                .padding([4, 8])
        };

        let quick_chips_row = row![
            quick_chip("⚙️ Settings", "open settings"),
            quick_chip("📁 Files", "open files"),
            quick_chip("⚡ Wave Optics", "create a 30 min capsule for Wave Optics"),
            quick_chip("🌙 DND", "turn on do not disturb"),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        // Recent messages (warm high-contrast Claymorphic cards with dark text)
        let mut msg_list = column![].spacing(6);
        let recent_msgs: Vec<&ChatMessageRecord> = messages.iter().rev().take(3).collect();
        for msg in recent_msgs.into_iter().rev() {
            if msg.content.trim().is_empty() {
                continue;
            }
            let is_user = msg.role == "user";
            let bubble: Element<'a, Message> = if is_user {
                let user_bg = neu.surface_elevated;
                let border_col = neu.shadow_dark;
                container(text(&msg.content).size(10).color(neu.text_primary))
                    .padding([6, 10])
                    .style(move |_t| container::Style {
                        background: Some(iced::Background::Color(user_bg)),
                        border: iced::Border {
                            radius: iced::border::Radius::from(12.0),
                            color: Color {
                                a: 0.20,
                                ..border_col
                            },
                            width: 1.0,
                        },
                        ..Default::default()
                    })
                    .into()
            } else {
                clay_tutor_card(
                    text(&msg.content)
                        .size(10)
                        .color(Color::from_rgb(0.09, 0.11, 0.22)),
                    16.0,
                )
            };

            let msg_row = if is_user {
                row![h_space(), bubble].align_y(Alignment::Center)
            } else {
                row![bubble, h_space()].align_y(Alignment::Center)
            };
            msg_list = msg_list.push(msg_row);
        }

        if messages.is_empty() {
            msg_list = msg_list.push(
                text("Try: 'Open Wi-Fi settings', 'Open files', or 'Create a 30 min capsule for Wave Optics'.")
                    .size(9)
                    .color(neu.text_muted),
            );
        }

        let chat_scroll = scrollable(msg_list).height(110).width(Length::Fill);

        // Command & Tutor Input Pill
        let input_field = text_input(
            "Ask Maindy or run OS command (e.g. 'open files')...",
            chat_input,
        )
        .on_input(Message::ChatInputChanged)
        .on_submit(Message::SendChatMessage)
        .style({
            let txt = neu.text_primary;
            let mut_col = neu.text_muted;
            move |_t, _s| text_input::Style {
                background: iced::Background::Color(Color::TRANSPARENT),
                border: iced::Border::default(),
                icon: mut_col,
                placeholder: mut_col,
                value: txt,
                selection: Color::from_rgba(0.08, 0.56, 0.78, 0.35),
            }
        })
        .padding(6)
        .width(Length::Fill);

        let input_well = neu_inset(input_field, neu, 10.0);

        let send_btn = button(text("Go →").size(10).color(Color::WHITE))
            .on_press(Message::SendChatMessage)
            .style({
                let grad_bot = clay.grad_bottom;
                move |_t, s| {
                    let bg = match s {
                        button::Status::Hovered => Color::from_rgb(0.42, 0.38, 0.98),
                        _ => grad_bot,
                    };
                    button::Style {
                        background: Some(iced::Background::Color(bg)),
                        border: iced::Border {
                            radius: iced::border::Radius::from(10.0),
                            color: Color::from_rgba(1.0, 1.0, 1.0, 0.30),
                            width: 1.0,
                        },
                        ..Default::default()
                    }
                }
            })
            .padding([6, 12]);

        let input_row = row![input_well, send_btn]
            .spacing(6)
            .align_y(Alignment::Center);

        let full_window_btn = button(
            row![
                text("Open Maindy Tutor Studio")
                    .size(9)
                    .color(neu.accent_primary),
                text("↗").size(10).color(neu.accent_primary),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        )
        .on_press(Message::OpenWindow(WindowId::LayaAiChat))
        .style(|_t, _s| button::Style {
            background: None,
            border: iced::Border::default(),
            ..Default::default()
        })
        .padding(2);

        let drawer_inner = column![
            header_clay,
            px_space(0.0, 2.0),
            quick_chips_row,
            px_space(0.0, 2.0),
            chat_scroll,
            px_space(0.0, 2.0),
            input_row,
            row![h_space(), full_window_btn].align_y(Alignment::Center),
        ]
        .spacing(6);

        let drawer_card = mouse_area(container(neu_card(drawer_inner, neu, 18.0, 1.0)).width(360))
            .on_press(Message::NoOp);

        column![
            drawer_card,
            px_space(0.0, 8.0),
            row![h_space(), interactive_orb].align_y(Alignment::Center),
        ]
        .align_x(Alignment::End)
        .into()
    } else {
        container(interactive_orb).into()
    }
}
