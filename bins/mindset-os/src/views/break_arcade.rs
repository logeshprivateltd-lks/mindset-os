//! 2-Minute Brain-Reset Arcade View — Pomodoro Break Formula Speed-Match
//!
//! Engaging 4x2 interactive card matching game strictly accessible during
//! Pomodoro breaks to refresh cognitive fatigue without dopamine rabbit holes.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use mindset_core::MindSetCoreState;

use crate::theme::{
    glass_card, glass_elevated, h_space, primary_button, secondary_button, styled_button,
    ButtonVariant, ThemePalette,
};
use crate::Message;

pub fn view<'a>(core: &'a MindSetCoreState, palette: ThemePalette) -> Element<'a, Message> {
    let arcade = match &core.break_arcade {
        Some(a) => a,
        None => {
            return container(text("Arcade is only available during Pomodoro breaks.").size(14))
                .padding(20)
                .into()
        }
    };

    // 1. Header with Break Timer & Close
    let mins = arcade.break_seconds_left / 60;
    let secs = arcade.break_seconds_left % 60;

    let header_row = row![
        column![
            text("🎮 2-MINUTE BRAIN-RESET ARCADE")
                .size(11)
                .color(palette.accent),
            text("Formula Speed-Match")
                .size(18)
                .color(palette.text_primary),
            text("Match all 4 law & formula pairs before the break timer runs out.")
                .size(12)
                .color(palette.text_secondary),
        ]
        .spacing(2),
        h_space(),
        container(
            row![
                text("⏳").size(14),
                text(format!("{:02}:{:02} Break Remaining", mins, secs))
                    .size(12)
                    .color(palette.warning),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding([6, 12])
        .style(move |_t| glass_card(palette)),
        button(text("✕ Close").size(12))
            .on_press(Message::CloseOverlays)
            .style(secondary_button(palette))
            .padding([6, 12]),
    ]
    .align_y(Alignment::Center);

    // 2. Stats bar (Pairs matched & moves)
    let stats_row = row![
        container(
            row![
                text("Pairs Matched:").size(11).color(palette.text_muted),
                text(format!("{} / {}", arcade.matches_found, arcade.total_pairs))
                    .size(13)
                    .color(palette.success),
            ]
            .spacing(6),
        )
        .padding([4, 10])
        .style(move |_t| glass_card(palette)),
        container(
            row![
                text("Total Moves:").size(11).color(palette.text_muted),
                text(format!("{}", arcade.moves_count))
                    .size(13)
                    .color(palette.accent),
            ]
            .spacing(6),
        )
        .padding([4, 10])
        .style(move |_t| glass_card(palette)),
        h_space(),
        if arcade.is_completed {
            text("🌟 ALL PAIRS MATCHED! +50 XP")
                .size(12)
                .color(palette.success)
        } else {
            text("Click 2 cards to test a match")
                .size(11)
                .color(palette.text_muted)
        },
    ]
    .align_y(Alignment::Center);

    // 3. 4x2 Grid of Formula Cards
    let mut grid_row1 = row![].spacing(10);
    let mut grid_row2 = row![].spacing(10);

    for (idx, card) in arcade.cards.iter().enumerate() {
        let (label_str, btn_variant) = if card.is_matched {
            (card.text.as_str(), ButtonVariant::Secondary)
        } else if card.is_selected {
            (card.text.as_str(), ButtonVariant::Primary)
        } else {
            ("❓ Tap to Flip", ButtonVariant::Secondary)
        };

        let card_color = if card.is_matched {
            palette.success
        } else if card.is_selected {
            palette.accent_text
        } else {
            palette.text_secondary
        };

        let card_btn = button(
            container(
                column![
                    text(if card.is_matched {
                        "✅"
                    } else if card.is_selected {
                        "💡"
                    } else {
                        "🎴"
                    })
                    .size(18),
                    text(label_str).size(11).color(card_color),
                ]
                .spacing(6)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fixed(80.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
        )
        .on_press(Message::SelectArcadeCard(idx))
        .style(styled_button(palette, btn_variant))
        .padding(8)
        .width(Length::FillPortion(1));

        if idx < 4 {
            grid_row1 = grid_row1.push(card_btn);
        } else {
            grid_row2 = grid_row2.push(card_btn);
        }
    }

    let cards_grid = column![grid_row1, grid_row2].spacing(10);

    // 4. Bottom message or Return to focus CTA
    let bottom_row = row![
        text("Designed to reset cognitive strain without social media doomscrolling.")
            .size(11)
            .color(palette.text_muted),
        h_space(),
        if arcade.is_completed {
            button(text("Return to Focus").size(12))
                .on_press(Message::CloseOverlays)
                .style(primary_button(palette))
                .padding([8, 16])
        } else {
            button(text("Return to Focus").size(12))
                .on_press(Message::CloseOverlays)
                .style(secondary_button(palette))
                .padding([8, 16])
        },
    ]
    .align_y(Alignment::Center);

    let modal_body = column![header_row, stats_row, cards_grid, bottom_row,]
        .spacing(14)
        .max_width(680);

    container(modal_body)
        .padding(20)
        .style(move |_t| glass_elevated(palette))
        .into()
}
