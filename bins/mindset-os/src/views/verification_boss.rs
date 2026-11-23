//! Verification Boss Battle View — Strict Proof-of-Study Lock
//!
//! 3-Stage Boss Battle Gate (Rapid Recall 30pts, Applied Problem 40pts, Feynman 30pts).
//! Enforces >= 70/100 threshold before transitioning any Daily Capsule to Completed.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length};
use mindset_core::engine::verification::CapsuleState;
use mindset_core::MindSetCoreState;

use crate::theme::{
    glass_card, h_space, modern_text_input, primary_button, px_space, styled_button,
    traffic_light_button, window_chrome_style, window_titlebar_style, ButtonVariant, ThemePalette,
    TYPE_BODY, TYPE_CAPTION, TYPE_METRIC, TYPE_SUBHEAD,
};
use crate::Message;

pub fn view<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    stage1_choices: &'a [Option<usize>],
    stage2_choices: &'a [Option<usize>],
    feynman_input: &'a str,
) -> Element<'a, Message> {
    let session = match &core.boss_session {
        Some(s) => s,
        None => {
            return container(text("No active Boss Battle session.").size(TYPE_SUBHEAD))
                .padding(20)
                .into()
        }
    };

    let exam = match &core.active_exam {
        Some(e) => e,
        None => {
            return container(text("Generating Boss questions...").size(TYPE_SUBHEAD))
                .padding(20)
                .into()
        }
    };

    let active_capsule = core.exam_plan.capsules.first();
    let capsule_state = active_capsule
        .map(|c| c.state.clone())
        .unwrap_or(CapsuleState::Active);

    // 1. Unified OS Window Titlebar Chrome with Traffic Lights
    let close_btn = button(
        container(text("×").size(9))
            .width(12)
            .height(12)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(Message::CloseOverlays)
    .style(traffic_light_button(
        Color::from_rgb(1.0, 0.373, 0.337),
        Color::from_rgb(0.88, 0.27, 0.24),
        Color::from_rgba(0.35, 0.0, 0.0, 0.85),
        true,
    ))
    .width(12)
    .height(12)
    .padding(0);

    let min_btn = button(
        container(text("−").size(9))
            .width(12)
            .height(12)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(Message::CloseOverlays)
    .style(traffic_light_button(
        Color::from_rgb(1.0, 0.741, 0.180),
        Color::from_rgb(0.87, 0.63, 0.14),
        Color::from_rgba(0.40, 0.25, 0.0, 0.85),
        true,
    ))
    .width(12)
    .height(12)
    .padding(0);

    let max_btn = button(
        container(text("+").size(9))
            .width(12)
            .height(12)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(Message::CloseOverlays)
    .style(traffic_light_button(
        Color::from_rgb(0.153, 0.788, 0.247),
        Color::from_rgb(0.10, 0.67, 0.16),
        Color::from_rgba(0.0, 0.35, 0.05, 0.85),
        true,
    ))
    .width(12)
    .height(12)
    .padding(0);

    let traffic_lights = row![close_btn, min_btn, max_btn]
        .spacing(8)
        .align_y(Alignment::Center);

    let titlebar_title = row![
        text("⚔️").size(13),
        text(format!("Proof-of-Study Verification: {}", session.boss_name))
            .size(12)
            .font(iced::font::Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            })
            .color(palette.text_primary),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let titlebar_tag = container(
        text("Mandatory Study Gate")
            .size(9)
            .color(palette.warning),
    )
    .padding([2, 6])
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.04))),
        border: iced::Border {
            radius: iced::border::Radius::from(4.0),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            width: 0.5,
        },
        ..Default::default()
    });

    let titlebar = container(
        row![
            traffic_lights,
            px_space(14.0, 0.0),
            titlebar_title,
            h_space(),
            titlebar_tag,
        ]
        .padding([8, 14])
        .align_y(Alignment::Center),
    )
    .style(move |_t| window_titlebar_style(palette, true));

    // 1.1 Boss Subheader Info Banner
    let boss_header = row![
        column![
            text("⚔️ STRICT PROOF-OF-STUDY BOSS BATTLE")
                .size(TYPE_CAPTION)
                .color(palette.warning),
            text(&session.boss_name)
                .size(TYPE_METRIC)
                .color(palette.text_primary),
            text(format!(
                "{}  •  Pass Threshold: >= 70 / 100 pts  •  Value: +{:.1} Marks",
                session.boss_title,
                active_capsule.map(|c| c.mark_value).unwrap_or(3.5)
            ))
            .size(TYPE_BODY)
            .color(palette.text_secondary),
        ]
        .spacing(2),
        h_space(),
        container(
            row![
                text(match &capsule_state {
                    CapsuleState::Completed { .. } => "✓ VERIFIED",
                    CapsuleState::NeedsRemediation { .. } => "⚠️ REVIEW NEEDED",
                    _ => "⚔️ CHALLENGE IN PROGRESS",
                })
                .size(TYPE_CAPTION)
                .color(match &capsule_state {
                    CapsuleState::Completed { .. } => palette.success,
                    CapsuleState::NeedsRemediation { .. } => palette.warning,
                    _ => palette.accent,
                }),
            ]
            .align_y(Alignment::Center),
        )
        .padding([4, 10])
        .style(move |_t| glass_card(palette)),
    ]
    .align_y(Alignment::Center);

    // 2. Live Boss HP Bar
    let hp_ratio = (session.current_hp / 100.0).clamp(0.0, 1.0);
    let hp_color = if hp_ratio > 0.5 {
        palette.danger
    } else if hp_ratio > 0.2 {
        palette.warning
    } else {
        palette.success
    };

    let hp_bar = container(row![
        container(iced::widget::Space::new())
            .width(Length::FillPortion((hp_ratio * 100.0) as u16 + 1))
            .height(Length::Fixed(12.0))
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(hp_color)),
                border: iced::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
        container(iced::widget::Space::new())
            .width(Length::FillPortion(((1.0 - hp_ratio) * 100.0) as u16 + 1))
            .height(Length::Fixed(12.0)),
    ])
    .width(Length::Fill)
    .height(Length::Fixed(12.0))
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgba(
            0.0, 0.0, 0.0, 0.5,
        ))),
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let hp_display = row![
        text("BOSS HEALTH:").size(TYPE_CAPTION).color(palette.text_muted),
        text(format!("{:.0} / 100 HP", session.current_hp))
            .size(TYPE_BODY)
            .color(hp_color),
        h_space(),
        text("🛡️ Focus Shield Active (Window Blur > 4s incurs -10 pt penalty)")
            .size(TYPE_CAPTION)
            .color(palette.text_muted),
    ]
    .align_y(Alignment::Center);

    let hp_card = column![hp_display, hp_bar].spacing(6);

    // Check if defeated or completed
    if session.is_victory || matches!(capsule_state, CapsuleState::Completed { .. }) {
        let victory_view = column![
            text("🏆 BOSS DEFEATED — PROOF-OF-STUDY VERIFIED!")
                .size(TYPE_SUBHEAD)
                .color(palette.success),
            row![
                text("🎯 Verified Mastery:").size(TYPE_BODY).color(palette.accent),
                text(format!(
                    "+{:.1} Marks locked into Predicted Exam Score.",
                    active_capsule.map(|c| c.mark_value).unwrap_or(3.5)
                ))
                .size(TYPE_BODY)
                .color(palette.text_primary),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
            text("Daily Capsule status transitioned to COMPLETED. MindSet XP awarded.")
                .size(TYPE_BODY)
                .color(palette.text_secondary),
            button(text("Return to Sanctuary").size(TYPE_BODY))
                .on_press(Message::CloseOverlays)
                .style(primary_button(palette))
                .padding([10, 24]),
        ]
        .spacing(12)
        .align_x(Alignment::Center);

        return container(
            column![
                titlebar,
                column![boss_header, hp_card, victory_view]
                    .spacing(14)
                    .padding(20),
            ]
            .spacing(0)
            .max_width(760),
        )
        .style(move |_t| window_chrome_style(palette, true))
        .into();
    }

    if matches!(capsule_state, CapsuleState::NeedsRemediation { .. }) {
        let remediation_view = column![
            row![
                text("🌱").size(TYPE_SUBHEAD),
                column![
                    text("Concept Reinforcement Opportunity")
                        .size(TYPE_SUBHEAD)
                        .color(palette.warning),
                    text("Score: Under 70-point target. True mastery is built through diagnostic review.")
                        .size(TYPE_CAPTION)
                        .color(palette.text_secondary),
                ]
                .spacing(2),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text("No penalty applied. Review the targeted micro-refresher below, then retry the Boss Battle.")
                .size(TYPE_BODY)
                .color(palette.text_primary),
            container(
                column![
                    text("📖 Targeted Micro-Refresher").size(TYPE_BODY).color(palette.accent),
                    text("1. Re-read: Integral of Electric Field over closed surface (Gauss Flux).")
                        .size(TYPE_CAPTION).color(palette.text_primary),
                    text("2. Check charge sign: inward flux is negative, outward is positive.")
                        .size(TYPE_CAPTION).color(palette.text_primary),
                    text("3. High-yield anchor: Φ = q_enclosed / ε₀.")
                        .size(TYPE_CAPTION).color(palette.text_primary),
                ]
                .spacing(4)
                .padding(12)
            )
            .width(Length::Fill)
            .style(move |_t| glass_card(palette)),
            button(text("🔄 Retry Verification Boss").size(TYPE_SUBHEAD))
                .on_press(Message::RetryBossSession)
                .style(primary_button(palette))
                .padding([10, 24]),
        ]
        .spacing(12)
        .align_x(Alignment::Center);

        return container(
            column![
                titlebar,
                column![boss_header, hp_card, remediation_view]
                    .spacing(14)
                    .padding(20),
            ]
            .spacing(0)
            .max_width(760),
        )
        .style(move |_t| window_chrome_style(palette, true))
        .into();
    }

    // 3. Stage 1: Rapid Recall Questions (30 pts)
    let mut stage1_col = column![row![
        text("STAGE 1: RAPID RECALL (30 PTS)")
            .size(TYPE_BODY)
            .color(palette.accent),
        h_space(),
        text("Rapid Conceptual MCQs (15 pts each)")
            .size(TYPE_CAPTION)
            .color(palette.text_muted),
    ]
    .align_y(Alignment::Center)]
    .spacing(10);

    for (q_idx, q) in exam.recall_questions.iter().enumerate() {
        let selected_opt = stage1_choices.get(q_idx).copied().flatten();
        let mut opts_col = column![].spacing(6);

        for (opt_idx, opt_text) in q.options.iter().enumerate() {
            let is_selected = selected_opt == Some(opt_idx);
            let btn_variant = if is_selected {
                ButtonVariant::Primary
            } else {
                ButtonVariant::Secondary
            };

            let opt_btn = button(
                row![
                    text(format!("{}. ", (opt_idx as u8 + b'A') as char))
                        .size(TYPE_BODY)
                        .color(if is_selected {
                            palette.accent_text
                        } else {
                            palette.text_secondary
                        }),
                    text(opt_text).size(TYPE_BODY).color(if is_selected {
                        palette.accent_text
                    } else {
                        palette.text_primary
                    }),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .on_press(Message::SelectBossStage1(q_idx, opt_idx))
            .style(styled_button(palette, btn_variant))
            .padding([6, 12])
            .width(Length::Fill);

            opts_col = opts_col.push(opt_btn);
        }

        let q_card = container(
            column![
                text(format!("Q{}: {}", q_idx + 1, q.question))
                    .size(TYPE_SUBHEAD)
                    .color(palette.text_primary),
                opts_col,
            ]
            .spacing(8)
            .padding(10),
        )
        .width(Length::Fill)
        .style(move |_t| glass_card(palette));

        stage1_col = stage1_col.push(q_card);
    }

    // 4. Stage 2: Applied Problems (40 pts)
    let mut stage2_col = column![row![
        text("STAGE 2: APPLIED PROBLEM & STEP CHECK (40 PTS)")
            .size(TYPE_BODY)
            .color(palette.accent),
        h_space(),
        text("Analytical Derivations (20 pts each)")
            .size(TYPE_CAPTION)
            .color(palette.text_muted),
    ]
    .align_y(Alignment::Center)]
    .spacing(10);

    for (q_idx, q) in exam.applied_questions.iter().enumerate() {
        let selected_opt = stage2_choices.get(q_idx).copied().flatten();
        let mut opts_col = column![].spacing(6);

        for (opt_idx, opt_text) in q.steps_or_options.iter().enumerate() {
            let is_selected = selected_opt == Some(opt_idx);
            let btn_variant = if is_selected {
                ButtonVariant::Primary
            } else {
                ButtonVariant::Secondary
            };

            let opt_btn = button(
                row![
                    text(format!("{}. ", (opt_idx as u8 + b'A') as char))
                        .size(TYPE_BODY)
                        .color(if is_selected {
                            palette.accent_text
                        } else {
                            palette.text_secondary
                        }),
                    text(opt_text).size(TYPE_BODY).color(if is_selected {
                        palette.accent_text
                    } else {
                        palette.text_primary
                    }),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .on_press(Message::SelectBossStage2(q_idx, opt_idx))
            .style(styled_button(palette, btn_variant))
            .padding([6, 12])
            .width(Length::Fill);

            opts_col = opts_col.push(opt_btn);
        }

        let q_card = container(
            column![
                text(format!("Problem {}: {}", q_idx + 1, q.problem_statement))
                    .size(TYPE_BODY)
                    .color(palette.text_secondary),
                text(&q.given_values).size(TYPE_CAPTION).color(palette.accent),
                opts_col,
            ]
            .spacing(8)
            .padding(10),
        )
        .width(Length::Fill)
        .style(move |_t| glass_card(palette));

        stage2_col = stage2_col.push(q_card);
    }

    // 5. Stage 3: Feynman Explain-It-Back (30 pts)
    let feynman = &exam.feynman_prompt;
    let word_count = feynman_input.split_whitespace().count();
    let word_color = if word_count >= 15 {
        palette.success
    } else {
        palette.text_muted
    };

    let feynman_input_box = text_input(
        "Explain the core concept simply, step by step, using causal reasoning ('because', 'therefore')...",
        feynman_input,
    )
    .on_input(Message::FeynmanInputChanged)
    .style(modern_text_input(palette))
    .padding(10)
    .width(Length::Fill);

    let stage3_card = container(
        column![
            row![
                text("STAGE 3: FEYNMAN EXPLAIN-IT-BACK (30 PTS)")
                    .size(TYPE_BODY)
                    .color(palette.accent),
                h_space(),
                text(format!("{} words", word_count))
                    .size(TYPE_CAPTION)
                    .color(word_color),
            ]
            .align_y(Alignment::Center),
            text(&feynman.prompt).size(TYPE_BODY).color(palette.text_secondary),
            feynman_input_box,
            row![
                text("Key terms expected:")
                    .size(TYPE_CAPTION)
                    .color(palette.text_muted),
                text(feynman.required_keywords.join(", "))
                    .size(TYPE_CAPTION)
                    .color(palette.accent),
            ]
            .spacing(6),
        ]
        .spacing(8)
        .padding(12),
    )
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    // 6. Submit Button
    let submit_btn = button(
        row![
            text("⚡").size(TYPE_SUBHEAD),
            text("SUBMIT PROOF OF STUDY FOR GRADING")
                .size(TYPE_SUBHEAD)
                .color(palette.accent_text),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .on_press(Message::SubmitVerificationBoss)
    .style(primary_button(palette))
    .padding([12, 28]);

    let action_row = row![
        text("Graded via Local Gemma:2b with instant offline keyword rubric fallback.")
            .size(TYPE_CAPTION)
            .color(palette.text_muted),
        h_space(),
        submit_btn,
    ]
    .align_y(Alignment::Center);

    let battle_scroll = scrollable(column![stage1_col, stage2_col, stage3_card,].spacing(16))
        .height(Length::Fixed(360.0));

    let modal_content = column![
        titlebar,
        column![boss_header, hp_card, battle_scroll, action_row]
            .spacing(14)
            .padding(20),
    ]
    .spacing(0)
    .max_width(760);

    container(modal_content)
        .style(move |_t| window_chrome_style(palette, true))
        .into()
}
