//! Liquidmorphism Design System Explorer App (Responsive Layout)

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};

use crate::theme::{
    danger_button, glass_card, glass_elevated, primary_button, secondary_button, ThemePalette,
};
use crate::Message;

pub fn view<'a>(palette: ThemePalette) -> Element<'a, Message> {
    // 1. Color Palette Swatches
    let color_tokens = row![
        color_swatch("Surface", palette.glass_surface),
        color_swatch("Elevated", palette.glass_surface_elevated),
        color_swatch("Accent", palette.accent),
        color_swatch("Success", palette.success),
        color_swatch("Warning", palette.warning),
        color_swatch("Danger", palette.danger),
    ]
    .spacing(10);

    let colors_card = container(
        column![
            text("WCAG AAA Frosted Glass Design Tokens")
                .size(12)
                .color(palette.text_primary),
            color_tokens,
        ]
        .spacing(10)
        .padding(14),
    )
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    // 2. Button Hierarchy Preview
    let button_gallery = row![
        button(text("Primary Button").size(11))
            .style(primary_button(palette))
            .padding([8, 14]),
        button(text("Secondary Glass").size(11))
            .style(secondary_button(palette))
            .padding([8, 14]),
        button(text("Danger Action").size(11))
            .style(danger_button(palette))
            .padding([8, 14]),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let components_card = container(
        column![
            text("Component Architecture & Micro-Interactions")
                .size(12)
                .color(palette.text_primary),
            button_gallery,
        ]
        .spacing(10)
        .padding(14),
    )
    .width(Length::Fill)
    .style(move |_t| glass_elevated(palette));

    let layout = column![colors_card, components_card]
        .spacing(14)
        .padding(16);

    scrollable(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn color_swatch<'a>(name: &'static str, color: Color) -> Element<'a, Message> {
    column![
        container(iced::widget::Space::new().width(50).height(30)).style(move |_t| {
            container::Style {
                background: Some(iced::Background::Color(color)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.4),
                    width: 1.0,
                },
                ..Default::default()
            }
        }),
        text(name).size(10),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
    .into()
}
