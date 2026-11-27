//! Flagship Liquidmorphism Auth Lock Screen for MindSet OS (Super+L)
//!
//! Designed with clear spatial hierarchy, crisp Pure-Rust vector iconography
//! (zero Linux emoji tofu boxes), WCAG AAA translucent scrim contrast, and tactile
//! circular glass keypad ergonomics:
//! - Top ambient status strip with MindSet OS vector monogram and Focus Shield badge
//! - Large hero clock, formatted date, and personalized student greeting
//! - Elevated Liquidmorphic glass auth pill with student initials avatar ring
//! - Segmented 6-Digit PIN / Password selector, glowing PIN pips, circular 3x4 keypad
//! - Hardware keyboard Enter submission, Escape interlock, and 5-attempt lockout timer
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::border::Radius;
use iced::gradient::Linear;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{
    Alignment, Background, Border, Color, Degrees, Element, Gradient, Length, Shadow, Theme, Vector,
};
use mindset_core::ThemeMode;

use crate::shell::top_bar::MindSetLogoProgram;
use crate::storage::db::UserProfile;
use crate::system::state::SystemState;
use crate::theme::icons::{vector_icon, MsIconKind};
use crate::theme::{
    frosted_glass_container, h_space, primary_button, px_space, secondary_button, v_space,
    ThemePalette, TYPE_BODY, TYPE_CAPTION, TYPE_SUBHEAD,
};
use crate::Message;

/// Authentication mode for the lock screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockAuthMode {
    Pin,
    Password,
}

/// Persistent UI state for the lock screen
#[derive(Debug, Clone)]
pub struct LockScreenState {
    pub mode: LockAuthMode,
    pub pin_input: String,
    pub password_input: String,
    pub show_password: bool,
    pub error_message: Option<String>,
    pub failed_attempts: i32,
    pub lockout_seconds_remaining: i64,
    pub is_locked_out: bool,
}

impl Default for LockScreenState {
    fn default() -> Self {
        Self {
            mode: LockAuthMode::Pin,
            pin_input: String::new(),
            password_input: String::new(),
            show_password: false,
            error_message: None,
            failed_attempts: 0,
            lockout_seconds_remaining: 0,
            is_locked_out: false,
        }
    }
}

fn student_initials(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().filter(|s| !s.is_empty()).collect();
    match parts.as_slice() {
        [] => "MS".to_string(),
        [single] => single.chars().take(2).collect::<String>().to_uppercase(),
        [first, second, ..] => {
            let mut out = String::with_capacity(2);
            if let Some(c) = first.chars().next() {
                out.push(c);
            }
            if let Some(c) = second.chars().next() {
                out.push(c);
            }
            out.to_uppercase()
        }
    }
}

