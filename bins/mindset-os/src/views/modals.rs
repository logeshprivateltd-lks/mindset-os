//! Modal & Overlay Views for MindSet OS
//!
//! Provides compile-time modal routing for Launcher (Ctrl+K), Control Center,
//! Mindful Guard distraction shield, Odyssey Gamified Map, and Quick Setup Wizard.

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};
use mindset_core::{ActiveOverlay, MindSetCoreState, NodeStatus, WizardStep};

use crate::theme::{
    danger_button, glass_card, glass_elevated, h_space, modal_backdrop, primary_button, px_space,
    secondary_button, styled_button, v_space, ButtonVariant, ThemePalette,
};
use crate::Message;

#[allow(clippy::too_many_arguments)]
pub fn view<'a>(
    overlay: ActiveOverlay,
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    search_query: &'a str,
    stage1_choices: &'a [Option<usize>],
    stage2_choices: &'a [Option<usize>],
    feynman_input: &'a str,
    excuse_selected_option: Option<usize>,
    pending_agent_action: &'a Option<mindset_intelligence::agent_tools::AgentAction>,
    network_status: &'a mindset_sys::network::NetworkStatus,
    nearby_networks: &'a [mindset_sys::network::WifiNetwork],
    wifi_menu_open: bool,
    wifi_connecting_ssid: Option<&'a str>,
    wifi_password_input: &'a str,
) -> Option<Element<'a, Message>> {
    if overlay == ActiveOverlay::None {
        return None;
    }

    // Anchored side widget for Notifications (avoids centered modal dialog)
    if overlay == ActiveOverlay::Notifications {
        return Some(render_anchored_notifications(core, palette));
    }

    let modal_content = match overlay {
        ActiveOverlay::None => return None,
        ActiveOverlay::Launcher => crate::shell::launcher::view(core, palette, search_query),
        ActiveOverlay::ControlCenter => crate::shell::control_center::view(
            core,
            palette,
            network_status,
            nearby_networks,
            wifi_menu_open,
            wifi_connecting_ssid,
            wifi_password_input,
        ),
        ActiveOverlay::LockScreen => crate::shell::lock_screen::view(palette),
        ActiveOverlay::WindowSwitcher => crate::shell::window_switcher::view(core, palette),
        ActiveOverlay::MindfulGuard => render_mindful_guard(core, palette),
        ActiveOverlay::OdysseyMap => render_odyssey_map(core, palette),
        ActiveOverlay::QuickSetupWizard => render_wizard(core, palette),
        ActiveOverlay::Notifications => unreachable!(),
        ActiveOverlay::PowerMenu => render_power_menu(palette),
        ActiveOverlay::ExamReport => crate::views::exam_report::view(core, palette),
        ActiveOverlay::VerificationBoss => crate::views::verification_boss::view(
            core,
            palette,
            stage1_choices,
            stage2_choices,
            feynman_input,
        ),
        ActiveOverlay::ExcuseRoast => {
            crate::views::excuse_roast::view(core, palette, excuse_selected_option)
        }
        ActiveOverlay::BreakArcade => crate::views::break_arcade::view(core, palette),
        ActiveOverlay::AgentPermission => {
            crate::views::permission_modal::view(pending_agent_action, palette)
        }
    };

    // Modal dialog centered over dim backdrop
    let backdrop = container(
        column![
            v_space(),
            row![h_space(), modal_content, h_space(),].align_y(Alignment::Center),
            v_space(),
        ]
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_t| modal_backdrop());

    Some(backdrop.into())
}

