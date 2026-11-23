//! Frosted Lock Screen Overlay for MindSet OS (Super+L)
//!
//! Features high-opacity frosted glass, live clock, dynamic academic greeting,
//! study quote, and one-click unlocking.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};

use crate::system::state::SystemState;
use crate::theme::{glass_elevated, h_space, primary_button, v_space, ThemePalette};
use crate::Message;

pub fn view<'a>(palette: ThemePalette) -> Element<'a, Message> {
    let clock_time = SystemState::formatted_clock();
    let clock_date = SystemState::formatted_date();
    let greeting = SystemState::current_greeting();

    let clock_box = column![
        text(clock_time).size(56).color(palette.text_primary),
        text(clock_date).size(16).color(palette.text_secondary),
        text(format!("{}, Scholar", greeting))
            .size(14)
            .color(palette.accent),
    ]
    .spacing(4)
    .align_x(Alignment::Center);

    let quote_box = container(
        column![
            text("“Discipline is choosing between what you want now and what you want most.”")
                .size(13)
                .color(palette.text_primary),
            text("TN Class 12 Board Target • 590/600 Target Protected")
                .size(11)
                .color(palette.text_muted),
        ]
        .spacing(4)
        .align_x(Alignment::Center)
        .padding(14),
    )
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(
            0.0, 0.0, 0.0, 0.20,
        ))),
        border: iced::Border {
            radius: iced::border::Radius::from(10.0),
            color: palette.glass_border,
            width: 1.0,
        },
        ..Default::default()
    });

    let unlock_btn = button(
        row![text("🔓").size(14), text("Unlock Workspace").size(12),]
            .spacing(8)
            .align_y(Alignment::Center),
    )
    .on_press(Message::CloseOverlays)
    .style(primary_button(palette))
    .padding([10, 24]);

    let card = container(
        column![clock_box, quote_box, unlock_btn,]
            .spacing(24)
            .align_x(Alignment::Center)
            .padding(32),
    )
    .width(480)
    .style(move |_t| glass_elevated(palette));

    let full_screen = column![
        v_space(),
        row![h_space(), card, h_space()].align_y(Alignment::Center),
        v_space(),
    ]
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fill);

    container(full_screen)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.02, 0.04, 0.08, 0.82,
            ))),
            ..Default::default()
        })
        .into()
}
