//! Toast Notification Overlay for MindSet OS
//!
//! Renders active toast popups docked in the top-right corner below the top bar.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};
use mindset_core::ToastQueue;

use crate::theme::{h_space, secondary_button, v_space, ThemePalette};
use crate::Message;

pub fn view<'a>(
    toast_queue: &'a ToastQueue,
    palette: ThemePalette,
) -> Option<Element<'a, Message>> {
    if toast_queue.toasts.is_empty() {
        return None;
    }

    let mut toast_items = column![].spacing(10).width(340);

    for toast in &toast_queue.toasts {
        let toast_id = toast.id;

        let icon_badge = container(text(&toast.icon).size(16))
            .padding(6)
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.12))),
                border: iced::Border {
                    radius: iced::border::Radius::from(8.0),
                    ..Default::default()
                },
                ..Default::default()
            });

        let toast_card = container(
            row![
                icon_badge,
                column![
                    text(&toast.title).size(12).color(palette.text_primary),
                    text(&toast.body).size(11).color(palette.text_secondary),
                ]
                .spacing(2)
                .width(Length::Fill),
                button(text("✕").size(10))
                    .on_press(Message::DismissToast(toast_id))
                    .style(secondary_button(palette))
                    .padding([3, 7]),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .padding(12),
        )
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(palette.glass_surface_elevated)),
            border: iced::Border {
                radius: iced::border::Radius::from(12.0),
                color: palette.glass_border,
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.28),
                offset: iced::Vector::new(0.0, 6.0),
                blur_radius: 16.0,
            },
            ..Default::default()
        });

        toast_items = toast_items.push(toast_card);
    }

    let overlay = container(column![
        row![
            h_space(),
            toast_items.padding(iced::Padding {
                top: 48.0,
                right: 18.0,
                bottom: 0.0,
                left: 0.0,
            }),
        ],
        v_space(),
    ])
    .width(Length::Fill)
    .height(Length::Fill);

    Some(overlay.into())
}
