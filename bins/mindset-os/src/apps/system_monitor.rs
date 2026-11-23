//! System Resource Monitor App (Live sysinfo & Theme-Aware Meters)
//!
//! Features real per-core CPU %, real RAM usage, MindSet OS process RSS,
//! live Ollama process polling, and responsive two-column layouts.

use iced::widget::{column, container, progress_bar, row, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::system::sysinfo_collector::SystemMetrics;
use crate::theme::{glass_card, glass_elevated, h_space, ThemePalette};
use crate::Message;

pub fn view<'a>(metrics: &'a SystemMetrics, palette: ThemePalette) -> Element<'a, Message> {
    // 1. Overview Metric Cards
    let cpu_card = container(
        column![
            row![
                text("⚡ CPU Load").size(12).color(palette.accent),
                h_space(),
                text(format!("{:.1}%", metrics.global_cpu_usage))
                    .size(14)
                    .color(palette.text_primary),
            ]
            .align_y(Alignment::Center),
            progress_bar(0.0..=100.0, metrics.global_cpu_usage),
            text(format!("{} Cores Active", metrics.per_core_cpu.len()))
                .size(10)
                .color(palette.text_muted),
        ]
        .spacing(6)
        .padding(14),
    )
    .width(Length::FillPortion(1))
    .style(move |_t| glass_card(palette));

    let ram_card = container(
        column![
            row![
                text("🧠 Memory (RAM)").size(12).color(palette.accent),
                h_space(),
                text(format!("{:.1}%", metrics.ram_percentage))
                    .size(14)
                    .color(palette.text_primary),
            ]
            .align_y(Alignment::Center),
            progress_bar(0.0..=100.0, metrics.ram_percentage),
            text(format!(
                "{:.1} GB / {:.1} GB Used",
                metrics.ram_used_mb as f32 / 1024.0,
                metrics.ram_total_mb as f32 / 1024.0
            ))
            .size(10)
            .color(palette.text_muted),
        ]
        .spacing(6)
        .padding(14),
    )
    .width(Length::FillPortion(1))
    .style(move |_t| glass_card(palette));

    let msos_card = container(
        column![
            row![
                text("🪟 MindSet OS RSS").size(12).color(palette.success),
                h_space(),
                text(format!("{:.1} MB", metrics.msos_rss_mb))
                    .size(14)
                    .color(palette.success),
            ]
            .align_y(Alignment::Center),
            progress_bar(0.0..=150.0, metrics.msos_rss_mb),
            text("Budget: < 150 MB (100% Pure Rust)")
                .size(10)
                .color(palette.text_muted),
        ]
        .spacing(6)
        .padding(14),
    )
    .width(Length::FillPortion(1))
    .style(move |_t| glass_card(palette));

    let overview_row = row![cpu_card, ram_card, msos_card].spacing(12);

    // 2. Per-Core CPU Breakdown
    let mut core_bars = column![].spacing(6);
    for (i, &usage) in metrics.per_core_cpu.iter().enumerate() {
        let core_row = row![
            text(format!("Core {}", i))
                .size(10)
                .color(palette.text_secondary),
            progress_bar(0.0..=100.0, usage),
            text(format!("{:.0}%", usage))
                .size(10)
                .color(palette.text_primary),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        core_bars = core_bars.push(core_row);
    }

    let per_core_card = container(
        column![
            text("Per-Core Execution Pipeline")
                .size(12)
                .color(palette.text_primary),
            core_bars,
        ]
        .spacing(8)
        .padding(14),
    )
    .width(Length::FillPortion(1))
    .style(move |_t| glass_elevated(palette));

    // 3. Ollama & Neural Bridge Status Card
    let ollama_status_text = if metrics.ollama_detected {
        format!("Active • {:.1} MB Memory", metrics.ollama_rss_mb)
    } else {
        "Offline / Standby (Starts on query)".to_string()
    };

    let ollama_card = container(
        column![
            text("Local Neural Accelerator (Ollama Gemma)")
                .size(12)
                .color(palette.text_primary),
            text(ollama_status_text)
                .size(11)
                .color(if metrics.ollama_detected {
                    palette.success
                } else {
                    palette.text_muted
                }),
            text("Offline deterministic fallback is active for Proof-of-Study verification.")
                .size(10)
                .color(palette.text_secondary),
            text(format!(
                "System Uptime: {}h {}m",
                metrics.uptime_secs / 3600,
                (metrics.uptime_secs % 3600) / 60
            ))
            .size(10)
            .color(palette.text_muted),
        ]
        .spacing(6)
        .padding(14),
    )
    .width(Length::FillPortion(1))
    .style(move |_t| glass_elevated(palette));

    let details_row = row![per_core_card, ollama_card].spacing(12);

    let content_stack = column![overview_row, details_row].spacing(14).padding(16);

    scrollable(content_stack)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
