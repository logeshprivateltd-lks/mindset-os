//! 3D Odyssey Learning Map Canvas Program for MindSet OS
//!
//! Dual-Cache "Mild Dynamic" Engine:
//! - `static_road_cache`: Perspective winding ribbon road + 3D puck extrusion milestones.
//! - `dynamic_ambient_cache`: Active node breathing pulse ring, animated gradient progress trail,
//!   floating glass hover tooltips, and drifting ambient particles.
//! - Eco Static toggle pauses ambient ticks to maintain strict 0% CPU at idle.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use iced::mouse;
use iced::widget::canvas::{self, Geometry, Path, Program, Stroke};
use iced::widget::{button, column, container, row, text, Action};
use iced::{Alignment, Color, Element, Point, Rectangle, Size};
use mindset_core::odyssey::{NodeStatus, OdysseyNode};

use crate::theme::neumorphism::{neu_card, NeuPalette};
use crate::theme::{h_space, px_space};
use crate::Message;

#[derive(Debug)]
pub struct OdysseyMapState {
    pub static_road_cache: canvas::Cache,
    pub dynamic_ambient_cache: canvas::Cache,
    pub hovered_node: Option<usize>,
    pub selected_node: Option<usize>,
    pub scroll_offset: f32,
    pub ambient_tick: u32,
}

impl Default for OdysseyMapState {
    fn default() -> Self {
        Self {
            static_road_cache: canvas::Cache::default(),
            dynamic_ambient_cache: canvas::Cache::default(),
            hovered_node: None,
            selected_node: None,
            scroll_offset: 0.0,
            ambient_tick: 0,
        }
    }
}

pub struct OdysseyMapProgram<'a> {
    pub nodes: &'a [OdysseyNode],
    pub map_theme: &'a str,
    pub hovered_node: Option<usize>,
    pub scroll_offset: f32,
    pub ambient_tick: u32,
    pub mild_motion_enabled: bool,
    pub static_cache: &'a canvas::Cache,
    pub dynamic_cache: &'a canvas::Cache,
}

pub struct ProjectPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

/// Perspective projection with subtle 3D parallax tilt and motion sway.
pub fn project(
    t: f32,
    w: f32,
    h: f32,
    tilt_x: f32,
    tilt_y: f32,
    sway: f32,
) -> ProjectPoint {
    let t_clamped = t.clamp(0.0, 1.0);
    let u = 0.5 + 0.22 * (2.6 * std::f32::consts::PI * t_clamped + 0.35).sin();
    let z = 1.0 - 0.52 * t_clamped;
    // Optical parallax depth: distant points (lower z) exhibit greater relative perspective shift
    let depth_factor = (1.0 - z).max(0.0);
    let px = (tilt_x + sway) * depth_factor;
    let py = tilt_y * depth_factor;

    let x = w * (0.5 + (u - 0.5) * z) + px;
    let y = h * (0.84 - 0.62 * t_clamped.powf(0.85)) + py;
    let r = 26.0 * z;
    ProjectPoint { x, y, z, r }
}

