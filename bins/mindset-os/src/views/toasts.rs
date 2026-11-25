//! OS Notification & Dynamic Island Toast Overlay for MindSet OS
//!
//! Renders centralized `OsNotificationBus` (`Info`, `Success`, `Warning`, `Error`) alerts
//! as Deepin Frosted Glass cards anchored beneath the top-right tray with 1-click action buttons.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};
use mindset_core::{NotificationSeverity, ToastQueue};

use crate::theme::{
    frosted_glass_container, h_space, primary_button, secondary_button, v_space, ThemePalette,
};
use crate::Message;

pub fn view<'a>(
    toast_queue: &'a ToastQueue,
    palette: ThemePalette,
) -> Option<Element<'a, Message>> {
    if toast_queue.toasts.is_empty() {
        return None;
    }

    let mut toast_items = column![].spacing(10).width(360);

    for toast in toast_queue.toasts.iter().rev().take(4) {
        let toast_id = toast.id;
        let accent_stripe = match toast.severity {
            NotificationSeverity::Info => palette.accent,
            NotificationSeverity::Success => palette.success,
            NotificationSeverity::Warning => palette.warning,
            NotificationSeverity::Error => palette.danger,
        };

        let icon_badge = container(text(&toast.icon).size(15))
            .padding([5, 7])
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(Color {
                    a: 0.22,
                    ..accent_stripe
                })),
                border: iced::Border {
                    radius: iced::border::Radius::from(8.0),
                    color: Color {
                        a: 0.45,
                        ..accent_stripe
                    },
                    width: 1.0,
                },
                ..Default::default()
            });

        let mut body_col = column![
            text(&toast.title)
                .size(12)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(palette.text_primary),
            text(&toast.body).size(11).color(palette.text_secondary),
        ]
        .spacing(3)
        .width(Length::Fill);

        if let (Some(label), Some(action)) = (&toast.action_label, &toast.action) {
            body_col = body_col.push(
                button(
                    text(label)
                        .size(10)
                        .color(palette.accent_text)
                        .wrapping(text::Wrapping::None),
                )
                .on_press(Message::TriggerNotificationAction(toast_id, action.clone()))
                .style(primary_button(palette))
                .padding([3, 9]),
            );
        }

        let toast_inner = row![
            icon_badge,
            body_col,
            button(text("✕").size(10))
                .on_press(Message::DismissToast(toast_id))
                .style(secondary_button(palette))
                .padding([3, 7]),
        ]
        .spacing(10)
        .align_y(Alignment::Start)
        .padding(12);

        let toast_card = frosted_glass_container(palette, 14.0, true, toast_inner);
        toast_items = toast_items.push(toast_card);
    }

    let overlay = container(column![
        row![
            h_space(),
            toast_items.padding(iced::Padding {
                top: 46.0,
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
