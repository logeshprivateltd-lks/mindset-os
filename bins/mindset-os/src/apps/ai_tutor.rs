//! Maindy AI Tutor Application with Local Ollama NDJSON Streaming & Memory Drawer
//!
//! Features:
//! - Local Ollama model execution & dynamic discovery
//! - Non-blocking streaming chat
//! - Memory Drawer with direct SQLite memory listing, addition, and deletion
//! - Low RAM profile with explicit Unload Model action
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length};
use mindset_core::MindSetCoreState;

use crate::storage::db::{ChatMessageRecord, ChatThread, UserMemory};
use crate::theme::claymorphism::clay_bubble;
use crate::theme::{
    danger_button, glass_card, glass_elevated, h_space, modern_text_input, primary_button,
    px_space, secondary_button, toggle_button, ThemePalette,
};
use crate::Message;

#[allow(clippy::too_many_arguments)]
pub fn view<'a>(
    _core: &'a MindSetCoreState,
    palette: ThemePalette,
    chat_input: &'a str,
    active_thread_id: &'a str,
    threads: &'a [ChatThread],
    messages: &'a [ChatMessageRecord],
    is_generating: bool,
    active_model: &'a str,
    memories: &'a [UserMemory],
    new_memory_input: &'a str,
    auto_speak: bool,
) -> Element<'a, Message> {
    // 1. Top Control Bar: Active Model Badge, Unload Model (Free RAM), Start Ollama, Generate Capsules
    let model_badge = container(
        row![
            text("🧠").size(12),
            text(if active_model.is_empty() {
                "gemma2:2b"
            } else {
                active_model
            })
            .size(11)
            .color(palette.text_primary),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding([4, 8])
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(
            1.0, 1.0, 1.0, 0.15,
        ))),
        border: iced::Border {
            radius: iced::border::Radius::from(6.0),
            color: palette.glass_border,
            width: 1.0,
        },
        ..Default::default()
    });

    let unload_btn = button(
        row![
            text("🧹").size(12),
            text("Unload Model (Free RAM)")
                .size(10)
                .color(palette.text_secondary),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .on_press(Message::UnloadOllamaModel)
    .style(secondary_button(palette))
    .padding([3, 8]);

    let start_ollama_btn = button(
        row![
            text("⚡").size(12),
            text("Start Ollama").size(10).color(palette.text_secondary),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .on_press(Message::StartOllama)
    .style(secondary_button(palette))
    .padding([3, 8]);

    let auto_speak_btn = button(
        row![
            text(if auto_speak { "🔊 Auto-Speak: ON" } else { "🔇 Auto-Speak: OFF" })
                .size(10)
                .color(palette.text_secondary),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .on_press(Message::ToggleAutoSpeak)
    .style(secondary_button(palette))
    .padding([3, 8]);

    let gen_capsules_btn = button(
        row![
            text("⚡").size(12),
            text("Generate Today's Capsules")
                .size(10)
                .color(palette.accent_text),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .on_press(Message::GenerateDailyCapsules)
    .style(primary_button(palette))
    .padding([3, 10]);

    let top_bar = container(
        row![
            model_badge,
            unload_btn,
            start_ollama_btn,
            auto_speak_btn,
            h_space(),
            gen_capsules_btn,
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .padding([6, 12]),
    )
    .width(Length::Fill)
    .style(move |_t| glass_elevated(palette));

    // 2. Left Thread Sidebar
    let new_chat_btn = button(
        row![text("➕").size(12), text("New Study Session").size(11)]
            .spacing(6)
            .align_y(Alignment::Center),
    )
    .on_press(Message::NewChatThread)
    .style(primary_button(palette))
    .padding([6, 12])
    .width(Length::Fill);

    let mut thread_items = column![new_chat_btn].spacing(6);

    for t in threads {
        let is_active = t.id == active_thread_id;
        let thread_id = t.id.clone();
        let btn = button(
            column![
                text(&t.title).size(11).color(if is_active {
                    palette.accent
                } else {
                    palette.text_primary
                }),
                text(&t.created_at[..10.min(t.created_at.len())])
                    .size(9)
                    .color(palette.text_muted),
            ]
            .spacing(1)
            .align_x(Alignment::Start),
        )
        .on_press(Message::SelectChatThread(thread_id))
        .style(toggle_button(palette, is_active))
        .padding([6, 8])
        .width(Length::Fill);

        thread_items = thread_items.push(btn);
    }

    let sidebar = container(scrollable(thread_items).height(Length::Fill))
        .padding(10)
        .width(170)
        .height(Length::Fill)
        .style(move |_t| glass_elevated(palette));

    // 3. Main Chat History Area
    let mut msg_list = column![].spacing(8);

    for m in messages {
        let is_user = m.role == "user";
        let role_label = if is_user { "You" } else { "Maindy AI" };
        let role_color = if is_user {
            palette.accent
        } else {
            palette.success
        };

        let header_row = if !is_user {
            row![
                text(role_label).size(10).color(role_color),
                h_space(),
                button(text("🔊 Speak").size(9).color(palette.accent))
                    .on_press(Message::SpeakMessage(m.content.clone()))
                    .style(|_t, s| {
                        let bg = match s {
                            button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                            _ => Color::TRANSPARENT,
                        };
                        button::Style {
                            background: Some(iced::Background::Color(bg)),
                            border: iced::Border {
                                radius: iced::border::Radius::from(4.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    })
                    .padding([1, 6]),
            ]
            .align_y(Alignment::Center)
        } else {
            row![text(role_label).size(10).color(role_color)].align_y(Alignment::Center)
        };

        let bubble: Element<'a, Message> = if is_user {
            container(
                column![
                    header_row,
                    text(&m.content).size(12).color(palette.text_primary),
                ]
                .spacing(3)
                .padding(10),
            )
            .width(Length::Fill)
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(
                    0.12, 0.48, 0.96, 0.16,
                ))),
                border: iced::Border {
                    radius: iced::border::Radius::from(16.0),
                    color: palette.glass_border,
                    width: 1.0,
                },
                ..Default::default()
            })
            .into()
        } else {
            clay_bubble(
                column![
                    header_row,
                    text(&m.content).size(12).color(Color::WHITE),
                ]
                .spacing(3),
                28.0,
            )
        };

        msg_list = msg_list.push(bubble);
    }

    let chat_scroll = scrollable(msg_list).height(Length::Fill);

    // 4. Input Row & Stop Button
    let action_btn = if is_generating {
        button(text("⏹ Stop").size(11))
            .on_press(Message::CancelAiStream)
            .style(danger_button(palette))
            .padding([8, 14])
    } else {
        button(text("Ask AI").size(11))
            .on_press(Message::SendChatMessage)
            .style(primary_button(palette))
            .padding([8, 16])
    };

    let input_row = row![
        text_input(
            "Ask Maindy AI anything about derivations, formulas, or exams...",
            chat_input
        )
        .on_input(Message::ChatInputChanged)
        .on_submit(Message::SendChatMessage)
        .style(modern_text_input(palette))
        .padding(10)
        .width(Length::Fill),
        action_btn,
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let chat_pane = column![chat_scroll, input_row,]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill);

    // 5. Right Memory Drawer: Persistent Context Facts & Learning Preferences
    let memory_header = row![
        text("🧠").size(13),
        text("Memory Drawer").size(12).color(palette.text_primary),
        h_space(),
        text(format!("{} facts", memories.len()))
            .size(9)
            .color(palette.text_muted),
    ]
    .align_y(Alignment::Center);

    let mut memory_items = column![].spacing(6);
    for m in memories {
        let m_id = m.id;
        let mem_row = row![
            column![
                text(&m.category).size(8).color(palette.accent),
                text(&m.memory_text).size(10).color(palette.text_primary),
            ]
            .spacing(1)
            .width(Length::Fill),
            button(text("×").size(12).color(palette.danger))
                .on_press(Message::DeleteUserMemory(m_id))
                .style(secondary_button(palette))
                .padding([1, 5]),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let mem_card = container(mem_row)
            .padding(6)
            .style(move |_t| glass_card(palette))
            .width(Length::Fill);

        memory_items = memory_items.push(mem_card);
    }

    if memories.is_empty() {
        memory_items = memory_items.push(
            text("No memories recorded. Add student context below.")
                .size(9)
                .color(palette.text_muted),
        );
    }

    let add_memory_input = text_input("Add memory or fact...", new_memory_input)
        .on_input(Message::NewMemoryInputChanged)
        .on_submit(Message::AddUserMemory)
        .style(modern_text_input(palette))
        .padding(6)
        .width(Length::Fill);

    let add_memory_btn = button(text("+ Add Fact").size(10))
        .on_press(Message::AddUserMemory)
        .style(primary_button(palette))
        .padding([4, 10]);

    let add_box = column![add_memory_input, add_memory_btn].spacing(4);

    let memory_drawer = container(
        column![
            memory_header,
            px_space(0.0, 4.0),
            scrollable(memory_items).height(Length::Fill),
            px_space(0.0, 4.0),
            add_box,
        ]
        .spacing(6)
        .padding(10),
    )
    .width(220)
    .height(Length::Fill)
    .style(move |_t| glass_elevated(palette));

    let body = row![sidebar, chat_pane, memory_drawer]
        .spacing(8)
        .height(Length::Fill);

    let layout = column![top_bar, body]
        .spacing(8)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill);

    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
