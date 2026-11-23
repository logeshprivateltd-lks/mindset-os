//! Study Session Reader App for MindSet OS
//!
//! Provides a distraction-free, high-comfort digital book reader with a two-pane
//! chapter navigator, step-by-step derivations, high-yield formula callouts, and
//! seamless Boss Battle verification gating.

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};
use mindset_core::{MindSetCoreState, STUDY_TOPICS};

use crate::theme::{
    glass_card, glass_elevated, h_space, primary_button, px_space, secondary_button, ThemePalette,
    TYPE_BODY, TYPE_CAPTION,
};
use crate::Message;

pub fn view<'a>(core: &'a MindSetCoreState, palette: ThemePalette) -> Element<'a, Message> {
    let active_idx = core.study_reader.active_tab_index.min(STUDY_TOPICS.len() - 1);
    let topic = &STUDY_TOPICS[active_idx];

    let streak_count = core.today.streak.count;
    let total_xp = core.odyssey.total_xp;
    let is_focused = core.preferences.focus_dnd || core.pomodoro.is_running;

    // ========================================================================
    // 1. Left Sidebar: Table of Contents & Chapter Navigator
    // ========================================================================
    let mut chapter_list = column![].spacing(6);

    for (i, t) in STUDY_TOPICS.iter().enumerate() {
        let is_selected = i == active_idx;
        let item_btn = button(
            row![
                text(if is_selected { "●" } else { "○" })
                    .size(9)
                    .color(if is_selected { palette.accent } else { palette.text_muted }),
                column![
                    text(t.title)
                        .size(12)
                        .font(iced::font::Font {
                            weight: if is_selected {
                                iced::font::Weight::Bold
                            } else {
                                iced::font::Weight::Normal
                            },
                            ..Default::default()
                        })
                        .color(if is_selected {
                            palette.text_primary
                        } else {
                            palette.text_secondary
                        }),
                    text(t.subject)
                        .size(10)
                        .color(palette.text_muted),
                ]
                .spacing(2),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .on_press(Message::SelectStudyTab(i))
        .style(move |_t, s| {
            let bg = if is_selected {
                Color::from_rgba(0.12, 0.48, 0.96, 0.22)
            } else if s == button::Status::Hovered {
                Color::from_rgba(1.0, 1.0, 1.0, 0.10)
            } else {
                Color::TRANSPARENT
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(8.0),
                    color: if is_selected {
                        palette.glass_border_focused
                    } else {
                        Color::TRANSPARENT
                    },
                    width: if is_selected { 1.0 } else { 0.0 },
                },
                ..Default::default()
            }
        })
        .padding([8, 10])
        .width(Length::Fill);

        chapter_list = chapter_list.push(item_btn);
    }

    let sidebar_header = row![
        text("📚").size(14),
        text("Syllabus Chapters")
            .size(12)
            .font(iced::font::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(palette.text_primary),
        h_space(),
        container(
            text(format!("{}/{}", active_idx + 1, STUDY_TOPICS.len()))
                .size(10)
                .color(palette.accent),
        )
        .padding([2, 6])
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
            border: iced::Border {
                radius: iced::border::Radius::from(6.0),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.10),
                width: 0.5,
            },
            ..Default::default()
        }),
    ]
    .align_y(Alignment::Center);

    let sidebar_footer = container(
        column![
            row![
                text(format!("🔥 {streak_count}d Streak")).size(10).color(palette.accent),
                h_space(),
                text(format!("{total_xp} XP")).size(10).color(palette.success),
            ]
            .align_y(Alignment::Center),
            text(if is_focused { "⏱️ Deep Focus Active" } else { "📖 Eye-Care Reading Active" })
                .size(9)
                .color(palette.text_muted),
        ]
        .spacing(4),
    )
    .padding(10)
    .style(move |_t| glass_card(palette));

    let sidebar = container(
        column![
            sidebar_header,
            px_space(0.0, 6.0),
            scrollable(chapter_list).height(Length::Fill),
            sidebar_footer,
        ]
        .spacing(8)
        .padding(12),
    )
    .width(Length::Fixed(240.0))
    .height(Length::Fill)
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.15))),
        border: iced::Border {
            color: palette.glass_border,
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ========================================================================
    // 2. Right Pane: Comfortable Digital Book Reading Sanctuary
    // ========================================================================
    let breadcrumb_bar = row![
        text(topic.subject.to_uppercase())
            .size(10)
            .font(iced::font::Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            })
            .color(palette.accent),
        text("•").size(10).color(palette.text_muted),
        text("NCERT Standard Reference")
            .size(10)
            .color(palette.text_muted),
        h_space(),
        container(
            row![
                text("👁️").size(11),
                px_space(2.0, 0.0),
                text("Comfort Reading Mode").size(10).color(palette.text_secondary),
            ]
            .align_y(Alignment::Center),
        )
        .padding([2, 8])
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.04))),
            border: iced::Border {
                radius: iced::border::Radius::from(6.0),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                width: 0.5,
            },
            ..Default::default()
        }),
    ]
    .align_y(Alignment::Center);

    let title_heading = text(topic.question)
        .size(17)
        .font(iced::font::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
        .color(palette.text_primary);

    let formula_card = container(
        column![
            row![
                text("📐 CORE THEOREM / MATHEMATICAL FORMULA")
                    .size(10)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(palette.accent),
                h_space(),
                text("Board Exam Weight: 5 Marks")
                    .size(9)
                    .color(palette.text_muted),
            ]
            .align_y(Alignment::Center),
            container(
                text(topic.formula)
                    .size(15)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(palette.accent_text),
            )
            .padding([8, 14])
            .width(Length::Fill)
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(0.08, 0.22, 0.45, 0.60))),
                border: iced::Border {
                    radius: iced::border::Radius::from(8.0),
                    color: palette.glass_border_focused,
                    width: 1.0,
                },
                ..Default::default()
            }),
        ]
        .spacing(8)
        .padding(14),
    )
    .width(Length::Fill)
    .style(move |_t| glass_elevated(palette));

    let derivation_card = container(
        column![
            row![
                text("📝 STEP-BY-STEP RIGOROUS DERIVATION")
                    .size(10)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(palette.accent),
                h_space(),
                text("Feynman Causal Progression")
                    .size(9)
                    .color(palette.text_muted),
            ]
            .align_y(Alignment::Center),
            text(topic.derivation_steps)
                .size(TYPE_BODY)
                .color(palette.text_primary),
        ]
        .spacing(10)
        .padding(16),
    )
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    let note_card = container(
        column![
            row![
                text("💡 HIGH-YIELD EXAMINATION TAKEAWAYS & TRAPS")
                    .size(10)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(palette.warning),
                h_space(),
                text("Examiner Insight")
                    .size(9)
                    .color(palette.text_muted),
            ]
            .align_y(Alignment::Center),
            text(topic.board_exam_note)
                .size(TYPE_CAPTION)
                .color(palette.text_secondary),
        ]
        .spacing(8)
        .padding(14),
    )
    .width(Length::Fill)
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(0.25, 0.18, 0.05, 0.35))),
        border: iced::Border {
            radius: iced::border::Radius::from(10.0),
            color: Color::from_rgba(0.98, 0.75, 0.20, 0.40),
            width: 1.0,
        },
        ..Default::default()
    });

    let prev_btn: Element<'a, Message> = if active_idx > 0 {
        button(text("← Previous Chapter").size(11))
            .on_press(Message::SelectStudyTab(active_idx - 1))
            .style(secondary_button(palette))
            .padding([8, 14])
            .into()
    } else {
        iced::widget::Space::new().into()
    };

    let next_btn: Element<'a, Message> = if active_idx + 1 < STUDY_TOPICS.len() {
        button(text("Next Chapter →").size(11))
            .on_press(Message::SelectStudyTab(active_idx + 1))
            .style(secondary_button(palette))
            .padding([8, 14])
            .into()
    } else {
        iced::widget::Space::new().into()
    };

    let boss_btn = button(
        row![
            text("⚔️").size(13),
            px_space(2.0, 0.0),
            text("Test Concept in Boss Battle")
                .size(12)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(Color::WHITE),
        ]
        .align_y(Alignment::Center),
    )
    .on_press(Message::OpenVerificationBoss)
    .style(primary_button(palette))
    .padding([8, 18]);

    let navigation_footer = row![
        prev_btn,
        h_space(),
        boss_btn,
        h_space(),
        next_btn,
    ]
    .align_y(Alignment::Center);

    let reading_content = column![
        breadcrumb_bar,
        title_heading,
        formula_card,
        derivation_card,
        note_card,
        px_space(0.0, 4.0),
        navigation_footer,
    ]
    .spacing(14)
    .padding(20)
    .max_width(740);

    let reader_scroll = scrollable(
        container(reading_content)
            .width(Length::Fill)
            .align_x(Alignment::Center),
    )
    .height(Length::Fill);

    row![sidebar, reader_scroll]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
