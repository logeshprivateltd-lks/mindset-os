//! Task Quotas & Syllabus Tasks App for MindSet OS
//!
//! Provides daily subject quotas, 10m study increments, and syllabus tasks.

use iced::widget::{
    button, checkbox, column, container, progress_bar, row, scrollable, text, text_input,
};
use iced::{Alignment, Element, Length};
use mindset_core::MindSetCoreState;

use crate::theme::{
    glass_card, h_space, modern_text_input, primary_button, secondary_button, ThemePalette,
};
use crate::Message;

pub fn view<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    new_task_input: &'a str,
) -> Element<'a, Message> {
    // 1. Subject Quotas List
    let mut subject_list =
        column![text("SUBJECT STUDY QUOTAS").size(11).color(palette.accent)].spacing(8);

    for s in &core.tasks.subjects {
        let pct = (s.completed_minutes as f32 / s.target_minutes.max(1) as f32).min(1.0);
        let log_btn = button(text("+10m").size(10))
            .on_press(Message::Log10mTask(s.name.clone()))
            .style(secondary_button(palette))
            .padding([2, 7]);

        let item = column![
            row![
                text(format!("{} {}", s.icon, s.name))
                    .size(12)
                    .color(palette.text_primary),
                h_space(),
                text(format!("{}m / {}m", s.completed_minutes, s.target_minutes))
                    .size(11)
                    .color(palette.text_secondary),
                log_btn,
            ]
            .align_y(Alignment::Center),
            progress_bar(0.0..=1.0, pct),
        ]
        .spacing(4);

        subject_list = subject_list.push(item);
    }

    // 2. Syllabus Tasks List
    let mut task_list =
        column![text("SYLLABUS TASK QUEUE").size(11).color(palette.accent)].spacing(6);

    for t in &core.tasks.tasks {
        let task_id = t.id.clone();
        let task_id_for_log = t.id.clone();
        let chk = checkbox(t.is_completed).on_toggle(move |_| Message::ToggleTask(task_id.clone()));

        let log_btn = button(text("+10m").size(10))
            .on_press(Message::Log10mTask(task_id_for_log))
            .style(secondary_button(palette))
            .padding([2, 6]);

        let row_item = row![
            chk,
            text(&t.title).size(12).color(palette.text_primary),
            h_space(),
            text(format!("{}m", t.completed_minutes))
                .size(11)
                .color(palette.text_muted),
            log_btn,
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        task_list = task_list.push(row_item);
    }

    // 3. Add Goal Input
    let add_row = row![
        text_input("Enter new syllabus goal...", new_task_input)
            .on_input(Message::NewTaskInputChanged)
            .style(modern_text_input(palette))
            .padding(8)
            .width(Length::Fill),
        button(text("Add Goal").size(11))
            .on_press(Message::AddTask(
                new_task_input.to_string(),
                "Physics".into(),
                30,
            ))
            .style(primary_button(palette))
            .padding([8, 14]),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let content = column![
        container(subject_list)
            .padding(14)
            .style(move |_t| glass_card(palette)),
        container(task_list)
            .padding(14)
            .style(move |_t| glass_card(palette)),
        add_row,
    ]
    .spacing(14)
    .padding(16);

    scrollable(content).height(Length::Fill).into()
}
