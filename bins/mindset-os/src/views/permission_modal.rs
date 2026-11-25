//! MindSet AI Permission Sheet Modal
//!
//! A high-trust, security-first confirmation overlay displayed whenever Maindy AI
//! requests permission to inspect directories, read note documents, or mutate OS controls.
//! Enforces the critical rule: Maindy AI MUST NEVER read files or mutate settings silently.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use mindset_intelligence::agent_tools::{action_description, AgentAction};

use crate::ai::agent::{extract_folder_scope, AgentPermissionDecision};
use crate::theme::{
    danger_button, glass_card, glass_elevated, h_space, primary_button, secondary_button, v_space,
    ThemePalette, TYPE_BODY, TYPE_CAPTION,
};
use crate::Message;

pub fn view<'a>(
    pending_action: &'a Option<AgentAction>,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let action = match pending_action {
        Some(act) => act,
        None => {
            return container(column![])
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }
    };

    let (title, target_display) = action_description(action);
    let folder_scope = match action {
        AgentAction::ReadDocument { file_path } => extract_folder_scope(file_path),
        AgentAction::ListFolder { path } => extract_folder_scope(path),
        _ => "This Session".to_string(),
    };

    let header = row![
        text("🛡️").size(24),
        column![
            text("MAINdy AI PERMISSION REQUEST")
                .size(10)
                .color(palette.accent),
            text(title).size(16).color(palette.text_primary),
        ]
        .spacing(2),
        h_space(),
        container(text("SECURITY INTERLOCK").size(10).color(palette.accent))
            .padding([3, 8])
            .style(move |_t| glass_card(palette)),
    ]
    .align_y(Alignment::Center);

    let target_card = container(
        column![
            text("Target Resource").size(10).color(palette.text_muted),
            text(target_display).size(13).color(palette.accent_text),
        ]
        .spacing(4)
        .padding(12),
    )
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    let (action_icon, action_purpose) = match action {
        AgentAction::ReadDocument { .. } => ("📖", "Synthesize study notes and extract formulas"),
        AgentAction::ListFolder { .. } => (
            "📁",
            "Discover curriculum documents to recommend study capsules",
        ),
        AgentAction::ControlSystem { .. } => (
            "⚙️",
            "Optimize display theme and activate distraction shields",
        ),
        _ => ("🤖", "Execute local student workspace enhancement"),
    };

    let explanation_card = container(
        column![
            row![
                text(action_icon).size(13),
                text("Purpose:").size(TYPE_BODY).color(palette.accent),
                text(action_purpose)
                    .size(TYPE_BODY)
                    .color(palette.text_primary),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
            row![
                text("🔒").size(13),
                text("Privacy:").size(TYPE_BODY).color(palette.success),
                text("100% On-Device Ollama Inference • Zero Cloud Transmission")
                    .size(TYPE_CAPTION)
                    .color(palette.text_secondary),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        ]
        .spacing(8),
    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    let allow_session_label = if folder_scope == "This Session" {
        "Allow for This Session".to_string()
    } else {
        format!("Allow Folder '{}' (Session)", folder_scope)
    };

    let buttons = row![
        button(
            row![text("✕ Deny").size(TYPE_BODY).color(palette.danger)].align_y(Alignment::Center)
        )
        .on_press(Message::AgentPermissionResponse(
            AgentPermissionDecision::Deny
        ))
        .style(danger_button(palette))
        .padding([8, 16]),
        h_space(),
        button(
            text(allow_session_label)
                .size(TYPE_BODY)
                .color(palette.text_primary)
        )
        .on_press(Message::AgentPermissionResponse(
            AgentPermissionDecision::AllowSession
        ))
        .style(secondary_button(palette))
        .padding([8, 16]),
        button(
            row![
                text("✓").size(14),
                text("Allow Once")
                    .size(TYPE_BODY)
                    .color(palette.accent_text),
            ]
            .spacing(6)
            .align_y(Alignment::Center)
        )
        .on_press(Message::AgentPermissionResponse(
            AgentPermissionDecision::AllowOnce
        ))
        .style(primary_button(palette))
        .padding([8, 20]),
    ]
    .align_y(Alignment::Center);

    let modal_card = container(
        column![header, target_card, explanation_card, buttons,]
            .spacing(16)
            .max_width(580)
            .padding(22),
    )
    .style(move |_t| glass_elevated(palette));

    // Semi-transparent scrim backdrop centered modal
    container(column![v_space(), modal_card, v_space(),].align_x(Alignment::Center))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                0.02, 0.03, 0.05, 0.72,
            ))),
            ..Default::default()
        })
        .into()
}
