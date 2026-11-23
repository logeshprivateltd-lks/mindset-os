//! Deep Focus Dashboard (Today View)
//!
//! Provides the primary student launchpad:
//! - Career-specific Daily Capsule with 4 dynamic stages from TrackBlueprint
//! - FIGHT BOSS (100 PTS) action trigger
//! - Dynamic Target Score from SQLite UserProfile (zero hardcoding)
//! - Clickable Today's Capsule Queue + Syllabus Mastery Progress Bar

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};
use mindset_core::engine::verification::CapsuleState;
use mindset_core::{MindSetCoreState, WindowId};

use crate::capsules::TrackBlueprint;
use crate::storage::db::{CapsuleRecord, UserProfile};
use crate::theme::{
    glass_card, glass_elevated, h_space, primary_button, secondary_button, ThemePalette,
};
use crate::Message;

pub fn view<'a>(
    core: &'a MindSetCoreState,
    profile: &'a UserProfile,
    capsules: &'a [CapsuleRecord],
    palette: ThemePalette,
) -> Element<'a, Message> {
    // 1. High-Contrast Header Pill: Greeting, Target & Streak Shield
    let streak_count = core.today.streak.count;
    let streak_str = format!("🔥 {} Day Streak", streak_count);

    let header_pill = container(
        row![
            column![
                text("TODAY'S DEEP FOCUS MISSION")
                    .size(10)
                    .color(palette.accent),
                text(&profile.target_label)
                    .size(17)
                    .color(palette.text_primary),
            ]
            .spacing(2),
            h_space(),
            container(
                row![
                    text(streak_str).size(11).color(palette.warning),
                    text("•").size(11).color(palette.text_muted),
                    text(format!(
                        "🛡️ {} Shield Held",
                        if streak_count >= 6 { 2 } else { 1 }
                    ))
                    .size(11)
                    .color(palette.success),
                    text("•").size(11).color(palette.text_muted),
                    text(format!("⏳ {}d Left", profile.days_remaining()))
                        .size(11)
                        .color(palette.accent),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding([4, 12])
            .style(move |_t| glass_card(palette)),
        ]
        .align_y(Alignment::Center)
        .padding([8, 14]),
    )
    .width(Length::Fill)
    .style(move |_t| glass_elevated(palette));

    // 2. Daily Capsule Card with 4 Stages & FIGHT BOSS
    let active_cap_record = capsules.first();
    let current_capsule = core
        .exam_plan
        .capsules
        .get(core.exam_plan.current_capsule_index);
    let capsule_state = current_capsule
        .map(|c| &c.state)
        .unwrap_or(&CapsuleState::Active);

    let mark_value = current_capsule.map(|c| c.mark_value).unwrap_or(5.0);

    let unit_name = active_cap_record
        .map(|c| c.unit.as_str())
        .or_else(|| current_capsule.map(|c| c.title.as_str()))
        .unwrap_or("Electrochemistry — Nernst Equation");

    let subject_name = active_cap_record
        .map(|c| c.subject.as_str())
        .or_else(|| current_capsule.map(|c| c.subject.as_str()))
        .unwrap_or("Physical Chemistry");

    let phase_label = current_capsule
        .map(|c| c.phase.label())
        .unwrap_or("Phase 1: High-Yield Core");

    let bp = TrackBlueprint::for_track(&profile.career_track);
    let total_mins = active_cap_record
        .map(|c| c.estimated_minutes)
        .unwrap_or_else(|| bp.total_minutes());

    let mark_badge = container(
        text(format!(
            "+{:.1} Marks ({})",
            mark_value, profile.career_track
        ))
        .size(10)
        .color(palette.success),
    )
    .padding([2, 8])
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(
            0.10, 0.72, 0.42, 0.15,
        ))),
        border: iced::Border {
            radius: iced::border::Radius::from(6.0),
            color: palette.success,
            width: 1.0,
        },
        ..Default::default()
    });

    let stages_row = row![
        stage_chip(bp.stage1_name, palette),
        stage_chip(bp.stage2_name, palette),
        stage_chip(bp.stage3_name, palette),
        stage_chip(bp.stage4_name, palette),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let action_controls: Element<'_, Message> = match capsule_state {
        CapsuleState::Completed {
            rank, final_score, ..
        } => container(
            row![
                text("✅").size(15),
                text(format!("VERIFIED: {:?} CLEAR ({}/100)", rank, final_score))
                    .size(12)
                    .color(palette.success),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding([8, 14])
        .style(move |_t| glass_card(palette))
        .into(),

        _ => row![
            button(
                row![
                    text("🎯").size(14),
                    text("START FOCUS").size(12).color(palette.accent_text),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .on_press(Message::StartFocusSession)
            .style(primary_button(palette))
            .padding([8, 16]),
            button(
                row![
                    text("⚔️").size(14),
                    text("FIGHT BOSS (100 PTS)")
                        .size(12)
                        .color(palette.text_primary),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .on_press(Message::OpenVerificationBoss)
            .style(secondary_button(palette))
            .padding([8, 16]),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into(),
    };

    let daily_capsule_card = container(
        column![
            row![
                column![
                    row![
                        text(format!("DAILY CAPSULE • {} MINS", total_mins))
                            .size(10)
                            .color(palette.accent),
                        mark_badge,
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                    text(unit_name).size(15).color(palette.text_primary),
                    text(subject_name).size(11).color(palette.text_secondary),
                ]
                .spacing(3),
                h_space(),
                action_controls,
            ]
            .align_y(Alignment::Center),
            stages_row,
        ]
        .spacing(12)
        .padding(16),
    )
    .width(Length::Fill)
    .style(move |_t| glass_elevated(palette));

    // 3. Three Metrics Cards: Predicted Score, Study Time, MindSet XP
    let predicted_score_card = container(
        column![
            text("PREDICTED EXAM SCORE")
                .size(10)
                .color(palette.text_muted),
            text(format!(
                "{:.0} / {}",
                core.exam_plan.predicted_exam_score, profile.target_score_max
            ))
            .size(18)
            .color(palette.warning),
            text(format!(
                "Goal: {}/{} ({})",
                profile.target_score_value, profile.target_score_max, phase_label
            ))
            .size(10)
            .color(palette.success),
        ]
        .spacing(3)
        .align_x(Alignment::Center),
    )
    .width(Length::FillPortion(1))
    .padding(14)
    .style(move |_t| glass_card(palette));

    let study_time_card = container(
        column![
            text("TODAY'S STUDY TIME")
                .size(10)
                .color(palette.text_muted),
            text(&core.today.rings[0].value_text)
                .size(18)
                .color(palette.text_primary),
            text(format!(
                "Goal: {}h ({})",
                profile.weekday_study_hours, profile.peak_energy_window
            ))
            .size(10)
            .color(palette.accent),
        ]
        .spacing(3)
        .align_x(Alignment::Center),
    )
    .width(Length::FillPortion(1))
    .padding(14)
    .style(move |_t| glass_card(palette));

    let xp_card = container(
        column![
            text("MINDSET XP").size(10).color(palette.text_muted),
            text(format!("{} XP", core.exam_plan.mindset_xp))
                .size(18)
                .color(palette.accent),
            text(format!("Coaching: {}", profile.coaching_style))
                .size(10)
                .color(palette.text_secondary),
        ]
        .spacing(3)
        .align_x(Alignment::Center),
    )
    .width(Length::FillPortion(1))
    .padding(14)
    .style(move |_t| glass_card(palette));

    let metrics_row = row![predicted_score_card, study_time_card, xp_card].spacing(12);

    // 4. Today's Clickable Capsule Queue & Progress Bar
    let mut capsule_queue_items = column![].spacing(6);

    for (idx, cap) in capsules.iter().enumerate() {
        let is_current = idx == 0;
        let status_color = match cap.status.as_str() {
            "Completed" => palette.success,
            "In Progress" => palette.accent,
            _ => palette.text_muted,
        };

        let item = container(
            row![
                text(if cap.status == "Completed" {
                    "✅"
                } else {
                    "📌"
                })
                .size(14),
                column![
                    text(&cap.unit).size(12).color(palette.text_primary),
                    text(format!(
                        "{} • {}m • {} XP • Stage: {}",
                        cap.subject, cap.estimated_minutes, cap.xp_reward, cap.stage
                    ))
                    .size(10)
                    .color(palette.text_secondary),
                ]
                .spacing(2)
                .width(Length::Fill),
                container(text(&cap.status).size(10).color(status_color))
                    .padding([2, 6])
                    .style(move |_t| glass_card(palette)),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .padding(10),
        )
        .width(Length::Fill)
        .style(move |_t| {
            if is_current {
                glass_elevated(palette)
            } else {
                glass_card(palette)
            }
        });

        capsule_queue_items = capsule_queue_items.push(item);
    }

    let queue_card = container(
        column![
            row![
                text("TODAY'S CAPSULE QUEUE").size(11).color(palette.accent),
                h_space(),
                text(format!("{} Capsules Scheduled", capsules.len()))
                    .size(11)
                    .color(palette.text_muted),
            ]
            .align_y(Alignment::Center),
            capsule_queue_items,
        ]
        .spacing(10)
        .padding(14),
    )
    .width(Length::Fill)
    .style(move |_t| glass_card(palette));

    // 5. Quick Actions Row
    let quick_actions = row![
        button(text("🗕 Show Desktop").size(11))
            .on_press(Message::ToggleDesktop)
            .style(secondary_button(palette))
            .padding([6, 14]),
        h_space(),
        button(text("📖 Study Reader").size(11))
            .on_press(Message::OpenWindow(WindowId::StudyReader))
            .style(secondary_button(palette))
            .padding([6, 14]),
        button(text("📋 Task Queue").size(11))
            .on_press(Message::OpenWindow(WindowId::Tasks))
            .style(secondary_button(palette))
            .padding([6, 14]),
        button(text("🧠 Maindy AI").size(11))
            .on_press(Message::OpenWindow(WindowId::LayaAiChat))
            .style(secondary_button(palette))
            .padding([6, 14]),
        button(text("⚙️ Profile & Situation").size(11))
            .on_press(Message::OpenWindow(WindowId::Settings))
            .style(secondary_button(palette))
            .padding([6, 14]),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let stack = column![
        header_pill,
        daily_capsule_card,
        metrics_row,
        queue_card,
        quick_actions,
    ]
    .spacing(14)
    .max_width(840);

    let scroll = scrollable(stack);

    container(scroll)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(iced::Padding {
            top: 14.0,
            right: 20.0,
            bottom: 14.0,
            left: 20.0,
        })
        .align_x(Alignment::Center)
        .into()
}

fn stage_chip<'a>(label: &'static str, palette: ThemePalette) -> Element<'a, Message> {
    container(text(label).size(10).color(palette.text_secondary))
        .padding([4, 8])
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                1.0, 1.0, 1.0, 0.12,
            ))),
            border: iced::Border {
                radius: iced::border::Radius::from(6.0),
                color: palette.glass_border,
                width: 1.0,
            },
            ..Default::default()
        })
        .into()
}
