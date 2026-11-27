//! Windows-Style Animated Emblem + Liquidmorphism "HELLO ACHIEVER" Boot Sequence
//!
//! Continues seamlessly from the early Plymouth boot splash into the Pure-Rust
//! desktop shell: renders the glowing MindSet OS 'MS' emblem, a smooth orbital
//! loading ring, and a sleek progress bar before transitioning to onboarding or
//! the active workspace with 0% idle CPU once finished.

use iced::mouse;
use iced::widget::canvas::{self, Action, Cache, Geometry, Path, Program, Stroke, Text};
use iced::{alignment, border::Radius, Color, Element, Length, Point, Rectangle, Size};
use mindset_motion::{Spring1D, SpringPreset};

use crate::Message;

pub const BOOT_DURATION_SECS: f32 = 2.8;

#[derive(Debug)]
pub struct BootSplashState {
    pub is_active: bool,
    pub is_finished: bool,
    pub elapsed: f32,
    pub duration_secs: f32,
    pub scale_spring: Spring1D,
    pub opacity_spring: Spring1D,
    pub cache: Cache,
}

impl Default for BootSplashState {
    fn default() -> Self {
        Self::new()
    }
}

impl BootSplashState {
    pub fn new() -> Self {
        Self {
            is_active: true,
            is_finished: false,
            elapsed: 0.0,
            duration_secs: BOOT_DURATION_SECS,
            scale_spring: Spring1D::with_preset(0.88, SpringPreset::FluidPanel),
            opacity_spring: Spring1D::with_preset(0.0, SpringPreset::SnappyMicro),
            cache: Cache::new(),
        }
    }

    /// Advances the boot splash physics by `dt` seconds.
    /// Returns `true` if the boot sequence has completed its 2.8s duration.
    pub fn update(&mut self, dt: f32) -> bool {
        if !self.is_active || self.is_finished {
            return true;
        }

        self.elapsed += dt;
        self.cache.clear();

        self.scale_spring.set_target(1.0);
        self.scale_spring.update(dt);

        self.opacity_spring.set_target(1.0);
        self.opacity_spring.update(dt);

        if self.elapsed >= self.duration_secs {
            self.is_finished = true;
            self.is_active = false;
            return true;
        }

        false
    }

    /// Skips the boot splash immediately (e.g., on click or keypress).
    pub fn skip(&mut self) {
        self.is_finished = true;
        self.is_active = false;
        self.cache.clear();
    }
}

pub struct BootSplashProgram<'a> {
    pub state: &'a BootSplashState,
}