// ----------------------------------------------------------------------------
// 3. Mindful Guard Distraction Shield Modal
// ----------------------------------------------------------------------------
fn render_mindful_guard<'a>(
    _core: &'a MindSetCoreState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let dialog = container(
        column![
            row![
                text("🛡️").size(22),
                column![
                    text("Mindful Guard Interception")
                        .size(15)
                        .color(palette.danger),
                    text("Attention Protection Shield Active")
                        .size(11)
                        .color(palette.text_secondary),
                ]
                .spacing(2),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            text(
                "You are in a focused study block. Browsing entertaining feeds or social media disrupts neural flow state and sets back syllabus mastery.",
            )
            .size(13)
            .color(palette.text_primary),
            row![
                button(text("Stay Focused").size(13))
                    .on_press(Message::MindfulStayFocused)
                    .style(primary_button(palette))
                    .padding([10, 22]),
                button(text("🎬 Roast My Excuse (15s Check)").size(12))
                    .on_press(Message::TriggerExcuseRoast(
                        "I just need 5 minutes of Instagram for quick refreshment".to_string(),
                    ))
                    .style(secondary_button(palette))
                    .padding([10, 16]),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(16)
        .padding(24),
    )
    .width(460)
    .style(move |_t| glass_elevated(palette));

    dialog.into()
}

// ----------------------------------------------------------------------------
// 4. Odyssey Gamified Learning Map Modal
// ----------------------------------------------------------------------------
fn render_odyssey_map<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let xp_badge = container(
        row![
            text("⚡").size(14),
            text(format!(
                "Level {} • {} XP",
                core.odyssey.level, core.odyssey.total_xp
            ))
            .size(12)
            .color(palette.warning),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([4, 12])
    .style(move |_t| glass_card(palette));

    let mut nodes_column =
        column![text("SYLLABUS ROADMAP").size(11).color(palette.accent)].spacing(8);

    for node in &core.odyssey.nodes {
        let node_id = node.id.clone();
        let status_badge = match node.status {
            NodeStatus::Completed => text("✓ Done").size(11).color(palette.success),
            NodeStatus::InProgress => text("▶ In Progress").size(11).color(palette.warning),
            NodeStatus::Locked => text("🔒 Locked").size(11).color(palette.text_muted),
        };

        let can_embark = node.status != NodeStatus::Locked;
        let variant = if can_embark {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Secondary
        };
        let mut embark_btn = button(text("Embark").size(11))
            .style(styled_button(palette, variant))
            .padding([3, 10]);

        if can_embark {
            embark_btn = embark_btn.on_press(Message::EmbarkNode(node_id.clone()));
        }

        let card = container(
            row![
                column![
                    row![
                        text(&node.title).size(13).color(palette.text_primary),
                        h_space(),
                        status_badge,
                    ]
                    .align_y(Alignment::Center),
                    text(&node.description)
                        .size(11)
                        .color(palette.text_secondary),
                ]
                .spacing(4)
                .width(Length::Fill),
                px_space(12.0, 0.0),
                embark_btn,
            ]
            .align_y(Alignment::Center)
            .padding(10),
        )
        .style(move |_t| glass_card(palette));

        nodes_column = nodes_column.push(card);
    }

    let dialog = container(
        column![
            row![
                text("🗺️ Gamified Learning Odyssey")
                    .size(16)
                    .color(palette.text_primary),
                h_space(),
                xp_badge,
                px_space(8.0, 0.0),
                button(text("✕").size(12))
                    .on_press(Message::CloseOverlays)
                    .style(secondary_button(palette))
                    .padding([2, 8]),
            ]
            .align_y(Alignment::Center),
            scrollable(nodes_column).height(320),
            row![
                button(text("Complete Current Stage (+XP)").size(12))
                    .on_press(Message::CompleteStage(
                        core.odyssey.nodes[core
                            .odyssey
                            .current_stage_index
                            .min(core.odyssey.nodes.len() - 1)]
                        .id
                        .clone(),
                    ))
                    .style(primary_button(palette))
                    .padding([8, 16]),
                h_space(),
                text(format!(
                    "Overall Marks: {}%",
                    core.odyssey.calculate_marks_percentage()
                ))
                .size(12)
                .color(palette.text_secondary),
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(14)
        .padding(18),
    )
    .width(620)
    .style(move |_t| glass_elevated(palette));

    dialog.into()
}

// ----------------------------------------------------------------------------
// 5. Onboarding Wizard Modal
// ----------------------------------------------------------------------------
fn render_wizard<'a>(core: &'a MindSetCoreState, palette: ThemePalette) -> Element<'a, Message> {
    let step = core.onboarding.current_step;
    let step_name = match step {
        WizardStep::Language => "Step 1 of 5: Language & Region",
        WizardStep::Identity => "Step 2 of 5: Identity & Exam Target",
        WizardStep::SubjectGoals => "Step 3 of 5: Daily Subject Quotas",
        WizardStep::FocusMode => "Step 4 of 5: Focus Strictness",
        WizardStep::Ready => "Step 5 of 5: Ready to Embark",
    };

    let step_body: Element<'a, Message> = match step {
        WizardStep::Language => text("Select English (US) as primary instructional medium.")
            .size(13)
            .into(),
        WizardStep::Identity => text("Target: JEE Advanced / NEET Aspirant. Goal: Top 500.")
            .size(13)
            .into(),
        WizardStep::SubjectGoals => {
            text("Default allocation: Physics (3h), Chemistry (2.5h), Math (3h).")
                .size(13)
                .into()
        }
        WizardStep::FocusMode => {
            text("Strict Focus Mode: Mindful Guard actively protects during study sessions.")
                .size(13)
                .into()
        }
        WizardStep::Ready => {
            text("You are fully set. Your journey to academic mastery begins now.")
                .size(13)
                .into()
        }
    };

    let dialog = container(
        column![
            row![
                text("🚀 Welcome to MindSet OS")
                    .size(16)
                    .color(palette.accent),
                h_space(),
                button(text("✕").size(12))
                    .on_press(Message::WizardClose)
                    .style(secondary_button(palette))
                    .padding([2, 8]),
            ]
            .align_y(Alignment::Center),
            text(step_name).size(12).color(palette.text_secondary),
            step_body,
            row![
                button(text("Back").size(12))
                    .on_press(Message::WizardPrev)
                    .style(secondary_button(palette))
                    .padding([6, 14]),
                h_space(),
                button(
                    text(if step == WizardStep::Ready {
                        "Finish Setup"
                    } else {
                        "Next Step"
                    })
                    .size(12)
                )
                .on_press(Message::WizardNext)
                .style(primary_button(palette))
                .padding([6, 16]),
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(14)
        .padding(20),
    )
    .width(480)
    .style(move |_t| glass_elevated(palette));

    dialog.into()
}

// ----------------------------------------------------------------------------
// 6. Anchored Notifications Dropdown Drawer & History Widget
// ----------------------------------------------------------------------------
fn render_anchored_notifications<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let drawer_content = render_notifications(core, palette);

    // Click-outside dismissal areas triggering Message::CloseOverlays
    let click_dismiss = || {
        button(container(row![]).width(Length::Fill).height(Length::Fill))
            .on_press(Message::CloseOverlays)
            .style(|_t, _s| button::Style {
                background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.14))),
                ..Default::default()
            })
            .padding(0)
    };

    let top_dismiss = button(container(row![]).width(Length::Fill).height(46))
        .on_press(Message::CloseOverlays)
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding(0);

    let bottom_dismiss = button(container(row![]).width(Length::Fill).height(Length::Fill))
        .on_press(Message::CloseOverlays)
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding(0);

    let right_dismiss = button(container(row![]).width(16).height(Length::Fill))
        .on_press(Message::CloseOverlays)
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding(0);

    let side_column = column![
        top_dismiss,
        drawer_content,
        bottom_dismiss,
    ]
    .width(360);

    row![
        click_dismiss(),
        side_column,
        right_dismiss,
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn render_notifications<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let unread_count = core.toast_queue.toasts.len();

    // 1. Header with bell, count badge, and close button
    let count_badge = if unread_count > 0 {
        container(text(format!("{unread_count}")).size(10).color(Color::WHITE))
            .padding([1, 6])
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(palette.accent)),
                border: iced::Border {
                    radius: iced::border::Radius::from(10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
    } else {
        container(text("0").size(10).color(palette.text_muted))
            .padding([1, 6])
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.08))),
                border: iced::Border {
                    radius: iced::border::Radius::from(10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
    };

    let header = row![
        row![
            text("🔔").size(14),
            text("Notifications")
                .size(13)
                .color(palette.text_primary),
            count_badge,
        ]
        .spacing(6)
        .align_y(Alignment::Center),
        h_space(),
        button(text("✕").size(11))
            .on_press(Message::CloseOverlays)
            .style(secondary_button(palette))
            .padding([2, 6]),
    ]
    .align_y(Alignment::Center);

    let subtitle = text("SYSTEM SIGNALS & STUDY ALERTS")
        .size(10)
        .color(palette.accent);

    // 2. Notification List / Empty State
    let content: Element<'a, Message> = if core.toast_queue.toasts.is_empty() {
        container(
            column![
                text("✨").size(24),
                text("All Caught Up")
                    .size(13)
                    .color(palette.text_primary),
                text("No pending study tasks, focus alerts, or notifications.")
                    .size(11)
                    .color(palette.text_muted),
            ]
            .spacing(4)
            .align_x(Alignment::Center),
        )
        .padding([20, 16])
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    } else {
        let mut items = column![].spacing(8);
        for t in &core.toast_queue.toasts {
            let toast_id = t.id;
            let icon_badge = container(text(&t.icon).size(15))
                .padding(6)
                .style(move |_t| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.12))),
                    border: iced::Border {
                        radius: iced::border::Radius::from(8.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });

            let card = container(
                row![
                    icon_badge,
                    column![
                        row![
                            text(&t.title).size(12).color(palette.text_primary),
                            h_space(),
                            text("Active").size(9).color(palette.accent),
                        ]
                        .align_y(Alignment::Center),
                        text(&t.body).size(11).color(palette.text_secondary),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                    button(text("✕").size(10))
                        .on_press(Message::DismissToast(toast_id))
                        .style(secondary_button(palette))
                        .padding([2, 5]),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding(10)
            .style(move |_t| glass_card(palette));

            items = items.push(card);
        }
        scrollable(items).height(Length::Shrink).into()
    };

    // 3. Footer with DND toggle and Close action
    let dnd_label = if core.preferences.focus_dnd {
        "🌙 DND Active"
    } else {
        "🔔 Sound On"
    };
    let dnd_btn = button(text(dnd_label).size(11))
        .on_press(Message::ToggleDnd)
        .style(secondary_button(palette))
        .padding([4, 10]);

    let footer = row![
        dnd_btn,
        h_space(),
        button(text("Dismiss All").size(11))
            .on_press(Message::CloseOverlays)
            .style(secondary_button(palette))
            .padding([4, 10]),
    ]
    .align_y(Alignment::Center);

    // 4. Assemble Glassmorphic Drawer Container
    container(
        column![
            header,
            subtitle,
            content,
            footer,
        ]
        .spacing(10)
        .padding(14),
    )
    .width(360)
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(palette.glass_surface_elevated)),
        border: iced::Border {
            radius: iced::border::Radius::from(16.0),
            color: palette.glass_border,
            width: 1.0,
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
            offset: iced::Vector::new(0.0, 8.0),
            blur_radius: 22.0,
        },
        text_color: Some(palette.text_primary),
        ..Default::default()
    })
    .into()
}

