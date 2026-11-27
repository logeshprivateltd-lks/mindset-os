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
    Lock,
    Unlock,
    Key,
    Shield,
    Eye,
    EyeOff,
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
            MsIconKind::Lock => {
                let body = Path::rounded_rectangle(
                    Point::new(w * 0.22, h * 0.44),
                    Size::new(w * 0.56, h * 0.42),
                    3.0.into(),
                );
                frame.stroke(&body, stroke);
                let mut shackle = canvas::path::Builder::new();
                shackle.move_to(Point::new(w * 0.33, h * 0.44));
                shackle.line_to(Point::new(w * 0.33, h * 0.28));
                shackle.line_to(Point::new(w * 0.38, h * 0.20));
                shackle.line_to(Point::new(w * 0.62, h * 0.20));
                shackle.line_to(Point::new(w * 0.67, h * 0.28));
                shackle.line_to(Point::new(w * 0.67, h * 0.44));
                frame.stroke(&shackle.build(), stroke);
                let dot = Path::circle(Point::new(w * 0.50, h * 0.64), w * 0.06);
                frame.fill(&dot, self.color);
            }
            MsIconKind::Unlock => {
                let body = Path::rounded_rectangle(
                    Point::new(w * 0.22, h * 0.44),
                    Size::new(w * 0.56, h * 0.42),
                    3.0.into(),
                );
                frame.stroke(&body, stroke);
                let mut shackle = canvas::path::Builder::new();
                shackle.move_to(Point::new(w * 0.33, h * 0.44));
                shackle.line_to(Point::new(w * 0.33, h * 0.26));
                shackle.line_to(Point::new(w * 0.40, h * 0.18));
                shackle.line_to(Point::new(w * 0.62, h * 0.18));
                shackle.line_to(Point::new(w * 0.67, h * 0.26));
                frame.stroke(&shackle.build(), stroke);
            }
            MsIconKind::Key => {
                let head = Path::circle(Point::new(w * 0.32, h * 0.50), w * 0.18);
                frame.stroke(&head, stroke);
                let mut shaft = canvas::path::Builder::new();
                shaft.move_to(Point::new(w * 0.50, h * 0.50));
                shaft.line_to(Point::new(w * 0.86, h * 0.50));
                shaft.line_to(Point::new(w * 0.86, h * 0.66));
                shaft.move_to(Point::new(w * 0.70, h * 0.50));
                shaft.line_to(Point::new(w * 0.70, h * 0.62));
                frame.stroke(&shaft.build(), stroke);
            }
            MsIconKind::Shield => {
                let mut b = canvas::path::Builder::new();
                b.move_to(Point::new(w * 0.50, h * 0.14));
                b.line_to(Point::new(w * 0.82, h * 0.26));
                b.line_to(Point::new(w * 0.80, h * 0.58));
                b.line_to(Point::new(w * 0.50, h * 0.86));
                b.line_to(Point::new(w * 0.20, h * 0.58));
                b.line_to(Point::new(w * 0.18, h * 0.26));
                b.close();
                frame.stroke(&b.build(), stroke);
                let mut chk = canvas::path::Builder::new();
                chk.move_to(Point::new(w * 0.36, h * 0.50));
                chk.line_to(Point::new(w * 0.46, h * 0.62));
                chk.line_to(Point::new(w * 0.66, h * 0.38));
                frame.stroke(&chk.build(), stroke);
            }
            MsIconKind::Eye => {
                let mut b = canvas::path::Builder::new();
                b.move_to(Point::new(w * 0.12, h * 0.50));
                b.quadratic_curve_to(
                    Point::new(w * 0.50, h * 0.18),
                    Point::new(w * 0.88, h * 0.50),
                );
                b.quadratic_curve_to(
                    Point::new(w * 0.50, h * 0.82),
                    Point::new(w * 0.12, h * 0.50),
                );
                frame.stroke(&b.build(), stroke);
                let pupil = Path::circle(Point::new(w * 0.50, h * 0.50), w * 0.11);
                frame.fill(&pupil, self.color);
            }
            MsIconKind::EyeOff => {
                let mut b = canvas::path::Builder::new();
                b.move_to(Point::new(w * 0.14, h * 0.50));
                b.quadratic_curve_to(
                    Point::new(w * 0.50, h * 0.22),
                    Point::new(w * 0.86, h * 0.50),
                );
                b.move_to(Point::new(w * 0.18, h * 0.82));
                b.line_to(Point::new(w * 0.82, h * 0.18));
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
