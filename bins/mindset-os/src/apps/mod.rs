//! Applications Subsystem for MindSet OS
//!
//! Provides unified high-opacity window chrome ([−], [□], [×] guaranteed 1-click controls),
//! active window focus highlights, and modular application dispatchers.

pub mod ai_tutor;
pub mod browser;
pub mod design_system;
pub mod file_explorer;
pub mod preferences;
pub mod study_reader;
pub mod system_monitor;
pub mod tasks;
pub mod today;

use std::collections::HashMap;

use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{mouse, Alignment, Color, Element, Length};
use mindset_core::agent::mcq_generator::{StudentAnswer, TestEvaluationReport, TestPaper};
use mindset_core::{MindSetCoreState, WindowId, WindowInstance, WindowMode};
use mindset_intelligence::{
    AcademicSearchEngine, DailyFatigueCheckin, LeitnerCard, PdfCitationMatch, PdfDocumentChunk,
    SearchProviderType, StudentMistakeRecord,
};

use crate::ai::maindy_system_bridge::{MaindyOsAction, StudyCapsule};
use crate::ai::persona::ParsedQuery;
use crate::apps::ai_tutor::{MaindyDrawerTab, MaindyScreen, MaindyStudioProps, DOCK_CLEARANCE};
use crate::shell::odyssey_map::OdysseyMapState;
use crate::storage::db::{CapsuleRecord, ChatMessageRecord, ChatThread, UserMemory, UserProfile};
use crate::system::state::SystemState;
use crate::system::sysinfo_collector::SystemMetrics;
use crate::theme::{
    h_space, px_space, traffic_light_button, window_chrome_style, window_titlebar_style,
    ThemePalette,
};
use crate::Message;