fn render_power_menu<'a>(palette: ThemePalette) -> Element<'a, Message> {
    let dialog = container(
        column![
            row![
                text("⏻ Power & Session")
                    .size(14)
                    .color(palette.text_primary),
                h_space(),
                button(text("✕").size(11))
                    .on_press(Message::CloseOverlays)
                    .style(secondary_button(palette))
                    .padding([2, 6]),
            ]
            .align_y(Alignment::Center),
            button(
                row![
                    text("🔒").size(13),
                    text("Lock Screen (Super+L)")
                        .size(12)
                        .color(palette.text_primary),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::ToggleOverlay(ActiveOverlay::LockScreen))
            .style(secondary_button(palette))
            .padding([8, 14])
            .width(Length::Fill),
            button(
                row![
                    text("🌙").size(13),
                    text("Sleep & Deep Focus")
                        .size(12)
                        .color(palette.text_primary),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::ToggleDnd)
            .style(secondary_button(palette))
            .padding([8, 14])
            .width(Length::Fill),
            button(
                row![
                    text("🔄").size(13),
                    text("Restart Workspace")
                        .size(12)
                        .color(palette.text_primary),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::CloseOverlays)
            .style(secondary_button(palette))
            .padding([8, 14])
            .width(Length::Fill),
            button(
                row![
                    text("⏻").size(13),
                    text("Shutdown / Exit to Host").size(12).color(Color::WHITE),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::CloseOverlays)
            .style(danger_button(palette))
            .padding([8, 14])
            .width(Length::Fill),
        ]
        .spacing(10)
        .padding(18),
    )
    .width(300)
    .style(move |_t| glass_elevated(palette));

    dialog.into()
}
