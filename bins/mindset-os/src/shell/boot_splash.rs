//! Liquidmorphism "HELLO ACHIEVER" Boot Sequence for MindSet OS
//!
//! Renders an ultra-smooth, full-screen Liquidmorphic intro canvas featuring
//! morphing translucent glass blobs and auroras driven by spring physics and
//! sine phase waves. Runs for ~2.8 seconds, then transitions cleanly to first-boot
//! onboarding or the desktop, with strictly 0% idle CPU once completed.

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
            scale_spring: Spring1D::with_preset(0.85, SpringPreset::FluidPanel),
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

        // Target springs: scale to 1.0, opacity to 1.0
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
        let geom = self.state.cache.draw(renderer, bounds.size(), |frame| {
            let w = bounds.width;
            let h = bounds.height;
            let cx = w * 0.5;
            let cy = h * 0.5;
            let t = self.state.elapsed;
            let progress = (t / self.state.duration_secs).clamp(0.0, 1.0);
            let alpha = self.state.opacity_spring.position.clamp(0.0, 1.0);
            let scale = self.state.scale_spring.position.clamp(0.5, 1.25);

            // 1. Deep Futuristic Liquid Canvas Background (#0B0D14)
            let bg_rect = Path::rectangle(Point::ORIGIN, Size::new(w, h));
            frame.fill(&bg_rect, Color::from_rgb(0.043, 0.051, 0.078));

            // 2. Liquidmorphism Morphing Glass Auroras (Sine Phase Waves)
            // 5 organic fluid blobs drifting and breathing
            let auroras = [
                // (base_x, base_y, radius, speed_x, speed_y, phase_x, phase_y, r, g, b, base_alpha)
                (
                    0.32, 0.38, 190.0, 1.1, 0.85, 0.0, 1.2, 0.20, 0.75, 0.95, 0.22,
                ), // Cyan Aqua
                (
                    0.68, 0.42, 220.0, 0.9, 1.25, 1.8, 0.5, 0.56, 0.48, 1.00, 0.26,
                ), // Lavender Indigo
                (
                    0.50, 0.68, 180.0, 1.3, 0.75, 3.1, 2.4, 1.00, 0.42, 0.60, 0.18,
                ), // Coral Rose
                (
                    0.22, 0.65, 160.0, 0.8, 1.10, 2.2, 3.8, 0.18, 0.88, 0.68, 0.16,
                ), // Emerald Mint
                (
                    0.76, 0.26, 170.0, 1.0, 0.90, 4.0, 1.7, 0.42, 0.36, 0.96, 0.20,
                ), // Electric Violet
            ];

            for (bx, by, rad, sx, sy, px, py, red, green, blue, base_a) in auroras {
                let wave_x = (t * sx + px).sin() * 75.0 + (t * 0.5 * sx).cos() * 30.0;
                let wave_y = (t * sy + py).cos() * 65.0 + (t * 0.4 * sy).sin() * 25.0;
                let wave_r = rad + (t * 2.2 + px).sin() * 32.0;

                let blob_cx = w * bx + wave_x;
                let blob_cy = h * by + wave_y;

                // Layered radial diffusion paths
                for layer in 0..3 {
                    let layer_factor = 1.0 + (layer as f32) * 0.45;
                    let layer_alpha = base_a / ((layer + 1) as f32 * 1.35) * alpha;
                    let circle = Path::circle(
                        Point::new(blob_cx, blob_cy),
                        (wave_r * layer_factor).max(10.0),
                    );
                    frame.fill(
                        &circle,
                        Color::from_rgba(red, green, blue, layer_alpha.clamp(0.0, 1.0)),
                    );
                }
            }

            // 3. Central Liquidmorphic Frosted Glass Plate
            let card_w = 600.0 * scale;
            let card_h = 340.0 * scale;
            let card_x = cx - card_w * 0.5;
            let card_y = cy - card_h * 0.5;

            // Outer soft atmospheric glow
            let glow_rect = Path::rounded_rectangle(
                Point::new(card_x - 16.0, card_y - 16.0),
                Size::new(card_w + 32.0, card_h + 32.0),
                Radius::from(36.0 * scale),
            );
            frame.fill(
                &glow_rect,
                Color::from_rgba(0.357, 0.322, 0.941, 0.12 * alpha),
            );

            // Frosted translucent plate
            let plate_rect = Path::rounded_rectangle(
                Point::new(card_x, card_y),
                Size::new(card_w, card_h),
                Radius::from(28.0 * scale),
            );
            frame.fill(
                &plate_rect,
                Color::from_rgba(0.082, 0.098, 0.137, 0.88 * alpha),
            );

            // Specular perimeter highlight rim
            frame.stroke(
                &plate_rect,
                Stroke::default()
                    .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.22 * alpha))
                    .with_width(1.5),
            );

            // 4. Sleek "MS" Monogram Orb
            let orb_radius = 38.0 * scale;
            let orb_cy = card_y + 70.0 * scale;
            let orb_center = Point::new(cx, orb_cy);

            // Monogram outer pulse ring
            let pulse_r = orb_radius + 6.0 + (t * 4.0).sin() * 3.5;
            let pulse_ring = Path::circle(orb_center, pulse_r);
            frame.stroke(
                &pulse_ring,
                Stroke::default()
                    .with_color(Color::from_rgba(0.56, 0.48, 1.0, 0.45 * alpha))
                    .with_width(1.5),
            );

            // Monogram core orb
            let orb_path = Path::circle(orb_center, orb_radius);
            frame.fill(
                &orb_path,
                Color::from_rgba(0.357, 0.322, 0.941, 0.85 * alpha),
            );
            frame.stroke(
                &orb_path,
                Stroke::default()
                    .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.65 * alpha))
                    .with_width(2.0),
            );

            // "MS" Text in Monogram
            frame.fill_text(Text {
                content: "MS".to_string(),
                position: Point::new(cx, orb_cy),
                color: Color::from_rgba(1.0, 1.0, 1.0, alpha),
                size: iced::Pixels(24.0 * scale),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });

            // 5. Hero Typography: "HELLO ACHIEVER"
            let hero_y = card_y + 155.0 * scale;
            frame.fill_text(Text {
                content: "HELLO ACHIEVER".to_string(),
                position: Point::new(cx, hero_y),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.98 * alpha),
                size: iced::Pixels(30.0 * scale),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });

            // 6. Subtitle Typography: "Initializing MindSet OS • Your Focus World"
            let sub_y = card_y + 195.0 * scale;
            frame.fill_text(Text {
                content: "Initializing MindSet OS  •  Your Focus World".to_string(),
                position: Point::new(cx, sub_y),
                color: Color::from_rgba(0.72, 0.78, 0.92, 0.85 * alpha),
                size: iced::Pixels(14.0 * scale),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });

            // 7. Sleek Liquid Progress Bar
            let bar_w = 260.0 * scale;
            let bar_h = 5.0 * scale;
            let bar_x = cx - bar_w * 0.5;
            let bar_y = card_y + 245.0 * scale;

            // Track background
            let track_rect = Path::rounded_rectangle(
                Point::new(bar_x, bar_y),
                Size::new(bar_w, bar_h),
                Radius::from(2.5 * scale),
            );
            frame.fill(&track_rect, Color::from_rgba(1.0, 1.0, 1.0, 0.10 * alpha));

            // Active progress fill
            let fill_w = (bar_w * progress).max(4.0);
            let fill_rect = Path::rounded_rectangle(
                Point::new(bar_x, bar_y),
                Size::new(fill_w, bar_h),
                Radius::from(2.5 * scale),
            );
            frame.fill(&fill_rect, Color::from_rgba(0.56, 0.48, 1.0, 0.95 * alpha));

            // 8. Skip / Continue Hint (gentle pulse)
            let hint_alpha = (0.35 + 0.25 * (t * 5.0).sin()) * alpha;
            let hint_y = card_y + 285.0 * scale;
            frame.fill_text(Text {
                content: "Click anywhere or press Enter to launch".to_string(),
                position: Point::new(cx, hint_y),
                color: Color::from_rgba(1.0, 1.0, 1.0, hint_alpha.clamp(0.0, 1.0)),
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
