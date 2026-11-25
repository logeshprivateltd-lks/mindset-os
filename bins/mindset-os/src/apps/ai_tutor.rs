//! Maindy AI Tutor Studio & Capsule Theater (Screens 1 & 2)
//!
//! Maindy is a Goal-Achieving OS-Native AI Tutor built around Study Capsules:
//! - Screen 1 (`TutorStudio`): Calm 3-area layout (Collapsible 220px Left Rail,
//!   Fluid Center Tutor & Inline Capsule Stage, 320px Toggleable 4-Tab Companion Drawer).
//! - Screen 2 (`CapsuleTheater`): Distraction-free interactive Capsule Presentation View
//!   with 4-phase stepper, mathematical formulas, toggleable hints, and Focus Sprint controls.
//!
//! Visual Hierarchy & Hard Rules:
//! - Warm Claymorphism exclusively on Maindy avatar, header, message cards & inline capsule cards.
//! - Zero system/error/stack-trace bubbles ever rendered in the Tutor chat stream.
//! - Mandatory bottom safe-area (`DOCK_CLEARANCE = 88.0`) so floating Dock never covers input/content.

use std::collections::HashMap;

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length};
use mindset_core::agent::mcq_generator::{
    ExamTarget, QuestionType, StudentAnswer, TestEvaluationReport, TestPaper,
};
use mindset_core::{MindSetCoreState, WindowId, STUDY_TOPICS};
use mindset_intelligence::{
    AcademicSearchEngine, DailyFatigueCheckin, FatigueLevel, LeitnerCard, PdfCitationMatch,
    PdfDocumentChunk, SearchProviderType, StudentMistakeRecord,
};

use crate::ai::maindy_system_bridge::{MaindyOsAction, StudyCapsule};
use crate::ai::persona::ParsedQuery;
use crate::storage::db::{ChatMessageRecord, ChatThread, UserMemory, UserProfile};
use crate::theme::claymorphism::{clay_capsule_card, clay_orb, clay_tutor_card};
use crate::theme::neumorphism::{neu_card, neu_inset, NeuPalette};
use crate::theme::{
    danger_button, h_space, modern_text_input, primary_button, px_space, secondary_button,
    toggle_button, ThemePalette, TYPE_BODY, TYPE_CAPTION, TYPE_SUBHEAD,
};
use crate::Message;

/// Reserved bottom safe-area inset so the floating Dock never overlaps tutor inputs or controls.
pub const DOCK_CLEARANCE: f32 = 88.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaindyScreen {
    #[default]
    TutorStudio,
    CapsuleTheater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaindyDrawerTab {
    #[default]
    Capsules,
    VaultPdf,
    Research,
    Memory,
}

pub struct MaindyStudioProps<'a> {
    pub core: &'a MindSetCoreState,
    pub profile: &'a UserProfile,
    pub palette: ThemePalette,
    pub chat_input: &'a str,
    pub active_thread_id: &'a str,
    pub threads: &'a [ChatThread],
    pub messages: &'a [ChatMessageRecord],
    pub is_generating: bool,
    pub active_model: &'a str,
    pub installed_models: &'a [String],
    pub memories: &'a [UserMemory],
    pub new_memory_input: &'a str,
    pub auto_speak: bool,
    pub rag_pdf_chunks: &'a [PdfDocumentChunk],
    pub rag_active_file: Option<&'a str>,
    pub rag_current_page: usize,
    pub rag_query_input: &'a str,
    pub rag_citations: &'a [PdfCitationMatch],
    pub academic_search: &'a AcademicSearchEngine,
    pub academic_search_input: &'a str,
    pub academic_search_provider: SearchProviderType,
    pub academic_search_loading: bool,
    pub academic_search_error: Option<&'a str>,
    pub today_checkin: Option<&'a DailyFatigueCheckin>,
    pub due_leitner_cards: &'a [LeitnerCard],
    pub unresolved_mistakes: &'a [StudentMistakeRecord],
    pub last_parsed_query: Option<&'a ParsedQuery>,
    pub active_mcq_paper: Option<&'a TestPaper>,
    pub mcq_answers: &'a HashMap<String, StudentAnswer>,
    pub mcq_report: Option<&'a TestEvaluationReport>,
    pub screen: MaindyScreen,
    pub drawer_tab: MaindyDrawerTab,
    pub left_rail_open: bool,
    pub right_drawer_open: bool,
    pub engine_popover_open: bool,
    pub study_capsules: &'a [StudyCapsule],
    pub active_capsule_id: Option<&'a str>,
    pub active_capsule_phase: usize,
    pub capsule_hint_open: bool,
    pub pending_destructive_action: Option<&'a MaindyOsAction>,
}

pub fn view<'a>(p: MaindyStudioProps<'a>) -> Element<'a, Message> {
    let neu = NeuPalette::for_theme(p.palette.mode);

    let active_view = match p.screen {
        MaindyScreen::TutorStudio => render_tutor_studio(&p, &neu),
        MaindyScreen::CapsuleTheater => render_capsule_theater(&p, &neu),
    };

    // Reserve clean bottom safe clearance so the floating Dock never overlaps inputs
    container(active_view)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(iced::Padding {
            top: 8.0,
            right: 12.0,
            bottom: DOCK_CLEARANCE * 0.25, // Inner clearance inside floating window
            left: 12.0,
        })
        .into()
}

