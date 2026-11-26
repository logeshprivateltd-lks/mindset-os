//! Pure-Rust Vector Canvas Icons for MindSet OS
//!
//! Eliminates Linux emoji tofu squares by drawing crisp HiDPI vector shapes.

use iced::mouse;
use iced::widget::canvas::{self, Frame, Geometry, Path, Program, Stroke};
use iced::{Color, Element, Point, Rectangle, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsIconKind {
    Search,
    Settings,
    Wifi,
    Power,
}

#[derive(Debug, Clone, Copy)]
pub struct MsVectorIcon {
    pub kind: MsIconKind,
    pub color: Color,
}

impl<Message> Program<Message, iced::Theme, iced::Renderer> for MsVectorIcon {
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
        let stroke = Stroke::default().with_color(self.color).with_width(1.6);

        match self.kind {
            MsIconKind::Search => {
                let circle = Path::circle(Point::new(w * 0.42, h * 0.42), w * 0.26);
                frame.stroke(&circle, stroke);
                let mut b = canvas::path::Builder::new();
                b.move_to(Point::new(w * 0.62, h * 0.62));
                b.line_to(Point::new(w * 0.86, h * 0.86));
                frame.stroke(&b.build(), stroke);
            }
            MsIconKind::Settings => {
                let outer = Path::circle(Point::new(w * 0.5, h * 0.5), w * 0.30);
                let inner = Path::circle(Point::new(w * 0.5, h * 0.5), w * 0.12);
                frame.stroke(&outer, stroke);
                frame.stroke(&inner, stroke);
            }
            MsIconKind::Wifi => {
                let dot = Path::circle(Point::new(w * 0.5, h * 0.78), w * 0.07);
                frame.fill(&dot, self.color);
                let arc = Path::rounded_rectangle(
                    Point::new(w * 0.22, h * 0.34),
                    Size::new(w * 0.56, h * 0.30),
                    4.0.into(),
                );
                frame.stroke(&arc, stroke);
            }
            MsIconKind::Power => {
                let ring = Path::circle(Point::new(w * 0.5, h * 0.55), w * 0.30);
                frame.stroke(&ring, stroke);
                let mut b = canvas::path::Builder::new();
                b.move_to(Point::new(w * 0.5, h * 0.16));
                b.line_to(Point::new(w * 0.5, h * 0.50));
                frame.stroke(&b.build(), stroke);
            }
        }
        vec![frame.into_geometry()]
    }
}

pub fn vector_icon<'a, M: 'a>(kind: MsIconKind, color: Color, size: f32) -> Element<'a, M> {
    iced::widget::canvas(MsVectorIcon { kind, color })
        .width(size)
        .height(size)
        .into()
}
