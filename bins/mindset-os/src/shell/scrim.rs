//! Visual Brightness & Night Light Click-Through Scrim Overlay
//!
//! Provides a hardware/software presentation overlay for Windows 11 & VirtualBox
//! that dims the display and applies warm night-light tinting.
//!
//! Enforces 100% click-through transparency (ignores all mouse/keyboard events),
//! allowing seamless interaction with underlying windows, dock, and desktop.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer::{Quad, Renderer};
use iced::advanced::widget::tree::Tree;
use iced::advanced::widget::Widget;
use iced::mouse;
use iced::{Border, Color, Element, Length, Rectangle, Shadow, Size};

pub struct ScrimOverlay {
    pub brightness: f32, // 0.25 to 1.0
    pub night_light: bool,
}

impl ScrimOverlay {
    pub fn new(brightness: f32, night_light: bool) -> Self {
        Self {
            brightness,
            night_light,
        }
    }
}

impl<Message, Theme, R: Renderer> Widget<Message, Theme, R> for ScrimOverlay {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &R, limits: &Limits) -> Node {
        Node::new(limits.max())
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut R,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        // 1. Black Dimming Quad
        let dim_alpha = (1.0 - self.brightness.clamp(0.20, 1.0)).clamp(0.0, 0.75);
        if dim_alpha > 0.01 {
            let dim_quad = Quad {
                bounds,
                border: Border::default(),
                shadow: Shadow::default(),
                snap: false,
            };
            renderer.fill_quad(
                dim_quad,
                iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, dim_alpha)),
            );
        }

        // 2. Warm Night Light Quad
        if self.night_light {
            let warm_quad = Quad {
                bounds,
                border: Border::default(),
                shadow: Shadow::default(),
                snap: false,
            };
            renderer.fill_quad(
                warm_quad,
                iced::Background::Color(Color::from_rgba(1.0, 0.58, 0.18, 0.14)),
            );
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &R,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }
}

impl<'a, Message: 'a, Theme: 'a, R: Renderer + 'a> From<ScrimOverlay>
    for Element<'a, Message, Theme, R>
{
    fn from(scrim: ScrimOverlay) -> Self {
        Element::new(scrim)
    }
}

pub fn scrim_overlay<'a, Message: 'a, Theme: 'a, R: Renderer + 'a>(
    brightness: f32,
    night_light: bool,
) -> Element<'a, Message, Theme, R> {
    ScrimOverlay::new(brightness, night_light).into()
}