impl<'a> Program<Message, iced::Theme, iced::Renderer> for BootSplashProgram<'a> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        match event {
            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if cursor.is_over(bounds) {
                    Some(Action::publish(Message::FinishBootSplash))
                } else {
                    None
                }
            }
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => {
                use iced::keyboard::{key::Named, Key};
                match key {
                    Key::Named(Named::Enter)
                    | Key::Named(Named::Space)
                    | Key::Named(Named::Escape) => Some(Action::publish(Message::FinishBootSplash)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let low_overhead = std::env::var("MINDSET_LOW_OVERHEAD").ok().as_deref() == Some("1");

        let geom = self.state.cache.draw(renderer, bounds.size(), |frame| {
            let w = bounds.width;
            let h = bounds.height;
            let cx = w * 0.5;
            let cy = h * 0.5;
            let t = self.state.elapsed;
            let progress = (t / self.state.duration_secs).clamp(0.0, 1.0);
            let alpha = self.state.opacity_spring.position.clamp(0.0, 1.0);
            let scale = self.state.scale_spring.position.clamp(0.5, 1.25);

            // 1. Deep Obsidian Canvas Background matching Plymouth early boot splash
            let bg_rect = Path::rectangle(Point::ORIGIN, Size::new(w, h));
            frame.fill(&bg_rect, Color::from_rgb(0.035, 0.043, 0.068));

            // 2. Ambient radial aurora halos (lightweight in VM low-overhead mode)
            let halo_layers = if low_overhead { 1 } else { 2 };
            let auroras = [
                (0.35, 0.38, 220.0, 0.16, 0.65, 0.96, 0.10),
                (0.66, 0.56, 240.0, 0.44, 0.36, 0.96, 0.11),
            ];
            for (bx, by, rad, red, green, blue, base_a) in auroras {
                let bx_px = w * bx + (t * 0.9).sin() * 24.0;
                let by_px = h * by + (t * 0.7).cos() * 18.0;
                for layer in 0..halo_layers {
                    let factor = 1.0 + (layer as f32) * 0.5;
                    let la = (base_a / ((layer + 1) as f32)) * alpha;
                    frame.fill(
                        &Path::circle(Point::new(bx_px, by_px), rad * factor),
                        Color::from_rgba(red, green, blue, la.clamp(0.0, 1.0)),
                    );
                }
            }

            // 3. Central Windows-Style MindSet OS Squircle Emblem
            let emblem_size = 104.0 * scale;
            let emblem_x = cx - emblem_size * 0.5;
            let emblem_y = cy - 118.0 * scale;

            // Soft outer halo behind emblem
            frame.fill(
                &Path::rounded_rectangle(
                    Point::new(emblem_x - 10.0, emblem_y - 10.0),
                    Size::new(emblem_size + 20.0, emblem_size + 20.0),
                    Radius::from(32.0 * scale),
                ),
                Color::from_rgba(0.22, 0.54, 0.98, 0.16 * alpha),
            );

            let emblem_rect = Path::rounded_rectangle(
                Point::new(emblem_x, emblem_y),
                Size::new(emblem_size, emblem_size),
                Radius::from(26.0 * scale),
            );
            frame.fill(
                &emblem_rect,
                Color::from_rgba(0.075, 0.095, 0.165, 0.96 * alpha),
            );
            frame.stroke(
                &emblem_rect,
                Stroke::default()
                    .with_color(Color::from_rgba(0.55, 0.82, 1.0, 0.45 * alpha))
                    .with_width(1.8),
            );

            // Crisp Geometric 'MS' Monogram inside the Emblem
            let m_left = cx - 28.0 * scale;
            let m_mid = cx - 15.0 * scale;
            let m_right = cx - 2.0 * scale;
            let mono_top = emblem_y + 34.0 * scale;
            let mono_bot = emblem_y + 70.0 * scale;
            let m_dip = emblem_y + 56.0 * scale;

            let m_path = Path::new(|b| {
                b.move_to(Point::new(m_left, mono_bot));
                b.line_to(Point::new(m_left, mono_top));
                b.line_to(Point::new(m_mid, m_dip));
                b.line_to(Point::new(m_right, mono_top));
                b.line_to(Point::new(m_right, mono_bot));
            });
            frame.stroke(
                &m_path,
                Stroke::default()
                    .with_color(Color::from_rgba(0.96, 0.98, 1.0, alpha))
                    .with_width(4.2 * scale),
            );

            let s_left = cx + 8.0 * scale;
            let s_right = cx + 28.0 * scale;
            let s_mid = emblem_y + 52.0 * scale;
            let s_path = Path::new(|b| {
                b.move_to(Point::new(s_right, mono_top + 4.0 * scale));
                b.line_to(Point::new(s_right, mono_top));
                b.line_to(Point::new(s_left, mono_top));
                b.line_to(Point::new(s_left, s_mid - 2.0 * scale));
                b.line_to(Point::new(s_right, s_mid + 2.0 * scale));
                b.line_to(Point::new(s_right, mono_bot));
                b.line_to(Point::new(s_left, mono_bot));
                b.line_to(Point::new(s_left, mono_bot - 4.0 * scale));
            });
            frame.stroke(
                &s_path,
                Stroke::default()
                    .with_color(Color::from_rgba(0.12, 0.86, 0.72, alpha))
                    .with_width(4.2 * scale),
            );
            frame.fill(
                &Path::circle(Point::new((s_left + s_right) * 0.5, mono_top), 3.2 * scale),
                Color::from_rgba(0.25, 0.94, 1.0, alpha),
            );

            // 4. Windows-Style Rotating Orbital Loading Ring below Emblem
            let spin_cy = emblem_y + emblem_size + 42.0 * scale;
            let spin_center = Point::new(cx, spin_cy);
            let spin_r = 16.0 * scale;

            // Subtle circular track
            frame.stroke(
                &Path::circle(spin_center, spin_r),
                Stroke::default()
                    .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.08 * alpha))
                    .with_width(2.5),
            );

            // Smooth trailing orbital dots/arc rotating at ~4.4 rad/s
            let base_angle = t * 4.4;
            let seg_count = 10usize;
            for idx in 0..seg_count {
                let frac = idx as f32 / seg_count as f32;
                let a0 = base_angle - frac * 3.6;
                let a1 = a0 + 0.34;
                let fade = (1.0 - frac).powf(1.5) * alpha;
                let p0 = Point::new(
                    spin_center.x + spin_r * a0.cos(),
                    spin_center.y + spin_r * a0.sin(),
                );
                let p1 = Point::new(
                    spin_center.x + spin_r * a1.cos(),
                    spin_center.y + spin_r * a1.sin(),
                );
                let seg_path = Path::line(p0, p1);
                frame.stroke(
                    &seg_path,
                    Stroke::default()
                        .with_color(Color::from_rgba(0.28, 0.82, 1.0, fade.clamp(0.0, 1.0)))
                        .with_width(2.8 * scale),
                );
            }

            // 5. Hero Typography & Subtitle
            let hero_y = spin_cy + 46.0 * scale;
            frame.fill_text(Text {
                content: "HELLO ACHIEVER".to_string(),
                position: Point::new(cx, hero_y),
                color: Color::from_rgba(0.97, 0.98, 1.0, 0.96 * alpha),
                size: iced::Pixels(26.0 * scale),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });

            let sub_y = hero_y + 30.0 * scale;
            frame.fill_text(Text {
                content: "MindSet OS 1.1  •  Preparing Your Deep Focus Workspace".to_string(),
                position: Point::new(cx, sub_y),
                color: Color::from_rgba(0.70, 0.77, 0.90, 0.85 * alpha),
                size: iced::Pixels(13.0 * scale),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });

            // 6. Sleek Thin Progress Bar
            let bar_w = 240.0 * scale;
            let bar_h = 4.0 * scale;
            let bar_x = cx - bar_w * 0.5;
            let bar_y = sub_y + 28.0 * scale;

            frame.fill(
                &Path::rounded_rectangle(
                    Point::new(bar_x, bar_y),
                    Size::new(bar_w, bar_h),
                    Radius::from(2.0 * scale),
                ),
                Color::from_rgba(1.0, 1.0, 1.0, 0.10 * alpha),
            );
            frame.fill(
                &Path::rounded_rectangle(
                    Point::new(bar_x, bar_y),
                    Size::new((bar_w * progress).max(6.0), bar_h),
                    Radius::from(2.0 * scale),
                ),
                Color::from_rgba(0.18, 0.82, 0.88, 0.95 * alpha),
            );

            let hint_alpha = (0.35 + 0.20 * (t * 4.0).sin()) * alpha;
            frame.fill_text(Text {
                content: "Click anywhere or press Enter to continue".to_string(),
                position: Point::new(cx, bar_y + 28.0 * scale),
                color: Color::from_rgba(0.85, 0.90, 0.98, hint_alpha.clamp(0.0, 1.0)),
                size: iced::Pixels(11.0 * scale),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });
        });

        vec![geom]
    }
}

pub fn view(state: &BootSplashState) -> Element<'_, Message> {
    canvas::Canvas::new(BootSplashProgram { state })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