impl<'a> Program<Message, iced::Theme, iced::Renderer> for OdysseyMapProgram<'a> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        let cursor_pos = match cursor.position_in(bounds) {
            Some(pos) => pos,
            None => {
                if self.hovered_node.is_some() {
                    return Some(Action::publish(Message::HoverOdysseyNode(None)));
                }
                return None;
            }
        };

        let w = bounds.width;
        let h = bounds.height;
        let n = self.nodes.len();

        let tilt_x = ((cursor_pos.x / w) - 0.5) * 18.0;
        let tilt_y = ((cursor_pos.y / h) - 0.5) * 12.0;
        let sway = if self.mild_motion_enabled {
            (self.ambient_tick as f32 * 0.05).sin() * 2.5
        } else {
            0.0
        };

        match event {
            iced::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let mut hit_idx = None;

                for (i, _) in self.nodes.iter().enumerate() {
                    let base_t = i as f32 / (n - 1).max(1) as f32;
                    let t = (base_t + self.scroll_offset).clamp(0.0, 1.0);
                    let pt = project(t, w, h, tilt_x, tilt_y, sway);
                    let dx = cursor_pos.x - pt.x;
                    let dy = cursor_pos.y - pt.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist <= pt.r * 1.35 {
                        hit_idx = Some(i);
                        break;
                    }
                }

                if hit_idx != self.hovered_node {
                    return Some(Action::publish(Message::HoverOdysseyNode(hit_idx)));
                }
            }
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                for (i, _) in self.nodes.iter().enumerate() {
                    let base_t = i as f32 / (n - 1).max(1) as f32;
                    let t = (base_t + self.scroll_offset).clamp(0.0, 1.0);
                    let pt = project(t, w, h, tilt_x, tilt_y, sway);
                    let dx = cursor_pos.x - pt.x;
                    let dy = cursor_pos.y - pt.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist <= pt.r * 1.35 {
                        let node_id = self.nodes[i].id.clone();
                        return Some(Action::publish(Message::EmbarkNode(node_id)));
                    }
                }
            }
            iced::Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                let scroll_y = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => *y,
                    mouse::ScrollDelta::Pixels { y, .. } => *y * 0.05,
                };
                return Some(Action::publish(Message::ScrollOdysseyMap(scroll_y)));
            }
            _ => {}
        }

        None
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let (road_fill, road_stroke, center_line_col, particle_col) = match self.map_theme {
            "GoldenValley" => (
                Color::from_rgba(0.70, 0.48, 0.12, 0.32),
                Color::from_rgba(1.0, 0.82, 0.32, 0.55),
                Color::from_rgba(1.0, 0.95, 0.80, 0.60),
                Color::from_rgba(1.0, 0.85, 0.40, 0.45),
            ),
            "MidnightStarpath" => (
                Color::from_rgba(0.22, 0.16, 0.52, 0.38),
                Color::from_rgba(0.62, 0.48, 1.0, 0.60),
                Color::from_rgba(0.82, 0.78, 1.0, 0.65),
                Color::from_rgba(0.72, 0.65, 1.0, 0.50),
            ),
            "MinimalMatte" => (
                Color::from_rgba(0.26, 0.30, 0.34, 0.30),
                Color::from_rgba(0.62, 0.68, 0.74, 0.50),
                Color::from_rgba(0.85, 0.88, 0.90, 0.55),
                Color::from_rgba(0.80, 0.85, 0.90, 0.25),
            ),
            _ => (
                // AlpineTrail
                Color::from_rgba(0.08, 0.42, 0.68, 0.32),
                Color::from_rgba(0.42, 0.82, 1.0, 0.55),
                Color::from_rgba(1.0, 1.0, 1.0, 0.65),
                Color::from_rgba(0.65, 0.92, 1.0, 0.45),
            ),
        };

        let cursor_pos = cursor.position_in(bounds);
        let tilt_x = cursor_pos
            .map(|p| ((p.x / bounds.width) - 0.5) * 18.0)
            .unwrap_or(0.0);
        let tilt_y = cursor_pos
            .map(|p| ((p.y / bounds.height) - 0.5) * 12.0)
            .unwrap_or(0.0);
        let sway = if self.mild_motion_enabled {
            (self.ambient_tick as f32 * 0.05).sin() * 2.5
        } else {
            0.0
        };

        // 1. Static Geometry (Ribbon + 3D Pucks with Distinct States)
        let static_geom = self.static_cache.draw(renderer, bounds.size(), |frame| {
            let w = bounds.width;
            let h = bounds.height;

            // 1.1 Compute Ribbon Edges with subtle 3D tilt
            let samples = 60;
            let mut left_points = Vec::with_capacity(samples + 1);
            let mut right_points = Vec::with_capacity(samples + 1);
            let mut center_points = Vec::with_capacity(samples + 1);

            for s in 0..=samples {
                let t = s as f32 / samples as f32;
                let pt = project(t, w, h, tilt_x, tilt_y, sway);
                center_points.push(Point::new(pt.x, pt.y));

                let dt = 0.01;
                let pt_next = project((t + dt).min(1.0), w, h, tilt_x, tilt_y, sway);
                let pt_prev = project((t - dt).max(0.0), w, h, tilt_x, tilt_y, sway);
                let dx = pt_next.x - pt_prev.x;
                let dy = pt_next.y - pt_prev.y;
                let len = (dx * dx + dy * dy).sqrt().max(0.001);
                let nx = -dy / len;
                let ny = dx / len;
                let half_w = 17.0 * pt.z;

                left_points.push(Point::new(pt.x + nx * half_w, pt.y + ny * half_w));
                right_points.push(Point::new(pt.x - nx * half_w, pt.y - ny * half_w));
            }

            // Fill Road Ribbon
            if left_points.len() >= 2 {
                let mut ribbon_builder = canvas::path::Builder::new();
                ribbon_builder.move_to(left_points[0]);
                for p in &left_points[1..] {
                    ribbon_builder.line_to(*p);
                }
                for p in right_points.iter().rev() {
                    ribbon_builder.line_to(*p);
                }
                ribbon_builder.close();
                let ribbon_path = ribbon_builder.build();

                frame.fill(&ribbon_path, road_fill);
                frame.stroke(
                    &ribbon_path,
                    Stroke::default().with_color(road_stroke).with_width(1.0),
                );
            }

            // Dashed Centerline Ticks
            for i in (0..center_points.len()).step_by(3) {
                if i + 1 < center_points.len() {
                    let mut tick = canvas::path::Builder::new();
                    tick.move_to(center_points[i]);
                    tick.line_to(center_points[i + 1]);
                    frame.stroke(
                        &tick.build(),
                        Stroke::default()
                            .with_color(center_line_col)
                            .with_width(1.5),
                    );
                }
            }

            // 1.2 Render 3D Extruded Milestone Pucks in Painter's Order (decreasing t)
            let n = self.nodes.len();
            let mut sorted_indices: Vec<usize> = (0..n).collect();
            sorted_indices.sort_by(|&a, &b| {
                let ta = a as f32 / (n - 1).max(1) as f32;
                let tb = b as f32 / (n - 1).max(1) as f32;
                tb.partial_cmp(&ta).unwrap_or(std::cmp::Ordering::Equal)
            });

            for &idx in &sorted_indices {
                let node = &self.nodes[idx];
                let base_t = idx as f32 / (n - 1).max(1) as f32;
                let t = (base_t + self.scroll_offset).clamp(0.0, 1.0);
                let pt = project(t, w, h, tilt_x, tilt_y, sway);
                let is_hovered = self.hovered_node == Some(idx);
                let lift = if is_hovered { 6.0 * pt.z } else { 0.0 };

                // A. Ground Contact Shadow
                let shadow_path = Path::circle(Point::new(pt.x, pt.y + 8.0 * pt.z), pt.r * 0.95);
                let shadow_alpha = match node.status {
                    NodeStatus::Completed => 0.42,
                    NodeStatus::InProgress => 0.45,
                    NodeStatus::Locked => 0.22,
                };
                frame.fill(&shadow_path, Color::from_rgba(0.0, 0.0, 0.0, shadow_alpha));

                // B. 3D Extrusion Base (Side Thickness)
                let base_path = Path::circle(Point::new(pt.x, pt.y + 5.0 * pt.z), pt.r);
                let base_color = match node.status {
                    NodeStatus::Completed => Color::from_rgb(0.06, 0.44, 0.26),
                    NodeStatus::InProgress => Color::from_rgb(0.04, 0.36, 0.58),
                    NodeStatus::Locked => Color::from_rgba(0.14, 0.17, 0.23, 0.50),
                };
                frame.fill(&base_path, base_color);

                // C. Top Face with Distinct Node States
                let top_center = Point::new(pt.x, pt.y - lift);
                let top_path = Path::circle(top_center, pt.r);
                let top_color = match node.status {
                    NodeStatus::Completed => Color::from_rgb(0.10, 0.74, 0.46), // Solid vibrant accent fill
                    NodeStatus::InProgress => Color::from_rgb(0.08, 0.62, 0.90), // Active electric sky
                    NodeStatus::Locked => Color::from_rgba(0.20, 0.25, 0.32, 0.52), // Frosted muted style
                };
                frame.fill(&top_path, top_color);

                // Specular Rim Stroke
                let rim_stroke = match node.status {
                    NodeStatus::Completed => Color::from_rgba(1.0, 1.0, 1.0, 0.65),
                    NodeStatus::InProgress => Color::from_rgba(1.0, 1.0, 1.0, 0.85),
                    NodeStatus::Locked => Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                };
                frame.stroke(
                    &top_path,
                    Stroke::default()
                        .with_color(rim_stroke)
                        .with_width(if node.status == NodeStatus::InProgress { 1.8 } else { 1.2 }),
                );

                // Centered Glyph
                let (glyph, glyph_color) = match node.status {
                    NodeStatus::Completed => ("✓", Color::WHITE),
                    NodeStatus::InProgress => match node.subject.as_str() {
                        "Physics" => ("⚡", Color::WHITE),
                        "Chemistry" => ("🧪", Color::WHITE),
                        "Mathematics" => ("📐", Color::WHITE),
                        _ => ("📖", Color::WHITE),
                    },
                    NodeStatus::Locked => ("🔒", Color::from_rgba(0.70, 0.75, 0.82, 0.55)),
                };

                let font_size = (14.0 * pt.z).max(9.0);
                frame.fill_text(canvas::Text {
                    content: glyph.to_string(),
                    position: Point::new(top_center.x, top_center.y - font_size * 0.5),
                    color: glyph_color,
                    size: iced::Pixels(font_size),
                    align_x: iced::alignment::Horizontal::Center.into(),
                    align_y: iced::alignment::Vertical::Center,
                    ..Default::default()
                });
            }
        });

        // 2. Dynamic Ambient Geometry:
        // - Animated gradient progress trail connecting completed nodes
        // - Active node animated pulsing glow ring
        // - Floating glass tooltip on hover
        // - Drifting ambient particles
        let dynamic_geom = self.dynamic_cache.draw(renderer, bounds.size(), |frame| {
            let w = bounds.width;
            let h = bounds.height;
            let n = self.nodes.len();

            // 2.1 Animated Gradient Progress Trail Connecting Completed Nodes
            let completed_indices: Vec<usize> = self
                .nodes
                .iter()
                .enumerate()
                .filter(|(_, node)| node.status == NodeStatus::Completed)
                .map(|(idx, _)| idx)
                .collect();

            if !completed_indices.is_empty() {
                let max_completed_idx = *completed_indices.last().unwrap();
                let trail_target_idx = (max_completed_idx + 1).min(n - 1);
                let t_max = (trail_target_idx as f32 / (n - 1).max(1) as f32 + self.scroll_offset)
                    .clamp(0.0, 1.0);

                let steps = 40;
                let mut trail_builder = canvas::path::Builder::new();
                let start_pt = project(0.0, w, h, tilt_x, tilt_y, sway);
                trail_builder.move_to(Point::new(start_pt.x, start_pt.y));

                for step in 1..=steps {
                    let t_step = (step as f32 / steps as f32) * t_max;
                    let pt = project(t_step, w, h, tilt_x, tilt_y, sway);
                    trail_builder.line_to(Point::new(pt.x, pt.y));
                }
                let trail_path = trail_builder.build();

                // Outer Glowing Understroke
                frame.stroke(
                    &trail_path,
                    Stroke::default()
                        .with_color(Color::from_rgba(0.10, 0.85, 0.55, 0.35))
                        .with_width(6.0),
                );
                // Inner Crisp Glowing Progress Core
                frame.stroke(
                    &trail_path,
                    Stroke::default()
                        .with_color(Color::from_rgba(0.25, 0.98, 0.70, 0.90))
                        .with_width(2.5),
                );

                // Animated energy beacon traveling forward along completed path
                if self.mild_motion_enabled && t_max > 0.01 {
                    let pulse_phase = ((self.ambient_tick as f32 * 0.02) % 1.0) * t_max;
                    let beacon_pt = project(pulse_phase, w, h, tilt_x, tilt_y, sway);
                    let beacon_dot = Path::circle(
                        Point::new(beacon_pt.x, beacon_pt.y),
                        4.5 * beacon_pt.z,
                    );
                    frame.fill(&beacon_dot, Color::from_rgba(0.40, 1.0, 0.75, 0.95));
                    let beacon_ring = Path::circle(
                        Point::new(beacon_pt.x, beacon_pt.y),
                        8.5 * beacon_pt.z,
                    );
                    frame.stroke(
                        &beacon_ring,
                        Stroke::default()
                            .with_color(Color::from_rgba(0.20, 0.95, 0.65, 0.50))
                            .with_width(1.5),
                    );
                }
            }

            // 2.2 Active Node Breathing Pulsing Glow Ring
            for (idx, node) in self.nodes.iter().enumerate() {
                if node.status == NodeStatus::InProgress {
                    let base_t = idx as f32 / (n - 1).max(1) as f32;
                    let t = (base_t + self.scroll_offset).clamp(0.0, 1.0);
                    let pt = project(t, w, h, tilt_x, tilt_y, sway);

                    let pulse = if self.mild_motion_enabled {
                        (self.ambient_tick as f32 * 0.12).sin() * 4.0
                    } else {
                        0.0
                    };

                    // Outer soft pulsing halo
                    let ring1_radius = pt.r + 7.0 + pulse;
                    let ring1_alpha = 0.35 + 0.20 * (self.ambient_tick as f32 * 0.12).sin().abs();
                    let ring1_path = Path::circle(Point::new(pt.x, pt.y), ring1_radius);
                    frame.stroke(
                        &ring1_path,
                        Stroke::default()
                            .with_color(Color::from_rgba(0.14, 0.65, 0.95, ring1_alpha))
                            .with_width(2.5),
                    );

                    // Inner sharp high-energy ring
                    let ring2_radius = pt.r + 3.0 + (pulse * 0.35);
                    let ring2_path = Path::circle(Point::new(pt.x, pt.y), ring2_radius);
                    frame.stroke(
                        &ring2_path,
                        Stroke::default()
                            .with_color(Color::from_rgba(0.45, 0.88, 1.0, 0.85))
                            .with_width(1.5),
                    );
                    break;
                }
            }

            // 2.3 Floating Glass Tooltip on Hover
            if let Some(hover_idx) = self.hovered_node {
                if let Some(node) = self.nodes.get(hover_idx) {
                    let base_t = hover_idx as f32 / (n - 1).max(1) as f32;
                    let t = (base_t + self.scroll_offset).clamp(0.0, 1.0);
                    let pt = project(t, w, h, tilt_x, tilt_y, sway);

                    let tip_w = 196.0;
                    let tip_h = 66.0;
                    let tip_x = (pt.x - tip_w * 0.5).clamp(8.0, w - tip_w - 8.0);
                    let tip_y = (pt.y - pt.r - tip_h - 12.0).clamp(6.0, h - tip_h - 6.0);

                    // Drop shadow for 3D depth
                    let shadow_rect = Path::rounded_rectangle(
                        Point::new(tip_x + 1.0, tip_y + 3.0),
                        Size::new(tip_w, tip_h),
                        iced::border::Radius::from(8.0),
                    );
                    frame.fill(&shadow_rect, Color::from_rgba(0.0, 0.0, 0.0, 0.50));

                    // Frosted Glass Card Body
                    let glass_rect = Path::rounded_rectangle(
                        Point::new(tip_x, tip_y),
                        Size::new(tip_w, tip_h),
                        iced::border::Radius::from(8.0),
                    );
                    frame.fill(&glass_rect, Color::from_rgba(0.06, 0.10, 0.18, 0.92));
                    frame.stroke(
                        &glass_rect,
                        Stroke::default()
                            .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.25))
                            .with_width(1.0),
                    );

                    // Downward Pointer Notch
                    let notch_cx = pt.x.clamp(tip_x + 16.0, tip_x + tip_w - 16.0);
                    let mut notch = canvas::path::Builder::new();
                    notch.move_to(Point::new(notch_cx - 5.0, tip_y + tip_h));
                    notch.line_to(Point::new(notch_cx, tip_y + tip_h + 6.0));
                    notch.line_to(Point::new(notch_cx + 5.0, tip_y + tip_h));
                    notch.close();
                    frame.fill(&notch.build(), Color::from_rgba(0.06, 0.10, 0.18, 0.92));

                    // Text Content
                    // 1. Topic Title
                    let title_display = if node.title.len() > 26 {
                        format!("{}...", &node.title[..24])
                    } else {
                        node.title.clone()
                    };
                    frame.fill_text(canvas::Text {
                        content: title_display,
                        position: Point::new(tip_x + 10.0, tip_y + 8.0),
                        color: Color::WHITE,
                        size: iced::Pixels(11.0),
                        ..Default::default()
                    });

                    // 2. Subject & Mastery Percentage
                    let mastery_text = format!(
                        "{} • Mastery: {}%",
                        node.subject,
                        node.mastery_percentage()
                    );
                    frame.fill_text(canvas::Text {
                        content: mastery_text,
                        position: Point::new(tip_x + 10.0, tip_y + 26.0),
                        color: Color::from_rgb(0.38, 0.85, 1.0),
                        size: iced::Pixels(9.5),
                        ..Default::default()
                    });

                    // 3. Estimated Study Time & Click Hint
                    let time_text = format!(
                        "Est: {} • Click to embark",
                        node.estimated_time_display()
                    );
                    frame.fill_text(canvas::Text {
                        content: time_text,
                        position: Point::new(tip_x + 10.0, tip_y + 44.0),
                        color: Color::from_rgba(0.85, 0.88, 0.94, 0.78),
                        size: iced::Pixels(9.0),
                        ..Default::default()
                    });
                }
            }

            // 2.4 Ambient Drifting Particles (if motion enabled)
            if self.mild_motion_enabled {
                for k in 0..10 {
                    let k_f = k as f32;
                    let px = (k_f * 51.0 + self.ambient_tick as f32 * 0.35) % w;
                    let wave = ((self.ambient_tick + k * 8) as f32 * 0.06).sin() * 12.0;
                    let py = (k_f * 33.0 + wave) % h;
                    let rad = (1.8 * (0.6 + 0.4 * (k_f * 0.7).sin())).max(1.0);

                    let dot = Path::circle(Point::new(px, py), rad);
                    frame.fill(&dot, particle_col);
                }
            }
        });

        vec![static_geom, dynamic_geom]
    }
}

