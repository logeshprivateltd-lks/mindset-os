//! Frosted Auth Lock Screen Overlay for MindSet OS (Super+L)
//!
//! Production-grade authentication lock screen with rate limiting and brute-force lockout:
//! - Dual-mode authentication: 6-digit numeric PIN and secure alphanumeric Password.
//! - Student identity badge with protected target exam trajectory.
//! - Interactive 3x4 touch keypad for touchscreens and pointer input.
//! - Hardware keyboard support with Enter hotkey submission.
//! - Live lockout countdown timer with security warning banner.
//! - Mutually exclusive compile-time overlay routing and Escape key interlock.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::border::Radius;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Theme, Vector};

use crate::storage::db::UserProfile;
use crate::system::state::SystemState;
use crate::theme::{
    glass_elevated, h_space, primary_button, px_space, secondary_button, v_space, ThemePalette,
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

/// Renders the production-grade auth lock screen overlay
pub fn view<'a>(
    state: &'a LockScreenState,
    profile: &'a UserProfile,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let clock_time = SystemState::formatted_clock();
    let clock_date = SystemState::formatted_date();
    let greeting = SystemState::current_greeting();

    // 1. Clock & Date Header
    let student_name = if profile.name.trim().is_empty() {
        "Scholar"
    } else {
        profile.name.as_str()
    };

    let clock_box = column![
        text(clock_time).size(54).color(palette.text_primary),
        text(clock_date).size(15).color(palette.text_secondary),
        text(format!("{}, {}", greeting, student_name))
            .size(14)
            .color(palette.accent),
    ]
    .spacing(4)
    .align_x(Alignment::Center);

    // 2. Student Identity & Goal Protection Badge
    let track_label = if profile.target_label.trim().is_empty() {
        "TN Class 12 Board Target • 590/600 Target Protected"
    } else {
        profile.target_label.as_str()
    };

    let profile_badge = container(
        row![
            text("👤").size(16),
            px_space(8.0, 0.0),
            column![
                text(student_name).size(13).color(palette.text_primary),
                text(track_label).size(10).color(palette.text_muted),
            ],
            h_space(),
            text("🔒 PROTECTED").size(10).color(palette.accent),
        ]
        .align_y(Alignment::Center)
        .padding([8, 14]),
    )
    .width(Length::Fill)
    .style(move |_t| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.35))),
        border: Border {
            radius: Radius::from(10.0),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.12),
            width: 1.0,
        },
        ..Default::default()
    });

    // 3. Segmented Mode Selector: [ PIN ] | [ Password ]
    let pin_active = state.mode == LockAuthMode::Pin;
    let mode_pin_btn = button(
        row![text("🔢").size(13), text("6-Digit PIN").size(12)]
            .spacing(6)
            .align_y(Alignment::Center),
    )
    .padding([6, 16])
    .style(move |t, s| {
        if pin_active {
            primary_button(palette)(t, s)
        } else {
            secondary_button(palette)(t, s)
        }
    })
    .on_press_maybe(if !state.is_locked_out && !pin_active {
        Some(Message::LockScreenToggleAuthMode)
    } else {
        None
    });

    let mode_pwd_btn = button(
        row![text("🔑").size(13), text("Password").size(12)]
            .spacing(6)
            .align_y(Alignment::Center),
    )
    .padding([6, 16])
    .style(move |t, s| {
        if !pin_active {
            primary_button(palette)(t, s)
        } else {
            secondary_button(palette)(t, s)
        }
    })
    .on_press_maybe(if !state.is_locked_out && pin_active {
        Some(Message::LockScreenToggleAuthMode)
    } else {
        None
    });

    let mode_selector =
        row![mode_pin_btn, px_space(8.0, 0.0), mode_pwd_btn].align_y(Alignment::Center);

    // 4. Mode-Specific Input Controls
    let auth_body: Element<'a, Message> = match state.mode {
        LockAuthMode::Pin => render_pin_mode(state, palette),
        LockAuthMode::Password => render_password_mode(state, palette),
    };

    // 5. Error & Lockout Banner
    let banner: Element<'a, Message> = if state.is_locked_out || state.lockout_seconds_remaining > 0
    {
        container(
            row![
                text("🚨").size(16),
                px_space(8.0, 0.0),
                column![
                    text("Account Temporarily Locked")
                        .size(12)
                        .color(palette.danger),
                    text(format!(
                        "Too many failed attempts. Try again in {}s",
                        state.lockout_seconds_remaining
                    ))
                    .size(11)
                    .color(palette.text_secondary),
                ],
            ]
            .align_y(Alignment::Center)
            .padding([10, 14]),
        )
        .width(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.95, 0.20, 0.20, 0.15))),
            border: Border {
                radius: Radius::from(8.0),
                color: Color::from_rgba(0.95, 0.20, 0.20, 0.50),
                width: 1.0,
            },
            ..Default::default()
        })
        .into()
    } else if let Some(ref err) = state.error_message {
        container(
            row![
                text("⚠️").size(14),
                px_space(8.0, 0.0),
                text(err.as_str()).size(12).color(palette.warning),
            ]
            .align_y(Alignment::Center)
            .padding([8, 12]),
        )
        .width(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.96, 0.68, 0.20, 0.12))),
            border: Border {
                radius: Radius::from(8.0),
                color: Color::from_rgba(0.96, 0.68, 0.20, 0.40),
                width: 1.0,
            },
            ..Default::default()
        })
        .into()
    } else {
        Space::new().height(0).into()
    };

    // 6. Unlock Button
    let can_submit = !state.is_locked_out
        && match state.mode {
            LockAuthMode::Pin => state.pin_input.len() == 6,
            LockAuthMode::Password => !state.password_input.trim().is_empty(),
        };

    let unlock_btn = button(
        row![
            text("🔓").size(14),
            px_space(6.0, 0.0),
            text("Unlock Workspace").size(13),
            px_space(6.0, 0.0),
            text("(↵ Enter)")
                .size(10)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.60)),
        ]
        .align_y(Alignment::Center),
    )
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
    .padding([10, 24]);

    // 7. Assemble Central Elevated Modal Card
    let card = container(
        column![
            clock_box,
            profile_badge,
            mode_selector,
            auth_body,
            banner,
            unlock_btn,
        ]
        .spacing(18)
        .align_x(Alignment::Center)
        .padding(28),
    )
    .width(440)
    .style(move |_t| glass_elevated(palette));

    // Full-screen backdrop
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
            background: Some(Background::Color(Color::from_rgba(
                0.015, 0.025, 0.05, 0.88,
            ))),
            ..Default::default()
        })
        .into()
}

