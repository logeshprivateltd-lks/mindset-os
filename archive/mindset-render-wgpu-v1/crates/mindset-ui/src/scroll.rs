//! Nested scroll containers, hardware scissor clipping, and high-performance VirtualList.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::layout::LayoutRect;
use std::ops::Range;

/// Nested scrollable container with draggable scrollbar and scissor bounds.
#[derive(Debug, Clone)]
pub struct ScrollContainer {
    pub viewport: LayoutRect,
    pub content_height: f32,
    pub scroll_y: f32,
    pub target_scroll_y: f32,
    pub is_dragging_thumb: bool,
    pub drag_start_y: f32,
    pub drag_start_scroll: f32,
}

impl ScrollContainer {
    pub fn new(viewport: LayoutRect, content_height: f32) -> Self {
        Self {
            viewport,
            content_height,
            scroll_y: 0.0,
            target_scroll_y: 0.0,
            is_dragging_thumb: false,
            drag_start_y: 0.0,
            drag_start_scroll: 0.0,
        }
    }

    pub fn max_scroll(&self) -> f32 {
        (self.content_height - self.viewport.h).max(0.0)
    }

    pub fn scroll_by(&mut self, delta: f32) {
        self.target_scroll_y = (self.target_scroll_y + delta).clamp(0.0, self.max_scroll());
        self.scroll_y = self.target_scroll_y;
    }

    pub fn set_scroll(&mut self, y: f32) {
        self.target_scroll_y = y.clamp(0.0, self.max_scroll());
        self.scroll_y = self.target_scroll_y;
    }

    /// Update animation tick
    pub fn update(&mut self, dt: f32) {
        let diff = self.target_scroll_y - self.scroll_y;
        if diff.abs() > 0.5 {
            self.scroll_y += diff * (dt * 18.0).min(1.0);
        } else {
            self.scroll_y = self.target_scroll_y;
        }
    }

    /// Hardware scissor clip rect (in physical screen pixels)
    pub fn scissor_clip_rect(&self) -> LayoutRect {
        self.viewport
    }

    /// Right margin scrollbar track rect
    pub fn track_rect(&self) -> LayoutRect {
        let w = 6.0;
        LayoutRect::new(
            self.viewport.x + self.viewport.w - w - 2.0,
            self.viewport.y + 2.0,
            w,
            (self.viewport.h - 4.0).max(0.0),
        )
    }

    /// Draggable scrollbar thumb rect
    pub fn thumb_rect(&self) -> Option<LayoutRect> {
        let max_s = self.max_scroll();
        if max_s <= 0.0 {
            return None;
        }
        let track = self.track_rect();
        let view_ratio = (self.viewport.h / self.content_height).clamp(0.05, 1.0);
        let thumb_h = (track.h * view_ratio).clamp(24.0, track.h);
        let available_track = track.h - thumb_h;
        let scroll_fraction = (self.scroll_y / max_s).clamp(0.0, 1.0);
        let thumb_y = track.y + scroll_fraction * available_track;
        Some(LayoutRect::new(track.x, thumb_y, track.w, thumb_h))
    }

    pub fn handle_mouse_down(&mut self, px: f32, py: f32) -> bool {
        if let Some(thumb) = self.thumb_rect() {
            if thumb.contains(px, py) {
                self.is_dragging_thumb = true;
                self.drag_start_y = py;
                self.drag_start_scroll = self.scroll_y;
                return true;
            }
        }
        let track = self.track_rect();
        if track.contains(px, py) {
            // Jump to position
            let thumb_h = self.thumb_rect().map_or(24.0, |t| t.h);
            let available_track = (track.h - thumb_h).max(1.0);
            let click_rel = (py - track.y - thumb_h * 0.5).clamp(0.0, available_track);
            let fraction = click_rel / available_track;
            self.set_scroll(fraction * self.max_scroll());
            self.is_dragging_thumb = true;
            self.drag_start_y = py;
            self.drag_start_scroll = self.scroll_y;
            return true;
        }
        false
    }

