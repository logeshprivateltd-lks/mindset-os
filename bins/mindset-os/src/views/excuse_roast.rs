//! Roast My Excuse Modal — Mindful Guard Cinema Roast & 15s Formula Bailout Check
//!
//! Delivers high-engagement cinematic roasts for procrastination excuses and
//! challenges the student to a 15-second rapid formula check before any focus unlock.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use mindset_core::MindSetCoreState;

use crate::theme::{
    glass_card, glass_elevated, h_space, primary_button, secondary_button, styled_button,
    ButtonVariant, ThemePalette, TYPE_BODY, TYPE_CAPTION, TYPE_SUBHEAD,
};
use crate::Message;

pub fn view<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    selected_option: Option<usize>,
) -> Element<'a, Message> {
    let roast = match &core.excuse_roast {
        Some(r) => r,
        None => {
            return container(text("No excuse roast active.").size(TYPE_BODY))
                .padding(20)
                .into()
        }
    };

    // 1. Header with cinema icon and dopamine streak cue
    let header_row = row![
        column![
            row![
                text("🎬 MINDFUL GUARD — ROAST MY EXCUSE")
                    .size(TYPE_CAPTION)
                    .color(palette.warning),
                text("•").size(TYPE_CAPTION).color(palette.text_muted),
                text("🔥 Focus Streak Protected")
                    .size(TYPE_CAPTION)
                    .color(palette.success),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
            text("Distraction Shield Intercept")
                .size(TYPE_SUBHEAD)
                .color(palette.text_primary),
        ]
        .spacing(2),
        h_space(),
        button(text("✕ Return to Focus").size(TYPE_BODY))
            .on_press(Message::MindfulStayFocused)
            .style(secondary_button(palette))
            .padding([6, 12]),
    ]
    .align_y(Alignment::Center);

    // 2. Student Excuse & Savage Cinema Roast
    let excuse_box = container(
        column![
            text("YOUR PROPOSED EXCUSE:")
                .size(TYPE_CAPTION)
                .color(palette.text_muted),
            text(format!("\"{}\"", roast.excuse_given))
                .size(TYPE_BODY)
                .color(palette.text_secondary),
        ]
        .spacing(4),
    )
    .padding(10)
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    let movie_badge = container(
        text(format!("🎬 {}", roast.movie_reference))
            .size(TYPE_CAPTION)
            .color(palette.warning),
    )
    .padding([4, 10])
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgba(
            0.96, 0.68, 0.20, 0.15,
        ))),
        border: iced::Border {
            radius: 6.0.into(),
            width: 1.0,
            color: palette.warning,
        },
        ..Default::default()
    });

    let roast_card = container(
        column![
            movie_badge,
            text(&roast.witty_roast)
                .size(TYPE_BODY)
                .color(palette.text_primary),
        ]
        .spacing(8)
        .padding(14),
    )
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    // 3. 15-Second Rapid Formula Bailout Check
    let formula_check = &roast.formula_check;
    let mut options_col = column![].spacing(6);

    let has_answered = selected_option.is_some();
    let is_correct = selected_option == Some(formula_check.correct_index);

    for (idx, opt_text) in formula_check.options.iter().enumerate() {
        let is_this_selected = selected_option == Some(idx);
        let btn_variant = if has_answered {
            if idx == formula_check.correct_index {
                ButtonVariant::Primary
            } else if is_this_selected {
                ButtonVariant::Danger
            } else {
                ButtonVariant::Secondary
            }
        } else {
            ButtonVariant::Secondary
        };

        let opt_btn = button(
            row![
                text(format!("{}. ", (idx as u8 + b'A') as char))
                    .size(TYPE_BODY)
                    .color(if is_this_selected {
                        palette.accent_text
                    } else {
                        palette.text_secondary
                    }),
                text(opt_text).size(TYPE_BODY).color(if is_this_selected {
                    palette.accent_text
                } else {
                    palette.text_primary
                }),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .on_press(Message::AnswerExcuseFormula(idx))
        .style(styled_button(palette, btn_variant))
        .padding([8, 12])
        .width(Length::Fill);

        options_col = options_col.push(opt_btn);
    }

    let bailout_card = container(
        column![
            row![
                text("⏱️ 15-SECOND RAPID FORMULA BAILOUT CHECK")
                    .size(TYPE_CAPTION)
                    .color(palette.accent),
                h_space(),
                text(format!("{}s Allotted", formula_check.seconds_allotted))
                    .size(TYPE_CAPTION)
                    .color(palette.text_muted),
            ]
            .align_y(Alignment::Center),
            text(&formula_check.prompt)
                .size(TYPE_BODY)
                .color(palette.text_primary),
            options_col,
        ]
        .spacing(10)
        .padding(14),
    )
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    // 4. Outcome or Actions
    let action_section: Element<'_, Message> = if let Some(_sel) = selected_option {
        if is_correct {
            column![
                text("✅ FORMULA VERIFIED! Bailout approved.")
                    .size(TYPE_SUBHEAD)
                    .color(palette.success),
                text("You proved retention under pressure. 5-minute temporary pass granted.")
                    .size(TYPE_BODY)
                    .color(palette.text_secondary),
                button(text("Claim 5m Pass").size(TYPE_BODY))
                    .on_press(Message::MindfulUnlock5m)
                    .style(primary_button(palette))
                    .padding([8, 20]),
            ]
            .spacing(8)
            .align_x(Alignment::Center)
            .into()
        } else {
            column![
                text("⚡ Nice try! The laws of physics remain undefeated.")
                    .size(TYPE_SUBHEAD)
                    .color(palette.warning),
                text("Formula needs a quick touch-up. Your brain is building mental stamina—let's get back in the zone!")
                    .size(TYPE_BODY)
                    .color(palette.text_secondary),
                button(text("Return to Focus").size(TYPE_BODY))
                    .on_press(Message::MindfulStayFocused)
                    .style(primary_button(palette))
                    .padding([8, 20]),
            ]
            .spacing(8)
            .align_x(Alignment::Center)
            .into()
        }
    } else {
        row![
            text("Solve the formula correctly within 15 seconds to earn a 5-minute break pass.")
                .size(TYPE_CAPTION)
                .color(palette.text_muted),
            h_space(),
            button(text("Stay Focused (Recommended)").size(TYPE_BODY))
                .on_press(Message::MindfulStayFocused)
                .style(primary_button(palette))
                .padding([8, 16]),
        ]
        .align_y(Alignment::Center)
        .into()
    };

    let modal_body = column![
        header_row,
        excuse_box,
        roast_card,
        bailout_card,
        action_section,
    ]
    .spacing(14)
    .max_width(620);

    container(modal_body)
        .padding(20)
        .style(move |_t| glass_elevated(palette))
        .into()
}
