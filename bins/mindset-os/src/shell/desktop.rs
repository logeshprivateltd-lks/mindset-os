//! Desktop Canvas & Student Overview Widgets for MindSet OS
//!
//! Neumorphic matte rails and cards matching Next-Gen Spatial Hybrid Architecture:
//! - Left Column: Live Weather & Target badge + 3 Split Micro-Cards (Upcoming Events, Study Momentum Sparkline, Study Radar).
//! - Center Column: Maindy AI Briefing with Zero-Decision "Start Next Capsule" CTA + Inset SQLite Scratchpad.
//! - Right Column: Daily Capsule Queue with Daily Boss Bar progress bar, #1 Priority highlight, non-wrapping Launch button + Deep Focus Stats.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::mouse;
use iced::widget::canvas::{Frame, Geometry, Program, Stroke};
use iced::widget::{
    button, column, container, mouse_area, progress_bar, row, scrollable, stack, text, text_input,
};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Size};
use mindset_core::{DesktopWidgetPlacement, MindSetCoreState, WindowId};

use crate::storage::db::{CapsuleRecord, UserProfile};
use crate::system::state::SystemState;
use crate::theme::liquidmorphism::{liquid_card, liquid_widget, LiquidPalette};
use crate::theme::neumorphism::{neu_card, neu_inset, NeuPalette};
use crate::theme::{
    h_space, primary_button, px_space, secondary_button, ThemePalette, TYPE_BODY, TYPE_CAPTION,
    TYPE_METRIC, TYPE_SUBHEAD,
};
use crate::Message;

/// Computed layout slot geometry for free-placement desktop widgets.
#[derive(Debug, Clone, Copy)]
struct WidgetSlotGeometry {
    top: f32,
    left: f32,
    width: f32,
}

impl WidgetSlotGeometry {
    fn from_placement(placement: &DesktopWidgetPlacement) -> Self {
        Self {
            top: (placement.y - 10.0).max(0.0),
            left: placement.x.clamp(0.0, 960.0),
            width: placement.width.clamp(240.0, 640.0),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn view<'a>(
    core: &'a MindSetCoreState,
    system: &'a SystemState,
    profile: &'a UserProfile,
    capsules: &'a [CapsuleRecord],
    odyssey_state: &'a super::odyssey_map::OdysseyMapState,
    _search_query: &'a str,
    _wallpaper_handle: Option<&'a iced::widget::image::Handle>,
    palette: ThemePalette,
    tunnel_vision_active: bool,
    hud_visible: bool,
) -> Element<'a, Message> {
    let liquid = LiquidPalette::for_theme(palette.mode);
    let neu = NeuPalette::for_theme(palette.mode);

    if tunnel_vision_active {
        let focus_hud = render_active_capsule_hud_widget(core, profile, capsules, palette, &neu);
        return container(
            row![h_space(), focus_hud, h_space()]
                .width(Length::Fill)
                .align_y(Alignment::Start),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .padding(iced::Padding {
            top: 24.0,
            right: 20.0,
            bottom: 84.0,
            left: 20.0,
        })
        .into();
    }

    if core.wm.widget_layout.free_placement {
        return render_free_placement_canvas(
            core,
            system,
            profile,
            capsules,
            odyssey_state,
            palette,
            &liquid,
            &neu,
        );
    }

    // True Open Wallpaper Desktop Canvas (elementary OS + Deepin hybrid):
    // Keeps wallpaper visible across >70% of screen while pinning at most 2 compact
    // Neomorphic Side HUD widgets (Active Capsule + Streak & Focus). Heavy views
    // launch as floating windows from the Dock.
    if !hud_visible {
        return container(px_space(0.0, 0.0))
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
    }

    let side_hud_col = column![
        render_active_capsule_hud_widget(core, profile, capsules, palette, &neu),
        render_streak_focus_hud_widget(core, capsules, palette, &neu),
    ]
    .spacing(12)
    .width(310);

    container(
        row![side_hud_col, h_space()]
            .width(Length::Fill)
            .align_y(Alignment::Start),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(iced::Padding {
        top: 16.0,
        right: 24.0,
        bottom: 88.0, // Clear floating dock
        left: 20.0,
    })
    .into()
}

/// Compact Neomorphic Side HUD Widget #1: Active Study Capsule & Safe Non-Overflow CTA.
fn render_active_capsule_hud_widget<'a>(
    _core: &'a MindSetCoreState,
    profile: &'a UserProfile,
    capsules: &'a [CapsuleRecord],
    palette: ThemePalette,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let target_display = profile.target_exam_display();
    let student_name = if profile.name.trim().is_empty() {
        "Scholar"
    } else {
        profile.name.trim()
    };

    let next_capsule = capsules.iter().find(|c| c.status != "Completed");
    let (cap_title, cap_meta, has_pending) = match next_capsule {
        Some(cap) => (
            cap.unit.as_str(),
            format!("{} • {} mins remaining", cap.subject, cap.estimated_minutes),
            true,
        ),
        None => (
            "All Daily Capsules Cleared",
            "Ready for verification or Odyssey exploration".to_string(),
            false,
        ),
    };

    let cta_btn = if has_pending {
        button(
            text("Start Next Capsule →")
                .size(TYPE_CAPTION)
                .color(palette.accent_text)
                .wrapping(text::Wrapping::None),
        )
        .on_press(Message::OpenWindow(WindowId::StudyReader))
        .style(primary_button(palette))
        .width(Length::Shrink)
        .padding([6, 12])
    } else {
        button(
            text("Daily Goal Cleared ✓")
                .size(TYPE_CAPTION)
                .color(palette.accent_text)
                .wrapping(text::Wrapping::None),
        )
        .on_press(Message::OpenWindow(WindowId::Today))
        .style(primary_button(palette))
        .width(Length::Shrink)
        .padding([6, 12])
    };

    let ask_maindy_btn = button(
        text("🧠 Maindy")
            .size(TYPE_CAPTION)
            .color(neu.text_primary)
            .wrapping(text::Wrapping::None),
    )
    .on_press(Message::OpenWindow(WindowId::LayaAiChat))
    .style(secondary_button(palette))
    .width(Length::Shrink)
    .padding([6, 10]);

    let hud_body = column![
        row![
            text("⚡ ACTIVE CAPSULE")
                .size(TYPE_CAPTION)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(neu.accent_primary),
            h_space(),
            container(
                text(target_display)
                    .size(TYPE_CAPTION)
                    .color(neu.text_muted)
                    .wrapping(text::Wrapping::None),
            )
            .padding([2, 6])
            .clip(true),
        ]
        .align_y(Alignment::Center),
        text(format!("Welcome, {student_name}"))
            .size(TYPE_CAPTION)
            .color(neu.text_muted),
        neu_inset(
            column![
                text(cap_title)
                    .size(TYPE_SUBHEAD)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Semibold,
                        ..Default::default()
                    })
                    .color(neu.text_primary),
                text(cap_meta).size(TYPE_CAPTION).color(neu.text_muted),
            ]
            .spacing(3)
            .width(Length::Fill),
            neu,
            10.0,
        ),
        row![cta_btn, h_space(), ask_maindy_btn]
            .spacing(8)
            .align_y(Alignment::Center),
    ]
    .spacing(8);