    pub fn handle_mouse_move(&mut self, py: f32) -> bool {
        if self.is_dragging_thumb {
            let track = self.track_rect();
            let thumb_h = self.thumb_rect().map_or(24.0, |t| t.h);
            let available_track = (track.h - thumb_h).max(1.0);
            let dy = py - self.drag_start_y;
            let scroll_delta = (dy / available_track) * self.max_scroll();
            self.set_scroll(self.drag_start_scroll + scroll_delta);
            return true;
        }
        false
    }

    pub fn handle_mouse_up(&mut self) {
        self.is_dragging_thumb = false;
    }
}

/// Virtualized list supporting 10,000+ items with instant viewport calculation.
#[derive(Debug, Clone)]
pub struct VirtualList {
    pub total_items: usize,
    pub item_height: f32,
    pub viewport_height: f32,
    pub scroll_offset: f32,
    pub buffer_items: usize,
}

impl VirtualList {
    pub fn new(total_items: usize, item_height: f32, viewport_height: f32) -> Self {
        Self {
            total_items,
            item_height,
            viewport_height,
            scroll_offset: 0.0,
            buffer_items: 4,
        }
    }

    pub fn total_content_height(&self) -> f32 {
        self.total_items as f32 * self.item_height
    }

    /// Calculates the range of indices currently visible inside the viewport (plus buffer).
    pub fn visible_range(&self) -> Range<usize> {
        if self.total_items == 0 || self.item_height <= 0.0 {
            return 0..0;
        }

        let first = (self.scroll_offset / self.item_height).floor() as usize;
        let start = first.saturating_sub(self.buffer_items);

        let visible_count = (self.viewport_height / self.item_height).ceil() as usize;
        let end = (first + visible_count + self.buffer_items).min(self.total_items);

        start..end
    }

    /// Returns the exact Y position for item at `index` relative to viewport top.
    pub fn item_y(&self, index: usize) -> f32 {
        index as f32 * self.item_height - self.scroll_offset
    }

    /// Layout rect for item at `index` given viewport bounds.
    pub fn item_rect(&self, index: usize, viewport: LayoutRect) -> LayoutRect {
        LayoutRect::new(
            viewport.x,
            viewport.y + self.item_y(index),
            viewport.w,
            self.item_height,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_list_10000_items_performance_and_range() {
        let start_time = std::time::Instant::now();
        let vlist = VirtualList::new(10_000, 40.0, 600.0);
        let range = vlist.visible_range();

        assert_eq!(range.start, 0);
        assert!(range.end <= 25);
        assert_eq!(vlist.total_content_height(), 400_000.0);

        // Scroll to middle
        let mut scrolled = vlist.clone();
        scrolled.scroll_offset = 200_000.0; // Index 5000
        let mid_range = scrolled.visible_range();
        assert!(mid_range.start >= 4990);
        assert!(mid_range.end <= 5030);

        let elapsed = start_time.elapsed();
        assert!(
            elapsed.as_millis() < 4,
            "VirtualList visible_range took too long: {:?}",
            elapsed
        );
    }

    #[test]
    fn test_scroll_container_clamping_and_thumb_calculation() {
        let mut container = ScrollContainer::new(LayoutRect::new(0.0, 0.0, 500.0, 400.0), 1200.0);
        assert_eq!(container.max_scroll(), 800.0);

        container.scroll_by(200.0);
        assert_eq!(container.scroll_y, 200.0);

        let thumb = container.thumb_rect();
        assert!(thumb.is_some());
        let t = thumb.unwrap();
        assert!(t.h >= 24.0);
        assert!(t.w == 6.0);

        // Scroll past max
        container.scroll_by(5000.0);
        assert_eq!(container.scroll_y, 800.0);
    }
}