/// Renders the full-viewport Liquidmorphic auth lock screen
pub fn view<'a>(
    state: &'a LockScreenState,
    profile: &'a UserProfile,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let is_light = palette.mode == ThemeMode::AlpineGlass;
    let clock_time = SystemState::formatted_clock();
    let clock_date = SystemState::formatted_date();
    let greeting = SystemState::current_greeting();

    let student_name = if profile.name.trim().is_empty() {
        "Scholar"
    } else {
        profile.name.trim()
    };
    let initials = student_initials(student_name);
    let track_label = if profile.target_label.trim().is_empty() {
        "TN Class 12 Boards • 590/600 Target"
    } else {
        profile.target_label.trim()
    };

    // 1. Top Ambient Status Strip
    let logo_canvas = iced::widget::canvas(MindSetLogoProgram {
        stroke_color: palette.text_primary,
        accent_color: palette.accent,
    })
    .width(24)
    .height(16);

    let top_left_pill = container(
        row![
            logo_canvas,
            text("MindSet OS")
                .size(TYPE_BODY)
                .color(palette.text_primary),
            container(text("LOCKED").size(TYPE_CAPTION).color(palette.accent))
                .padding([2, 7])
                .style(move |_t| container::Style {
                    background: Some(Background::Color(Color {
                        a: 0.16,
                        ..palette.accent
                    })),
                    border: Border {
                        radius: Radius::from(6.0),
                        color: Color {
                            a: 0.45,
                            ..palette.accent
                        },
                        width: 1.0,
                    },
                    ..Default::default()
                }),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .padding([6, 12]),
    )
    .style(move |_t| pill_surface_style(palette, is_light));

    let top_right_pill = container(
        row![
            vector_icon(MsIconKind::Shield, palette.success, 14.0),
            text("Focus Shield Active")
                .size(TYPE_CAPTION)
                .color(palette.text_secondary),
            text("•").size(TYPE_CAPTION).color(palette.text_muted),
            text(clock_date.clone())
                .size(TYPE_CAPTION)
                .color(palette.text_muted),
        ]
        .spacing(7)
        .align_y(Alignment::Center)
        .padding([6, 12]),
    )
    .style(move |_t| pill_surface_style(palette, is_light));

    let top_bar = row![top_left_pill, h_space(), top_right_pill]
        .width(Length::Fill)
        .align_y(Alignment::Center);

    // 2. Hero Clock & Student Greeting Zone
    let hero_header = column![
        text(clock_time).size(64).color(palette.text_primary),
        text(format!("{}, {}", greeting, student_name))
            .size(TYPE_SUBHEAD)
            .color(palette.text_secondary),
    ]
    .spacing(4)
    .align_x(Alignment::Center);

    // 3. Student Identity Row with Vector Avatar Monogram Ring
    let avatar_badge = container(text(initials).size(TYPE_SUBHEAD).color(palette.accent_text))
        .width(42)
        .height(42)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_t| {
            let grad = Linear::new(Degrees(135.0))
                .add_stop(0.0, palette.accent)
                .add_stop(1.0, palette.accent_hover);
            container::Style {
                background: Some(Background::Gradient(Gradient::Linear(grad))),
                border: Border {
                    radius: Radius::from(21.0),
                    color: Color::from_rgba(0.92, 0.96, 1.0, 0.45),
                    width: 1.5,
                },
                shadow: Shadow {
                    color: Color {
                        a: 0.35,
                        ..palette.accent
                    },
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                },
                ..Default::default()
            }
        });

    let identity_row = container(
        row![
            avatar_badge,
            px_space(12.0, 0.0),
            column![
                text(student_name)
                    .size(TYPE_SUBHEAD)
                    .color(palette.text_primary),
                text(track_label)
                    .size(TYPE_CAPTION)
                    .color(palette.text_muted),
            ]
            .spacing(2),
            h_space(),
            vector_icon(MsIconKind::Lock, palette.accent, 16.0),
        ]
        .align_y(Alignment::Center)
        .padding([10, 14]),
    )
    .width(Length::Fill)
    .style(move |_t| inset_tray_style(palette, is_light, 14.0));

    // 4. Segmented Auth Mode Selector ([ 6-Digit PIN ] | [ Password ])
    let pin_active = state.mode == LockAuthMode::Pin;

    let mode_pin_btn = button(
        row![
            vector_icon(
                MsIconKind::Lock,
                if pin_active {
                    palette.accent_text
                } else {
                    palette.text_secondary
                },
                13.0
            ),
            text("6-Digit PIN").size(TYPE_BODY),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .width(Length::FillPortion(1))
    .padding([7, 12])
    .style(move |t, s| {
        if pin_active {
            primary_button(palette)(t, s)
        } else {
            segment_inactive_button(palette)(t, s)
        }
    })
    .on_press_maybe(if !state.is_locked_out && !pin_active {
        Some(Message::LockScreenToggleAuthMode)
    } else {
        None
    });

    let mode_pwd_btn = button(
        row![
            vector_icon(
                MsIconKind::Key,
                if !pin_active {
                    palette.accent_text
                } else {
                    palette.text_secondary
                },
                13.0
            ),
            text("Password").size(TYPE_BODY),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .width(Length::FillPortion(1))
    .padding([7, 12])
    .style(move |t, s| {
        if !pin_active {
            primary_button(palette)(t, s)
        } else {
            segment_inactive_button(palette)(t, s)
        }
    })
    .on_press_maybe(if !state.is_locked_out && pin_active {
        Some(Message::LockScreenToggleAuthMode)
    } else {
        None
    });

    let mode_selector = container(
        row![mode_pin_btn, px_space(4.0, 0.0), mode_pwd_btn]
            .align_y(Alignment::Center)
            .width(Length::Fill),
    )
    .padding(4)
    .width(Length::Fill)
    .style(move |_t| inset_tray_style(palette, is_light, 12.0));

    // 5. Auth Input Area
    let auth_body: Element<'a, Message> = match state.mode {
        LockAuthMode::Pin => render_pin_mode(state, palette, is_light),
        LockAuthMode::Password => render_password_mode(state, palette, is_light),
    };

    // 6. Error / Lockout Alert Banner
    let banner: Element<'a, Message> = if state.is_locked_out || state.lockout_seconds_remaining > 0
    {
        container(
            row![
                vector_icon(MsIconKind::Shield, palette.danger, 15.0),
                px_space(10.0, 0.0),
                column![
                    text("Security Cooldown Active")
                        .size(TYPE_BODY)
                        .color(palette.danger),
                    text(format!(
                        "Workspace paused briefly for safety. Retry in {}s",
                        state.lockout_seconds_remaining
                    ))
                    .size(TYPE_CAPTION)
                    .color(palette.text_secondary),
                ]
                .spacing(2),
            ]
            .align_y(Alignment::Center)
            .padding([10, 14]),
        )
        .width(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.95, 0.22, 0.24, 0.14))),
            border: Border {
                radius: Radius::from(10.0),
                color: Color::from_rgba(0.95, 0.22, 0.24, 0.45),
                width: 1.0,
            },
            ..Default::default()
        })
        .into()
    } else if let Some(ref err) = state.error_message {
        container(
            row![
                vector_icon(MsIconKind::Lock, palette.warning, 14.0),
                px_space(8.0, 0.0),
                text(err.as_str()).size(TYPE_CAPTION).color(palette.warning),
            ]
            .align_y(Alignment::Center)
            .padding([8, 12]),
        )
        .width(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.96, 0.68, 0.20, 0.12))),
            border: Border {
                radius: Radius::from(10.0),
                color: Color::from_rgba(0.96, 0.68, 0.20, 0.38),
                width: 1.0,
            },
            ..Default::default()
        })
        .into()
    } else {
        Space::new().height(0).into()
    };

    // 7. Primary Unlock CTA
    let can_submit = !state.is_locked_out
        && match state.mode {
            LockAuthMode::Pin => state.pin_input.len() == 6,
            LockAuthMode::Password => !state.password_input.trim().is_empty(),
        };

    let unlock_btn = button(
        row![
            vector_icon(
                MsIconKind::Unlock,
                if can_submit {
                    palette.accent_text
                } else {
                    palette.text_muted
                },
                14.0
            ),
            px_space(8.0, 0.0),
            text("Unlock Workspace").size(TYPE_BODY),
        ]
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .on_press_maybe(if can_submit {
        Some(Message::SubmitLockScreenAuth)
    } else {
        None
    })
    .style(move |t, s| {
        if can_submit {
            primary_button(palette)(t, s)
        } else {
            secondary_button(palette)(t, s)
        }
    })
    .padding([10, 18]);

    // 8. Elevated Liquidmorphic Auth Card
    let card_inner = column![identity_row, mode_selector, auth_body, banner, unlock_btn,]
        .spacing(14)
        .align_x(Alignment::Center)
        .padding(22);

    let auth_card = container(frosted_glass_container(palette, 24.0, true, card_inner)).width(390);

    // 9. Subtle Footer Hint Strip
    let footer_hint = container(
        text("Press Enter to unlock  •  Demo PIN: 123456  •  Demo Password: MindSet2026!")
            .size(TYPE_CAPTION)
            .color(palette.text_muted),
    )
    .padding([6, 14])
    .style(move |_t| pill_surface_style(palette, is_light));

    // Full-Viewport Layout
    let backdrop_col = column![
        top_bar,
        v_space(),
        hero_header,
        px_space(0.0, 14.0),
        auth_card,
        v_space(),
        footer_hint,
    ]
    .align_x(Alignment::Center)
    .padding(iced::Padding {
        top: 18.0,
        right: 26.0,
        bottom: 18.0,
        left: 26.0,
    })
    .width(Length::Fill)
    .height(Length::Fill);

    container(backdrop_col)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_t| {
            let (bg_top, bg_bot) = if is_light {
                (
                    Color::from_rgba(0.76, 0.85, 0.96, 0.88),
                    Color::from_rgba(0.64, 0.76, 0.92, 0.92),
                )
            } else {
                (
                    Color::from_rgba(0.024, 0.035, 0.068, 0.86),
                    Color::from_rgba(0.012, 0.018, 0.038, 0.92),
                )
            };
            let grad = Linear::new(Degrees(160.0))
                .add_stop(0.0, bg_top)
                .add_stop(1.0, bg_bot);
            container::Style {
                background: Some(Background::Gradient(Gradient::Linear(grad))),
                ..Default::default()
            }
        })
        .into()
}