// ===========================================================================
// Screen 1: Maindy Tutor Studio (Clean 3-Area Adaptive Layout)
// ===========================================================================
fn render_tutor_studio<'a>(p: &MaindyStudioProps<'a>, neu: &NeuPalette) -> Element<'a, Message> {
    let palette = p.palette;

    // 1. Calm Header Bar (Avatar + Goal + Pacing + Collapsible Engine Popover + toggles)
    let (cap_mins, brk_mins, _energy) = if let Some(chk) = p.today_checkin {
        (
            chk.recommended_capsule_minutes,
            chk.recommended_break_minutes,
            chk.energy_level.as_str(),
        )
    } else {
        (p.core.pomodoro.duration_mins, 5, "Fresh")
    };

    let rail_toggle_btn = button(
        text(if p.left_rail_open { "◧" } else { "☰" })
            .size(13)
            .color(neu.text_primary),
    )
    .on_press(Message::MaindyToggleLeftRail)
    .style(secondary_button(palette))
    .padding([5, 8]);

    let avatar = clay_orb(text("🧠").size(15).color(Color::WHITE), 30.0);

    let title_block = row![
        avatar,
        column![
            text("Maindy AI Tutor")
                .size(TYPE_SUBHEAD)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(neu.text_primary),
            text(format!(
                "Goal: {}/{} • {}m Focus • {}m Break",
                p.profile.target_score_value, p.profile.target_score_max, cap_mins, brk_mins
            ))
            .size(TYPE_CAPTION)
            .color(neu.text_muted),
        ]
        .spacing(1),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let engine_btn = button(
        text(if p.engine_popover_open {
            "⚙ Engine ▴"
        } else {
            "⚙ Engine ▾"
        })
        .size(TYPE_CAPTION)
        .color(if p.engine_popover_open {
            palette.accent_text
        } else {
            neu.text_primary
        })
        .wrapping(text::Wrapping::None),
    )
    .on_press(Message::MaindyToggleEnginePopover)
    .style(toggle_button(palette, p.engine_popover_open))
    .padding([5, 10]);

    let theater_btn: Element<'a, Message> = if let Some(cap) = p.study_capsules.first() {
        let cap_id = cap.id.clone();
        button(
            text("🎭 Capsule Theater")
                .size(TYPE_CAPTION)
                .color(palette.accent_text)
                .wrapping(text::Wrapping::None),
        )
        .on_press(Message::OpenStudyCapsule(cap_id))
        .style(primary_button(palette))
        .padding([5, 10])
        .into()
    } else {
        button(
            text("⚡ Generate Capsules")
                .size(TYPE_CAPTION)
                .color(palette.accent_text)
                .wrapping(text::Wrapping::None),
        )
        .on_press(Message::GenerateDailyCapsules)
        .style(primary_button(palette))
        .padding([5, 10])
        .into()
    };

    let drawer_btn = button(
        text(if p.right_drawer_open {
            "🗂 Drawer ▸"
        } else {
            "◂ Drawer"
        })
        .size(TYPE_CAPTION)
        .color(neu.text_primary)
        .wrapping(text::Wrapping::None),
    )
    .on_press(Message::MaindyToggleRightDrawer)
    .style(secondary_button(palette))
    .padding([5, 10]);

    let calm_header = container(neu_card(
        row![
            rail_toggle_btn,
            title_block,
            h_space(),
            engine_btn,
            theater_btn,
            drawer_btn,
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        neu,
        14.0,
        1.0,
    ))
    .width(Length::Fill);

    // 2. Collapsible Left Rail (220px): Sessions, Goal Pill, My Capsules, Syllabus Units
    let left_rail: Option<Element<'a, Message>> = if p.left_rail_open {
        Some(render_left_rail(p, palette, neu))
    } else {
        None
    };

    // 3. Center Tutor & Inline Capsule Stage (Fluid Primary Area)
    let center_stage = render_center_stage(p, palette, neu);

    // 4. Collapsible Right Contextual Companion Drawer (320px, 4 Tabs in ONE Drawer)
    let right_drawer: Option<Element<'a, Message>> = if p.right_drawer_open {
        Some(render_right_companion_drawer(p, palette, neu))
    } else {
        None
    };

    let mut areas = row![].spacing(10).height(Length::Fill).width(Length::Fill);
    if let Some(l) = left_rail {
        areas = areas.push(l);
    }
    areas = areas.push(center_stage);
    if let Some(r) = right_drawer {
        areas = areas.push(r);
    }

    column![calm_header, areas]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn render_left_rail<'a>(
    p: &MaindyStudioProps<'a>,
    palette: ThemePalette,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let new_btn = button(
        row![
            text("➕").size(11),
            text("New Study Session")
                .size(TYPE_CAPTION)
                .color(palette.accent_text)
                .wrapping(text::Wrapping::None),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .on_press(Message::NewChatThread)
    .style(primary_button(palette))
    .padding([7, 10])
    .width(Length::Fill);

    // Target Goal Progress Pill
    let goal_pill = neu_inset(
        column![
            text("TARGET GOAL")
                .size(9)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(neu.accent_primary),
            text(format!(
                "{} / {}",
                p.profile.target_score_value, p.profile.target_score_max
            ))
            .size(TYPE_SUBHEAD)
            .font(iced::font::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(neu.text_primary),
            text(p.profile.target_exam_display())
                .size(9)
                .color(neu.text_muted),
        ]
        .spacing(2),
        neu,
        10.0,
    );

    let mut rail_col = column![new_btn, goal_pill].spacing(8);

    // My Capsules list (click opens interactive Capsule Theater)
    rail_col = rail_col.push(
        row![
            text("MY CAPSULES")
                .size(9)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(neu.text_muted),
            h_space(),
            button(text("+").size(10).color(neu.accent_primary))
                .on_press(Message::GenerateDailyCapsules)
                .style(secondary_button(palette))
                .padding([1, 5]),
        ]
        .align_y(Alignment::Center),
    );

    if p.study_capsules.is_empty() {
        rail_col = rail_col.push(
            text("No custom capsules yet.\nType 'Create a capsule for Gauss's Law'")
                .size(9)
                .color(neu.text_muted),
        );
    } else {
        for cap in p.study_capsules.iter().take(4) {
            let cid = cap.id.clone();
            let is_sel = p.active_capsule_id == Some(cap.id.as_str());
            let txt_col = if is_sel {
                palette.accent_text
            } else {
                neu.text_primary
            };
            rail_col = rail_col.push(
                button(
                    column![
                        text(&cap.title).size(10).color(txt_col),
                        text(format!("{} • {}m", cap.subject, cap.duration_mins))
                            .size(8)
                            .color(if is_sel {
                                palette.accent_text
                            } else {
                                neu.text_muted
                            }),
                    ]
                    .spacing(1),
                )
                .on_press(Message::OpenStudyCapsule(cid))
                .style(toggle_button(palette, is_sel))
                .padding([5, 8])
                .width(Length::Fill),
            );
        }
    }

    // Sessions (threads) — Crisp white text on active accent fill
    rail_col = rail_col.push(px_space(0.0, 4.0)).push(
        text("STUDY SESSIONS")
            .size(9)
            .font(iced::font::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(neu.text_muted),
    );

    for t in p.threads.iter().take(4) {
        let is_active = t.id == p.active_thread_id;
        let tid = t.id.clone();
        let btn = button(
            column![
                text(&t.title).size(10).color(if is_active {
                    palette.accent_text
                } else {
                    neu.text_primary
                }),
                text(&t.created_at[..10.min(t.created_at.len())])
                    .size(8)
                    .color(if is_active {
                        palette.accent_text
                    } else {
                        neu.text_muted
                    }),
            ]
            .spacing(1),
        )
        .on_press(Message::SelectChatThread(tid))
        .style(toggle_button(palette, is_active))
        .padding([5, 8])
        .width(Length::Fill);

        rail_col = rail_col.push(btn);
    }

    // Syllabus Units — Fixed unreadable selected-item contrast (palette.accent_text on accent fill!)
    rail_col = rail_col.push(px_space(0.0, 4.0)).push(
        text("SYLLABUS UNITS")
            .size(9)
            .font(iced::font::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(neu.text_muted),
    );

    for (idx, top) in STUDY_TOPICS.iter().enumerate() {
        let is_selected = idx == p.core.study_reader.active_tab_index;
        let t_btn = button(text(top.title).size(10).color(if is_selected {
            palette.accent_text
        } else {
            neu.text_primary
        }))
        .on_press(Message::SelectStudyTab(idx))
        .style(toggle_button(palette, is_selected))
        .padding([5, 8])
        .width(Length::Fill);
        rail_col = rail_col.push(t_btn);
    }

    container(neu_card(
        scrollable(rail_col).height(Length::Fill),
        neu,
        14.0,
        1.0,
    ))
    .width(Length::Fixed(220.0))
    .height(Length::Fill)
    .clip(true)
    .into()
}

fn render_center_stage<'a>(
    p: &MaindyStudioProps<'a>,
    palette: ThemePalette,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let mut stage = column![]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill);

    // Collapsible Tutor Engine Popover (Developer controls tucked cleanly out of main bar)
    if p.engine_popover_open {
        let model_name = if p.active_model.is_empty() {
            "gemma2:2b"
        } else {
            p.active_model
        };
        let mut model_chips = row![].spacing(4).align_y(Alignment::Center);
        for m in p.installed_models.iter().take(3) {
            let is_cur = m == p.active_model;
            model_chips = model_chips.push(
                button(
                    text(m)
                        .size(9)
                        .color(if is_cur {
                            palette.accent_text
                        } else {
                            neu.text_primary
                        })
                        .wrapping(text::Wrapping::None),
                )
                .on_press(Message::SelectModel(m.clone()))
                .style(toggle_button(palette, is_cur))
                .padding([3, 7]),
            );
        }

        let engine_panel = neu_inset(
            column![
                row![
                    text(format!("⚡ Local Tutor Engine: {model_name}"))
                        .size(10)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(neu.text_primary),
                    h_space(),
                    button(
                        text(if p.auto_speak {
                            "🔊 Auto-Speak: ON"
                        } else {
                            "🔇 Auto-Speak: OFF"
                        })
                        .size(9)
                        .color(neu.text_primary),
                    )
                    .on_press(Message::ToggleAutoSpeak)
                    .style(secondary_button(palette))
                    .padding([3, 8]),
                    button(text("🧹 Unload Model").size(9).color(neu.text_primary))
                        .on_press(Message::UnloadOllamaModel)
                        .style(secondary_button(palette))
                        .padding([3, 8]),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
                row![
                    text("Models:").size(9).color(neu.text_muted),
                    model_chips,
                    h_space(),
                    button(
                        text("⚡ Generate Daily Plan")
                            .size(9)
                            .color(palette.accent_text),
                    )
                    .on_press(Message::GenerateDailyCapsules)
                    .style(primary_button(palette))
                    .padding([3, 8]),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            ]
            .spacing(6),
            neu,
            10.0,
        );
        stage = stage.push(engine_panel);
    }

    // Thanglish / Subject Parser Tag
    if let Some(pq) = p.last_parsed_query {
        let mut parts = Vec::new();
        if let Some(sub) = &pq.subject {
            parts.push(sub.display_name().to_string());
        }
        if let Some(ch) = pq.chapter {
            parts.push(format!("Ch {ch}"));
        }
        if let Some(sec) = &pq.section {
            parts.push(format!("Sec {sec}"));
        }
        parts.push(pq.intent.code().to_string());
        stage = stage.push(
            container(
                text(format!(
                    "🗣️ Maindy NLP: {} ({}%)",
                    parts.join(" • "),
                    pq.confidence
                ))
                .size(9)
                .color(neu.accent_primary),
            )
            .padding([3, 8]),
        );
    }

    // Single Inline Confirmation Card ONLY for destructive actions (Delete / PowerOff)
    if let Some(destructive) = p.pending_destructive_action {
        let confirm_card = clay_capsule_card(
            row![
                column![
                    text("Confirm Destructive System Action")
                        .size(11)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(Color::from_rgb(0.45, 0.14, 0.04)),
                    text(destructive.human_label())
                        .size(10)
                        .color(Color::from_rgb(0.18, 0.14, 0.10)),
                ]
                .spacing(2)
                .width(Length::Fill),
                button(text("✓ Confirm").size(10).color(Color::WHITE))
                    .on_press(Message::ConfirmDestructiveOsAction)
                    .style(danger_button(palette))
                    .padding([5, 10]),
                button(text("Cancel").size(10).color(neu.text_primary))
                    .on_press(Message::CancelDestructiveOsAction)
                    .style(secondary_button(palette))
                    .padding([5, 10]),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            14.0,
        );
        stage = stage.push(confirm_card);
    }

    // Socratic Stream + Embedded Inline Live Study Capsule Cards (Zero errors ever rendered)
    let mut stream_col = column![].spacing(10).width(Length::Fill);

    if p.messages.is_empty() && p.study_capsules.is_empty() {
        stream_col = stream_col.push(clay_tutor_card(
            column![
                text("Welcome to Maindy AI Tutor Studio")
                    .size(13)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(Color::from_rgb(0.09, 0.11, 0.22)),
                text("I am your Goal-Achieving Tutor & OS Assistant. Tell me what topic you want to master or command the OS:")
                    .size(11)
                    .color(Color::from_rgb(0.20, 0.23, 0.36)),
                text("• 'Create a capsule for Gauss's Law'   • 'Open Settings'   • 'Open Files'")
                    .size(10)
                    .color(Color::from_rgb(0.24, 0.20, 0.62)),
            ]
            .spacing(6),
            16.0,
        ));
    }

    for m in p.messages.iter().take(24) {
        if m.content.trim().is_empty() {
            continue;
        }
        let is_user = m.role == "user";
        if is_user {
            let user_bubble = container(
                column![
                    row![
                        text("You")
                            .size(9)
                            .font(iced::font::Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            })
                            .color(neu.accent_primary),
                        h_space(),
                    ],
                    text(&m.content).size(TYPE_BODY).color(neu.text_primary),
                ]
                .spacing(3),
            )
            .padding([10, 14])
            .style(move |_t| container::Style {
                background: Some(iced::Background::Color(Color {
                    a: 0.12,
                    ..palette.accent
                })),
                border: iced::Border {
                    radius: iced::border::Radius::from(14.0),
                    color: Color {
                        a: 0.30,
                        ..palette.accent
                    },
                    width: 1.0,
                },
                ..Default::default()
            });
            stream_col = stream_col.push(user_bubble);
        } else {
            let speak_btn = button(
                text("🔊 Speak")
                    .size(9)
                    .color(Color::from_rgb(0.24, 0.20, 0.62)),
            )
            .on_press(Message::SpeakMessage(m.content.clone()))
            .style(move |_t, s| button::Style {
                background: Some(iced::Background::Color(if s == button::Status::Hovered {
                    Color::from_rgba(0.28, 0.22, 0.72, 0.14)
                } else {
                    Color::from_rgba(0.28, 0.22, 0.72, 0.08)
                })),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .padding([2, 7]);

            let tutor_bubble = clay_tutor_card(
                column![
                    row![
                        text("🧠 Maindy")
                            .size(10)
                            .font(iced::font::Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            })
                            .color(Color::from_rgb(0.24, 0.18, 0.62)),
                        h_space(),
                        speak_btn,
                    ]
                    .align_y(Alignment::Center),
                    text(&m.content)
                        .size(TYPE_BODY)
                        .color(Color::from_rgb(0.09, 0.11, 0.22)),
                ]
                .spacing(4),
                16.0,
            );
            stream_col = stream_col.push(tutor_bubble);
        }
    }

    // Inline Live Claymorphic Study Capsule Cards right inside the Tutor stream
    for cap in p.study_capsules.iter().take(2) {
        let cap_id_open = cap.id.clone();
        let cap_id_sprint = cap.id.clone();
        let formula_preview = cap
            .core_formulas
            .first()
            .map(String::as_str)
            .unwrap_or("Step-by-step concept mastery");

        let inline_card = clay_capsule_card(
            column![
                row![
                    container(
                        text(format!("{} • {}m", cap.subject, cap.duration_mins))
                            .size(9)
                            .color(Color::WHITE),
                    )
                    .padding([2, 7])
                    .style(move |_t| container::Style {
                        background: Some(iced::Background::Color(Color::from_rgb(
                            0.85, 0.45, 0.06
                        ))),
                        border: iced::Border {
                            radius: iced::border::Radius::from(6.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    px_space(6.0, 0.0),
                    text(&cap.target_marks_impact)
                        .size(9)
                        .color(Color::from_rgb(0.55, 0.26, 0.04)),
                    h_space(),
                    button(
                        text("🎭 Present Capsule →")
                            .size(9)
                            .color(palette.accent_text)
                            .wrapping(text::Wrapping::None),
                    )
                    .on_press(Message::OpenStudyCapsule(cap_id_open))
                    .style(primary_button(palette))
                    .padding([4, 10]),
                ]
                .align_y(Alignment::Center),
                text(&cap.title)
                    .size(13)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(Color::from_rgb(0.11, 0.10, 0.14)),
                text(format!("Core Theorem: {formula_preview}"))
                    .size(10)
                    .font(iced::Font::MONOSPACE)
                    .color(Color::from_rgb(0.18, 0.22, 0.34)),
                row![
                    button(
                        text(format!("⚡ Start {}m Focus Sprint", cap.duration_mins))
                            .size(9)
                            .color(Color::from_rgb(0.12, 0.14, 0.24)),
                    )
                    .on_press(Message::StartCapsuleFocusSprint(
                        cap_id_sprint,
                        cap.duration_mins
                    ))
                    .style(secondary_button(palette))
                    .padding([3, 8]),
                    text(format!("{} interactive phases ready", cap.phases.len()))
                        .size(9)
                        .color(Color::from_rgb(0.32, 0.28, 0.22)),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(6),
            16.0,
        );
        stream_col = stream_col.push(inline_card);
    }

    let scroll_stream = scrollable(stream_col).height(Length::Fill);
    stage = stage.push(scroll_stream);

    // Transient animated typing indicator BELOW the stream (never persisted in chat)
    if p.is_generating {
        stage = stage.push(
            container(
                row![
                    text("🧠 Maindy is synthesizing your study step...")
                        .size(TYPE_CAPTION)
                        .color(neu.accent_primary),
                    h_space(),
                    button(text("⏹ Stop").size(9).color(Color::WHITE))
                        .on_press(Message::CancelAiStream)
                        .style(danger_button(palette))
                        .padding([3, 8]),
                ]
                .align_y(Alignment::Center),
            )
            .padding([4, 10]),
        );
    }

    // Quick-Action Command & Tutor Chips
    let chip = |label: &'a str, val: &'a str| {
        button(
            text(label)
                .size(9)
                .color(neu.text_primary)
                .wrapping(text::Wrapping::None),
        )
        .on_press(Message::ChatInputChanged(val.to_string()))
        .style(secondary_button(palette))
        .padding([4, 8])
    };
    let chips_row = row![
        chip(
            "⚡ Create Gauss's Law Capsule",
            "Create a capsule for Gauss's Law"
        ),
        chip("⚙️ Open Settings", "open settings"),
        chip("📁 Open Files", "open files"),
        chip("🧪 Nernst Proof", "Explain Nernst equation step-by-step"),
    ]
    .spacing(6);

    // Clean Pill Command & Tutor Input Bar
    let input_box = text_input(
        "Ask a study question or run an OS command (e.g. 'Create a capsule for Gauss's Law', 'Open Settings')...",
        p.chat_input,
    )
    .on_input(Message::ChatInputChanged)
    .on_submit(Message::SendChatMessage)
    .style(modern_text_input(palette))
    .padding(8)
    .width(Length::Fill);

    let send_btn = button(
        text(if p.is_generating {
            "Thinking..."
        } else {
            "Send →"
        })
        .size(11)
        .color(palette.accent_text)
        .wrapping(text::Wrapping::None),
    )
    .on_press(Message::SendChatMessage)
    .style(primary_button(palette))
    .padding([8, 14]);

    let command_bar = row![input_box, send_btn]
        .spacing(8)
        .align_y(Alignment::Center);

    stage = stage.push(chips_row).push(command_bar);

    container(neu_card(stage, neu, 16.0, 1.0))
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(true)
        .into()
}

// ===========================================================================
// Area 3: Single Toggleable 4-Tab Right Companion Drawer (320px)
// ===========================================================================
fn render_right_companion_drawer<'a>(
    p: &MaindyStudioProps<'a>,
    palette: ThemePalette,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let tab_btn = |tab: MaindyDrawerTab, icon: &'a str, label: &'a str| {
        let is_sel = p.drawer_tab == tab;
        button(
            row![
                text(icon).size(10),
                text(label).size(9).color(if is_sel {
                    palette.accent_text
                } else {
                    neu.text_primary
                }),
            ]
            .spacing(3)
            .align_y(Alignment::Center),
        )
        .on_press(Message::MaindySelectDrawerTab(tab))
        .style(toggle_button(palette, is_sel))
        .padding([4, 6])
        .width(Length::FillPortion(1))
    };

    let tab_bar = row![
        tab_btn(MaindyDrawerTab::Capsules, "⚡", "Capsules"),
        tab_btn(MaindyDrawerTab::VaultPdf, "📖", "Vault"),
        tab_btn(MaindyDrawerTab::Research, "🔍", "Research"),
        tab_btn(MaindyDrawerTab::Memory, "🧠", "Memory"),
    ]
    .spacing(4)
    .width(Length::Fill);

    let tab_body: Element<'a, Message> = match p.drawer_tab {
        MaindyDrawerTab::Capsules => render_zone1_capsule_queue(
            p.core,
            palette,
            p.study_capsules,
            p.active_mcq_paper,
            p.mcq_answers,
            p.mcq_report,
        ),
        MaindyDrawerTab::VaultPdf => render_zone3_pdf_vault_reader(
            p.core,
            palette,
            p.rag_pdf_chunks,
            p.rag_active_file,
            p.rag_current_page,
            p.rag_query_input,
            p.rag_citations,
        ),
        MaindyDrawerTab::Research => render_zone4_academic_search(
            palette,
            p.academic_search,
            p.academic_search_input,
            p.academic_search_provider,
            p.academic_search_loading,
            p.academic_search_error,
        ),
        MaindyDrawerTab::Memory => render_personalization_drawer(
            palette,
            neu,
            p.memories,
            p.new_memory_input,
            p.today_checkin,
            p.due_leitner_cards,
            p.unresolved_mistakes,
        ),
    };

    container(neu_card(
        column![tab_bar, tab_body]
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Fill),
        neu,
        14.0,
        1.0,
    ))
    .width(Length::Fixed(320.0))
    .height(Length::Fill)
    .clip(true)
    .into()
}

// ===========================================================================
// Screen 2: Capsule Creation & Interactive Presentation Screen ("Capsule Theater")
// ===========================================================================
fn render_capsule_theater<'a>(p: &MaindyStudioProps<'a>, neu: &NeuPalette) -> Element<'a, Message> {
    let palette = p.palette;

    // Resolve active structured capsule or synthesize one on the fly from active syllabus topic
    let fallback_topic = p.core.study_reader.current_topic();
    let owned_fallback;
    let capsule: &StudyCapsule = if let Some(cid) = p.active_capsule_id {
        if let Some(found) = p.study_capsules.iter().find(|c| c.id == cid) {
            found
        } else if let Some(first) = p.study_capsules.first() {
            first
        } else {
            owned_fallback = StudyCapsule {
                id: "syllabus-preview".to_string(),
                title: fallback_topic.title.to_string(),
                subject: fallback_topic.subject.to_string(),
                target_marks_impact: "+5 Board Marks".to_string(),
                duration_mins: 25,
                phases: crate::ai::maindy_system_bridge::MaindySystemBridge::build_capsule(
                    fallback_topic.title,
                    Some(fallback_topic.subject),
                    25,
                )
                .phases,
                core_formulas: vec![fallback_topic.formula.to_string()],
                step_by_step_notes: fallback_topic
                    .derivation_steps
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty())
                    .collect(),
                created_at: String::new(),
            };
            &owned_fallback
        }
    } else if let Some(first) = p.study_capsules.first() {
        first
    } else {
        owned_fallback = crate::ai::maindy_system_bridge::MaindySystemBridge::build_capsule(
            fallback_topic.title,
            Some(fallback_topic.subject),
            25,
        );
        &owned_fallback
    };

    let active_phase_idx = p
        .active_capsule_phase
        .min(capsule.phases.len().saturating_sub(1));
    let active_phase = &capsule.phases[active_phase_idx];

    // 1. Header: Back to Tutor + Subject Badge + Marks Weightage + Duration
    let header = container(neu_card(
        row![
            button(
                text("← Back to Tutor")
                    .size(TYPE_CAPTION)
                    .color(neu.text_primary)
                    .wrapping(text::Wrapping::None),
            )
            .on_press(Message::SetMaindyScreen(MaindyScreen::TutorStudio))
            .style(secondary_button(palette))
            .padding([5, 10]),
            column![
                row![
                    container(
                        text(capsule.subject.clone())
                            .size(9)
                            .color(palette.accent_text)
                    )
                    .padding([2, 6])
                    .style(move |_t| container::Style {
                        background: Some(iced::Background::Color(palette.accent)),
                        border: iced::Border {
                            radius: iced::border::Radius::from(5.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    text(capsule.target_marks_impact.clone())
                        .size(10)
                        .color(neu.accent_primary),
                    text(format!("⏱️ {} mins total", capsule.duration_mins))
                        .size(10)
                        .color(neu.text_muted),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                text(capsule.title.clone())
                    .size(17)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(neu.text_primary),
            ]
            .spacing(3),
            h_space(),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        neu,
        14.0,
        1.0,
    ))
    .width(Length::Fill);

    // 2. Interactive 4-Step Phase Progress Stepper
    let mut stepper_row = row![].spacing(8).width(Length::Fill);
    for (idx, phase) in capsule.phases.iter().enumerate() {
        let is_cur = idx == active_phase_idx;
        let is_done = idx < active_phase_idx;
        let label = format!("{}. {} ({}m)", idx + 1, phase.name, phase.duration_mins);
        stepper_row = stepper_row.push(
            button(
                text(label)
                    .size(10)
                    .color(if is_cur || is_done {
                        palette.accent_text
                    } else {
                        neu.text_primary
                    })
                    .wrapping(text::Wrapping::None),
            )
            .on_press(Message::SelectCapsulePhase(idx))
            .style(toggle_button(palette, is_cur))
            .padding([7, 10])
            .width(Length::FillPortion(1)),
        );
    }

    // 3. Interactive Phase Content Stage
    let mut formulas_col = column![].spacing(6);
    for f in &capsule.core_formulas {
        formulas_col = formulas_col.push(
            text(f.clone())
                .size(13)
                .font(iced::Font::MONOSPACE)
                .color(neu.text_primary),
        );
    }

    let mut notes_col = column![].spacing(6);
    for line in &capsule.step_by_step_notes {
        notes_col = notes_col.push(
            text(format!("• {line}"))
                .size(TYPE_BODY)
                .color(Color::from_rgb(0.11, 0.14, 0.24)),
        );
    }

    let hint_section: Element<'a, Message> = if p.capsule_hint_open {
        container(
            column![
                text("💡 Maindy Hint")
                    .size(10)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(Color::from_rgb(0.471, 0.208, 0.059)),
                text(active_phase.hint.clone())
                    .size(TYPE_BODY)
                    .color(Color::from_rgb(0.271, 0.102, 0.012)),
            ]
            .spacing(4)
            .padding(12),
        )
        .width(Length::Fill)
        .style(move |_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(
                0.996, 0.953, 0.780,
            ))),
            border: iced::Border {
                radius: iced::border::Radius::from(10.0),
                color: Color::from_rgb(0.851, 0.467, 0.024),
                width: 1.2,
            },
            ..Default::default()
        })
        .into()
    } else {
        container(row![]).into()
    };

    let explain_prompt = format!(
        "Explain Step {} ({}) of the capsule '{}' with an intuitive example.",
        active_phase_idx + 1,
        active_phase.name,
        capsule.title
    );

    let phase_stage = scrollable(
        column![
            clay_tutor_card(
                column![
                    row![
                        text(format!(
                            "Phase {}: {} ({} mins)",
                            active_phase_idx + 1,
                            active_phase.name,
                            active_phase.duration_mins
                        ))
                        .size(14)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(Color::from_rgb(0.20, 0.15, 0.56)),
                        h_space(),
                        button(
                            text(if p.capsule_hint_open {
                                "💡 Hide Hint"
                            } else {
                                "💡 Show Hint"
                            })
                            .size(10)
                            .color(neu.text_primary),
                        )
                        .on_press(Message::ToggleCapsuleHint)
                        .style(secondary_button(palette))
                        .padding([4, 10]),
                        button(
                            text("🧠 Ask Maindy to explain this step")
                                .size(10)
                                .color(palette.accent_text)
                                .wrapping(text::Wrapping::None),
                        )
                        .on_press(Message::ExplainCapsuleStepInTutor(explain_prompt))
                        .style(primary_button(palette))
                        .padding([5, 10]),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                    text(active_phase.summary.clone())
                        .size(TYPE_BODY)
                        .color(Color::from_rgb(0.11, 0.14, 0.24)),
                    hint_section,
                    notes_col,
                ]
                .spacing(10),
                16.0,
            ),
            neu_card(
                column![
                    text("📐 CORE FORMULAS & EQUATIONS")
                        .size(10)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(neu.accent_primary),
                    neu_inset(formulas_col, neu, 10.0),
                ]
                .spacing(8),
                neu,
                14.0,
                1.0,
            ),
        ]
        .spacing(12),
    )
    .height(Length::Fill);

    // 4. Footer Actions: Start Focus Sprint, Pin to Desktop HUD, Back to Tutor
    let cap_id_sprint = capsule.id.clone();
    let cap_id_pin = capsule.id.clone();
    let sprint_mins = capsule.duration_mins;

    let footer = container(neu_card(
        row![
            button(
                text(format!("⚡ Start {}m Focus Sprint", sprint_mins))
                    .size(12)
                    .color(palette.accent_text)
                    .wrapping(text::Wrapping::None),
            )
            .on_press(Message::StartCapsuleFocusSprint(cap_id_sprint, sprint_mins))
            .style(primary_button(palette))
            .padding([8, 16]),
            button(
                text("📌 Pin to Desktop HUD")
                    .size(11)
                    .color(neu.text_primary)
                    .wrapping(text::Wrapping::None),
            )
            .on_press(Message::PinCapsuleToDesktopHud(cap_id_pin))
            .style(secondary_button(palette))
            .padding([8, 14]),
            h_space(),
            button(
                text("⚔️ Verify in Boss Battle")
                    .size(11)
                    .color(neu.text_primary)
                    .wrapping(text::Wrapping::None),
            )
            .on_press(Message::OpenVerificationBoss)
            .style(secondary_button(palette))
            .padding([8, 14]),
            button(
                text("← Back to Tutor")
                    .size(11)
                    .color(neu.text_primary)
                    .wrapping(text::Wrapping::None),
            )
            .on_press(Message::SetMaindyScreen(MaindyScreen::TutorStudio))
            .style(secondary_button(palette))
            .padding([8, 14]),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        neu,
        14.0,
        1.0,
    ))
    .width(Length::Fill);

    column![header, stepper_row, phase_stage, footer]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// ===========================================================================
// Drawer Tab 1: Active Capsule Queue, 590/600 Reviewer & Competitive MCQ Drill
// ===========================================================================
fn render_zone1_capsule_queue<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    study_capsules: &'a [StudyCapsule],
    active_mcq_paper: Option<&'a TestPaper>,
    mcq_answers: &'a HashMap<String, StudentAnswer>,
    mcq_report: Option<&'a TestEvaluationReport>,
) -> Element<'a, Message> {
    let neu = NeuPalette::for_theme(palette.mode);
    let mut capsule_list = column![].spacing(6);

    for cap in study_capsules.iter().take(4) {
        let cid = cap.id.clone();
        capsule_list = capsule_list.push(
            button(
                row![
                    column![
                        text(&cap.title).size(10).color(neu.text_primary),
                        text(format!("{} • {} mins", cap.subject, cap.duration_mins))
                            .size(8)
                            .color(neu.text_muted),
                    ]
                    .spacing(1)
                    .width(Length::Fill),
                    text("Open →").size(9).color(neu.accent_primary),
                ]
                .align_y(Alignment::Center),
            )
            .on_press(Message::OpenStudyCapsule(cid))
            .style(secondary_button(palette))
            .padding([5, 8])
            .width(Length::Fill),
        );
    }

    if let Some(first_cap) = core.exam_plan.capsules.first() {
        let cap_card = neu_inset(
            column![
                row![
                    container(text(&first_cap.subject).size(8).color(palette.accent_text))
                        .padding([2, 5])
                        .style(move |_t| container::Style {
                            background: Some(iced::Background::Color(palette.accent)),
                            border: iced::Border {
                                radius: iced::border::Radius::from(4.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    h_space(),
                    text(format!("{} mins", first_cap.total_minutes))
                        .size(9)
                        .color(neu.text_muted),
                ]
                .align_y(Alignment::Center),
                text(&first_cap.title)
                    .size(11)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Semibold,
                        ..Default::default()
                    })
                    .color(neu.text_primary),
                text(&first_cap.why_matters).size(9).color(neu.text_muted),
            ]
            .spacing(4),
            &neu,
            10.0,
        );
        capsule_list = capsule_list.push(cap_card);
    }

    if let Some(paper) = active_mcq_paper {
        let mut mcq_col = column![row![
            text(format!("🎯 {}", paper.title))
                .size(10)
                .color(neu.accent_primary),
            h_space(),
            text(format!("{} Qs", paper.total_questions))
                .size(8)
                .color(neu.text_muted),
        ]
        .align_y(Alignment::Center)]
        .spacing(4);

        for (q_idx, q) in paper.questions.iter().enumerate() {
            let mut q_box =
                column![
                    text(format!("Q{}. [{}] {}", q_idx + 1, q.topic, q.question_text))
                        .size(9)
                        .color(neu.text_primary),
                ]
                .spacing(2);

            if let Some(ta_q) = &q.question_ta {
                q_box = q_box.push(text(ta_q).size(8).color(neu.text_muted));
            }
            if let Some(stmts) = &q.statements {
                for st in stmts {
                    q_box = q_box.push(text(format!("  • {st}")).size(8).color(neu.text_muted));
                }
            }

            if q.question_type == QuestionType::Numerical {
                let current_ans = match mcq_answers.get(&q.id) {
                    Some(StudentAnswer::Numerical(v)) => format!("{:.2}", v),
                    _ => "Unattempted".to_string(),
                };
                q_box = q_box.push(
                    text(format!("Numerical Answer: {current_ans}"))
                        .size(8)
                        .color(neu.accent_primary),
                );
            } else {
                let selected_idx = match mcq_answers.get(&q.id) {
                    Some(StudentAnswer::Option(i)) => Some(*i),
                    _ => None,
                };
                for (opt_i, opt_label) in q.options.iter().enumerate() {
                    let is_sel = selected_idx == Some(opt_i);
                    let ta_suffix = q
                        .options_ta
                        .as_ref()
                        .and_then(|v| v.get(opt_i))
                        .map(|s| format!(" / {s}"))
                        .unwrap_or_default();
                    let btn = button(
                        text(format!(
                            "{}) {}{}",
                            (b'A' + opt_i as u8) as char,
                            opt_label,
                            ta_suffix
                        ))
                        .size(8)
                        .color(if is_sel {
                            palette.accent_text
                        } else {
                            neu.text_primary
                        }),
                    )
                    .on_press(Message::SelectCompetitiveMcqOption(q.id.clone(), opt_i))
                    .style(toggle_button(palette, is_sel))
                    .padding([2, 6])
                    .width(Length::Fill);
                    q_box = q_box.push(btn);
                }
            }
            mcq_col = mcq_col.push(neu_inset(q_box, &neu, 8.0));
        }

        mcq_col = mcq_col.push(
            button(
                text("✅ Submit Competitive MCQ")
                    .size(9)
                    .color(palette.accent_text),
            )
            .on_press(Message::SubmitCompetitiveMcq)
            .style(primary_button(palette))
            .padding([4, 10]),
        );

        if let Some(rep) = mcq_report {
            mcq_col = mcq_col.push(
                text(format!(
                    "Result: {:.1}/{:.1} ({:.1}% • Accuracy {:.0}%)",
                    rep.raw_score, rep.max_marks, rep.percentage, rep.accuracy_percentage
                ))
                .size(9)
                .color(palette.success),
            );
        }
        capsule_list = capsule_list.push(mcq_col);
    }

    let action_row = row![
        button(text("⚡ 25m Focus").size(9).color(palette.accent_text))
            .on_press(Message::StartFocusSession)
            .style(primary_button(palette))
            .padding([4, 8]),
        button(text("⚔️ Boss").size(9).color(neu.text_primary))
            .on_press(Message::OpenVerificationBoss)
            .style(secondary_button(palette))
            .padding([4, 8]),
        button(text("📊 Report").size(9).color(neu.text_primary))
            .on_press(Message::OpenExamReport)
            .style(secondary_button(palette))
            .padding([4, 8]),
    ]
    .spacing(4);

    let exam_tools_row = row![
        button(text("📝 Grade PDF").size(8).color(neu.text_primary))
            .on_press(Message::PickExamPaperToGrade)
            .style(secondary_button(palette))
            .padding([3, 6]),
        button(text("🎯 JEE").size(8).color(neu.text_primary))
            .on_press(Message::GenerateCompetitiveMcq {
                target: ExamTarget::JeeMain,
                subject: "Physics".to_string(),
                topic: None,
                count: 3,
            })
            .style(secondary_button(palette))
            .padding([3, 6]),
        button(text("🏛️ TNPSC").size(8).color(neu.text_primary))
            .on_press(Message::GenerateCompetitiveMcq {
                target: ExamTarget::TnpscGroup1,
                subject: "General Studies".to_string(),
                topic: None,
                count: 3,
            })
            .style(secondary_button(palette))
            .padding([3, 6]),
        button(text("⚖️ UPSC").size(8).color(neu.text_primary))
            .on_press(Message::GenerateCompetitiveMcq {
                target: ExamTarget::UpscCse,
                subject: "General Studies".to_string(),
                topic: None,
                count: 3,
            })
            .style(secondary_button(palette))
            .padding([3, 6]),
    ]
    .spacing(4);

    column![
        scrollable(capsule_list).height(Length::Fill),
        action_row,
        exam_tools_row,
    ]
    .spacing(6)
    .height(Length::Fill)
    .into()
}

// ===========================================================================
// Drawer Tab 2: Vault / PDF Textbook & RAG Citations Reader
// ===========================================================================
fn render_zone3_pdf_vault_reader<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    rag_pdf_chunks: &'a [PdfDocumentChunk],
    rag_active_file: Option<&'a str>,
    rag_current_page: usize,
    rag_query_input: &'a str,
    rag_citations: &'a [PdfCitationMatch],
) -> Element<'a, Message> {
    let neu = NeuPalette::for_theme(palette.mode);
    let mut distinct_files: Vec<&str> = Vec::new();
    for c in rag_pdf_chunks {
        if !distinct_files.contains(&c.file_name.as_str()) {
            distinct_files.push(c.file_name.as_str());
        }
    }

    let active_file_name = rag_active_file.or_else(|| distinct_files.first().copied());
    let file_chunks: Vec<&PdfDocumentChunk> = if let Some(fname) = active_file_name {
        rag_pdf_chunks
            .iter()
            .filter(|c| c.file_name == fname)
            .collect()
    } else {
        Vec::new()
    };

    let total_pages = file_chunks
        .iter()
        .map(|c| c.page_number as usize)
        .max()
        .unwrap_or(1)
        .max(1);
    let current_page_display = if file_chunks.is_empty() {
        core.study_reader.active_tab_index + 1
    } else {
        rag_current_page.clamp(1, total_pages)
    };
    let total_pages_display = if file_chunks.is_empty() {
        STUDY_TOPICS.len()
    } else {
        total_pages
    };

    let nav_row = row![
        button(text("📄 Upload PDF").size(9).color(palette.accent_text))
            .on_press(Message::PickVaultPdfToIngest)
            .style(primary_button(palette))
            .padding([3, 7]),
        button(text("◀").size(9).color(neu.text_primary))
            .on_press(Message::Zone3PrevPage)
            .style(secondary_button(palette))
            .padding([3, 7]),
        text(format!("{}/{}", current_page_display, total_pages_display))
            .size(9)
            .color(neu.accent_primary),
        button(text("▶").size(9).color(neu.text_primary))
            .on_press(Message::Zone3NextPage)
            .style(secondary_button(palette))
            .padding([3, 7]),
        h_space(),
        button(text("📁 Files").size(9).color(neu.text_primary))
            .on_press(Message::OpenWindow(WindowId::FileVault))
            .style(secondary_button(palette))
            .padding([3, 7]),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let mut file_pills = row![].spacing(4);
    for fname in distinct_files.iter().take(3) {
        let is_sel = active_file_name == Some(*fname);
        file_pills = file_pills.push(
            button(text(*fname).size(8).color(if is_sel {
                palette.accent_text
            } else {
                neu.text_primary
            }))
            .on_press(Message::Zone3SelectPdf((*fname).to_string()))
            .style(toggle_button(palette, is_sel))
            .padding([2, 6]),
        );
    }

    let rag_search_bar = text_input("Search PDF citations...", rag_query_input)
        .on_input(Message::Zone3SearchChanged)
        .style(modern_text_input(palette))
        .padding(5)
        .width(Length::Fill);

    let mut page_content = column![].spacing(5);
    for hit in rag_citations.iter().take(3) {
        let prompt_q = format!(
            "Explain '{}' from {} (Page {}): {}",
            hit.chunk.heading.as_deref().unwrap_or("this section"),
            hit.chunk.file_name,
            hit.chunk.page_number,
            hit.chunk.text_content.chars().take(160).collect::<String>()
        );
        page_content = page_content.push(neu_inset(
            column![
                row![
                    text(&hit.citation_label).size(8).color(neu.accent_primary),
                    h_space(),
                    button(text("Ask Maindy").size(8).color(neu.text_primary))
                        .on_press(Message::ChatInputChanged(prompt_q))
                        .style(secondary_button(palette))
                        .padding([2, 5]),
                ]
                .align_y(Alignment::Center),
                text(hit.chunk.text_content.chars().take(150).collect::<String>())
                    .size(8)
                    .color(neu.text_muted),
            ]
            .spacing(3),
            &neu,
            8.0,
        ));
    }

    let current_page_chunks: Vec<&PdfDocumentChunk> = file_chunks
        .iter()
        .copied()
        .filter(|c| c.page_number as usize == current_page_display)
        .collect();

    let ask_page_prompt: String;
    if !current_page_chunks.is_empty() {
        let mut combined_text = String::new();
        for ch in &current_page_chunks {
            if let Some(h) = &ch.heading {
                page_content = page_content.push(text(h).size(10).color(neu.text_primary));
            }
            page_content = page_content.push(text(&ch.text_content).size(9).color(neu.text_muted));
            combined_text.push_str(&ch.text_content);
            combined_text.push(' ');
        }
        ask_page_prompt = format!(
            "Explain Page {} of {}: {}",
            current_page_display,
            active_file_name.unwrap_or("PDF"),
            combined_text.chars().take(220).collect::<String>()
        );
    } else {
        let topic = core.study_reader.current_topic();
        page_content = page_content.push(
            text(topic.title)
                .size(10)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(neu.text_primary),
        );
        page_content = page_content.push(
            text(format!("Formula: {}", topic.formula))
                .size(9)
                .color(neu.accent_primary),
        );
        page_content =
            page_content.push(text(topic.derivation_steps).size(9).color(neu.text_muted));
        ask_page_prompt = format!(
            "Explain formula '{}' and steps for {}.",
            topic.formula, topic.title
        );
    }

    let ask_page_btn = button(
        text("💬 Ask Maindy About Page")
            .size(9)
            .color(palette.accent_text),
    )
    .on_press(Message::ChatInputChanged(ask_page_prompt))
    .style(primary_button(palette))
    .padding([4, 10]);

    column![
        nav_row,
        file_pills,
        rag_search_bar,
        scrollable(page_content).height(Length::Fill),
        ask_page_btn,
    ]
    .spacing(5)
    .height(Length::Fill)
    .into()
}

// ===========================================================================
// Drawer Tab 3: Academic Research with AdBlock Shield
// ===========================================================================
fn render_zone4_academic_search<'a>(
    palette: ThemePalette,
    academic_search: &'a AcademicSearchEngine,
    academic_search_input: &'a str,
    academic_search_provider: SearchProviderType,
    academic_search_loading: bool,
    _academic_search_error: Option<&'a str>,
) -> Element<'a, Message> {
    let neu = NeuPalette::for_theme(palette.mode);
    let shield = &academic_search.shield;
    let shield_pill = container(
        text(format!(
            "🛡️ AdBlock Shield: {} ({} Ads Blocked)",
            if shield.is_enabled {
                "ACTIVE"
            } else {
                "PAUSED"
            },
            shield.ads_blocked_count
        ))
        .size(9)
        .color(palette.success),
    )
    .padding([3, 6]);

    let providers = [
        (SearchProviderType::Encyclopedia, "Encyc"),
        (SearchProviderType::ArXiv, "ArXiv"),
        (SearchProviderType::DuckDuckGo, "DDG"),
        (SearchProviderType::Google, "Google"),
    ];
    let mut provider_row = row![].spacing(4);
    for (prov, label) in providers {
        let is_sel = academic_search_provider == prov;
        provider_row = provider_row.push(
            button(text(label).size(8).color(if is_sel {
                palette.accent_text
            } else {
                neu.text_primary
            }))
            .on_press(Message::Zone4SelectProvider(prov))
            .style(toggle_button(palette, is_sel))
            .padding([3, 7]),
        );
    }

    let mut hit_list = column![].spacing(5);
    if academic_search_loading {
        hit_list = hit_list.push(
            text("Searching academic sources...")
                .size(9)
                .color(neu.accent_primary),
        );
    }

    if academic_search.results.is_empty() && !academic_search_loading {
        hit_list = hit_list.push(
            text("Search STEM papers, constants, or syllabus theorems above.")
                .size(9)
                .color(neu.text_muted),
        );
    } else {
        for res in &academic_search.results {
            let prompt_query = format!(
                "Explain {} based on: {}",
                res.title,
                res.snippet.chars().take(180).collect::<String>()
            );
            hit_list = hit_list.push(neu_inset(
                column![
                    row![
                        text(&res.title)
                            .size(10)
                            .color(neu.text_primary)
                            .width(Length::Fill),
                        button(text("Ask").size(8).color(neu.text_primary))
                            .on_press(Message::ChatInputChanged(prompt_query))
                            .style(secondary_button(palette))
                            .padding([2, 6]),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                    text(&res.snippet).size(8).color(neu.text_muted),
                ]
                .spacing(2),
                &neu,
                8.0,
            ));
        }
    }

    let search_bar = row![
        text_input("Search Google, ArXiv, STEM...", academic_search_input)
            .on_input(Message::Zone4SearchInputChanged)
            .on_submit(Message::Zone4ExecuteSearch)
            .style(modern_text_input(palette))
            .padding(5)
            .width(Length::Fill),
        button(text("Go").size(9).color(palette.accent_text))
            .on_press(Message::Zone4ExecuteSearch)
            .style(primary_button(palette))
            .padding([5, 10]),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    column![
        shield_pill,
        provider_row,
        search_bar,
        scrollable(hit_list).height(Length::Fill),
    ]
    .spacing(5)
    .height(Length::Fill)
    .into()
}

// ===========================================================================
// Drawer Tab 4: Memory & Deduplicated Personalization Facts
// ===========================================================================
fn render_personalization_drawer<'a>(
    palette: ThemePalette,
    neu: &NeuPalette,
    memories: &'a [UserMemory],
    new_memory_input: &'a str,
    today_checkin: Option<&'a DailyFatigueCheckin>,
    due_leitner_cards: &'a [LeitnerCard],
    unresolved_mistakes: &'a [StudentMistakeRecord],
) -> Element<'a, Message> {
    let active_energy = today_checkin.map(|c| c.energy_level);
    let fatigue_row =
        row![
            button(
                text("High")
                    .size(8)
                    .color(if active_energy == Some(FatigueLevel::High) {
                        palette.accent_text
                    } else {
                        neu.text_primary
                    })
            )
            .on_press(Message::SubmitFatigueCheckin(FatigueLevel::High, 4.0))
            .style(toggle_button(
                palette,
                active_energy == Some(FatigueLevel::High)
            ))
            .padding([3, 6]),
            button(
                text("Normal")
                    .size(8)
                    .color(if active_energy == Some(FatigueLevel::Normal) {
                        palette.accent_text
                    } else {
                        neu.text_primary
                    })
            )
            .on_press(Message::SubmitFatigueCheckin(FatigueLevel::Normal, 3.0))
            .style(toggle_button(
                palette,
                active_energy == Some(FatigueLevel::Normal)
            ))
            .padding([3, 6]),
            button(text("Fatigued").size(8).color(
                if active_energy == Some(FatigueLevel::Fatigued) {
                    palette.accent_text
                } else {
                    neu.text_primary
                }
            ))
            .on_press(Message::SubmitFatigueCheckin(FatigueLevel::Fatigued, 2.0))
            .style(toggle_button(
                palette,
                active_energy == Some(FatigueLevel::Fatigued)
            ))
            .padding([3, 6]),
        ]
        .spacing(4);

    let mut box_counts = [0usize; 5];
    for card in due_leitner_cards {
        let idx = (card.current_box as usize).saturating_sub(1).min(4);
        box_counts[idx] += 1;
    }

    let mut spaced_col = column![
        text("Daily Energy Check-in").size(9).color(neu.text_muted),
        fatigue_row,
        row![
            text("Leitner Spaced Repetition")
                .size(9)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(neu.accent_primary),
            h_space(),
            button(text("+ Card").size(8).color(neu.text_primary))
                .on_press(Message::AddLeitnerCard {
                    front: "Gauss's Law Electric Flux".to_string(),
                    back: "Phi_E = Q_enc / epsilon_0".to_string(),
                    subject: "Physics".to_string(),
                })
                .style(secondary_button(palette))
                .padding([2, 6]),
        ]
        .align_y(Alignment::Center),
        text(format!(
            "B1:{} • B2:{} • B3:{} • B4:{} • B5:{} ({} due)",
            box_counts[0],
            box_counts[1],
            box_counts[2],
            box_counts[3],
            box_counts[4],
            due_leitner_cards.len()
        ))
        .size(8)
        .color(neu.text_muted),
        text(format!(
            "Open Misconceptions: {}",
            unresolved_mistakes.len()
        ))
        .size(9)
        .color(neu.text_primary),
    ]
    .spacing(4);

    for card in due_leitner_cards.iter().take(2) {
        let cid_ok = card.id.clone();
        let cid_fail = card.id.clone();
        spaced_col = spaced_col.push(
            row![
                text(format!("[B{}] {}", card.current_box, card.front_prompt))
                    .size(8)
                    .color(neu.text_primary)
                    .width(Length::Fill),
                button(text("✓").size(8).color(palette.accent_text))
                    .on_press(Message::ReviewLeitnerCard {
                        id: cid_ok,
                        success: true,
                    })
                    .style(primary_button(palette))
                    .padding([2, 5]),
                button(text("↻").size(8).color(neu.text_primary))
                    .on_press(Message::ReviewLeitnerCard {
                        id: cid_fail,
                        success: false,
                    })
                    .style(secondary_button(palette))
                    .padding([2, 5]),
            ]
            .spacing(3)
            .align_y(Alignment::Center),
        );
    }

    for m in unresolved_mistakes.iter().take(2) {
        spaced_col = spaced_col.push(
            row![
                text(format!("• [{}] {}", m.subject, m.chapter))
                    .size(8)
                    .color(neu.text_muted)
                    .width(Length::Fill),
                button(text("Resolve").size(7).color(neu.text_primary))
                    .on_press(Message::ResolveStudentMistake(m.id))
                    .style(secondary_button(palette))
                    .padding([2, 5]),
            ]
            .spacing(3)
            .align_y(Alignment::Center),
        );
    }

    let mut memory_items = column![neu_inset(spaced_col, neu, 10.0)].spacing(6);

    for m in memories {
        let m_id = m.id;
        memory_items = memory_items.push(neu_inset(
            row![
                column![
                    text(&m.category)
                        .size(8)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(neu.accent_primary),
                    text(&m.memory_text).size(9).color(neu.text_primary),
                ]
                .spacing(2)
                .width(Length::Fill),
                button(text("×").size(10).color(palette.danger))
                    .on_press(Message::DeleteUserMemory(m_id))
                    .style(secondary_button(palette))
                    .padding([2, 6]),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
            neu,
            8.0,
        ));
    }

    let add_memory_input = text_input("Add student fact...", new_memory_input)
        .on_input(Message::NewMemoryInputChanged)
        .on_submit(Message::AddUserMemory)
        .style(modern_text_input(palette))
        .padding(5)
        .width(Length::Fill);

    let add_row = row![
        add_memory_input,
        button(text("+ Add").size(9).color(palette.accent_text))
            .on_press(Message::AddUserMemory)
            .style(primary_button(palette))
            .padding([5, 10]),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    column![
        text(format!("Student Profile Facts ({})", memories.len()))
            .size(9)
            .color(neu.text_muted),
        scrollable(memory_items).height(Length::Fill),
        add_row,
    ]
    .spacing(6)
    .height(Length::Fill)
    .into()
}