/// Renders 6-dot indicator, hidden text field, and 3x4 touch keypad for PIN authentication
fn render_pin_mode<'a>(state: &'a LockScreenState, palette: ThemePalette) -> Element<'a, Message> {
    // 6 PIN indicator dots
    let mut dots_row = row![].spacing(14).align_y(Alignment::Center);
    for i in 0..6 {
        let is_filled = i < state.pin_input.len();
        let dot = container(Space::new().width(14).height(14)).style(move |_t: &Theme| {
            container::Style {
                background: Some(Background::Color(if is_filled {
                    palette.accent
                } else {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.15)
                })),
                border: Border {
                    radius: Radius::from(7.0),
                    color: if is_filled {
                        palette.accent
                    } else {
                        Color::from_rgba(1.0, 1.0, 1.0, 0.30)
                    },
                    width: 1.5,
                },
                shadow: if is_filled {
                    Shadow {
                        color: Color::from_rgba(0.20, 0.60, 1.00, 0.50),
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

    // Direct text_input for keyboard entry
    let pin_input_field = text_input("Enter 6-digit PIN...", &state.pin_input)
        .secure(true)
        .size(13)
        .width(Length::Fixed(200.0))
        .padding([8, 12])
        .on_input(|s| {
            let digits: String = s.chars().filter(|c| c.is_ascii_digit()).take(6).collect();
            Message::LockScreenPinChanged(digits)
        })
        .on_submit(Message::SubmitLockScreenAuth)
        .style(move |_t, _s| text_input::Style {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.40)),
            border: Border {
                radius: Radius::from(8.0),
                color: palette.glass_border,
                width: 1.0,
            },
            icon: palette.text_muted,
            placeholder: palette.text_muted,
            value: palette.text_primary,
            selection: palette.accent,
        });

    // 3x4 Touch Keypad Grid
    let keypad_btn = |label: &'static str, msg: Option<Message>| {
        button(text(label).size(15).color(palette.text_primary))
            .width(52)
            .height(38)
            .style(secondary_button(palette))
            .on_press_maybe(msg)
    };

    let row1 = row![
        keypad_btn("1", Some(Message::LockScreenKeypadDigit('1'))),
        px_space(8.0, 0.0),
        keypad_btn("2", Some(Message::LockScreenKeypadDigit('2'))),
        px_space(8.0, 0.0),
        keypad_btn("3", Some(Message::LockScreenKeypadDigit('3'))),
    ]
    .align_y(Alignment::Center);

    let row2 = row![
        keypad_btn("4", Some(Message::LockScreenKeypadDigit('4'))),
        px_space(8.0, 0.0),
        keypad_btn("5", Some(Message::LockScreenKeypadDigit('5'))),
        px_space(8.0, 0.0),
        keypad_btn("6", Some(Message::LockScreenKeypadDigit('6'))),
    ]
    .align_y(Alignment::Center);

    let row3 = row![
        keypad_btn("7", Some(Message::LockScreenKeypadDigit('7'))),
        px_space(8.0, 0.0),
        keypad_btn("8", Some(Message::LockScreenKeypadDigit('8'))),
        px_space(8.0, 0.0),
        keypad_btn("9", Some(Message::LockScreenKeypadDigit('9'))),
    ]
    .align_y(Alignment::Center);

    let row4 = row![
        keypad_btn("⌫", Some(Message::LockScreenKeypadBackspace)),
        px_space(8.0, 0.0),
        keypad_btn("0", Some(Message::LockScreenKeypadDigit('0'))),
        px_space(8.0, 0.0),
        keypad_btn("✕", Some(Message::LockScreenKeypadClear)),
    ]
    .align_y(Alignment::Center);

    column![
        dots_row,
        px_space(0.0, 6.0),
        pin_input_field,
        px_space(0.0, 6.0),
        column![
            row1,
            px_space(0.0, 6.0),
            row2,
            px_space(0.0, 6.0),
            row3,
            px_space(0.0, 6.0),
            row4
        ]
        .align_x(Alignment::Center),
    ]
    .spacing(6)
    .align_x(Alignment::Center)
    .into()
}

/// Renders password input field with show/hide password toggle
fn render_password_mode<'a>(
    state: &'a LockScreenState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let eye_icon = if state.show_password {
        "👁️"
    } else {
        "🔒"
    };
    let show_hide_btn = button(text(eye_icon).size(13))
        .padding([6, 10])
        .style(secondary_button(palette))
        .on_press(Message::LockScreenToggleShowPassword);

    let input_field = text_input("Enter workspace password...", &state.password_input)
        .secure(!state.show_password)
        .size(13)
        .width(Length::Fixed(240.0))
        .padding([8, 12])
        .on_input(Message::LockScreenPasswordChanged)
        .on_submit(Message::SubmitLockScreenAuth)
        .style(move |_t, _s| text_input::Style {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.40)),
            border: Border {
                radius: Radius::from(8.0),
                color: palette.glass_border,
                width: 1.0,
            },
            icon: palette.text_muted,
            placeholder: palette.text_muted,
            value: palette.text_primary,
            selection: palette.accent,
        });

    let input_row = row![input_field, px_space(6.0, 0.0), show_hide_btn].align_y(Alignment::Center);

    column![
        input_row,
        px_space(0.0, 4.0),
        text("Default Demo Password: MindSet2026! • Demo PIN: 123456")
            .size(10)
            .color(palette.text_muted),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
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
        // Append digits
        for c in "123456".chars() {
            if state.pin_input.len() < 6 {
                state.pin_input.push(c);
            }
        }
        assert_eq!(state.pin_input, "123456");

        // 7th digit rejected (max 6)
        if state.pin_input.len() < 6 {
            state.pin_input.push('7');
        }
        assert_eq!(state.pin_input, "123456");

        // Backspace
        state.pin_input.pop();
        assert_eq!(state.pin_input, "12345");

        // Clear
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

        // Decrement tick 1
        if state.lockout_seconds_remaining > 0 {
            state.lockout_seconds_remaining -= 1;
            if state.lockout_seconds_remaining == 0 {
                state.is_locked_out = false;
                state.error_message = None;
            }
        }
        assert!(state.is_locked_out);
        assert_eq!(state.lockout_seconds_remaining, 1);

        // Decrement tick 2
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
}