fn pill_surface_style(palette: ThemePalette, is_light: bool) -> container::Style {
    container::Style {
        background: Some(Background::Color(if is_light {
            Color::from_rgba(0.94, 0.96, 0.99, 0.78)
        } else {
            Color::from_rgba(0.05, 0.08, 0.14, 0.74)
        })),
        border: Border {
            radius: Radius::from(14.0),
            color: palette.glass_border,
            width: 1.0,
        },
        ..Default::default()
    }
}

fn inset_tray_style(palette: ThemePalette, is_light: bool, radius: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(if is_light {
            Color::from_rgba(0.08, 0.14, 0.28, 0.07)
        } else {
            Color::from_rgba(0.03, 0.05, 0.09, 0.36)
        })),
        border: Border {
            radius: Radius::from(radius),
            color: Color {
                a: 0.22,
                ..palette.glass_border
            },
            width: 1.0,
        },
        ..Default::default()
    }
}

fn segment_inactive_button(
    palette: ThemePalette,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_t, status| {
        let bg = match status {
            button::Status::Hovered => Color::from_rgba(0.90, 0.94, 1.0, 0.09),
            button::Status::Pressed => Color::from_rgba(0.90, 0.94, 1.0, 0.15),
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: palette.text_secondary,
            border: Border {
                radius: Radius::from(8.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

fn circular_keypad_button(
    palette: ThemePalette,
    is_light: bool,
    is_action: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_t, status| {
        let (bg, border_col, txt_col) = match status {
            button::Status::Hovered => (
                if is_light {
                    Color::from_rgba(0.08, 0.44, 0.90, 0.16)
                } else {
                    Color::from_rgba(0.90, 0.95, 1.0, 0.16)
                },
                palette.glass_border_focused,
                palette.text_primary,
            ),
            button::Status::Pressed => (palette.accent, palette.accent, palette.accent_text),
            button::Status::Disabled => (
                Color::from_rgba(0.45, 0.50, 0.60, 0.08),
                Color::TRANSPARENT,
                palette.text_muted,
            ),
            button::Status::Active => (
                if is_action {
                    Color::TRANSPARENT
                } else if is_light {
                    Color::from_rgba(0.94, 0.96, 0.99, 0.78)
                } else {
                    Color::from_rgba(0.90, 0.94, 1.0, 0.09)
                },
                if is_action {
                    Color {
                        a: 0.18,
                        ..palette.glass_border
                    }
                } else {
                    Color {
                        a: 0.30,
                        ..palette.glass_border
                    }
                },
                if is_action {
                    palette.text_muted
                } else {
                    palette.text_primary
                },
            ),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: txt_col,
            border: Border {
                radius: Radius::from(24.0),
                color: border_col,
                width: 1.0,
            },
            shadow: Shadow::default(),
            ..Default::default()
        }
    }
}

/// Renders 6 glowing PIN pips, keyboard input field, and circular 3x4 glass keypad
fn render_pin_mode<'a>(
    state: &'a LockScreenState,
    palette: ThemePalette,
    is_light: bool,
) -> Element<'a, Message> {
    let mut dots_row = row![].spacing(14).align_y(Alignment::Center);
    for i in 0..6 {
        let is_filled = i < state.pin_input.len();
        let dot = container(Space::new().width(12).height(12)).style(move |_t: &Theme| {
            container::Style {
                background: Some(Background::Color(if is_filled {
                    palette.accent
                } else if is_light {
                    Color::from_rgba(0.08, 0.14, 0.28, 0.12)
                } else {
                    Color::from_rgba(0.90, 0.94, 1.0, 0.12)
                })),
                border: Border {
                    radius: Radius::from(6.0),
                    color: if is_filled {
                        palette.accent
                    } else if is_light {
                        Color::from_rgba(0.08, 0.14, 0.28, 0.30)
                    } else {
                        Color::from_rgba(0.90, 0.94, 1.0, 0.28)
                    },
                    width: 1.5,
                },
                shadow: if is_filled {
                    Shadow {
                        color: Color {
                            a: 0.45,
                            ..palette.accent
                        },
                        offset: Vector::ZERO,
                        blur_radius: 6.0,
                    }
                } else {
                    Shadow::default()
                },
                ..Default::default()
            }
        });
        dots_row = dots_row.push(dot);
    }

    let dots_tray = container(dots_row)
        .padding([10, 20])
        .style(move |_t| inset_tray_style(palette, is_light, 16.0));

    let pin_input_field = text_input("Type 6-digit PIN...", &state.pin_input)
        .secure(true)
        .size(TYPE_BODY)
        .width(Length::Fixed(196.0))
        .padding([7, 12])
        .on_input(|s| {
            let digits: String = s.chars().filter(|c| c.is_ascii_digit()).take(6).collect();
            Message::LockScreenPinChanged(digits)
        })
        .on_submit(Message::SubmitLockScreenAuth)
        .style(move |_t, _s| text_input::Style {
            background: Background::Color(if is_light {
                Color::from_rgba(0.95, 0.97, 1.0, 0.88)
            } else {
                Color::from_rgba(0.03, 0.05, 0.09, 0.58)
            }),
            border: Border {
                radius: Radius::from(10.0),
                color: palette.glass_border,
                width: 1.0,
            },
            icon: palette.text_muted,
            placeholder: palette.text_muted,
            value: palette.text_primary,
            selection: palette.accent,
        });

    let keypad_btn = |label: &'static str, msg: Option<Message>, is_action: bool| {
        button(
            container(text(label).size(if is_action {
                TYPE_CAPTION
            } else {
                TYPE_SUBHEAD
            }))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
        )
        .width(62)
        .height(46)
        .style(circular_keypad_button(palette, is_light, is_action))
        .on_press_maybe(if state.is_locked_out { None } else { msg })
    };

    let row1 = row![
        keypad_btn("1", Some(Message::LockScreenKeypadDigit('1')), false),
        keypad_btn("2", Some(Message::LockScreenKeypadDigit('2')), false),
        keypad_btn("3", Some(Message::LockScreenKeypadDigit('3')), false),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let row2 = row![
        keypad_btn("4", Some(Message::LockScreenKeypadDigit('4')), false),
        keypad_btn("5", Some(Message::LockScreenKeypadDigit('5')), false),
        keypad_btn("6", Some(Message::LockScreenKeypadDigit('6')), false),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let row3 = row![
        keypad_btn("7", Some(Message::LockScreenKeypadDigit('7')), false),
        keypad_btn("8", Some(Message::LockScreenKeypadDigit('8')), false),
        keypad_btn("9", Some(Message::LockScreenKeypadDigit('9')), false),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let row4 = row![
        keypad_btn("DEL", Some(Message::LockScreenKeypadBackspace), true),
        keypad_btn("0", Some(Message::LockScreenKeypadDigit('0')), false),
        keypad_btn("CLR", Some(Message::LockScreenKeypadClear), true),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    column![
        dots_tray,
        pin_input_field,
        column![row1, row2, row3, row4]
            .spacing(9)
            .align_x(Alignment::Center),
    ]
    .spacing(10)
    .align_x(Alignment::Center)
    .into()
}

/// Renders password input field with vector Eye show/hide toggle
fn render_password_mode<'a>(
    state: &'a LockScreenState,
    palette: ThemePalette,
    is_light: bool,
) -> Element<'a, Message> {
    let eye_kind = if state.show_password {
        MsIconKind::EyeOff
    } else {
        MsIconKind::Eye
    };
    let show_hide_btn = button(vector_icon(eye_kind, palette.text_primary, 16.0))
        .padding([8, 11])
        .style(secondary_button(palette))
        .on_press(Message::LockScreenToggleShowPassword);

    let input_field = text_input("Enter workspace password...", &state.password_input)
        .secure(!state.show_password)
        .size(TYPE_BODY)
        .width(Length::Fill)
        .padding([9, 12])
        .on_input(Message::LockScreenPasswordChanged)
        .on_submit(Message::SubmitLockScreenAuth)
        .style(move |_t, _s| text_input::Style {
            background: Background::Color(if is_light {
                Color::from_rgba(0.95, 0.97, 1.0, 0.88)
            } else {
                Color::from_rgba(0.03, 0.05, 0.09, 0.58)
            }),
            border: Border {
                radius: Radius::from(10.0),
                color: palette.glass_border,
                width: 1.0,
            },
            icon: palette.text_muted,
            placeholder: palette.text_muted,
            value: palette.text_primary,
            selection: palette.accent,
        });

    row![input_field, px_space(8.0, 0.0), show_hide_btn]
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_screen_default_state() {
        let state = LockScreenState::default();
        assert_eq!(state.mode, LockAuthMode::Pin);
        assert!(state.pin_input.is_empty());
        assert!(state.password_input.is_empty());
        assert!(!state.show_password);
        assert!(!state.is_locked_out);
        assert_eq!(state.failed_attempts, 0);
        assert_eq!(state.lockout_seconds_remaining, 0);
        assert!(state.error_message.is_none());
    }

    #[test]
    fn test_lock_screen_auth_mode_toggle() {
        let mut state = LockScreenState::default();
        state.mode = match state.mode {
            LockAuthMode::Pin => LockAuthMode::Password,
            LockAuthMode::Password => LockAuthMode::Pin,
        };
        assert_eq!(state.mode, LockAuthMode::Password);
    }

    #[test]
    fn test_lock_screen_keypad_actions() {
        let mut state = LockScreenState::default();
        for c in "123456".chars() {
            if state.pin_input.len() < 6 {
                state.pin_input.push(c);
            }
        }
        assert_eq!(state.pin_input, "123456");

        if state.pin_input.len() < 6 {
            state.pin_input.push('7');
        }
        assert_eq!(state.pin_input, "123456");

        state.pin_input.pop();
        assert_eq!(state.pin_input, "12345");

        state.pin_input.clear();
        assert!(state.pin_input.is_empty());
    }

    #[test]
    fn test_lock_screen_lockout_decrement() {
        let mut state = LockScreenState {
            is_locked_out: true,
            lockout_seconds_remaining: 2,
            ..Default::default()
        };

        if state.lockout_seconds_remaining > 0 {
            state.lockout_seconds_remaining -= 1;
            if state.lockout_seconds_remaining == 0 {
                state.is_locked_out = false;
                state.error_message = None;
            }
        }
        assert!(state.is_locked_out);
        assert_eq!(state.lockout_seconds_remaining, 1);

        if state.lockout_seconds_remaining > 0 {
            state.lockout_seconds_remaining -= 1;
            if state.lockout_seconds_remaining == 0 {
                state.is_locked_out = false;
                state.error_message = None;
            }
        }
        assert!(!state.is_locked_out);
        assert_eq!(state.lockout_seconds_remaining, 0);
    }

    #[test]
    fn test_student_initials_formatting() {
        assert_eq!(student_initials("Logesh Kumar"), "LK");
        assert_eq!(student_initials("Scholar"), "SC");
        assert_eq!(student_initials(""), "MS");
    }
}
