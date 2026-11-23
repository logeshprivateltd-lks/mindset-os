//! Exam Intelligence Report View — Exam Scout Agent
//!
//! Visualizes high-stakes syllabus ROI analysis, quadrant breakdown (Goldmines, Heavyweights,
//! QuickWins, Traps), baseline gap math, and strategic scout recommendations.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};
use mindset_core::agent::exam_scout::{ExamProfile, ExamScoutAgent, RoiQuadrant};
use mindset_core::MindSetCoreState;

use crate::theme::{
    glass_card, glass_elevated, h_space, primary_button, secondary_button, ThemePalette,
    TYPE_BODY, TYPE_CAPTION, TYPE_METRIC, TYPE_SUBHEAD,
};
use crate::Message;

pub fn view<'a>(core: &'a MindSetCoreState, palette: ThemePalette) -> Element<'a, Message> {
    let profile = ExamProfile::all_builtin()
        .into_iter()
        .find(|p| p.id == core.exam_plan.exam_id)
        .unwrap_or_else(ExamProfile::tn_class_12_board);

    let report = ExamScoutAgent::generate_report(
        &profile,
        core.exam_plan.target_score,
        core.exam_plan.baseline_readiness,
    );

    // 1. Header
    let title_row = row![
        column![
            text("📊 EXAM SCOUT INTELLIGENCE REPORT")
                .size(TYPE_CAPTION)
                .color(palette.accent),
            text(report.exam_name.clone())
                .size(TYPE_METRIC)
                .color(palette.text_primary),
            text(format!(
                "Target: {} / {} Marks  •  Baseline: {} Marks  •  Gap: +{:.0} Marks Needed",
                report.target_score,
                report.total_marks,
                report.baseline_readiness,
                report.total_score_gap
            ))
            .size(TYPE_BODY)
            .color(palette.text_secondary),
        ]
        .spacing(4),
        h_space(),
        button(text("✕ Close").size(TYPE_BODY))
            .on_press(Message::CloseOverlays)
            .style(secondary_button(palette))
            .padding([6, 14]),
    ]
    .align_y(Alignment::Center);

    // 2. High-Yield Key Metrics Grid
    let study_days_card = container(
        column![
            text("REMAINING STUDY DAYS")
                .size(TYPE_CAPTION)
                .color(palette.text_muted),
            text(format!("{} Days", core.exam_plan.remaining_study_days))
                .size(TYPE_SUBHEAD)
                .color(palette.text_primary),
            text(format!(
                "+{} Buffer Days Reserved",
                core.exam_plan.buffer_days_remaining
            ))
            .size(TYPE_CAPTION)
            .color(palette.success),
        ]
        .spacing(2)
        .align_x(Alignment::Center),
    )
    .width(Length::FillPortion(1))
    .padding(12)
    .style(move |_t| glass_card(palette));

    let predicted_score_card = container(
        column![
            text("PREDICTED EXAM SCORE")
                .size(TYPE_CAPTION)
                .color(palette.text_muted),
            text(format!(
                "{:.1} / {}",
                core.exam_plan.predicted_exam_score, report.total_marks
            ))
            .size(TYPE_SUBHEAD)
            .color(palette.warning),
            text(format!("Target: {} (On Track)", report.target_score))
                .size(TYPE_CAPTION)
                .color(palette.success),
        ]
        .spacing(2)
        .align_x(Alignment::Center),
    )
    .width(Length::FillPortion(1))
    .padding(12)
    .style(move |_t| glass_card(palette));

    let xp_card = container(
        column![
            text("MINDSET XP").size(TYPE_CAPTION).color(palette.text_muted),
            text(format!("{} XP", core.exam_plan.mindset_xp))
                .size(TYPE_SUBHEAD)
                .color(palette.accent),
            text("Independent from Exam Marks")
                .size(TYPE_CAPTION)
                .color(palette.text_secondary),
        ]
        .spacing(2)
        .align_x(Alignment::Center),
    )
    .width(Length::FillPortion(1))
    .padding(12)
    .style(move |_t| glass_card(palette));

    let metrics_row = row![study_days_card, predicted_score_card, xp_card].spacing(10);

    // 3. Strategic Scout Insights
    let mut insights_col = column![text("🎯 AGENT STRATEGIC RECOMMENDATIONS")
        .size(TYPE_CAPTION)
        .color(palette.accent)]
    .spacing(6);

    for insight in &report.strategic_insights {
        insights_col = insights_col.push(
            row![
                text("•").size(TYPE_BODY).color(palette.accent),
                text(insight.clone()).size(TYPE_BODY).color(palette.text_primary),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        );
    }

    let insights_card = container(insights_col)
        .width(Length::Fill)
        .padding(14)
        .style(move |_t| glass_card(palette));

    // 4. ROI Quadrant Chapters Table
    let mut chapters_col = column![row![
        text("RANKED SYLLABUS CHAPTERS BY ROI")
            .size(TYPE_CAPTION)
            .color(palette.accent),
        h_space(),
        text("Sorted: Goldmines → Heavyweights → QuickWins → Traps")
            .size(TYPE_CAPTION)
            .color(palette.text_muted),
    ]
    .align_y(Alignment::Center)]
    .spacing(8);

    for ch in report.top_roi_chapters.iter().take(8) {
        let (badge_label, badge_color) = match ch.quadrant {
            RoiQuadrant::Goldmine => ("🌟 GOLDMINE", palette.warning),
            RoiQuadrant::Heavyweight => ("⚡ HEAVYWEIGHT", palette.accent),
            RoiQuadrant::QuickWin => ("🚀 QUICK WIN", palette.success),
            RoiQuadrant::Trap => ("⚠️ TRAP", palette.danger),
        };

        let badge = container(text(badge_label).size(TYPE_CAPTION).color(palette.background))
            .padding([3, 8])
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(badge_color)),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

        let chapter_row = container(
            row![
                column![
                    text(ch.chapter_name.clone())
                        .size(TYPE_BODY)
                        .color(palette.text_primary),
                    text(format!(
                        "{} • {:.1} Marks • {:.1} Study Hours",
                        ch.subject, ch.weightage_marks, ch.effort_hours
                    ))
                    .size(TYPE_CAPTION)
                    .color(palette.text_secondary),
                ]
                .spacing(2),
                h_space(),
                column![
                    badge,
                    text(format!("PYQ Freq: {:.0}%", ch.pyq_frequency_percentage))
                        .size(TYPE_CAPTION)
                        .color(palette.text_muted),
                ]
                .spacing(4)
                .align_x(Alignment::End),
            ]
            .align_y(Alignment::Center)
            .padding([8, 12]),
        )
        .width(Length::Fill)
        .style(move |_t| glass_card(palette));

        chapters_col = chapters_col.push(chapter_row);
    }

    let chapters_scroll = scrollable(chapters_col).height(Length::Fixed(220.0));

    let bottom_actions = row![
        text("Absorbs missed study days into reserved buffer automatically with zero guilt.")
            .size(TYPE_CAPTION)
            .color(palette.text_muted),
        h_space(),
        button(text("Start Today's Capsule").size(TYPE_BODY))
            .on_press(Message::OpenVerificationBoss)
            .style(primary_button(palette))
            .padding([8, 18]),
    ]
    .align_y(Alignment::Center);

    let modal_body = column![
        title_row,
        metrics_row,
        insights_card,
        chapters_scroll,
        bottom_actions,
    ]
    .spacing(14)
    .max_width(740);

    container(modal_body)
        .padding(20)
        .style(move |_t| glass_elevated(palette))
        .into()
}