#[allow(clippy::too_many_arguments)]
pub fn render_window<'a>(
    instance: &'a WindowInstance,
    core: &'a MindSetCoreState,
    system: &'a SystemState,
    odyssey_state: &'a OdysseyMapState,
    profile: &'a UserProfile,
    capsules: &'a [CapsuleRecord],
    palette: ThemePalette,
    chat_input: &'a str,
    new_task_input: &'a str,
    metrics: &'a SystemMetrics,
    threads: &'a [ChatThread],
    messages: &'a [ChatMessageRecord],
    active_thread_id: &'a str,
    is_generating: bool,
    active_model: &'a str,
    memories: &'a [UserMemory],
    new_memory_input: &'a str,
    auto_speak: bool,
    ai_activity_log: &'a [crate::storage::AiActivityRecord],
    _browser_url: &'a str,
    _browser_focus_shield: bool,
    browser_status_note: Option<&'a str>,
    network_status: &'a mindset_sys::network::NetworkStatus,
    rag_pdf_chunks: &'a [PdfDocumentChunk],
    rag_active_file: Option<&'a str>,
    rag_current_page: usize,
    rag_query_input: &'a str,
    rag_citations: &'a [PdfCitationMatch],
    academic_search: &'a AcademicSearchEngine,
    academic_search_input: &'a str,
    academic_search_provider: SearchProviderType,
    academic_search_loading: bool,
    academic_search_error: Option<&'a str>,
    today_checkin: Option<&'a DailyFatigueCheckin>,
    due_leitner_cards: &'a [LeitnerCard],
    unresolved_mistakes: &'a [StudentMistakeRecord],
    last_parsed_query: Option<&'a ParsedQuery>,
    active_mcq_paper: Option<&'a TestPaper>,
    mcq_answers: &'a HashMap<String, StudentAnswer>,
    mcq_report: Option<&'a TestEvaluationReport>,
    installed_models: &'a [String],
    maindy_screen: MaindyScreen,
    maindy_drawer_tab: MaindyDrawerTab,
    maindy_left_rail_open: bool,
    maindy_right_drawer_open: bool,
    maindy_engine_popover_open: bool,
    study_capsules: &'a [StudyCapsule],
    active_capsule_id: Option<&'a str>,
    active_capsule_phase: usize,
    capsule_hint_open: bool,
    pending_destructive_action: Option<&'a MaindyOsAction>,
) -> Element<'a, Message> {
    let id = instance.id;
    let is_focused = core.wm.active_window() == Some(id);

    // 1. Unified Native Desktop Window Titlebar & Traffic Lights
    let close_btn = button(
        container(text("×").size(9))
            .width(12)
            .height(12)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(Message::CloseWindow(id))
    .style(traffic_light_button(
        Color::from_rgb(1.0, 0.373, 0.337),
        Color::from_rgb(0.88, 0.27, 0.24),
        Color::from_rgba(0.35, 0.0, 0.0, 0.85),
        is_focused,
    ))
    .width(12)
    .height(12)
    .padding(0);

    let min_btn = button(
        container(text("−").size(9))
            .width(12)
            .height(12)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(Message::MinimizeWindow(id))
    .style(traffic_light_button(
        Color::from_rgb(1.0, 0.741, 0.180),
        Color::from_rgb(0.87, 0.63, 0.14),
        Color::from_rgba(0.40, 0.25, 0.0, 0.85),
        is_focused,
    ))
    .width(12)
    .height(12)
    .padding(0);

    let max_btn = button(
        container(
            text(if instance.mode == WindowMode::Maximized {
                "−"
            } else {
                "+"
            })
            .size(9),
        )
        .width(12)
        .height(12)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
    )
    .on_press(Message::MaximizeWindow(id))
    .style(traffic_light_button(
        Color::from_rgb(0.153, 0.788, 0.247),
        Color::from_rgb(0.10, 0.67, 0.16),
        Color::from_rgba(0.0, 0.35, 0.05, 0.85),
        is_focused,
    ))
    .width(12)
    .height(12)
    .padding(0);

    let traffic_lights = row![close_btn, min_btn, max_btn]
        .spacing(8)
        .align_y(Alignment::Center);

    let title_badge = row![
        text(&instance.icon).size(13),
        text(&instance.title)
            .size(12)
            .font(iced::font::Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            })
            .color(if is_focused {
                palette.text_primary
            } else {
                palette.text_secondary
            }),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let window_tag = container(
        text(if instance.mode == WindowMode::Maximized {
            "Standard Mode (❐)"
        } else {
            "Desktop Window"
        })
        .size(9)
        .color(palette.text_muted),
    )
    .padding([2, 6])
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(
            1.0, 1.0, 1.0, 0.04,
        ))),
        border: iced::Border {
            radius: iced::border::Radius::from(4.0),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            width: 0.5,
        },
        ..Default::default()
    });

    let header = container(
        row![
            traffic_lights,
            px_space(12.0, 0.0),
            title_badge,
            h_space(),
            window_tag,
        ]
        .padding([8, 14])
        .align_y(Alignment::Center),
    )
    .style(move |_t| window_titlebar_style(palette, is_focused));

    let draggable_header = mouse_area(header)
        .on_press(Message::StartWindowDrag(id))
        .on_double_click(Message::MaximizeWindow(id))
        .interaction(mouse::Interaction::Grab);

    // 2. Application Body Dispatcher
    let body: Element<'a, Message> = match id {
        WindowId::Today => today::view(core, profile, capsules, palette),
        WindowId::StudyReader => study_reader::view(core, palette),
        WindowId::OdysseyMap => {
            crate::shell::desktop::view_odyssey_window(core, profile, odyssey_state, palette)
        }
        WindowId::Scratchpad => crate::shell::desktop::view_scratchpad_window(system, palette),
        WindowId::Tasks => tasks::view(core, palette, new_task_input),
        WindowId::LayaAiChat => ai_tutor::view(MaindyStudioProps {
            core,
            profile,
            palette,
            chat_input,
            active_thread_id,
            threads,
            messages,
            is_generating,
            active_model,
            installed_models,
            memories,
            new_memory_input,
            auto_speak,
            rag_pdf_chunks,
            rag_active_file,
            rag_current_page,
            rag_query_input,
            rag_citations,
            academic_search,
            academic_search_input,
            academic_search_provider,
            academic_search_loading,
            academic_search_error,
            today_checkin,
            due_leitner_cards,
            unresolved_mistakes,
            last_parsed_query,
            active_mcq_paper,
            mcq_answers,
            mcq_report,
            screen: maindy_screen,
            drawer_tab: maindy_drawer_tab,
            left_rail_open: maindy_left_rail_open,
            right_drawer_open: maindy_right_drawer_open,
            engine_popover_open: maindy_engine_popover_open,
            study_capsules,
            active_capsule_id,
            active_capsule_phase,
            capsule_hint_open,
            pending_destructive_action,
        }),
        WindowId::Settings => preferences::view(core, profile, palette, ai_activity_log),
        WindowId::SystemMonitor => system_monitor::view(metrics, palette),
        WindowId::FileVault => file_explorer::view(palette),
        WindowId::SearchBrowser => browser::view(palette, browser_status_note, network_status),
        WindowId::DesignSystem => design_system::view(palette),
        _ => render_generic(instance, palette),
    };

    let window_layout = column![draggable_header, body].spacing(0);

    let mut win_container = container(window_layout)
        .clip(true)
        .style(move |_t| window_chrome_style(palette, is_focused));

    // 3. Dimensions & Mode — reserve bottom DOCK_CLEARANCE on maximized windows so Dock never covers input
    if instance.mode == WindowMode::Maximized {
        win_container = win_container.width(Length::Fill).height(Length::Fill);
        container(win_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(iced::Padding {
                top: 0.0,
                right: 0.0,
                bottom: DOCK_CLEARANCE,
                left: 0.0,
            })
            .into()
    } else {
        win_container = win_container.width(instance.width).height(instance.height);
        mouse_area(win_container)
            .on_press(Message::FocusWindow(id))
            .into()
    }
}

fn render_generic<'a>(instance: &'a WindowInstance, palette: ThemePalette) -> Element<'a, Message> {
    container(
        column![
            text(format!("{} is running.", instance.title))
                .size(13)
                .color(palette.text_primary),
            text("Native Liquidmorphism process container active.")
                .size(11)
                .color(palette.text_muted),
        ]
        .spacing(6)
        .padding(24),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
