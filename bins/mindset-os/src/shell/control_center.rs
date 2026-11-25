//! Control Center Popover for MindSet OS
//!
//! Pixel-faithful reproduction of the Control Center card matching reference:
//! - Header: "Control Center" with Edit (✏️), Play (▶️), Pin (📌), and Close (✕) controls
//! - 4 Quick-Toggle rows with blue circular icon badges, custom toggle pills with "ON"/"OFF" labels
//! - Smooth Brightness (☀️) and Volume (🔊) sliders directly bound to live preferences
//! - Action row with rotation lock and settings buttons
//! - Dual Quick-Action Tiles: live Wi-Fi status and Do Not Disturb state
//! - Quick Theme Switcher row: Alpine Glass, Dark Space, Violet Nebula, Solar Amber
//! - Footer with Cast/System Monitor (🗔) and Settings (⚙️) buttons.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::widget::{button, column, container, row, slider, text, Space};
use iced::{Alignment, Color, Element, Length};
use mindset_core::{MindSetCoreState, ThemeMode, WindowId};

use crate::theme::{
    frosted_glass_container, frosted_glass_style, h_space, modern_slider, px_space,
    secondary_button, toggle_button, ThemePalette, TYPE_BODY, TYPE_CAPTION, TYPE_SUBHEAD,
};
use crate::Message;

/// Truncates a Wi-Fi SSID or label safely on UTF-8 char boundaries using `'…'` so pills never collide.
pub fn truncate_ellipsis(text_val: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let count = text_val.chars().count();
    if count <= max_chars {
        return text_val.to_string();
    }
    if max_chars == 1 {
        return "…".to_string();
    }
    let mut out: String = text_val.chars().take(max_chars - 1).collect();
    out.push('…');
    out
}

