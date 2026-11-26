//! Dynamic Top Navigation Bar for MindSet OS
//!
//! Clean 3-Zone Floating Frosted Glass Panel (elementary OS + Deepin hybrid):
//! - Zone 1 (Left Section): Crisp minimalist 'MS OS' monogram menu button
//!   (opens System Menu with Launcher, Side HUD Toggle, Lock, Logout, Reboot, Power Off)
//!   and active focused application title.
//! - Zone 2 (Center Section): Cleanly anchored center isolation clearance slot for the Dynamic Notch.
//! - Zone 3 (Right Section): Compact system tray indicators (Wi-Fi, Audio, DND, Notifications)
//!   and live formatted clock in a compact frosted glass pill.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::mouse;
use iced::widget::canvas::{self, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{button, container, row, text};
use iced::{Alignment, Color, Element, Length, Point, Rectangle};
use mindset_core::{ActiveOverlay, MindSetCoreState, WindowId};

use crate::theme::{frosted_glass_container, h_space, px_space, ThemePalette};
use crate::Message;

// ----------------------------------------------------------------------------
// Crisp Minimalist Vector Canvas Logo for MindSet OS ('MS' Monogram)
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct MindSetLogoProgram {
    pub stroke_color: Color,
    pub accent_color: Color,
}

impl<Message> Program<Message, iced::Theme, iced::Renderer> for MindSetLogoProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let w = bounds.width;
        let h = bounds.height;

        // 1. Stylized 'M' Monogram (left half)
        let mut m_builder = canvas::path::Builder::new();
        let m_x1 = w * 0.08;
        let m_x2 = w * 0.26;
        let m_x3 = w * 0.44;
        let m_top = h * 0.18;
        let m_bot = h * 0.82;
        let m_mid = h * 0.58;

        m_builder.move_to(Point::new(m_x1, m_bot));
        m_builder.line_to(Point::new(m_x1, m_top));
        m_builder.line_to(Point::new(m_x2, m_mid));
        m_builder.line_to(Point::new(m_x3, m_top));
        m_builder.line_to(Point::new(m_x3, m_bot));
        let m_path = m_builder.build();

        frame.stroke(
            &m_path,
            Stroke::default()
                .with_color(self.stroke_color)
                .with_width(2.0),
        );

        // 2. Stylized 'S' Monogram (right half)
        let mut s_builder = canvas::path::Builder::new();
        let s_x1 = w * 0.56;
        let s_x2 = w * 0.73;
        let s_x3 = w * 0.92;
        let s_top = h * 0.18;
        let s_mid_top = h * 0.46;
        let s_mid_bot = h * 0.54;
        let s_bot = h * 0.82;

        s_builder.move_to(Point::new(s_x3, s_top + h * 0.08));
        s_builder.line_to(Point::new(s_x3, s_top));
        s_builder.line_to(Point::new(s_x1, s_top));
        s_builder.line_to(Point::new(s_x1, s_mid_top));
        s_builder.line_to(Point::new(s_x3, s_mid_bot));
        s_builder.line_to(Point::new(s_x3, s_bot));
        s_builder.line_to(Point::new(s_x1, s_bot));
        s_builder.line_to(Point::new(s_x1, s_bot - h * 0.08));
        let s_path = s_builder.build();

        frame.stroke(
            &s_path,
            Stroke::default()
                .with_color(self.accent_color)
                .with_width(2.0),
        );

        // 3. Neural node accent glow at apex
        let spark_center = Point::new(s_x2, s_top);
        let spark_dot = Path::circle(spark_center, 1.2);
        frame.fill(&spark_dot, self.accent_color);

        vec![frame.into_geometry()]
    }
}