/// Floating Popover rendered over the map when a node is selected.
pub fn render_node_popover<'a>(node: &'a OdysseyNode, neu: &NeuPalette) -> Element<'a, Message> {
    let status_str = match node.status {
        NodeStatus::Completed => "Completed ✓",
        NodeStatus::InProgress => "Active Priority ⚡",
        NodeStatus::Locked => "Locked 🔒",
    };
    let status_col = match node.status {
        NodeStatus::Completed => neu.status_done,
        NodeStatus::InProgress => neu.accent_primary,
        NodeStatus::Locked => neu.text_muted,
    };

    let header = row![
        text(&node.subject).size(10).color(status_col),
        text("•").size(9).color(neu.text_muted),
        text(status_str).size(9).color(status_col),
        h_space(),
        button(text("✕").size(10).color(neu.text_muted))
            .on_press(Message::CloseOdysseyPopover)
            .style(|_t, _s| button::Style {
                background: None,
                ..Default::default()
            })
            .padding(2),
    ]
    .align_y(Alignment::Center);

    let title_txt = text(&node.title).size(12).color(neu.text_primary);

    let desc_txt = text(&node.description).size(9).color(neu.text_muted);

    let marks_txt = text(format!(
        "Mastery: {}% • Est: {} • {} XP",
        node.mastery_percentage(),
        node.estimated_time_display(),
        node.xp
    ))
    .size(9)
    .color(neu.accent_primary);

    let launch_btn = button(
        row![
            text("Launch Topic Study").size(10).color(Color::WHITE),
            text("→").size(10).color(Color::WHITE),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .on_press(Message::LaunchOdysseyNode(node.id.clone()))
    .style({
        let accent = neu.accent_primary;
        move |_t, s| {
            let bg = match s {
                button::Status::Hovered => Color { a: 0.90, ..accent },
                _ => accent,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    radius: iced::border::Radius::from(6.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    })
    .padding([5, 12]);

    let popover_box = column![
        header,
        title_txt,
        desc_txt,
        px_space(0.0, 2.0),
        row![marks_txt, h_space(), launch_btn].align_y(Alignment::Center),
    ]
    .spacing(5)
    .padding(10);

    container(neu_card(popover_box, neu, 12.0, 1.0))
        .width(360)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mindset_core::odyssey::OdysseyJourneyState;

    #[test]
    fn test_odyssey_map_state_default() {
        let state = OdysseyMapState::default();
        assert!(state.hovered_node.is_none());
        assert!(state.selected_node.is_none());
        assert_eq!(state.scroll_offset, 0.0);
        assert_eq!(state.ambient_tick, 0);
    }

    #[test]
    fn test_project_parallax_depth() {
        let w = 460.0;
        let h = 260.0;

        let p_neutral = project(0.0, w, h, 0.0, 0.0, 0.0);
        let p_tilted = project(0.0, w, h, 18.0, 10.0, 0.0);
        // At t = 0.0, z = 1.0, depth_factor = 0.0, so foreground point does not drift wildly
        assert!((p_neutral.x - p_tilted.x).abs() < 0.01);
        assert!((p_neutral.y - p_tilted.y).abs() < 0.01);

        // At t = 1.0 (distant background), z ~ 0.48, depth_factor > 0.5, so distant point shifts with parallax
        let p_back_neutral = project(1.0, w, h, 0.0, 0.0, 0.0);
        let p_back_tilted = project(1.0, w, h, 18.0, 10.0, 0.0);
        assert!((p_back_neutral.x - p_back_tilted.x).abs() > 5.0);
    }

    #[test]
    fn test_node_states_and_tooltips() {
        let journey = OdysseyJourneyState::new();
        assert_eq!(journey.nodes.len(), 4);
        assert_eq!(journey.nodes[0].status, NodeStatus::InProgress);
        assert_eq!(journey.nodes[0].mastery_percentage(), 85);
        assert_eq!(journey.nodes[0].estimated_time_display(), "35 mins");
        assert_eq!(journey.nodes[1].status, NodeStatus::Locked);
    }
}