pub fn view<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    network_status: &'a mindset_sys::network::NetworkStatus,
    nearby_networks: &'a [mindset_sys::network::WifiNetwork],
    wifi_menu_open: bool,
    wifi_connecting_ssid: Option<&'a str>,
    wifi_password_input: &'a str,
) -> Element<'a, Message> {
    // 1. Header with Edit, Play, Pin, Close buttons matching screenshot
    let header = row![
        text("Control Center")
            .size(TYPE_SUBHEAD)
            .color(palette.text_primary),
        h_space(),
        row![
            button(text("✏️").size(TYPE_CAPTION))
                .style(secondary_button(palette))
                .padding([2, 5]),
            button(text("▶️").size(TYPE_CAPTION))
                .style(secondary_button(palette))
                .padding([2, 5]),
            button(text("📌").size(TYPE_CAPTION))
                .style(secondary_button(palette))
                .padding([2, 5]),
            button(text("✕").size(TYPE_CAPTION))
                .on_press(Message::CloseOverlays)
                .style(secondary_button(palette))
                .padding([2, 5]),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    ]
    .align_y(Alignment::Center);

    // 2. 4 Toggle Rows with Circular Blue Badges and ON/OFF text inside toggles
    let airplane_on = core.preferences.airplane_mode;

    let connected_display = if airplane_on {
        "Airplane Mode".to_string()
    } else if !core.preferences.wifi_enabled {
        "Wi-Fi Disabled".to_string()
    } else if let Some(ref ssid) = network_status.connected_ssid {
        truncate_ellipsis(ssid, 16)
    } else {
        "Wi-Fi Disconnected".to_string()
    };

    let wifi_row = toggle_row_styled(
        "📶",
        connected_display,
        core.preferences.wifi_enabled,
        Message::ToggleWifi,
        airplane_on,
        palette,
    );

    let bt_row = toggle_row_styled(
        "ᛒ",
        "Bluetooth",
        core.preferences.bluetooth_enabled,
        Message::ToggleBluetooth,
        airplane_on,
        palette,
    );

    let dnd_row = toggle_row_styled(
        "🌙",
        "Do Not Disturb",
        core.preferences.focus_dnd,
        Message::ToggleDnd,
        false,
        palette,
    );

    let airplane_row = toggle_row_styled(
        "✈️",
        "Airplane mode",
        airplane_on,
        Message::ToggleAirplaneMode,
        false,
        palette,
    );

    let toggles_col = column![wifi_row, bt_row, dnd_row, airplane_row].spacing(6);

    // 3. Dual Sliders: Brightness & Volume
    let brightness_slider = row![
        text("☀️").size(TYPE_SUBHEAD),
        slider(
            0.20..=1.0,
            core.preferences.brightness,
            Message::BrightnessChanged,
        )
        .step(0.05_f32)
        .style(modern_slider(palette)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let volume_slider = row![
        text("🔊").size(TYPE_SUBHEAD),
        slider(0.0..=1.0, core.preferences.volume, Message::VolumeChanged)
            .step(0.05_f32)
            .style(modern_slider(palette)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let sliders_col = column![brightness_slider, volume_slider].spacing(8);

    // 4. Center Quick Action Row (Rotation / Settings icons)
    let action_row = row![
        h_space(),
        container(
            row![
                button(text("🔄").size(TYPE_BODY))
                    .style(secondary_button(palette))
                    .padding([3, 7]),
                button(text("⚙️").size(TYPE_BODY))
                    .on_press(Message::OpenWindow(WindowId::Settings))
                    .style(secondary_button(palette))
                    .padding([3, 7]),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding(2),
        px_space(8.0, 0.0),
    ]
    .align_y(Alignment::Center);

    // 5. Dual Action Tiles with guaranteed equal split & UTF-8 ellipsis truncation (never collide with "DND Off")
    let (tile_wifi_label, wifi_is_primary) = if airplane_on {
        ("Airplane".to_string(), false)
    } else if !core.preferences.wifi_enabled {
        ("Wi-Fi Off".to_string(), false)
    } else if let Some(ref ssid) = network_status.connected_ssid {
        (truncate_ellipsis(ssid, 9), true)
    } else {
        ("Wi-Fi".to_string(), false)
    };

    let tile_network = container(
        button(
            row![
                text(if airplane_on { "✈️" } else { "📶" }).size(TYPE_BODY),
                text(tile_wifi_label)
                    .size(TYPE_BODY)
                    .color(if wifi_is_primary {
                        palette.accent_text
                    } else {
                        palette.text_primary
                    }),
                h_space(),
                text(if wifi_menu_open { "▴" } else { "▾" })
                    .size(TYPE_CAPTION)
                    .color(if wifi_is_primary {
                        palette.accent_text
                    } else {
                        palette.text_secondary
                    }),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .on_press(if airplane_on {
            Message::ToggleAirplaneMode
        } else {
            Message::ToggleWifiMenu
        })
        .style(toggle_button(palette, wifi_is_primary))
        .padding([8, 10])
        .width(Length::Fill),
    )
    .width(Length::FillPortion(1))
    .clip(true);

    let dnd_active = core.preferences.focus_dnd;
    let tile_mode = container(
        button(
            row![
                text("🌙").size(TYPE_BODY),
                text(if dnd_active { "DND On" } else { "DND Off" })
                    .size(TYPE_BODY)
                    .color(if dnd_active {
                        palette.accent_text
                    } else {
                        palette.text_primary
                    }),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .on_press(Message::ToggleDnd)
        .style(toggle_button(palette, dnd_active))
        .padding([8, 10])
        .width(Length::Fill),
    )
    .width(Length::FillPortion(1))
    .clip(true);

    let tiles_row = row![tile_network, tile_mode]
        .spacing(10)
        .width(Length::Fill);

    // 5b. Expandable Wi-Fi Network Selector & Connection Menu
    let mut wifi_menu_widget = column![].spacing(6);
    if wifi_menu_open && core.preferences.wifi_enabled && !airplane_on {
        let mut list_col = column![row![
            text("Nearby Wi-Fi").size(TYPE_BODY).color(palette.accent),
            h_space(),
            button(text("🔄 Scan").size(TYPE_CAPTION))
                .on_press(Message::ScanWifiNetworks)
                .style(secondary_button(palette))
                .padding([2, 6]),
        ]
        .align_y(Alignment::Center),]
        .spacing(6);

        if nearby_networks.is_empty() {
            list_col = list_col.push(
                text("Scanning for Wi-Fi networks...")
                    .size(TYPE_CAPTION)
                    .color(palette.text_muted),
            );
        } else {
            for net in nearby_networks.iter().take(5) {
                let is_current = net.in_use
                    || network_status
                        .connected_ssid
                        .as_deref()
                        .map(|s| s == net.ssid)
                        .unwrap_or(false);

                let signal_icon = if net.signal_percent >= 70 {
                    "📶"
                } else if net.signal_percent >= 35 {
                    "🛜"
                } else {
                    "📡"
                };

                let lock_icon = if !net.security.is_empty() && net.security != "--" {
                    "🔒"
                } else {
                    ""
                };

                let net_btn = button(
                    row![
                        text(signal_icon).size(TYPE_BODY),
                        text(truncate_ellipsis(&net.ssid, 18))
                            .size(TYPE_CAPTION)
                            .color(if is_current {
                                palette.accent_text
                            } else {
                                palette.text_primary
                            }),
                        text(lock_icon).size(TYPE_CAPTION).color(if is_current {
                            palette.accent_text
                        } else {
                            palette.text_muted
                        }),
                        h_space(),
                        text(format!("{}%", net.signal_percent))
                            .size(TYPE_CAPTION)
                            .color(if is_current {
                                palette.accent_text
                            } else {
                                palette.text_muted
                            }),
                        if is_current {
                            text("✓").size(TYPE_CAPTION).color(palette.accent_text)
                        } else {
                            text("").size(0)
                        },
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                )
                .on_press(Message::SelectWifiNetwork(net.ssid.clone()))
                .style(toggle_button(palette, is_current))
                .padding([4, 6])
                .width(Length::Fill);

                list_col = list_col.push(net_btn);
            }
        }

        // Connecting password prompt if selected
        if let Some(target_ssid) = wifi_connecting_ssid {
            let pass_input = iced::widget::text_input("Wi-Fi Password...", wifi_password_input)
                .on_input(Message::WifiPasswordInputChanged)
                .on_submit(Message::SubmitWifiConnect)
                .secure(true)
                .style(crate::theme::modern_text_input(palette))
                .padding(5)
                .width(Length::Fill);

            let prompt = column![
                text(format!("Connect: {target_ssid}"))
                    .size(TYPE_CAPTION)
                    .color(palette.accent),
                pass_input,
                row![
                    button(text("Cancel").size(TYPE_CAPTION))
                        .on_press(Message::CancelWifiConnect)
                        .style(secondary_button(palette))
                        .padding([3, 6]),
                    h_space(),
                    button(
                        text("Connect")
                            .size(TYPE_CAPTION)
                            .color(palette.accent_text)
                    )
                    .on_press(Message::SubmitWifiConnect)
                    .style(crate::theme::primary_button(palette))
                    .padding([3, 10]),
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(4);

            list_col = list_col.push(prompt);
        }

        let wifi_box = container(list_col)
            .padding(8)
            .style(move |_t| frosted_glass_style(palette, 12.0, false))
            .width(Length::Fill);

        wifi_menu_widget = wifi_menu_widget.push(wifi_box);
    }

    // 6. Quick Theme Switcher Row (Live binding to ThemeMode)
    let current_theme = core.preferences.theme;
    let theme_pills = row![
        quick_theme_btn("🏔️", ThemeMode::AlpineGlass, current_theme, palette),
        quick_theme_btn("🌌", ThemeMode::DarkSpace, current_theme, palette),
        quick_theme_btn("🔮", ThemeMode::VioletNebula, current_theme, palette),
        quick_theme_btn("☀️", ThemeMode::SolarAmber, current_theme, palette),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let theme_row = row![
        text("🎨").size(TYPE_BODY),
        px_space(4.0, 0.0),
        text("Theme")
            .size(TYPE_CAPTION)
            .color(palette.text_secondary),
        h_space(),
        theme_pills,
    ]
    .align_y(Alignment::Center);

    // 7. Footer Bar matching screenshot
    let footer = row![
        button(text("🗔").size(TYPE_SUBHEAD))
            .on_press(Message::OpenWindow(WindowId::SystemMonitor))
            .style(secondary_button(palette))
            .padding([4, 8]),
        h_space(),
        button(text("⚙️").size(TYPE_SUBHEAD))
            .on_press(Message::OpenWindow(WindowId::Settings))
            .style(secondary_button(palette))
            .padding([4, 8]),
    ]
    .align_y(Alignment::Center);

    let mut popover_col =
        column![header, toggles_col, sliders_col, action_row, tiles_row,].spacing(10);

    if wifi_menu_open && core.preferences.wifi_enabled && !airplane_on {
        popover_col = popover_col.push(wifi_menu_widget);
    }

    let popover_content = popover_col.push(theme_row).push(footer).padding(16);

    frosted_glass_container(palette, 18.0, true, container(popover_content).width(300))
}

fn quick_theme_btn<'a>(
    icon: &'static str,
    mode: ThemeMode,
    current_mode: ThemeMode,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let is_selected = mode == current_mode;
    button(text(icon).size(TYPE_BODY))
        .on_press(Message::ThemeSelected(mode))
        .style(toggle_button(palette, is_selected))
        .padding([4, 8])
        .into()
}

fn toggle_row_styled<'a>(
    icon: &'static str,
    label: impl Into<String>,
    is_on: bool,
    msg: Message,
    is_locked: bool,
    palette: ThemePalette,
) -> Element<'a, Message> {
    // Circular blue badge around icon
    let icon_badge = container(text(icon).size(TYPE_BODY).color(Color::WHITE))
        .width(26)
        .height(26)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.42, 0.88))),
            border: iced::Border {
                radius: iced::border::Radius::from(13.0),
                ..Default::default()
            },
            ..Default::default()
        });

    let thumb = container(Space::new().width(12).height(12)).style(|_t| container::Style {
        background: Some(iced::Background::Color(Color::WHITE)),
        border: iced::Border {
            radius: iced::border::Radius::from(6.0),
            ..Default::default()
        },
        ..Default::default()
    });

    let switch_content = if is_on {
        row![
            text("ON").size(TYPE_CAPTION).color(Color::WHITE),
            h_space(),
            thumb
        ]
        .align_y(Alignment::Center)
    } else {
        row![
            thumb,
            h_space(),
            text("OFF")
                .size(TYPE_CAPTION)
                .color(Color::from_rgb(0.92, 0.95, 0.99)),
        ]
        .align_y(Alignment::Center)
    };

    let track_bg = if is_locked {
        Color::from_rgba(0.25, 0.25, 0.25, 0.40)
    } else if is_on {
        Color::from_rgb(0.12, 0.45, 0.92)
    } else {
        Color::from_rgba(0.22, 0.28, 0.38, 0.50)
    };

    let switch_widget = container(switch_content.padding([1, 4]))
        .width(42)
        .height(18)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(track_bg)),
            border: iced::Border {
                radius: iced::border::Radius::from(9.0),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.35),
                width: 1.0,
            },
            ..Default::default()
        });

    let label_color = if is_locked {
        palette.text_muted
    } else {
        palette.text_primary
    };

    let mut row_btn = button(
        row![
            icon_badge,
            px_space(6.0, 0.0),
            text(label.into()).size(TYPE_BODY).color(label_color),
            h_space(),
            switch_widget,
        ]
        .align_y(Alignment::Center),
    )
    .style(move |_t, s| {
        let bg = match s {
            button::Status::Hovered if !is_locked => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.15),
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
    .padding([3, 6])
    .width(Length::Fill);

    if !is_locked {
        row_btn = row_btn.on_press(msg);
    }

    row_btn.into()
}
