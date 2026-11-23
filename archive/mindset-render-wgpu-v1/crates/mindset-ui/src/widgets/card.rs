//! Surface container card with optional accent header.

use crate::layout::{LayoutRect, CARD_RADIUS, INNER_GAP};
use crate::theme::Theme;

pub struct Card {
    pub rect: LayoutRect,
    pub title: &'static str,
    pub has_accent_header: bool,
}

impl Card {
    pub fn new(rect: LayoutRect, title: &'static str) -> Self {
        Self {
            rect,
            title,
            has_accent_header: false,
        }
    }

    pub fn with_accent_header(mut self) -> Self {
        self.has_accent_header = true;
        self
    }

    /// Inner content rect (inset by standard padding; below header if present).
    pub fn content_rect(&self) -> LayoutRect {
        let top_offset = if self.has_accent_header { 36.0 } else { 0.0 };
        LayoutRect::new(
            self.rect.x + INNER_GAP,
            self.rect.y + top_offset + INNER_GAP,
            (self.rect.w - INNER_GAP * 2.0).max(0.0),
            (self.rect.h - top_offset - INNER_GAP * 2.0).max(0.0),
        )
    }

    /// Top 36px accent header strip (only meaningful when `has_accent_header` is true).
    pub fn header_rect(&self) -> LayoutRect {
        LayoutRect::new(self.rect.x, self.rect.y, self.rect.w, 36.0)
    }

    /// Card surface color from the current theme palette.
    pub fn surface_color(&self, theme: &Theme) -> [f32; 4] {
        theme.palette.surface
    }

    /// Card corner radius constant.
    pub fn radius() -> f32 {
        CARD_RADIUS
    }

    /// Border color from theme.
    pub fn border_color(&self, theme: &Theme) -> [f32; 4] {
        theme.palette.border
    }
}
