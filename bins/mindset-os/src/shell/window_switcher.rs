//! Window Switcher Overlay for MindSet OS (Alt+Tab)
//!
//! Provides fast visual switching between open application windows.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use mindset_core::MindSetCoreState;

use crate::theme::{glass_elevated, h_space, secondary_button, v_space, ThemePalette};
use crate::Message;

pub fn view<'a>(core: &'a MindSetCoreState, palette: ThemePalette) -> Element<'a, Message> {
    let mut items_row = row![].spacing(12).align_y(Alignment::Center);

    let active_id = core.wm.active_window();

    for &id in &core.wm.z_stack {
        if let Some(w) = core.wm.get_window(id) {
            if w.is_open {
                let is_active = Some(id) == active_id;
                let btn = button(
                    column![
                        text(&w.icon).size(28),
                        text(&w.title).size(11).color(palette.text_primary),
                    ]
                    .spacing(6)
                    .align_x(Alignment::Center),
                )
                .on_press(Message::FocusWindow(id))
                .style(move |_t, s| {
                    let bg = if is_active {
                        iced::Color::from_rgba(0.12, 0.48, 0.96, 0.40)
                    } else if s == button::Status::Hovered {
                        iced::Color::from_rgba(1.0, 1.0, 1.0, 0.20)
                    } else {
                        iced::Color::from_rgba(1.0, 1.0, 1.0, 0.08)
                    };
                    button::Style {
                        background: Some(iced::Background::Color(bg)),
                        border: iced::Border {
                            radius: iced::border::Radius::from(10.0),
                            color: if is_active {
                                palette.glass_border_focused
                            } else {
                                palette.glass_border
                            },
                            width: if is_active { 1.5 } else { 1.0 },
                        },
                        shadow: iced::Shadow::default(),
                        ..Default::default()
                    }
                })
                .padding([12, 16]);

                items_row = items_row.push(btn);
            }
        }
    }

    let switcher_box = container(
        column![
            row![
                text("🗔 Active Windows (Alt+Tab)")
                    .size(12)
                    .color(palette.accent),
                h_space(),
                button(text("Dismiss [Esc]").size(10))
                    .on_press(Message::CloseOverlays)
                    .style(secondary_button(palette))
                    .padding([2, 6]),
            ]
            .align_y(Alignment::Center),
            items_row,
        ]
        .spacing(12)
        .padding(16),
    )
    .style(move |_t| glass_elevated(palette));

    let overlay = column![
        v_space(),
        row![h_space(), switcher_box, h_space()].align_y(Alignment::Center),
        v_space(),
    ]
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fill);

    container(overlay)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