    container(neu_card(hud_body, neu, 16.0, 0.94))
        .width(Length::Fill)
        .clip(true)
        .into()
}

/// Compact Neomorphic Side HUD Widget #2: Streak, Boss Progress & Focus Launcher.
fn render_streak_focus_hud_widget<'a>(
    core: &'a MindSetCoreState,
    capsules: &'a [CapsuleRecord],
    palette: ThemePalette,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let streak_days = core.today.streak.count.max(1);
    let cleared_count = capsules.iter().filter(|c| c.status == "Completed").count();
    let total_count = capsules.len();
    let focus_time = core.pomodoro.format_time();

    let hud_body = column![
        row![
            text("🎯 STREAK & FOCUS")
                .size(TYPE_CAPTION)
                .font(iced::font::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(neu.accent_primary),
            h_space(),
            text(format!("{cleared_count}/{total_count} Cleared"))
                .size(TYPE_CAPTION)
                .color(neu.text_muted),
        ]
        .align_y(Alignment::Center),
        row![
            container(neu_inset(
                column![
                    text("🔥 Streak").size(TYPE_CAPTION).color(neu.text_muted),
                    text(format!("{streak_days} Days"))
                        .size(TYPE_BODY)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(neu.text_primary),
                ]
                .spacing(2),
                neu,
                8.0,
            ))
            .width(Length::FillPortion(1)),
            container(neu_inset(
                column![
                    text("⏱️ Focus").size(TYPE_CAPTION).color(neu.text_muted),
                    text(focus_time)
                        .size(TYPE_BODY)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(neu.text_primary),
                ]
                .spacing(2),
                neu,
                8.0,
            ))
            .width(Length::FillPortion(1)),
        ]
        .spacing(8)
        .width(Length::Fill),
        progress_bar(0.0..=(total_count.max(1) as f32), cleared_count as f32).style({
            let bar_col = neu.accent_primary;
            let bg_col = neu.surface_inset;
            move |_t| progress_bar::Style {
                background: iced::Background::Color(bg_col),
                bar: iced::Background::Color(bar_col),
                border: iced::Border {
                    radius: iced::border::Radius::from(3.0),
                    ..Default::default()
                },
            }
        }),
        row![
            button(
                text("Start Focus Arena →")
                    .size(TYPE_CAPTION)
                    .color(neu.text_primary)
                    .wrapping(text::Wrapping::None),
            )
            .on_press(Message::OpenWindow(WindowId::Tasks))
            .style(secondary_button(palette))
            .width(Length::Shrink)
            .padding([5, 10]),
            h_space(),
            button(
                text("🗺️ Odyssey")
                    .size(TYPE_CAPTION)
                    .color(neu.text_primary)
                    .wrapping(text::Wrapping::None),
            )
            .on_press(Message::OpenWindow(WindowId::OdysseyMap))
            .style(secondary_button(palette))
            .width(Length::Shrink)
            .padding([5, 10]),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    ]
    .spacing(8);

    container(neu_card(hud_body, neu, 16.0, 0.94))
        .width(Length::Fill)
        .clip(true)
        .into()
}

/// Floating Neomorphic Window View for 3D Syllabus Odyssey Map (launched from Dock).
pub fn view_odyssey_window<'a>(
    core: &'a MindSetCoreState,
    profile: &'a UserProfile,
    odyssey_state: &'a super::odyssey_map::OdysseyMapState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let neu = NeuPalette::for_theme(palette.mode);
    let odyssey_canvas = iced::widget::canvas(super::odyssey_map::OdysseyMapProgram {
        nodes: &core.odyssey.nodes,
        map_theme: &profile.map_theme,
        hovered_node: odyssey_state.hovered_node,
        scroll_offset: odyssey_state.scroll_offset,
        ambient_tick: odyssey_state.ambient_tick,
        mild_motion_enabled: profile.mild_motion_enabled,
        static_cache: &odyssey_state.static_road_cache,
        dynamic_cache: &odyssey_state.dynamic_ambient_cache,
    })
    .width(Length::Fill)
    .height(Length::Fill);

    let total_stages = core.odyssey.nodes.len();
    let completed_stages = core.odyssey.completed_count();
    let progress_pct = core.odyssey.overall_progress_pct();

    let mut map_stack = stack![odyssey_canvas];
    if let Some(sel_idx) = odyssey_state.selected_node {
        if let Some(node) = core.odyssey.nodes.get(sel_idx) {
            let popover = super::odyssey_map::render_node_popover(node, &neu);
            map_stack = map_stack.push(
                container(popover)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            );
        }
    }

    container(
        column![
            row![
                text("🗺️ 3D Syllabus Odyssey Trail")
                    .size(TYPE_SUBHEAD)
                    .color(neu.text_primary),
                h_space(),
                text(format!(
                    "{}/{} Done ({}%)",
                    completed_stages, total_stages, progress_pct
                ))
                .size(TYPE_CAPTION)
                .color(neu.accent_primary),
            ]
            .align_y(Alignment::Center),
            neu_inset(map_stack, &neu, 12.0),
        ]
        .spacing(10)
        .padding(14),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Floating Neomorphic Window View for Quick Study Scratchpad (launched from Dock).
pub fn view_scratchpad_window<'a>(
    system: &'a SystemState,
    palette: ThemePalette,
) -> Element<'a, Message> {
    let neu = NeuPalette::for_theme(palette.mode);
    let curr_scratchpad = system.scratchpad_text.clone();
    let make_formula_chip = |formula: &'static str| {
        let text_to_set = if curr_scratchpad.trim().is_empty() {
            formula.to_string()
        } else {
            format!("{}\n• {}", curr_scratchpad.trim(), formula)
        };
        button(
            text(formula)
                .size(TYPE_CAPTION)
                .font(iced::Font::MONOSPACE)
                .color(neu.accent_primary),
        )
        .on_press(Message::ScratchpadChanged(text_to_set))
        .style(secondary_button(palette))
        .width(Length::Shrink)
        .padding([3, 8])
    };

    let scratchpad_input = text_input(
        "Type quick formulas, derivations, or study reminders here...",
        &system.scratchpad_text,
    )
    .on_input(Message::ScratchpadChanged)
    .font(iced::Font::MONOSPACE)
    .size(TYPE_BODY)
    .style(crate::theme::modern_text_input(palette))
    .padding(10)
    .width(Length::Fill);

    let copy_btn = button(text("📋 Copy").size(TYPE_CAPTION).color(neu.text_primary))
        .on_press(Message::CopyScratchpad)
        .style(secondary_button(palette))
        .padding([4, 10]);

    let is_empty = system.scratchpad_text.trim().is_empty();
    let clear_btn = button(
        text(if is_empty {
            "✨ Template"
        } else {
            "🗑️ Clear"
        })
        .size(TYPE_CAPTION)
        .color(neu.text_primary),
    )
    .on_press(Message::ScratchpadChanged(if is_empty {
        SystemState::default_scratchpad()
    } else {
        String::new()
    }))
    .style(secondary_button(palette))
    .padding([4, 10]);

    container(
        column![
            row![
                text("📝 Quick Study Scratchpad")
                    .size(TYPE_SUBHEAD)
                    .color(neu.text_primary),
                h_space(),
                text(system.scratchpad_save_status)
                    .size(TYPE_CAPTION)
                    .color(neu.accent_primary),
                copy_btn,
                clear_btn,
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            neu_inset(scratchpad_input, &neu, 10.0),
            row![
                text("Formulas:").size(TYPE_CAPTION).color(neu.text_muted),
                make_formula_chip("E = mc²"),
                make_formula_chip("hf = W + KE"),
                make_formula_chip("Φ = ∮ E·dA"),
                make_formula_chip("λ = h/p"),
                make_formula_chip("F = ma"),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .padding(16),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

// ----------------------------------------------------------------------------
// 1. Left Column: Live Weather & Target + 3 Split Breathing Micro-Cards
// ----------------------------------------------------------------------------
fn render_left_widgets<'a>(
    core: &'a MindSetCoreState,
    system: &'a SystemState,
    profile: &'a UserProfile,
    palette: ThemePalette,
    liquid: &LiquidPalette,
) -> Element<'a, Message> {
    let target_display = profile.target_exam_display();

    // 1.1 Weather & Target Score Card
    let target_pill = container(
        row![
            text("🎯").size(TYPE_BODY),
            text(target_display)
                .size(TYPE_CAPTION)
                .color(liquid.accent_active),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding([3, 8])
    .style({
        let accent = liquid.accent_active;
        let inset = liquid.surface_elevated;
        move |_t| container::Style {
            background: Some(iced::Background::Color(Color { a: 0.45, ..inset })),
            border: iced::Border {
                radius: iced::border::Radius::from(6.0),
                color: Color { a: 0.35, ..accent },
                width: 1.0,
            },
            ..Default::default()
        }
    });

    let weather_content = column![
        row![
            text("⛅").size(TYPE_METRIC),
            column![
                text(&system.weather.location)
                    .size(TYPE_BODY)
                    .color(liquid.text_muted),
                text(&system.weather.temperature)
                    .size(TYPE_SUBHEAD)
                    .color(liquid.text_primary),
                text(&system.weather.condition)
                    .size(TYPE_CAPTION)
                    .color(liquid.text_muted),
            ]
            .spacing(1),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        px_space(0.0, 4.0),
        target_pill,
    ]
    .spacing(4);

    let weather_card = container(liquid_widget(weather_content, 14.0, palette, false)).width(270);

    // 1.2 Micro-Card 1: Upcoming Events (Dynamic Task Scheduler)
    let next_task = core.tasks.tasks.iter().find(|t| !t.is_completed);
    let (event_title, event_detail) = if let Some(task) = next_task {
        (
            task.title.clone(),
            format!("{} • {}m sprint", task.subject, task.target_minutes),
        )
    } else {
        (
            "Deep Focus Ready".to_string(),
            "All scheduled blocks cleared — ready for Deep Focus".to_string(),
        )
    };

    let events_content = column![
        row![
            text("Upcoming Events")
                .size(TYPE_BODY)
                .color(liquid.text_muted),
            h_space(),
            button(text("↗").size(TYPE_CAPTION).color(liquid.text_muted))
                .on_press(Message::OpenWindow(WindowId::Tasks))
                .style({
                    let elevated = liquid.surface_elevated;
                    let border_col = liquid.rim_specular_light;
                    move |_t, s| {
                        let bg = match s {
                            button::Status::Hovered => elevated,
                            _ => Color::TRANSPARENT,
                        };
                        button::Style {
                            background: Some(iced::Background::Color(bg)),
                            border: iced::Border {
                                radius: iced::border::Radius::from(6.0),
                                color: Color {
                                    a: 0.25,
                                    ..border_col
                                },
                                width: 1.0,
                            },
                            ..Default::default()
                        }
                    }
                })
                .padding([1, 5]),
        ]
        .align_y(Alignment::Center),
        px_space(0.0, 2.0),
        text(event_title).size(TYPE_BODY).color(liquid.text_primary),
        text(event_detail)
            .size(TYPE_CAPTION)
            .color(liquid.text_muted),
    ]
    .spacing(4);

    let events_card = container(liquid_widget(events_content, 14.0, palette, false)).width(270);

    // 1.3 Micro-Card 2: Study Momentum Sparkline
    let momentum_content = column![
        row![
            text("Study Momentum")
                .size(TYPE_BODY)
                .color(liquid.text_muted),
            h_space(),
            text("+18.4%").size(TYPE_CAPTION).color(liquid.status_audio),
        ]
        .align_y(Alignment::Center),
        px_space(0.0, 2.0),
        text("94.2 Focus Index")
            .size(TYPE_BODY)
            .color(liquid.text_primary),
        px_space(0.0, 4.0),
        container(
            iced::widget::canvas(StockSparkline {
                stroke_color: liquid.status_audio,
            })
            .width(Length::Fill)
            .height(44),
        )
        .width(Length::Fill)
        .height(44),
    ]
    .spacing(4);

    let momentum_card = container(liquid_widget(momentum_content, 14.0, palette, false)).width(270);

    // 1.4 Micro-Card 3: Study Radar
    let radar_content = column![
        text("Study Radar").size(TYPE_BODY).color(liquid.text_muted),
        px_space(0.0, 2.0),
        text("Daily Capsule Queue Ready")
            .size(TYPE_BODY)
            .color(liquid.text_primary),
        text("Review weak topic derivations before boss battle")
            .size(TYPE_CAPTION)
            .color(liquid.text_muted),
    ]
    .spacing(3);

    let radar_card = container(liquid_widget(radar_content, 14.0, palette, false)).width(270);

    column![weather_card, events_card, momentum_card, radar_card]
        .spacing(12)
        .into()
}

// ----------------------------------------------------------------------------
// Smooth Neon Canvas Sparkline for Study Momentum
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
struct StockSparkline {
    stroke_color: Color,
}

impl<Message> Program<Message, iced::Theme, iced::Renderer> for StockSparkline {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let w = bounds.width;
        let h = bounds.height;

        let raw_points = [
            (0.02, 0.75),
            (0.12, 0.70),
            (0.22, 0.78),
            (0.32, 0.58),
            (0.42, 0.64),
            (0.52, 0.46),
            (0.62, 0.50),
            (0.72, 0.35),
            (0.82, 0.40),
            (0.92, 0.18),
            (0.98, 0.15),
        ];

        let points: Vec<Point> = raw_points
            .iter()
            .map(|(rx, ry)| Point::new(rx * w, ry * h))
            .collect();

        if points.len() >= 2 {
            let mut path_builder = iced::widget::canvas::path::Builder::new();
            path_builder.move_to(points[0]);

            for i in 0..points.len() - 1 {
                let p0 = points[i];
                let p1 = points[i + 1];
                let mid_x = (p0.x + p1.x) / 2.0;
                let c1 = Point::new(mid_x, p0.y);
                let c2 = Point::new(mid_x, p1.y);
                path_builder.bezier_curve_to(c1, c2, p1);
            }

            let stroke_path = path_builder.build();

            frame.stroke(
                &stroke_path,
                Stroke::default()
                    .with_color(Color {
                        a: 0.25,
                        ..self.stroke_color
                    })
                    .with_width(4.5),
            );

            frame.stroke(
                &stroke_path,
                Stroke::default()
                    .with_color(self.stroke_color)
                    .with_width(2.0),
            );

            if let Some(&last) = points.last() {
                frame.fill_rectangle(
                    Point::new(last.x - 2.5, last.y - 2.5),
                    Size::new(5.0, 5.0),
                    self.stroke_color,
                );
            }
        }

        vec![frame.into_geometry()]
    }
}

// ----------------------------------------------------------------------------
// 2. Center Column: Maindy AI Briefing, #1 Priority CTA & SQLite Scratchpad
// ----------------------------------------------------------------------------
fn render_briefing_card<'a>(
    core: &'a MindSetCoreState,
    profile: &'a UserProfile,
    capsules: &'a [CapsuleRecord],
    palette: ThemePalette,
    liquid: &LiquidPalette,
) -> Element<'a, Message> {
    let now = chrono::Local::now();
    let hour = now.format("%H").to_string().parse::<u32>().unwrap_or(12);
    let lang = crate::i18n::AppLanguage::from_code(&profile.language_code);
    let greeting = match lang {
        crate::i18n::AppLanguage::Tamil => {
            if hour < 12 {
                "காலை வணக்கம்"
            } else if hour < 17 {
                "மதிய வணக்கம்"
            } else {
                "மாலை வணக்கம்"
            }
        }
        crate::i18n::AppLanguage::Malayalam => {
            if hour < 12 {
                "സുപ്രഭാതം"
            } else if hour < 17 {
                "ശുഭദിനം"
            } else {
                "ശുഭസായാഹ്നം"
            }
        }
        _ => {
            if hour < 12 {
                "Good Morning"
            } else if hour < 17 {
                "Good Afternoon"
            } else {
                "Good Evening"
            }
        }
    };

    let student_name = if profile.name.trim().is_empty() {
        "Scholar"
    } else {
        profile.name.trim()
    };

    let first_incomplete = capsules.iter().find(|c| c.status != "Completed");
    let cta_button = if first_incomplete.is_some() {
        button(
            row![text("Start Next Capsule →")
                .size(TYPE_CAPTION)
                .color(palette.accent_text)
                .wrapping(text::Wrapping::None),]
            .spacing(4)
            .align_y(Alignment::Center),
        )
        .on_press(Message::OpenWindow(WindowId::StudyReader))
        .width(Length::Shrink)
        .style(primary_button(palette))
        .padding([6, 12])
    } else {
        button(
            row![text("Daily Goal Cleared ✓")
                .size(TYPE_CAPTION)
                .color(palette.accent_text)
                .wrapping(text::Wrapping::None),]
            .align_y(Alignment::Center),
        )
        .on_press(Message::OpenWindow(WindowId::Today))
        .width(Length::Shrink)
        .style(primary_button(palette))
        .padding([6, 12])
    };

    let is_briefing_pinned = core
        .wm
        .get_widget_placement(mindset_core::DesktopWidgetId::ScholarBriefing)
        .map(|p| p.is_pinned)
        .unwrap_or(false);
    let briefing_pin_btn =
        button(text(if is_briefing_pinned { "📌" } else { "📍" }).size(TYPE_BODY))
            .on_press(Message::ToggleWidgetPin(
                mindset_core::DesktopWidgetId::ScholarBriefing,
            ))
            .style(|_t, _s| button::Style {
                background: Some(iced::Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            })
            .padding([2, 4]);

    let briefing_close_btn = button(text("✕").size(TYPE_CAPTION).color(liquid.text_muted))
        .on_press(Message::ToggleWidgetVisibility(
            mindset_core::DesktopWidgetId::ScholarBriefing,
        ))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding([2, 4]);

    let briefing_header = mouse_area(
        row![
            text("🧠").size(TYPE_METRIC),
            column![
                text(format!("{}, {}!", greeting, student_name))
                    .size(TYPE_SUBHEAD)
                    .color(liquid.text_primary)
                    .wrapping(text::Wrapping::Word),
                text(format!("Target Track: {}", profile.track_name()))
                    .size(TYPE_CAPTION)
                    .color(liquid.text_muted)
                    .wrapping(text::Wrapping::None),
            ]
            .spacing(1),
            h_space(),
            briefing_pin_btn,
            briefing_close_btn,
            text("⠿").size(TYPE_BODY).color(liquid.text_muted),
            px_space(4.0, 0.0),
            row![
                button(
                    row![
                        text("Ask Maindy").size(TYPE_CAPTION),
                        text("→").size(TYPE_CAPTION)
                    ]
                    .spacing(3)
                    .align_y(Alignment::Center),
                )
                .on_press(Message::OpenWindow(WindowId::LayaAiChat))
                .style({
                    let elevated = liquid.surface_elevated;
                    let border_col = liquid.rim_specular_light;
                    move |_t, s| {
                        let bg = match s {
                            button::Status::Hovered => elevated,
                            _ => Color::TRANSPARENT,
                        };
                        button::Style {
                            background: Some(iced::Background::Color(bg)),
                            border: iced::Border {
                                radius: iced::border::Radius::from(6.0),
                                color: Color {
                                    a: 0.25,
                                    ..border_col
                                },
                                width: 1.0,
                            },
                            ..Default::default()
                        }
                    }
                })
                .padding([4, 10]),
                cta_button,
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .on_press(Message::StartWidgetDrag(
        mindset_core::DesktopWidgetId::ScholarBriefing,
    ))
    .interaction(mouse::Interaction::Grab);

    let ai_briefing_content = column![
        briefing_header,
        px_space(0.0, 4.0),
        text(format!(
            "Priority Focus: Work on {} to maximize exam readiness.",
            if profile.weak_topics.is_empty() {
                "Core Fundamentals".to_string()
            } else {
                profile.weak_topics.join(", ")
            }
        ))
        .size(TYPE_BODY)
        .color(liquid.text_muted),
    ]
    .spacing(6);

    container(liquid_card(ai_briefing_content, 14.0, palette))
        .width(520)
        .into()
}

fn render_odyssey_card<'a>(
    core: &'a MindSetCoreState,
    profile: &'a UserProfile,
    odyssey_state: &'a super::odyssey_map::OdysseyMapState,
    palette: ThemePalette,
    liquid: &LiquidPalette,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let odyssey_canvas = iced::widget::canvas(super::odyssey_map::OdysseyMapProgram {
        nodes: &core.odyssey.nodes,
        map_theme: &profile.map_theme,
        hovered_node: odyssey_state.hovered_node,
        scroll_offset: odyssey_state.scroll_offset,
        ambient_tick: odyssey_state.ambient_tick,
        mild_motion_enabled: profile.mild_motion_enabled,
        static_cache: &odyssey_state.static_road_cache,
        dynamic_cache: &odyssey_state.dynamic_ambient_cache,
    })
    .width(Length::Fill)
    .height(260);

    let total_stages = core.odyssey.nodes.len();
    let completed_stages = core.odyssey.completed_count();
    let progress_pct = core.odyssey.overall_progress_pct();

    let header_progress = container(progress_bar(0.0..=100.0, progress_pct as f32).style({
        let bar_col = liquid.status_audio;
        let bg_col = liquid.surface_elevated;
        move |_t| progress_bar::Style {
            background: iced::Background::Color(Color { a: 0.5, ..bg_col }),
            bar: iced::Background::Color(bar_col),
            border: iced::Border {
                radius: iced::border::Radius::from(3.0),
                ..Default::default()
            },
        }
    }))
    .width(80)
    .height(6);

    let is_odyssey_pinned = core
        .wm
        .get_widget_placement(mindset_core::DesktopWidgetId::OdysseyTrail)
        .map(|p| p.is_pinned)
        .unwrap_or(false);
    let odyssey_pin_btn = button(text(if is_odyssey_pinned { "📌" } else { "📍" }).size(TYPE_BODY))
        .on_press(Message::ToggleWidgetPin(
            mindset_core::DesktopWidgetId::OdysseyTrail,
        ))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding([2, 4]);

    let odyssey_close_btn = button(text("✕").size(TYPE_CAPTION).color(liquid.text_muted))
        .on_press(Message::ToggleWidgetVisibility(
            mindset_core::DesktopWidgetId::OdysseyTrail,
        ))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding([2, 4]);

    let map_header = mouse_area(
        row![
            text("🗺️").size(TYPE_SUBHEAD),
            text("3D Syllabus Odyssey Trail")
                .size(TYPE_BODY)
                .color(liquid.text_primary),
            h_space(),
            odyssey_pin_btn,
            odyssey_close_btn,
            text("⠿").size(TYPE_BODY).color(liquid.text_muted),
            px_space(4.0, 0.0),
            row![
                text(format!(
                    "{}/{} Done ({}%)",
                    completed_stages, total_stages, progress_pct
                ))
                .size(TYPE_CAPTION)
                .color(liquid.accent_active),
                header_progress,
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .on_press(Message::StartWidgetDrag(
        mindset_core::DesktopWidgetId::OdysseyTrail,
    ))
    .interaction(mouse::Interaction::Grab);

    let mut map_stack = stack![odyssey_canvas];
    if let Some(sel_idx) = odyssey_state.selected_node {
        if let Some(node) = core.odyssey.nodes.get(sel_idx) {
            let popover = super::odyssey_map::render_node_popover(node, neu);
            map_stack = map_stack.push(
                container(popover)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            );
        }
    }

    let odyssey_content = column![
        map_header,
        px_space(0.0, 4.0),
        container(map_stack).width(Length::Fill).height(260).style({
            let inset = liquid.surface_elevated;
            let border_col = liquid.rim_specular_light;
            move |_t| container::Style {
                background: Some(iced::Background::Color(Color { a: 0.35, ..inset })),
                border: iced::Border {
                    radius: iced::border::Radius::from(10.0),
                    color: Color {
                        a: 0.25,
                        ..border_col
                    },
                    width: 1.0,
                },
                ..Default::default()
            }
        }),
    ]
    .spacing(4);

    container(liquid_card(odyssey_content, 14.0, palette))
        .width(520)
        .into()
}

fn render_scratchpad_card<'a>(
    core: &'a MindSetCoreState,
    system: &'a SystemState,
    palette: ThemePalette,
    liquid: &LiquidPalette,
) -> Element<'a, Message> {
    let is_saved = system.scratchpad_save_status.contains("Auto-saved")
        || system.scratchpad_save_status.contains("Copied");
    let status_dot_color = if is_saved {
        liquid.status_audio
    } else {
        liquid.accent_active
    };

    let copy_btn = button(
        row![
            text("📋").size(TYPE_CAPTION),
            text("Copy").size(TYPE_CAPTION).color(liquid.text_primary),
        ]
        .spacing(3)
        .align_y(Alignment::Center),
    )
    .on_press(Message::CopyScratchpad)
    .style({
        let elevated = liquid.surface_elevated;
        let border_col = liquid.rim_specular_light;
        move |_t, s| {
            let bg = match s {
                button::Status::Hovered => elevated,
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(5.0),
                    color: Color {
                        a: 0.25,
                        ..border_col
                    },
                    width: 1.0,
                },
                ..Default::default()
            }
        }
    })
    .padding([2, 6]);

    let is_empty = system.scratchpad_text.trim().is_empty();
    let clear_label = if is_empty {
        "✨ Template"
    } else {
        "🗑️ Clear"
    };
    let clear_target = if is_empty {
        SystemState::default_scratchpad()
    } else {
        String::new()
    };

    let clear_btn = button(
        text(clear_label)
            .size(TYPE_CAPTION)
            .color(liquid.text_muted),
    )
    .on_press(Message::ScratchpadChanged(clear_target))
    .style({
        let elevated = liquid.surface_elevated;
        let border_col = liquid.rim_specular_light;
        move |_t, s| {
            let bg = match s {
                button::Status::Hovered => elevated,
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(5.0),
                    color: Color {
                        a: 0.20,
                        ..border_col
                    },
                    width: 1.0,
                },
                ..Default::default()
            }
        }
    })
    .padding([2, 6]);

    let curr_scratchpad = system.scratchpad_text.clone();
    let make_formula_chip = |formula: &'static str| {
        let text_to_set = if curr_scratchpad.trim().is_empty() {
            formula.to_string()
        } else {
            format!("{}\n• {}", curr_scratchpad.trim(), formula)
        };

        button(
            text(formula)
                .size(TYPE_CAPTION)
                .font(iced::Font::MONOSPACE)
                .color(liquid.accent_active),
        )
        .on_press(Message::ScratchpadChanged(text_to_set))
        .style({
            let inset = liquid.surface_base;
            let elevated = liquid.surface_elevated;
            let border_col = liquid.rim_specular_light;
            move |_t, s| {
                let bg = match s {
                    button::Status::Hovered => elevated,
                    _ => inset,
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(4.0),
                        color: Color {
                            a: 0.25,
                            ..border_col
                        },
                        width: 1.0,
                    },
                    ..Default::default()
                }
            }
        })
        .padding([2, 6])
    };

    let formula_bar = row![
        text("Formulas:")
            .size(TYPE_CAPTION)
            .color(liquid.text_muted),
        make_formula_chip("E = mc²"),
        make_formula_chip("hf = W + KE"),
        make_formula_chip("Φ = ∮ E·dA"),
        make_formula_chip("λ = h/p"),
        make_formula_chip("F = ma"),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let scratchpad_input = text_input(
        "Type quick formulas, thoughts, or study reminders here...",
        &system.scratchpad_text,
    )
    .on_input(Message::ScratchpadChanged)
    .font(iced::Font::MONOSPACE)
    .size(TYPE_BODY)
    .style({
        let txt = liquid.text_primary;
        let mut_col = liquid.text_muted;
        let sel_col = Color {
            a: 0.35,
            ..liquid.accent_active
        };
        move |_t, _s| text_input::Style {
            background: iced::Background::Color(Color::TRANSPARENT),
            border: iced::Border::default(),
            icon: mut_col,
            placeholder: mut_col,
            value: txt,
            selection: sel_col,
        }
    })
    .padding(6)
    .width(Length::Fill);

    let scratchpad_well = container(scratchpad_input).padding(8).style({
        let bg = liquid.surface_elevated;
        let border_col = liquid.rim_specular_dark;
        move |_t| container::Style {
            background: Some(iced::Background::Color(Color { a: 0.55, ..bg })),
            border: iced::Border {
                radius: iced::border::Radius::from(8.0),
                color: border_col,
                width: 1.0,
            },
            ..Default::default()
        }
    });

    let is_scratchpad_pinned = core
        .wm
        .get_widget_placement(mindset_core::DesktopWidgetId::Scratchpad)
        .map(|p| p.is_pinned)
        .unwrap_or(false);
    let scratchpad_pin_btn =
        button(text(if is_scratchpad_pinned { "📌" } else { "📍" }).size(TYPE_BODY))
            .on_press(Message::ToggleWidgetPin(
                mindset_core::DesktopWidgetId::Scratchpad,
            ))
            .style(|_t, _s| button::Style {
                background: Some(iced::Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            })
            .padding([2, 4]);

    let scratchpad_close_btn = button(text("✕").size(TYPE_CAPTION).color(liquid.text_muted))
        .on_press(Message::ToggleWidgetVisibility(
            mindset_core::DesktopWidgetId::Scratchpad,
        ))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding([2, 4]);

    let scratchpad_header = mouse_area(
        row![
            row![
                text("📝").size(TYPE_SUBHEAD),
                text("Quick Study Scratchpad")
                    .size(TYPE_BODY)
                    .color(liquid.text_primary),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
            h_space(),
            scratchpad_pin_btn,
            scratchpad_close_btn,
            text("⠿").size(TYPE_BODY).color(liquid.text_muted),
            px_space(4.0, 0.0),
            row![
                text("●").size(TYPE_CAPTION).color(status_dot_color),
                text(system.scratchpad_save_status)
                    .size(TYPE_CAPTION)
                    .color(liquid.accent_active),
                px_space(2.0, 0.0),
                copy_btn,
                clear_btn,
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        ]
        .align_y(Alignment::Center),
    )
    .on_press(Message::StartWidgetDrag(
        mindset_core::DesktopWidgetId::Scratchpad,
    ))
    .interaction(mouse::Interaction::Grab);

    let scratchpad_content = column![scratchpad_header, scratchpad_well, formula_bar,].spacing(6);

    container(liquid_card(scratchpad_content, 14.0, palette))
        .width(520)
        .into()
}

fn render_docked_boss_card<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    liquid: &LiquidPalette,
) -> Element<'a, Message> {
    let is_pinned = core
        .wm
        .get_widget_placement(mindset_core::DesktopWidgetId::VerificationBoss)
        .map(|p| p.is_pinned)
        .unwrap_or(false);

    let pin_btn = button(text(if is_pinned { "📌" } else { "📍" }).size(TYPE_BODY))
        .on_press(Message::ToggleWidgetPin(
            mindset_core::DesktopWidgetId::VerificationBoss,
        ))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding([2, 4]);

    let close_btn = button(text("✕").size(TYPE_CAPTION).color(liquid.text_muted))
        .on_press(Message::ToggleWidgetVisibility(
            mindset_core::DesktopWidgetId::VerificationBoss,
        ))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding([2, 4]);

    let header = mouse_area(
        row![
            text("⚔️").size(TYPE_SUBHEAD),
            px_space(4.0, 0.0),
            text("Active Boss Challenge")
                .size(TYPE_BODY)
                .color(liquid.text_primary),
            h_space(),
            pin_btn,
            close_btn,
            text("⠿").size(TYPE_BODY).color(liquid.text_muted),
        ]
        .align_y(Alignment::Center),
    )
    .on_press(Message::StartWidgetDrag(
        mindset_core::DesktopWidgetId::VerificationBoss,
    ))
    .interaction(mouse::Interaction::Grab);

    let content = column![
        header,
        px_space(0.0, 4.0),
        text("Syllabus mastery boss challenge active. Verify your recall with multi-stage Feynman synthesis.")
            .size(TYPE_CAPTION)
            .color(liquid.text_muted),
        px_space(0.0, 4.0),
        button(
            row![text("Launch Boss Challenge 🚀").size(TYPE_CAPTION), text("→").size(TYPE_CAPTION)]
                .spacing(4)
                .align_y(Alignment::Center),
        )
        .on_press(Message::ToggleOverlay(mindset_core::ActiveOverlay::VerificationBoss))
        .padding([4, 12]),
    ]
    .spacing(4);

    container(liquid_widget(content, 14.0, palette, true))
        .width(520)
        .into()
}

fn render_docked_tutor_card<'a>(
    core: &'a MindSetCoreState,
    palette: ThemePalette,
    liquid: &LiquidPalette,
) -> Element<'a, Message> {
    let is_pinned = core
        .wm
        .get_widget_placement(mindset_core::DesktopWidgetId::QuickTutor)
        .map(|p| p.is_pinned)
        .unwrap_or(false);

    let pin_btn = button(text(if is_pinned { "📌" } else { "📍" }).size(TYPE_BODY))
        .on_press(Message::ToggleWidgetPin(
            mindset_core::DesktopWidgetId::QuickTutor,
        ))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding([2, 4]);

    let close_btn = button(text("✕").size(TYPE_CAPTION).color(liquid.text_muted))
        .on_press(Message::ToggleWidgetVisibility(
            mindset_core::DesktopWidgetId::QuickTutor,
        ))
        .style(|_t, _s| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .padding([2, 4]);

    let header = mouse_area(
        row![
            text("💡").size(TYPE_SUBHEAD),
            px_space(4.0, 0.0),
            text("Quick Concept Tutor")
                .size(TYPE_BODY)
                .color(liquid.text_primary),
            h_space(),
            pin_btn,
            close_btn,
            text("⠿").size(TYPE_BODY).color(liquid.text_muted),
        ]
        .align_y(Alignment::Center),
    )
    .on_press(Message::StartWidgetDrag(
        mindset_core::DesktopWidgetId::QuickTutor,
    ))
    .interaction(mouse::Interaction::Grab);

    let content = column![
        header,
        px_space(0.0, 4.0),
        text("Search any formula, derivation, or capsule concept with instant peer breakdown.")
            .size(TYPE_CAPTION)
            .color(liquid.text_muted),
        px_space(0.0, 4.0),
        button(
            row![
                text("Open Tutor Launcher 🔍").size(TYPE_CAPTION),
                text("→").size(TYPE_CAPTION)
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        )
        .on_press(Message::ToggleOverlay(
            mindset_core::ActiveOverlay::Launcher
        ))
        .padding([4, 12]),
    ]
    .spacing(4);

    container(liquid_widget(content, 14.0, palette, true))
        .width(520)
        .into()
}

#[allow(clippy::too_many_arguments)]
fn render_free_placement_canvas<'a>(
    core: &'a MindSetCoreState,
    system: &'a SystemState,
    profile: &'a UserProfile,
    capsules: &'a [CapsuleRecord],
    odyssey_state: &'a super::odyssey_map::OdysseyMapState,
    palette: ThemePalette,
    liquid: &LiquidPalette,
    neu: &NeuPalette,
) -> Element<'a, Message> {
    let left_column = render_left_widgets(core, system, profile, palette, liquid);
    let right_column = render_right_column(capsules, palette, liquid);

    let side_rails = container(
        row![left_column, h_space(), right_column,]
            .width(Length::Fill)
            .align_y(Alignment::Start),
    )
    .max_width(1120.0)
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .padding(iced::Padding {
        top: 10.0,
        right: 24.0,
        bottom: 84.0,
        left: 24.0,
    });

    let mut canvas_stack = stack![side_rails];

    for placement in &core.wm.widget_layout.placements {
        if !placement.is_visible {
            continue;
        }

        let slot = WidgetSlotGeometry::from_placement(placement);
        let widget_card: Element<'a, Message> = match placement.id {
            mindset_core::DesktopWidgetId::ScholarBriefing => {
                render_briefing_card(core, profile, capsules, palette, liquid)
            }
            mindset_core::DesktopWidgetId::OdysseyTrail => {
                render_odyssey_card(core, profile, odyssey_state, palette, liquid, neu)
            }
            mindset_core::DesktopWidgetId::Scratchpad => {
                render_scratchpad_card(core, system, palette, liquid)
            }
            mindset_core::DesktopWidgetId::VerificationBoss => {
                render_docked_boss_card(core, palette, liquid)
            }
            mindset_core::DesktopWidgetId::QuickTutor => {
                render_docked_tutor_card(core, palette, liquid)
            }
        };

        let positioned = container(container(widget_card).max_width(slot.width))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(iced::Padding {
                top: slot.top,
                left: slot.left,
                right: 0.0,
                bottom: 0.0,
            });

        canvas_stack = canvas_stack.push(positioned);
    }

    scrollable(
        container(canvas_stack)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

// ----------------------------------------------------------------------------
// 3. Right Column: Daily Capsule Queue (with Boss Bar) & Focus Metrics
// ----------------------------------------------------------------------------
fn render_right_column<'a>(
    capsules: &'a [CapsuleRecord],
    palette: ThemePalette,
    liquid: &LiquidPalette,
) -> Element<'a, Message> {
    // 3.1 Daily Capsule Queue Card with Boss Bar
    let cleared_count = capsules.iter().filter(|c| c.status == "Completed").count();
    let total_count = capsules.len();

    let queue_title = row![
        text("⚡").size(TYPE_SUBHEAD),
        text("Daily Capsule Queue")
            .size(TYPE_BODY)
            .color(liquid.text_primary),
        h_space(),
        button(
            text("View All →")
                .size(TYPE_CAPTION)
                .color(liquid.accent_active)
        )
        .on_press(Message::OpenWindow(WindowId::Today))
        .style({
            let elevated = liquid.surface_elevated;
            let border_col = liquid.rim_specular_light;
            move |_t, s| {
                let bg = match s {
                    button::Status::Hovered => elevated,
                    _ => Color::TRANSPARENT,
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(6.0),
                        color: Color {
                            a: 0.25,
                            ..border_col
                        },
                        width: 1.0,
                    },
                    ..Default::default()
                }
            }
        })
        .padding([2, 6]),
    ]
    .align_y(Alignment::Center);

    let boss_bar = column![
        row![
            text("Daily Boss Bar")
                .size(TYPE_CAPTION)
                .color(liquid.text_muted),
            h_space(),
            text(format!("{}/{} Cleared", cleared_count, total_count))
                .size(TYPE_CAPTION)
                .color(liquid.accent_active),
        ]
        .align_y(Alignment::Center),
        px_space(0.0, 2.0),
        container(
            progress_bar(0.0..=(total_count.max(1) as f32), cleared_count as f32).style({
                let bar_col = liquid.accent_active;
                let bg_col = liquid.surface_elevated;
                move |_t| progress_bar::Style {
                    background: iced::Background::Color(Color { a: 0.5, ..bg_col }),
                    bar: iced::Background::Color(bar_col),
                    border: iced::Border {
                        radius: iced::border::Radius::from(3.0),
                        ..Default::default()
                    },
                }
            })
        )
        .height(6),
    ]
    .spacing(2);

    let first_incomplete_idx = capsules.iter().position(|c| c.status != "Completed");
    let mut capsule_list = column![].spacing(6);
    let preview_capsules = capsules.iter().take(3).enumerate();
    let mut count = 0;

    for (idx, cap) in preview_capsules {
        count += 1;
        let is_done = cap.status == "Completed";
        let is_priority = !is_done && Some(idx) == first_incomplete_idx;

        let status_icon = if is_done { "✓" } else { "○" };
        let status_color = if is_done {
            liquid.status_audio
        } else if is_priority {
            liquid.accent_active
        } else {
            liquid.text_muted
        };

        let priority_pill: Element<'a, Message> = if is_priority {
            container(
                text("NEXT UP")
                    .size(TYPE_CAPTION)
                    .color(liquid.accent_active),
            )
            .padding([1, 4])
            .style({
                let accent = liquid.accent_active;
                let inset = liquid.surface_elevated;
                move |_t| container::Style {
                    background: Some(iced::Background::Color(Color { a: 0.45, ..inset })),
                    border: iced::Border {
                        radius: iced::border::Radius::from(4.0),
                        color: Color { a: 0.35, ..accent },
                        width: 1.0,
                    },
                    ..Default::default()
                }
            })
            .into()
        } else {
            container(text("").size(1)).into()
        };

        let cap_item = button(
            row![
                text(status_icon).size(TYPE_BODY).color(status_color),
                column![
                    row![
                        text(&cap.unit)
                            .size(TYPE_BODY)
                            .color(liquid.text_primary)
                            .wrapping(text::Wrapping::None),
                        priority_pill,
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                    text(format!("{} • {}m", cap.subject, cap.estimated_minutes))
                        .size(TYPE_CAPTION)
                        .color(liquid.text_muted),
                ]
                .spacing(1)
                .width(Length::Fill),
                container(
                    text("Launch →")
                        .size(TYPE_CAPTION)
                        .color(liquid.accent_active)
                        .wrapping(text::Wrapping::None),
                )
                .padding([2, 6]),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .on_press(Message::OpenWindow(WindowId::StudyReader))
        .style({
            let elevated = liquid.surface_elevated;
            let base = liquid.surface_base;
            let accent = liquid.accent_active;
            let border_col = liquid.rim_specular_light;
            move |_t, s| {
                let bg = match s {
                    button::Status::Hovered => elevated,
                    button::Status::Pressed => base,
                    _ => Color::TRANSPARENT,
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(8.0),
                        color: if is_priority {
                            Color { a: 0.65, ..accent }
                        } else {
                            Color {
                                a: 0.22,
                                ..border_col
                            }
                        },
                        width: if is_priority { 1.5 } else { 1.0 },
                    },
                    ..Default::default()
                }
            }
        })
        .padding([5, 8])
        .width(Length::Fill);

        capsule_list = capsule_list.push(cap_item);
    }

    if count == 0 {
        capsule_list = capsule_list.push(
            text("No capsules queued. Open Today to generate.")
                .size(TYPE_CAPTION)
                .color(liquid.text_muted),
        );
    }

    let capsule_queue_content = column![
        queue_title,
        px_space(0.0, 4.0),
        boss_bar,
        px_space(0.0, 4.0),
        capsule_list
    ]
    .spacing(6);

    let capsule_queue_card =
        container(liquid_widget(capsule_queue_content, 14.0, palette, false)).width(270);

    // 3.2 Focus & Streak Stats Card
    let stats_content = column![
        text("Deep Focus Stats")
            .size(TYPE_BODY)
            .color(liquid.text_muted),
        px_space(0.0, 4.0),
        row![
            metric_chip("🔥 Streak", "7 Days", liquid, true),
            metric_chip("⏱️ Focus", "2h 45m", liquid, false),
        ]
        .spacing(8),
        px_space(0.0, 4.0),
        button(
            row![
                text("Start Pomodoro Arena").size(TYPE_CAPTION),
                text("🎯").size(TYPE_BODY)
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .on_press(Message::OpenWindow(WindowId::Tasks))
        .style({
            let accent = liquid.accent_active;
            move |_t, s| {
                let bg = match s {
                    button::Status::Hovered => Color { a: 0.90, ..accent },
                    _ => accent,
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        radius: iced::border::Radius::from(8.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }
        })
        .padding([5, 10])
        .width(Length::Fill),
    ]
    .spacing(6);

    let stats_card = container(liquid_widget(stats_content, 14.0, palette, false)).width(270);

    column![capsule_queue_card, stats_card].spacing(12).into()
}

fn metric_chip<'a>(
    label: &'static str,
    value: &'static str,
    liquid: &LiquidPalette,
    is_streak: bool,
) -> Element<'a, Message> {
    let val_color = if is_streak {
        liquid.status_focus
    } else {
        liquid.text_primary
    };
    let inset = liquid.surface_elevated;
    let border_col = liquid.rim_specular_light;

    container(
        column![
            text(label).size(TYPE_CAPTION).color(liquid.text_muted),
            text(value).size(TYPE_BODY).color(val_color),
        ]
        .spacing(1),
    )
    .padding([4, 8])
    .width(Length::Fill)
    .style(move |_t| container::Style {
        background: Some(iced::Background::Color(Color { a: 0.45, ..inset })),
        border: iced::Border {
            radius: iced::border::Radius::from(6.0),
            color: Color {
                a: 0.22,
                ..border_col
            },
            width: 1.0,
        },
        ..Default::default()
    })
    .into()
}