pub fn view<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    _language_code: &'a str,
    _is_audio_playing: bool,
    _tunnel_vision_active: bool,
    network_status: &'a mindset_sys::network::NetworkStatus,
) -> Element<'a, Message> {
    // ------------------------------------------------------------------------
    // Zone 1: Left Section (Prominent "MS OS" Monogram + Active App Name)
    // ------------------------------------------------------------------------
    let logo_canvas = iced::widget::canvas(MindSetLogoProgram {
        stroke_color: Color::WHITE,
        accent_color: palette.accent,
    })
    .width(28)
    .height(18);

    let logo_btn = button(
        row![
            logo_canvas,
            px_space(8.0, 0.0),
            text("MindSet OS")
                .size(13)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(palette.text_primary),
            px_space(3.0, 0.0),
            text("▾").size(10).color(palette.text_secondary),
        ]
        .align_y(Alignment::Center),
    )
    .on_press(Message::ToggleOverlay(ActiveOverlay::SystemMenu))
    .style(move |_t, s| {
        let bg = match s {
            button::Status::Hovered => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.18),
            _ => iced::Color::TRANSPARENT,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(8.0),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .padding([2, 8]);

    // Active frontmost desktop application indicator (elementary OS / macOS unified menubar style)
    let active_app_title = match core.wm.active_window() {
        Some(WindowId::Today) | None => "MindSet OS",
        Some(id) => match id {
            WindowId::StudyReader => "Study Reader",
            WindowId::LayaAiChat => "Maindy AI",
            WindowId::OdysseyMap => "3D Odyssey",
            WindowId::Scratchpad => "Scratchpad",
            WindowId::Tasks => "Focus Arena",
            WindowId::SearchBrowser => "MindStrict",
            WindowId::FileVault => "Study Vault",
            WindowId::SystemMonitor => "System Monitor",
            WindowId::Settings => "System Settings",
            WindowId::DesignSystem => "Design System",
            _ => core
                .wm
                .get_window(id)
                .map(|w| w.title.as_str())
                .unwrap_or("MindSet OS"),
        },
    };

    let divider_tint = palette.glass_border;
    let mut left_row = row![logo_btn].align_y(Alignment::Center);

    if active_app_title != "MindSet OS" {
        left_row = left_row
            .push(px_space(4.0, 0.0))
            .push(
                container(px_space(1.0, 14.0)).style(move |_t| container::Style {
                    background: Some(iced::Background::Color(divider_tint)),
                    ..Default::default()
                }),
            )
            .push(px_space(6.0, 0.0))
            .push(
                text(active_app_title)
                    .size(12)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Semibold,
                        ..Default::default()
                    })
                    .color(palette.text_secondary),
            );
    }

    let left_hud_btn = button(text("◫").size(12).color(palette.text_secondary))
        .on_press(Message::ToggleSideHud)
        .style(top_bar_icon_style())
        .padding([2, 5]);

    left_row = left_row.push(px_space(4.0, 0.0)).push(left_hud_btn);

    let left_section = frosted_glass_container(
        palette,
        17.0,
        true,
        container(left_row)
            .height(32)
            .padding(iced::Padding {
                top: 2.0,
                right: 8.0,
                bottom: 2.0,
                left: 4.0,
            })
            .align_y(Alignment::Center),
    );

    // ------------------------------------------------------------------------
    // Zone 2: Center Section (Clean Dynamic Notch Clearance Slot)
    // ------------------------------------------------------------------------
    let center_flank = container(
        // Dynamic Notch clearance slot (matches compact notch width 300px + margin)
        container(row![]).width(306).height(34),
    )
    .height(34)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    // ------------------------------------------------------------------------
    // Zone 3: Right Section (Compact Frosted Tray: Wi-Fi, Audio, DND, Clock)
    // ------------------------------------------------------------------------
    let now = chrono::Local::now();
    let clock_str = format!("{}", now.format("%a %l:%M %p"));
    let clock_text = text(clock_str)
        .size(11)
        .font(iced::font::Font {
            weight: iced::font::Weight::Medium,
            ..Default::default()
        })
        .color(palette.text_primary);

    let wifi_icon = if core.preferences.airplane_mode {
        "✈️"
    } else if !core.preferences.wifi_enabled {
        "❌"
    } else if network_status.connected_ssid.is_none() {
        "⚠️"
    } else {
        "📶"
    };
    let wifi_btn = button(text(wifi_icon).size(11).color(palette.text_primary))
        .on_press(Message::ToggleOverlay(ActiveOverlay::ControlCenter))
        .style(top_bar_icon_style())
        .padding([2, 4]);

    let vol_icon = if core.preferences.volume > 0.5 {
        "🔊"
    } else if core.preferences.volume > 0.05 {
        "🔉"
    } else {
        "🔇"
    };
    let volume_btn = button(text(vol_icon).size(11).color(palette.text_primary))
        .on_press(Message::ToggleOverlay(ActiveOverlay::ControlCenter))
        .style(top_bar_icon_style())
        .padding([2, 4]);

    let dnd_active = core.preferences.focus_dnd;
    let dnd_btn = button(text(if dnd_active { "🌙" } else { "🔕" }).size(11).color(
        if dnd_active {
            palette.accent
        } else {
            palette.text_secondary
        },
    ))
    .on_press(Message::ToggleDnd)
    .style(top_bar_icon_style())
    .padding([2, 4]);

    let unread_count = core.toast_queue.toasts.len();
    let bell_content: Element<'a, Message> = if unread_count > 0 {
        row![
            text("🔔").size(11).color(palette.text_primary),
            px_space(2.0, 0.0),
            container(
                text(format!("{}", unread_count))
                    .size(9)
                    .color(Color::WHITE)
            )
            .padding([1, 4])
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(palette.accent)),
                border: iced::Border {
                    radius: iced::border::Radius::from(8.0),
                    ..Default::default()
                },
                ..Default::default()
            }),
        ]
        .align_y(Alignment::Center)
        .into()
    } else {
        text("🔔").size(11).color(palette.text_primary).into()
    };

    let bell_btn = button(bell_content)
        .on_press(Message::ToggleOverlay(ActiveOverlay::Notifications))
        .style(top_bar_icon_style())
        .padding([2, 4]);

    let right_section = frosted_glass_container(
        palette,
        17.0,
        true,
        container(
            row![
                wifi_btn,
                volume_btn,
                dnd_btn,
                bell_btn,
                px_space(2.0, 0.0),
                container(px_space(1.0, 12.0)).style(move |_t| container::Style {
                    background: Some(iced::Background::Color(divider_tint)),
                    ..Default::default()
                }),
                px_space(3.0, 0.0),
                button(clock_text)
                    .on_press(Message::ToggleOverlay(ActiveOverlay::ControlCenter))
                    .style(top_bar_icon_style())
                    .padding([1, 4]),
            ]
            .spacing(3)
            .align_y(Alignment::Center),
        )
        .height(32)
        .padding(iced::Padding {
            top: 2.0,
            right: 8.0,
            bottom: 2.0,
            left: 6.0,
        })
        .align_y(Alignment::Center),
    );

    // ------------------------------------------------------------------------
    // Assemble 3-Zone Floating Taskbar
    // ------------------------------------------------------------------------
    let top_bar_row = row![
        left_section,
        h_space(),
        center_flank,
        h_space(),
        right_section,
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center);

    container(top_bar_row)
        .padding(iced::Padding {
            top: 6.0,
            right: 18.0,
            bottom: 2.0,
            left: 18.0,
        })
        .width(Length::Fill)
        .into()
}

fn top_bar_icon_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_t, s| {
        let bg = match s {
            button::Status::Hovered => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.22),
            _ => iced::Color::TRANSPARENT,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(6.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
