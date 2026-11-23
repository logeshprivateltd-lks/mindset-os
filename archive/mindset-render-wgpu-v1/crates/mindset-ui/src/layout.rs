//! Deterministic constraint, Flexbox, and Grid layout solver for MindSet OS UI.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::ElementId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Layout constants
pub const SIDEBAR_WIDTH: f32 = 220.0;
pub const HEADER_HEIGHT: f32 = 48.0;
pub const OUTER_PAD: f32 = 24.0;
pub const INNER_GAP: f32 = 16.0;
pub const CARD_RADIUS: f32 = 12.0;

/// Axis-aligned bounding rectangle for all layout operations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct LayoutRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl LayoutRect {
    #[inline]
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Returns true if the point (px, py) lies inside this rect.
    #[inline]
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }

    /// Shrink all four sides inward by `pad` pixels.
    #[inline]
    pub fn inset(&self, pad: f32) -> Self {
        Self {
            x: self.x + pad,
            y: self.y + pad,
            w: (self.w - pad * 2.0).max(0.0),
            h: (self.h - pad * 2.0).max(0.0),
        }
    }

    /// Inset with distinct horizontal and vertical padding.
    #[inline]
    pub fn inset_xy(&self, pad_x: f32, pad_y: f32) -> Self {
        Self {
            x: self.x + pad_x,
            y: self.y + pad_y,
            w: (self.w - pad_x * 2.0).max(0.0),
            h: (self.h - pad_y * 2.0).max(0.0),
        }
    }

    /// Split horizontally: returns (left rect of `width`, remaining right rect).
    #[inline]
    pub fn split_left(&self, width: f32) -> (LayoutRect, LayoutRect) {
        let w = width.min(self.w);
        let left = Self::new(self.x, self.y, w, self.h);
        let right = Self::new(self.x + w, self.y, (self.w - w).max(0.0), self.h);
        (left, right)
    }

    /// Split vertically: returns (top rect of `height`, remaining bottom rect).
    #[inline]
    pub fn split_top(&self, height: f32) -> (LayoutRect, LayoutRect) {
        let h = height.min(self.h);
        let top = Self::new(self.x, self.y, self.w, h);
        let bottom = Self::new(self.x, self.y + h, self.w, (self.h - h).max(0.0));
        (top, bottom)
    }

    /// Returns the rect for child `index` of `count` items laid out in a row with `gap` between them.
    pub fn child_row(&self, index: usize, count: usize, gap: f32) -> LayoutRect {
        if count == 0 {
            return *self;
        }
        let total_gap = gap * (count as f32 - 1.0).max(0.0);
        let child_w = (self.w - total_gap) / count as f32;
        let x = self.x + index as f32 * (child_w + gap);
        Self::new(x, self.y, child_w.max(0.0), self.h)
    }

    /// Returns the rect for child `index` of `count` items laid out in a column with `gap` between them.
    pub fn child_column(&self, index: usize, count: usize, gap: f32) -> LayoutRect {
        if count == 0 {
            return *self;
        }
        let total_gap = gap * (count as f32 - 1.0).max(0.0);
        let child_h = (self.h - total_gap) / count as f32;
        let y = self.y + index as f32 * (child_h + gap);
        Self::new(self.x, y, self.w, child_h.max(0.0))
    }

    /// Returns the rect for cell (row, col) in a rows×cols grid with `gap` between cells.
    pub fn child_grid(
        &self,
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
        gap: f32,
    ) -> LayoutRect {
        let cols = cols.max(1);
        let rows = rows.max(1);
        let total_gap_x = gap * (cols as f32 - 1.0).max(0.0);
        let total_gap_y = gap * (rows as f32 - 1.0).max(0.0);
        let cell_w = (self.w - total_gap_x) / cols as f32;
        let cell_h = (self.h - total_gap_y) / rows as f32;
        let x = self.x + col as f32 * (cell_w + gap);
        let y = self.y + row as f32 * (cell_h + gap);
        Self::new(x, y, cell_w.max(0.0), cell_h.max(0.0))
    }

    /// Bottom rect of given height
    pub fn split_bottom(&self, height: f32) -> (LayoutRect, LayoutRect) {
        let h = height.min(self.h);
        let top = Self::new(self.x, self.y, self.w, (self.h - h).max(0.0));
        let bottom = Self::new(self.x, self.y + (self.h - h).max(0.0), self.w, h);
        (top, bottom)
    }

    /// Right rect of given width
    pub fn split_right(&self, width: f32) -> (LayoutRect, LayoutRect) {
        let w = width.min(self.w);
        let left = Self::new(self.x, self.y, (self.w - w).max(0.0), self.h);
        let right = Self::new(self.x + (self.w - w).max(0.0), self.y, w, self.h);
        (left, right)
    }

    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    #[inline]
    pub fn center_x(&self) -> f32 {
        self.x + self.w * 0.5
    }

    #[inline]
    pub fn center_y(&self) -> f32 {
        self.y + self.h * 0.5
    }

    #[inline]
    pub fn center(&self) -> (f32, f32) {
        (self.center_x(), self.center_y())
    }

    /// Returns the maximum characters per line that fit within this rect minus horizontal padding.
    #[inline]
    pub fn max_chars_padded(&self, pad_x: f32, avg_char_w: f32) -> usize {
        BoundedTextRules::card_max_chars(self.w, pad_x, avg_char_w)
    }

    /// Wraps text strictly within this rect minus horizontal padding.
    #[inline]
    pub fn wrap_text_padded(&self, text: &str, pad_x: f32, avg_char_w: f32) -> String {
        BoundedTextRules::wrap_card_text(text, *self, pad_x, avg_char_w)
    }

    /// Converts this LayoutRect into mindset_render::ScissorRect for GPU viewport clipping.
    #[inline]
    pub fn to_scissor_rect(&self) -> mindset_render::ScissorRect {
        mindset_render::ScissorRect {
            x: self.x.max(0.0).round() as u32,
            y: self.y.max(0.0).round() as u32,
            width: self.w.max(1.0).round() as u32,
            height: self.h.max(1.0).round() as u32,
        }
    }
}

/// Flexbox direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
}

/// Flexbox alignment on main axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum JustifyContent {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Flexbox alignment on cross axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AlignItems {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}

/// Sizing constraint for layout items.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum SizeConstraint {
    Px(f32),
    Percent(f32),
    #[default]
    Auto,
}

/// Responsive Flexbox container layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlexContainer {
    pub direction: FlexDirection,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub gap: f32,
    pub padding: f32,
}

impl Default for FlexContainer {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            justify: JustifyContent::Start,
            align: AlignItems::Start,
            gap: 8.0,
            padding: 0.0,
        }
    }
}

impl FlexContainer {
    pub fn row(gap: f32) -> Self {
        Self {
            direction: FlexDirection::Row,
            gap,
            ..Default::default()
        }
    }

    pub fn column(gap: f32) -> Self {
        Self {
            direction: FlexDirection::Column,
            gap,
            ..Default::default()
        }
    }

    /// Solves layout for a list of child sizes within container bounds.
    pub fn solve(&self, bounds: LayoutRect, item_sizes: &[(f32, f32)]) -> Vec<LayoutRect> {
        let inner = bounds.inset(self.padding);
        let n = item_sizes.len();
        if n == 0 {
            return Vec::new();
        }

        let mut rects = Vec::with_capacity(n);

        match self.direction {
            FlexDirection::Row => {
                let total_item_w: f32 = item_sizes.iter().map(|(w, _)| *w).sum();
                let total_gap = self.gap * (n as f32 - 1.0).max(0.0);
                let remaining_space = (inner.w - total_item_w - total_gap).max(0.0);

                let (mut curr_x, extra_gap) = match self.justify {
                    JustifyContent::Start => (inner.x, 0.0),
                    JustifyContent::Center => (inner.x + remaining_space * 0.5, 0.0),
                    JustifyContent::End => (inner.x + remaining_space, 0.0),
                    JustifyContent::SpaceBetween => {
                        let eg = if n > 1 {
                            remaining_space / (n as f32 - 1.0)
                        } else {
                            0.0
                        };
                        (inner.x, eg)
                    }
                    JustifyContent::SpaceAround => {
                        let eg = remaining_space / n as f32;
                        (inner.x + eg * 0.5, eg)
                    }
                    JustifyContent::SpaceEvenly => {
                        let eg = remaining_space / (n as f32 + 1.0);
                        (inner.x + eg, eg)
                    }
                };

                for &(w, h) in item_sizes {
                    let child_h = match self.align {
                        AlignItems::Stretch => inner.h,
                        _ => h.min(inner.h),
                    };
                    let child_y = match self.align {
                        AlignItems::Start => inner.y,
                        AlignItems::Center => inner.y + (inner.h - child_h) * 0.5,
                        AlignItems::End => inner.y + inner.h - child_h,
                        AlignItems::Stretch => inner.y,
                    };
                    rects.push(LayoutRect::new(curr_x, child_y, w, child_h));
                    curr_x += w + self.gap + extra_gap;
                }
            }
            FlexDirection::Column => {
                let total_item_h: f32 = item_sizes.iter().map(|(_, h)| *h).sum();
                let total_gap = self.gap * (n as f32 - 1.0).max(0.0);
                let remaining_space = (inner.h - total_item_h - total_gap).max(0.0);

                let (mut curr_y, extra_gap) = match self.justify {
                    JustifyContent::Start => (inner.y, 0.0),
                    JustifyContent::Center => (inner.y + remaining_space * 0.5, 0.0),
                    JustifyContent::End => (inner.y + remaining_space, 0.0),
                    JustifyContent::SpaceBetween => {
                        let eg = if n > 1 {
                            remaining_space / (n as f32 - 1.0)
                        } else {
                            0.0
                        };
                        (inner.y, eg)
                    }
                    JustifyContent::SpaceAround => {
                        let eg = remaining_space / n as f32;
                        (inner.y + eg * 0.5, eg)
                    }
                    JustifyContent::SpaceEvenly => {
                        let eg = remaining_space / (n as f32 + 1.0);
                        (inner.y + eg, eg)
                    }
                };

                for &(w, h) in item_sizes {
                    let child_w = match self.align {
                        AlignItems::Stretch => inner.w,
                        _ => w.min(inner.w),
                    };
                    let child_x = match self.align {
                        AlignItems::Start => inner.x,
                        AlignItems::Center => inner.x + (inner.w - child_w) * 0.5,
                        AlignItems::End => inner.x + inner.w - child_w,
                        AlignItems::Stretch => inner.x,
                    };
                    rects.push(LayoutRect::new(child_x, curr_y, child_w, h));
                    curr_y += h + self.gap + extra_gap;
                }
            }
        }

        rects
    }

    /// Solves flex layout taking into account flex_grow, flex_shrink, and min_size.
    /// Strictly guarantees Rule 2 collision prevention for shrinking text columns and action pills.
    pub fn solve_flex_items(&self, bounds: LayoutRect, items: &[FlexItem]) -> Vec<LayoutRect> {
        let inner = bounds.inset(self.padding);
        let n = items.len();
        if n == 0 {
            return Vec::new();
        }

        let mut rects = Vec::with_capacity(n);

        match self.direction {
            FlexDirection::Row => {
                let total_gap = self.gap * (n as f32 - 1.0).max(0.0);
                let sum_basis: f32 = items.iter().map(|item| item.basis).sum();
                let remaining = inner.w - sum_basis - total_gap;

                let mut resolved_w: Vec<f32> = items.iter().map(|item| item.basis).collect();

                if remaining > 0.0 {
                    let total_grow: f32 = items.iter().map(|item| item.flex_grow).sum();
                    if total_grow > 0.0 {
                        for (i, item) in items.iter().enumerate() {
                            if item.flex_grow > 0.0 {
                                let mut w = item.basis + remaining * (item.flex_grow / total_grow);
                                if let Some(max_w) = item.max_size {
                                    w = w.min(max_w);
                                }
                                resolved_w[i] = w;
                            }
                        }
                    }
                } else if remaining < 0.0 {
                    let deficit = -remaining;
                    let total_shrink_weight: f32 =
                        items.iter().map(|item| item.basis * item.flex_shrink).sum();

                    if total_shrink_weight > 0.0 {
                        for (i, item) in items.iter().enumerate() {
                            if item.flex_shrink > 0.0 {
                                let weight = item.basis * item.flex_shrink;
                                let shrink_amt = deficit * (weight / total_shrink_weight);
                                resolved_w[i] = (item.basis - shrink_amt).max(item.min_size);
                            }
                            // Action pills with flex_shrink == 0.0 do not shrink!
                        }
                    }
                }

                let total_resolved_w: f32 = resolved_w.iter().sum();
                let unused = (inner.w - total_resolved_w - total_gap).max(0.0);

                let (mut curr_x, extra_gap) = match self.justify {
                    JustifyContent::Start => (inner.x, 0.0),
                    JustifyContent::Center => (inner.x + unused * 0.5, 0.0),
                    JustifyContent::End => (inner.x + unused, 0.0),
                    JustifyContent::SpaceBetween => {
                        let eg = if n > 1 {
                            unused / (n as f32 - 1.0)
                        } else {
                            0.0
                        };
                        (inner.x, eg)
                    }
                    JustifyContent::SpaceAround => {
                        let eg = unused / n as f32;
                        (inner.x + eg * 0.5, eg)
                    }
                    JustifyContent::SpaceEvenly => {
                        let eg = unused / (n as f32 + 1.0);
                        (inner.x + eg, eg)
                    }
                };

                for (i, item) in items.iter().enumerate() {
                    let w = resolved_w[i];
                    let child_h = match self.align {
                        AlignItems::Stretch => inner.h,
                        _ => item.cross_size.min(inner.h),
                    };
                    let child_y = match self.align {
                        AlignItems::Start => inner.y,
                        AlignItems::Center => inner.y + (inner.h - child_h) * 0.5,
                        AlignItems::End => inner.y + inner.h - child_h,
                        AlignItems::Stretch => inner.y,
                    };
                    rects.push(LayoutRect::new(curr_x, child_y, w, child_h));
                    curr_x += w + self.gap + extra_gap;
                }
            }
            FlexDirection::Column => {
                let total_gap = self.gap * (n as f32 - 1.0).max(0.0);
                let sum_basis: f32 = items.iter().map(|item| item.basis).sum();
                let remaining = inner.h - sum_basis - total_gap;

                let mut resolved_h: Vec<f32> = items.iter().map(|item| item.basis).collect();

                if remaining > 0.0 {
                    let total_grow: f32 = items.iter().map(|item| item.flex_grow).sum();
                    if total_grow > 0.0 {
                        for (i, item) in items.iter().enumerate() {
                            if item.flex_grow > 0.0 {
                                let mut h = item.basis + remaining * (item.flex_grow / total_grow);
                                if let Some(max_h) = item.max_size {
                                    h = h.min(max_h);
                                }
                                resolved_h[i] = h;
                            }
                        }
                    }
                } else if remaining < 0.0 {
                    let deficit = -remaining;
                    let total_shrink_weight: f32 =
                        items.iter().map(|item| item.basis * item.flex_shrink).sum();

                    if total_shrink_weight > 0.0 {
                        for (i, item) in items.iter().enumerate() {
                            if item.flex_shrink > 0.0 {
                                let weight = item.basis * item.flex_shrink;
                                let shrink_amt = deficit * (weight / total_shrink_weight);
                                resolved_h[i] = (item.basis - shrink_amt).max(item.min_size);
                            }
                        }
                    }
                }

                let total_resolved_h: f32 = resolved_h.iter().sum();
                let unused = (inner.h - total_resolved_h - total_gap).max(0.0);

                let (mut curr_y, extra_gap) = match self.justify {
                    JustifyContent::Start => (inner.y, 0.0),
                    JustifyContent::Center => (inner.y + unused * 0.5, 0.0),
                    JustifyContent::End => (inner.y + unused, 0.0),
                    JustifyContent::SpaceBetween => {
                        let eg = if n > 1 {
                            unused / (n as f32 - 1.0)
                        } else {
                            0.0
                        };
                        (inner.y, eg)
                    }
                    JustifyContent::SpaceAround => {
                        let eg = unused / n as f32;
                        (inner.y + eg * 0.5, eg)
                    }
                    JustifyContent::SpaceEvenly => {
                        let eg = unused / (n as f32 + 1.0);
                        (inner.y + eg, eg)
                    }
                };

                for (i, item) in items.iter().enumerate() {
                    let h = resolved_h[i];
                    let child_w = match self.align {
                        AlignItems::Stretch => inner.w,
                        _ => item.cross_size.min(inner.w),
                    };
                    let child_x = match self.align {
                        AlignItems::Start => inner.x,
                        AlignItems::Center => inner.x + (inner.w - child_w) * 0.5,
                        AlignItems::End => inner.x + inner.w - child_w,
                        AlignItems::Stretch => inner.x,
                    };
                    rects.push(LayoutRect::new(child_x, curr_y, child_w, h));
                    curr_y += h + self.gap + extra_gap;
                }
            }
        }

        rects
    }

    /// Specialized row solver enforcing Rule 2 for 3-column / multi-column repair cards & rows:
    /// - Optional leading fixed icon (`flex_shrink: 0.0`)
    /// - Center shrinking text column (`flex_grow: 1.0`, `min_size: 0.0`, `flex_shrink: 1.0`)
    /// - One or more trailing action pills (`flex_shrink: 0.0`)
    pub fn solve_row_shrinking_text_with_pills(
        row_rect: LayoutRect,
        gap: f32,
        padding: f32,
        icon_size: Option<f32>,
        pill_widths: &[f32],
    ) -> (Option<LayoutRect>, LayoutRect, Vec<LayoutRect>) {
        let inner = row_rect.inset(padding);
        let mut curr_x = inner.x;

        // 1. Leading icon (flex_shrink: 0.0)
        let icon_rect = icon_size.map(|sz| {
            let r = LayoutRect::new(curr_x, inner.y + (inner.h - sz) * 0.5, sz, sz);
            curr_x += sz + gap;
            r
        });

        // 2. Trailing action pills (flex_shrink: 0.0)
        let total_pills_w: f32 = pill_widths.iter().sum();
        let total_pills_gaps = if pill_widths.is_empty() {
            0.0
        } else {
            gap * (pill_widths.len() - 1) as f32
        };
        let pills_start_x = inner.x + inner.w - total_pills_w - total_pills_gaps;

        let mut pill_rects = Vec::with_capacity(pill_widths.len());
        let mut px = pills_start_x;
        for &pw in pill_widths {
            pill_rects.push(LayoutRect::new(px, inner.y, pw, inner.h));
            px += pw + gap;
        }

        // 3. Center shrinking text column (min_size: 0.0, flex_grow: 1.0, flex_shrink: 1.0)
        let text_max_w = (pills_start_x - curr_x - gap).max(0.0);
        let text_rect = LayoutRect::new(curr_x, inner.y, text_max_w, inner.h);

        (icon_rect, text_rect, pill_rects)
    }
}

/// Sizing and shrink/grow constraints for flex children per Rule 2.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FlexItem {
    pub basis: f32,
    pub cross_size: f32,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub min_size: f32,
    pub max_size: Option<f32>,
}

impl FlexItem {
    pub fn fixed(width: f32, height: f32) -> Self {
        Self {
            basis: width,
            cross_size: height,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            min_size: width,
            max_size: Some(width),
        }
    }

    /// Shrinking text column with min_size: 0.0, flex_grow: 1.0, flex_shrink: 1.0 (Rule 2).
    pub fn shrinking_text(preferred_w: f32, height: f32) -> Self {
        Self {
            basis: preferred_w,
            cross_size: height,
            flex_grow: 1.0,
            flex_shrink: 1.0,
            min_size: 0.0,
            max_size: None,
        }
    }

    /// Action pill with flex_shrink: 0.0, flex_grow: 0.0 (Rule 2).
    pub fn action_pill(width: f32, height: f32) -> Self {
        Self {
            basis: width,
            cross_size: height,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            min_size: width,
            max_size: Some(width),
        }
    }
}

/// Responsive Grid container layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridContainer {
    pub cols: usize,
    pub row_height: f32,
    pub gap: f32,
    pub padding: f32,
}

impl GridContainer {
    pub fn new(cols: usize, row_height: f32, gap: f32) -> Self {
        Self {
            cols: cols.max(1),
            row_height,
            gap,
            padding: 0.0,
        }
    }

    pub fn solve(&self, bounds: LayoutRect, count: usize) -> Vec<LayoutRect> {
        let inner = bounds.inset(self.padding);
        let mut rects = Vec::with_capacity(count);
        let cols = self.cols.max(1);
        let total_gap_x = self.gap * (cols as f32 - 1.0).max(0.0);
        let cell_w = (inner.w - total_gap_x) / cols as f32;

        for i in 0..count {
            let row = i / cols;
            let col = i % cols;
            let x = inner.x + col as f32 * (cell_w + self.gap);
            let y = inner.y + row as f32 * (self.row_height + self.gap);
            rects.push(LayoutRect::new(x, y, cell_w, self.row_height));
        }

        rects
    }
}

/// Per-ElementId bounding box cache with hit/miss telemetry.
#[derive(Debug, Clone, Default)]
pub struct LayoutCache {
    cache: HashMap<ElementId, LayoutRect>,
    pub hits: usize,
    pub misses: usize,
}

impl LayoutCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&mut self, id: ElementId) -> Option<LayoutRect> {
        if let Some(rect) = self.cache.get(&id) {
            self.hits += 1;
            Some(*rect)
        } else {
            self.misses += 1;
            None
        }
    }

    pub fn insert(&mut self, id: ElementId, rect: LayoutRect) {
        self.cache.insert(id, rect);
    }

    pub fn invalidate(&mut self, id: ElementId) {
        self.cache.remove(&id);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn hit_ratio(&self) -> f32 {
        let total = self.hits + self.misses;
        if total == 0 {
            1.0
        } else {
            self.hits as f32 / total as f32
        }
    }
}

/// Screen/display DPI scaling and coordinate transformation context.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DpiScaleContext {
    /// Hardware monitor scale factor reported by winit (1.0 = 100%, 1.25 = 125%, 1.5 = 150%, 2.0 = 200%).
    pub dpi_scale: f32,
    /// User UI zoom preference multiplier (default 1.0, range 0.75..=2.0).
    pub ui_zoom: f32,
}

impl Default for DpiScaleContext {
    fn default() -> Self {
        Self {
            dpi_scale: 1.0,
            ui_zoom: 1.0,
        }
    }
}

impl DpiScaleContext {
    pub fn new(dpi_scale: f32, ui_zoom: f32) -> Self {
        Self {
            dpi_scale: dpi_scale.max(0.5),
            ui_zoom: ui_zoom.clamp(0.5, 3.0),
        }
    }

    /// Combined effective scale factor: dpi_scale * ui_zoom.
    #[inline]
    pub fn effective_scale(&self) -> f32 {
        self.dpi_scale * self.ui_zoom
    }

    /// Transforms logical points (pt) to physical framebuffer pixels (px).
    #[inline]
    pub fn to_physical(&self, pt: f32) -> f32 {
        pt * self.effective_scale()
    }

    /// Transforms physical framebuffer pixels (px) to logical points (pt).
    #[inline]
    pub fn to_logical(&self, px: f32) -> f32 {
        let s = self.effective_scale();
        if s > 0.0 {
            px / s
        } else {
            px
        }
    }

    /// Snaps a logical coordinate to the nearest physical integer pixel boundary,
    /// preventing blurry subpixel rendering of borders and divider lines.
    #[inline]
    pub fn snap_to_pixel(&self, pt: f32) -> f32 {
        let s = self.effective_scale();
        if s > 0.0 {
            (pt * s).round() / s
        } else {
            pt
        }
    }

    /// Snaps an entire LayoutRect to physical pixel boundaries.
    pub fn snap_rect(&self, rect: LayoutRect) -> LayoutRect {
        let s = self.effective_scale();
        if s <= 0.0 {
            return rect;
        }
        let x_px = (rect.x * s).round();
        let y_px = (rect.y * s).round();
        let right_px = ((rect.x + rect.w) * s).round();
        let bottom_px = ((rect.y + rect.h) * s).round();
        LayoutRect {
            x: x_px / s,
            y: y_px / s,
            w: ((right_px - x_px) / s).max(0.0),
            h: ((bottom_px - y_px) / s).max(0.0),
        }
    }

    /// Converts a logical LayoutRect to physical scissor / viewport bounds (u32 integers).
    pub fn to_physical_scissor(
        &self,
        rect: LayoutRect,
        screen_w: u32,
        screen_h: u32,
    ) -> (u32, u32, u32, u32) {
        let s = self.effective_scale();
        let x = (rect.x * s).round().max(0.0) as u32;
        let y = (rect.y * s).round().max(0.0) as u32;
        let w = (rect.w * s).round().max(0.0) as u32;
        let h = (rect.h * s).round().max(0.0) as u32;
        let clamped_w = w.min(screen_w.saturating_sub(x));
        let clamped_h = h.min(screen_h.saturating_sub(y));
        (x, y, clamped_w, clamped_h)
    }
}

/// Responsive layout breakpoints for adaptive window and shell sizing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ResponsiveBreakpoint {
    /// Window width < 640pt (phones, narrow split views): Sidebar collapses to overlay, compact headers.
    Compact,
    /// Window width 640pt..=1024pt (tablets, half-screen snap): Standard sidebar, medium padding.
    Medium,
    /// Window width > 1024pt (full desktops, ultrawide): Generous padding, expanded auxiliary panels.
    Expanded,
}

impl ResponsiveBreakpoint {
    pub fn from_width(width: f32) -> Self {
        if width < 640.0 {
            Self::Compact
        } else if width <= 1024.0 {
            Self::Medium
        } else {
            Self::Expanded
        }
    }
}

/// Standard 4-zone responsive window layout solver: Header, Sidebar, Main Content, Footer.
/// Eliminates hardcoded absolute coordinates across all MindSet OS applications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowShellLayout {
    pub header: LayoutRect,
    pub sidebar: Option<LayoutRect>,
    pub content: LayoutRect,
    pub footer: LayoutRect,
    pub breakpoint: ResponsiveBreakpoint,
}

impl WindowShellLayout {
    pub fn solve(
        window_bounds: LayoutRect,
        header_height: f32,
        footer_height: f32,
        sidebar_preferred_width: f32,
        sidebar_collapsed: bool,
    ) -> Self {
        let breakpoint = ResponsiveBreakpoint::from_width(window_bounds.w);
        let (header, below_header) = window_bounds.split_top(header_height);
        let (content_and_sidebar, footer) = below_header.split_bottom(footer_height);

        let (sidebar, content) = if sidebar_collapsed || breakpoint == ResponsiveBreakpoint::Compact
        {
            (None, content_and_sidebar)
        } else {
            let actual_sidebar_w = sidebar_preferred_width.min(content_and_sidebar.w * 0.45);
            let (sb, cnt) = content_and_sidebar.split_left(actual_sidebar_w);
            (Some(sb), cnt)
        };

        Self {
            header,
            sidebar,
            content,
            footer,
            breakpoint,
        }
    }
}

/// Text truncation and collision prevention rules.
#[derive(Debug, Clone)]
pub struct BoundedTextRules;

impl BoundedTextRules {
    /// Unicode ellipsis glyph (U+2026 '…') - single character instead of three dots.
    pub const ELLIPSIS: &'static str = "…";

    /// Bounded single-line text fitting algorithm with grapheme awareness.
    /// Clamps text to max_w pixels using character width estimation or shaped clusters,
    /// inserting Unicode ellipsis '…' if clipped.
    pub fn fit_single_line(text: &str, avg_char_w: f32, max_w: f32) -> String {
        if max_w <= 0.0 {
            return String::new();
        }
        let est_total_w = text.chars().count() as f32 * avg_char_w;
        if est_total_w <= max_w {
            return text.to_string();
        }

        let ellipsis_w = avg_char_w * 1.1; // Ellipsis width allowance
        let available_w = (max_w - ellipsis_w).max(0.0);
        let max_chars = (available_w / avg_char_w).floor() as usize;

        if max_chars == 0 {
            return Self::ELLIPSIS.to_string();
        }

        let truncated: String = text.chars().take(max_chars).collect();
        format!("{}{}", truncated.trim_end(), Self::ELLIPSIS)
    }

    /// Solves collision-free split row layout (e.g. setting label on left, value on right).
    /// Guarantees that the right text is given priority, and the left label is strictly bounded.
    pub fn solve_split_row(
        row_rect: LayoutRect,
        right_w: f32,
        min_gap: f32,
        padding: f32,
    ) -> (LayoutRect, LayoutRect) {
        let inner = row_rect.inset_xy(padding, 0.0);
        let right_rect = LayoutRect::new(
            inner.x + inner.w - right_w,
            inner.y,
            right_w.min(inner.w),
            inner.h,
        );
        let left_max_w = (inner.w - right_w - min_gap).max(0.0);
        let left_rect = LayoutRect::new(inner.x, inner.y, left_max_w, inner.h);
        (left_rect, right_rect)
    }

    /// Calculates the maximum number of characters that fit within a card or container of given width,
    /// strictly enforcing padding on both sides.
    #[inline]
    pub fn card_max_chars(card_width: f32, padding: f32, avg_char_w: f32) -> usize {
        let avail_w = (card_width - padding * 2.0).max(0.0);
        if avg_char_w <= 0.0 {
            0
        } else {
            (avail_w / avg_char_w).floor() as usize
        }
    }

    /// Wraps text based on the parent card's width minus padding, enforcing strict `wrap_text_chars` boundaries.
    /// Guarantees that text never renders outside its parent card's bounding box.
    pub fn wrap_card_text(
        text: &str,
        card_rect: LayoutRect,
        padding: f32,
        avg_char_w: f32,
    ) -> String {
        let max_chars = Self::card_max_chars(card_rect.w, padding, avg_char_w);
        mindset_render::wrap_text_chars(text, max_chars)
    }

    /// Wraps text to fit inside an arbitrary bounding rect with horizontal padding,
    /// enforcing strict character boundaries and returning lines.
    pub fn wrap_to_rect_lines(
        text: &str,
        rect: LayoutRect,
        pad_x: f32,
        avg_char_w: f32,
    ) -> Vec<String> {
        let max_chars = Self::card_max_chars(rect.w, pad_x, avg_char_w);
        let wrapped = mindset_render::wrap_text_chars(text, max_chars);
        wrapped.lines().map(|s| s.to_string()).collect()
    }

    /// Wraps and clamps lines to fit both horizontally and vertically inside a card or container rect.
    /// If lines exceed available vertical space (card height minus vertical padding),
    /// the last visible line is truncated with ellipsis to prevent vertical bleeding.
    pub fn fit_multiline_to_card(
        text: &str,
        card_rect: LayoutRect,
        pad_x: f32,
        pad_y: f32,
        avg_char_w: f32,
        line_height: f32,
    ) -> Vec<String> {
        let avail_h = (card_rect.h - pad_y * 2.0).max(0.0);
        let avail_w = (card_rect.w - pad_x * 2.0).max(0.0);
        if avail_w <= 0.0 || avail_h < line_height || line_height <= 0.0 {
            return Vec::new();
        }

        let max_lines = (avail_h / line_height).floor() as usize;
        let lines = Self::wrap_to_rect_lines(text, card_rect, pad_x, avg_char_w);

        if lines.len() <= max_lines {
            lines
        } else {
            let mut result: Vec<String> = lines.into_iter().take(max_lines).collect();
            if let Some(last) = result.last_mut() {
                let ellipsis = Self::ELLIPSIS;
                let ellipsis_w = avg_char_w * 1.1;
                let available_for_text = (avail_w - ellipsis_w).max(0.0);
                let max_chars = (available_for_text / avg_char_w).floor() as usize;
                let truncated: String = last.chars().take(max_chars).collect();
                *last = format!("{}{}", truncated.trim_end(), ellipsis);
            }
            result
        }
    }
}

/// Declarative layout style model mirroring Taffy flexbox and grid specifications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaffyStyleDescriptor {
    pub display: TaffyDisplay,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub gap: f32,
    pub padding: [f32; 4], // top, right, bottom, left
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
}

impl TaffyStyleDescriptor {
    /// Factory for a shrinking text column (Rule 2).
    pub fn shrinking_text() -> Self {
        Self {
            display: TaffyDisplay::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            flex_grow: 1.0,
            flex_shrink: 1.0,
            gap: 2.0,
            padding: [0.0; 4],
            min_width: Some(0.0), // min_size 0.0 prevents collision
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }

    /// Factory for a trailing non-shrinking action pill/button (Rule 2).
    pub fn action_pill(width: f32) -> Self {
        Self {
            display: TaffyDisplay::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 0.0,
            flex_shrink: 0.0, // never shrink below required pill width
            gap: 0.0,
            padding: [4.0, 12.0, 4.0, 12.0],
            min_width: Some(width),
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaffyDisplay {
    #[default]
    Flex,
    Grid,
    None,
}

// ─── Taffy Tree Constraint Architecture (Rule 1 & Rule 2) ─────────────────────

/// 2D Size container for Taffy layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

/// 2D Point coordinate for Taffy layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// 4-sided Rect container for padding / margins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Rect<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T> Rect<T> {
    pub const fn new(left: T, right: T, top: T, bottom: T) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

impl<T: Copy> Rect<T> {
    pub const fn all(val: T) -> Self {
        Self {
            left: val,
            right: val,
            top: val,
            bottom: val,
        }
    }
}

/// Layout dimension for width/height styles.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Dimension {
    Length(f32),
    Percent(f32),
    Auto,
}

#[inline]
pub const fn length(val: f32) -> Dimension {
    Dimension::Length(val)
}

#[inline]
pub const fn percent(val: f32) -> Dimension {
    Dimension::Percent(val)
}

#[inline]
pub const fn auto() -> Dimension {
    Dimension::Auto
}

/// Overflow handling per axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
    Clip,
}

/// Display mode for Taffy layout nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Display {
    #[default]
    Flex,
    Grid,
    None,
}

/// Available space passed to measurement and layout passes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AvailableSpace {
    Definite(f32),
    MinContent,
    MaxContent,
}

impl AvailableSpace {
    pub const fn definite(val: f32) -> Self {
        Self::Definite(val)
    }

    pub fn into_option(self) -> Option<f32> {
        match self {
            Self::Definite(v) => Some(v),
            _ => None,
        }
    }
}

/// Declarative style definition for a TaffyTree node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Style {
    pub display: Display,
    pub flex_direction: FlexDirection,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub gap: Size<f32>,
    pub padding: Rect<f32>,
    pub overflow: Point<Overflow>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: Some(JustifyContent::Start),
            align_items: Some(AlignItems::Start),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Dimension::Auto,
            size: Size::new(Dimension::Auto, Dimension::Auto),
            min_size: Size::new(Dimension::Auto, Dimension::Auto),
            max_size: Size::new(Dimension::Auto, Dimension::Auto),
            gap: Size::new(0.0, 0.0),
            padding: Rect::all(0.0),
            overflow: Point::new(Overflow::Visible, Overflow::Visible),
        }
    }
}

impl Style {
    /// Factory for a shrinking text column strictly adhering to Rule 2:
    /// `min_size: Size { width: length(0.0), height: auto() }` and
    /// `overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden }`,
    /// with `flex_grow: 1.0`, `flex_shrink: 1.0`.
    pub fn shrinking_text_column() -> Self {
        Self {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: Some(JustifyContent::Center),
            align_items: Some(AlignItems::Start),
            flex_grow: 1.0,
            flex_shrink: 1.0,
            min_size: Size::new(length(0.0), auto()),
            overflow: Point::new(Overflow::Hidden, Overflow::Hidden),
            ..Default::default()
        }
    }

    /// Factory for a trailing action pill/button strictly adhering to Rule 2:
    /// `flex_shrink: 0.0`, `flex_grow: 0.0`, fixed size.
    pub fn action_pill(width_px: f32, height_px: f32) -> Self {
        Self {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: Some(JustifyContent::Center),
            align_items: Some(AlignItems::Center),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            size: Size::new(length(width_px), length(height_px)),
            min_size: Size::new(length(width_px), length(height_px)),
            ..Default::default()
        }
    }

    /// Factory for a flex row container.
    pub fn flex_row(gap_px: f32) -> Self {
        Self {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            gap: Size::new(gap_px, 0.0),
            ..Default::default()
        }
    }

    /// Factory for a flex column container.
    pub fn flex_col(gap_px: f32) -> Self {
        Self {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            gap: Size::new(0.0, gap_px),
            ..Default::default()
        }
    }
}

/// Computed layout geometry output for a node.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Layout {
    pub order: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub content_size: Size<f32>,
}

impl Layout {
    pub fn to_rect(&self) -> LayoutRect {
        LayoutRect::new(
            self.location.x,
            self.location.y,
            self.size.width,
            self.size.height,
        )
    }
}

/// Identifier for a node in a TaffyTree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaffyError {
    InvalidNode(NodeId),
}

type NodeMeasureFn =
    Box<dyn Fn(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32> + Send + Sync>;

struct TaffyNode {
    style: Style,
    children: Vec<NodeId>,
    layout: Layout,
    measure: Option<NodeMeasureFn>,
}

/// Pure-Rust TaffyTree constraint layout solver implementing Flexbox and Grid.
/// Enforces Rule 1 (Zero hardcoded coordinates) and Rule 2 (Min-content collision prevention).
pub struct TaffyTree {
    nodes: Vec<TaffyNode>,
}

impl Default for TaffyTree {
    fn default() -> Self {
        Self::new()
    }
}

impl TaffyTree {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Creates a new leaf node with the specified style.
    pub fn new_leaf(&mut self, style: Style) -> Result<NodeId, TaffyError> {
        let id = NodeId(self.nodes.len());
        self.nodes.push(TaffyNode {
            style,
            children: Vec::new(),
            layout: Layout::default(),
            measure: None,
        });
        Ok(id)
    }

    /// Creates a new leaf node with a custom intrinsic measurement function (e.g. for text).
    pub fn new_leaf_with_measure(
        &mut self,
        style: Style,
        measure: impl Fn(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32> + Send + Sync + 'static,
    ) -> Result<NodeId, TaffyError> {
        let id = NodeId(self.nodes.len());
        self.nodes.push(TaffyNode {
            style,
            children: Vec::new(),
            layout: Layout::default(),
            measure: Some(Box::new(measure)),
        });
        Ok(id)
    }

    /// Creates a new container node with children.
    pub fn new_with_children(
        &mut self,
        style: Style,
        children: &[NodeId],
    ) -> Result<NodeId, TaffyError> {
        let id = NodeId(self.nodes.len());
        self.nodes.push(TaffyNode {
            style,
            children: children.to_vec(),
            layout: Layout::default(),
            measure: None,
        });
        Ok(id)
    }

    /// Appends a child to a parent node.
    pub fn add_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), TaffyError> {
        if parent.0 >= self.nodes.len() {
            return Err(TaffyError::InvalidNode(parent));
        }
        self.nodes[parent.0].children.push(child);
        Ok(())
    }

    /// Sets or updates the style of an existing node.
    pub fn set_style(&mut self, node: NodeId, style: Style) -> Result<(), TaffyError> {
        if node.0 >= self.nodes.len() {
            return Err(TaffyError::InvalidNode(node));
        }
        self.nodes[node.0].style = style;
        Ok(())
    }

    /// Retrieves the style of a node.
    pub fn style(&self, node: NodeId) -> Result<&Style, TaffyError> {
        self.nodes
            .get(node.0)
            .map(|n| &n.style)
            .ok_or(TaffyError::InvalidNode(node))
    }

    /// Retrieves the computed layout of a node.
    pub fn layout(&self, node: NodeId) -> Result<&Layout, TaffyError> {
        self.nodes
            .get(node.0)
            .map(|n| &n.layout)
            .ok_or(TaffyError::InvalidNode(node))
    }

    /// Computes layout for the entire tree starting at `root`.
    pub fn compute_layout(
        &mut self,
        root: NodeId,
        available_space: Size<AvailableSpace>,
    ) -> Result<(), TaffyError> {
        self.compute_node(root, available_space, Point::new(0.0, 0.0))
    }

    /// Computes layout with intrinsic measurement support.
    pub fn compute_layout_with_measure(
        &mut self,
        root: NodeId,
        available_space: Size<AvailableSpace>,
    ) -> Result<(), TaffyError> {
        self.compute_layout(root, available_space)
    }

    fn compute_node(
        &mut self,
        node_id: NodeId,
        available_space: Size<AvailableSpace>,
        origin: Point<f32>,
    ) -> Result<(), TaffyError> {
        if node_id.0 >= self.nodes.len() {
            return Err(TaffyError::InvalidNode(node_id));
        }

        let (style, children, has_measure) = {
            let n = &self.nodes[node_id.0];
            (n.style.clone(), n.children.clone(), n.measure.is_some())
        };

        if style.display == Display::None {
            self.nodes[node_id.0].layout = Layout {
                order: 0,
                size: Size::new(0.0, 0.0),
                location: origin,
                content_size: Size::new(0.0, 0.0),
            };
            return Ok(());
        }

        // 1. If leaf with custom intrinsic measure function (Rule 1 & Rule 5)
        if has_measure && children.is_empty() {
            let known = Size::new(
                match style.size.width {
                    Dimension::Length(w) => Some(w),
                    _ => None,
                },
                match style.size.height {
                    Dimension::Length(h) => Some(h),
                    _ => None,
                },
            );
            let measured = if let Some(ref m) = self.nodes[node_id.0].measure {
                m(known, available_space)
            } else {
                Size::new(0.0, 0.0)
            };
            self.nodes[node_id.0].layout = Layout {
                order: 0,
                size: measured,
                location: origin,
                content_size: measured,
            };
            return Ok(());
        }

        // 2. Determine container dimensions
        let parent_avail_w = available_space.width.into_option();
        let parent_avail_h = available_space.height.into_option();

        let w = match style.size.width {
            Dimension::Length(val) => val,
            Dimension::Percent(pct) => pct * parent_avail_w.unwrap_or(0.0),
            Dimension::Auto => match available_space.width {
                AvailableSpace::Definite(val) => val,
                _ => match style.min_size.width {
                    Dimension::Length(val) => val,
                    _ => 0.0,
                },
            },
        };

        let h = match style.size.height {
            Dimension::Length(val) => val,
            Dimension::Percent(pct) => pct * parent_avail_h.unwrap_or(0.0),
            Dimension::Auto => match available_space.height {
                AvailableSpace::Definite(val) => val,
                _ => match style.min_size.height {
                    Dimension::Length(val) => val,
                    _ => 0.0,
                },
            },
        };

        // If no children, size is resolved
        if children.is_empty() {
            let final_size = Size::new(w, h);
            self.nodes[node_id.0].layout = Layout {
                order: 0,
                size: final_size,
                location: origin,
                content_size: final_size,
            };
            return Ok(());
        }

        let pad_left = style.padding.left;
        let pad_top = style.padding.top;
        let pad_right = style.padding.right;
        let pad_bottom = style.padding.bottom;

        let inner_w = (w - pad_left - pad_right).max(0.0);
        let inner_h = (h - pad_top - pad_bottom).max(0.0);

        let n = children.len();
        match style.flex_direction {
            FlexDirection::Row => {
                let gap_x = style.gap.width;
                let total_gap = gap_x * (n as f32 - 1.0).max(0.0);

                let mut base_widths = Vec::with_capacity(n);
                let mut base_heights = Vec::with_capacity(n);

                for &child_id in &children {
                    let child_style = &self.nodes[child_id.0].style;
                    let child_measure = &self.nodes[child_id.0].measure;

                    let (bw, bh) = if let Some(ref m) = child_measure {
                        let sz = m(
                            Size::new(None, None),
                            Size::new(
                                AvailableSpace::MinContent,
                                AvailableSpace::Definite(inner_h),
                            ),
                        );
                        (sz.width, sz.height)
                    } else {
                        let cw = match child_style.size.width {
                            Dimension::Length(px) => px,
                            Dimension::Percent(pct) => pct * inner_w,
                            Dimension::Auto => match child_style.flex_basis {
                                Dimension::Length(px) => px,
                                _ => 0.0,
                            },
                        };
                        let ch = match child_style.size.height {
                            Dimension::Length(px) => px,
                            Dimension::Percent(pct) => pct * inner_h,
                            Dimension::Auto => inner_h,
                        };
                        (cw, ch)
                    };

                    let clamped_w = match child_style.min_size.width {
                        Dimension::Length(min_px) => bw.max(min_px),
                        _ => bw,
                    };
                    base_widths.push(clamped_w);
                    base_heights.push(bh);
                }

                let sum_base_w: f32 = base_widths.iter().sum();
                let remaining = inner_w - sum_base_w - total_gap;

                let mut final_widths = base_widths.clone();

                if remaining > 0.0 {
                    let total_grow: f32 = children
                        .iter()
                        .map(|&c| self.nodes[c.0].style.flex_grow)
                        .sum();
                    if total_grow > 0.0 {
                        for (i, &child_id) in children.iter().enumerate() {
                            let grow = self.nodes[child_id.0].style.flex_grow;
                            if grow > 0.0 {
                                final_widths[i] += remaining * (grow / total_grow);
                            }
                        }
                    }
                } else if remaining < 0.0 {
                    // Rule 2 Collision Prevention: Distribute deficit strictly to items with flex_shrink > 0.0
                    let deficit = -remaining;
                    let total_shrink_weight: f32 = children
                        .iter()
                        .enumerate()
                        .map(|(i, &c)| base_widths[i] * self.nodes[c.0].style.flex_shrink)
                        .sum();

                    if total_shrink_weight > 0.0 {
                        for (i, &child_id) in children.iter().enumerate() {
                            let child_style = &self.nodes[child_id.0].style;
                            if child_style.flex_shrink > 0.0 {
                                let weight = base_widths[i] * child_style.flex_shrink;
                                let shrink_amt = deficit * (weight / total_shrink_weight);
                                let mut new_w = base_widths[i] - shrink_amt;

                                // Rule 2: Clamp to min_size.width (0.0 for shrinking text columns)
                                let min_w = match child_style.min_size.width {
                                    Dimension::Length(min_px) => min_px,
                                    Dimension::Percent(pct) => pct * inner_w,
                                    Dimension::Auto => 0.0,
                                };
                                if new_w < min_w {
                                    new_w = min_w;
                                }
                                final_widths[i] = new_w;
                            }
                            // Action pills with flex_shrink == 0.0 retain 100% of base_widths[i]!
                        }
                    }
                }

                let total_final_w: f32 = final_widths.iter().sum();
                let unused_space = (inner_w - total_final_w - total_gap).max(0.0);
                let (mut curr_x, extra_gap) =
                    match style.justify_content.unwrap_or(JustifyContent::Start) {
                        JustifyContent::Start => (origin.x + pad_left, 0.0),
                        JustifyContent::Center => (origin.x + pad_left + unused_space * 0.5, 0.0),
                        JustifyContent::End => (origin.x + pad_left + unused_space, 0.0),
                        JustifyContent::SpaceBetween => {
                            let eg = if n > 1 {
                                unused_space / (n as f32 - 1.0)
                            } else {
                                0.0
                            };
                            (origin.x + pad_left, eg)
                        }
                        JustifyContent::SpaceAround => {
                            let eg = unused_space / n as f32;
                            (origin.x + pad_left + eg * 0.5, eg)
                        }
                        JustifyContent::SpaceEvenly => {
                            let eg = unused_space / (n as f32 + 1.0);
                            (origin.x + pad_left + eg, eg)
                        }
                    };

                for (i, &child_id) in children.iter().enumerate() {
                    let cw = final_widths[i];
                    let ch = match style.align_items.unwrap_or(AlignItems::Start) {
                        AlignItems::Stretch => inner_h,
                        _ => base_heights[i].min(inner_h),
                    };
                    let cy = match style.align_items.unwrap_or(AlignItems::Start) {
                        AlignItems::Start => origin.y + pad_top,
                        AlignItems::Center => origin.y + pad_top + (inner_h - ch) * 0.5,
                        AlignItems::End => origin.y + pad_top + inner_h - ch,
                        AlignItems::Stretch => origin.y + pad_top,
                    };

                    let child_origin = Point::new(curr_x, cy);
                    let child_avail =
                        Size::new(AvailableSpace::Definite(cw), AvailableSpace::Definite(ch));
                    self.compute_node(child_id, child_avail, child_origin)?;

                    self.nodes[child_id.0].layout.size = Size::new(cw, ch);
                    self.nodes[child_id.0].layout.location = child_origin;

                    curr_x += cw + gap_x + extra_gap;
                }
            }
            FlexDirection::Column => {
                let gap_y = style.gap.height;
                let total_gap = gap_y * (n as f32 - 1.0).max(0.0);

                let mut base_widths = Vec::with_capacity(n);
                let mut base_heights = Vec::with_capacity(n);

                for &child_id in &children {
                    let child_style = &self.nodes[child_id.0].style;
                    let child_measure = &self.nodes[child_id.0].measure;

                    let (bw, bh) = if let Some(ref m) = child_measure {
                        let sz = m(
                            Size::new(None, None),
                            Size::new(
                                AvailableSpace::Definite(inner_w),
                                AvailableSpace::MinContent,
                            ),
                        );
                        (sz.width, sz.height)
                    } else {
                        let cw = match child_style.size.width {
                            Dimension::Length(px) => px,
                            Dimension::Percent(pct) => pct * inner_w,
                            Dimension::Auto => inner_w,
                        };
                        let ch = match child_style.size.height {
                            Dimension::Length(px) => px,
                            Dimension::Percent(pct) => pct * inner_h,
                            Dimension::Auto => match child_style.flex_basis {
                                Dimension::Length(px) => px,
                                _ => 0.0,
                            },
                        };
                        (cw, ch)
                    };

                    let clamped_h = match child_style.min_size.height {
                        Dimension::Length(min_px) => bh.max(min_px),
                        _ => bh,
                    };
                    base_widths.push(bw);
                    base_heights.push(clamped_h);
                }

                let sum_base_h: f32 = base_heights.iter().sum();
                let remaining = inner_h - sum_base_h - total_gap;

                let mut final_heights = base_heights.clone();

                if remaining > 0.0 {
                    let total_grow: f32 = children
                        .iter()
                        .map(|&c| self.nodes[c.0].style.flex_grow)
                        .sum();
                    if total_grow > 0.0 {
                        for (i, &child_id) in children.iter().enumerate() {
                            let grow = self.nodes[child_id.0].style.flex_grow;
                            if grow > 0.0 {
                                final_heights[i] += remaining * (grow / total_grow);
                            }
                        }
                    }
                } else if remaining < 0.0 {
                    let deficit = -remaining;
                    let total_shrink_weight: f32 = children
                        .iter()
                        .enumerate()
                        .map(|(i, &c)| base_heights[i] * self.nodes[c.0].style.flex_shrink)
                        .sum();

                    if total_shrink_weight > 0.0 {
                        for (i, &child_id) in children.iter().enumerate() {
                            let child_style = &self.nodes[child_id.0].style;
                            if child_style.flex_shrink > 0.0 {
                                let weight = base_heights[i] * child_style.flex_shrink;
                                let shrink_amt = deficit * (weight / total_shrink_weight);
                                let mut new_h = base_heights[i] - shrink_amt;

                                let min_h = match child_style.min_size.height {
                                    Dimension::Length(min_px) => min_px,
                                    Dimension::Percent(pct) => pct * inner_h,
                                    Dimension::Auto => 0.0,
                                };
                                if new_h < min_h {
                                    new_h = min_h;
                                }
                                final_heights[i] = new_h;
                            }
                        }
                    }
                }

                let total_final_h: f32 = final_heights.iter().sum();
                let unused_space = (inner_h - total_final_h - total_gap).max(0.0);
                let (mut curr_y, extra_gap) =
                    match style.justify_content.unwrap_or(JustifyContent::Start) {
                        JustifyContent::Start => (origin.y + pad_top, 0.0),
                        JustifyContent::Center => (origin.y + pad_top + unused_space * 0.5, 0.0),
                        JustifyContent::End => (origin.y + pad_top + unused_space, 0.0),
                        JustifyContent::SpaceBetween => {
                            let eg = if n > 1 {
                                unused_space / (n as f32 - 1.0)
                            } else {
                                0.0
                            };
                            (origin.y + pad_top, eg)
                        }
                        JustifyContent::SpaceAround => {
                            let eg = unused_space / n as f32;
                            (origin.y + pad_top + eg * 0.5, eg)
                        }
                        JustifyContent::SpaceEvenly => {
                            let eg = unused_space / (n as f32 + 1.0);
                            (origin.y + pad_top + eg, eg)
                        }
                    };

                for (i, &child_id) in children.iter().enumerate() {
                    let ch = final_heights[i];
                    let cw = match style.align_items.unwrap_or(AlignItems::Start) {
                        AlignItems::Stretch => inner_w,
                        _ => base_widths[i].min(inner_w),
                    };
                    let cx = match style.align_items.unwrap_or(AlignItems::Start) {
                        AlignItems::Start => origin.x + pad_left,
                        AlignItems::Center => origin.x + pad_left + (inner_w - cw) * 0.5,
                        AlignItems::End => origin.x + pad_left + inner_w - cw,
                        AlignItems::Stretch => origin.x + pad_left,
                    };

                    let child_origin = Point::new(cx, curr_y);
                    let child_avail =
                        Size::new(AvailableSpace::Definite(cw), AvailableSpace::Definite(ch));
                    self.compute_node(child_id, child_avail, child_origin)?;

                    self.nodes[child_id.0].layout.size = Size::new(cw, ch);
                    self.nodes[child_id.0].layout.location = child_origin;

                    curr_y += ch + gap_y + extra_gap;
                }
            }
        }

        self.nodes[node_id.0].layout = Layout {
            order: 0,
            size: Size::new(w, h),
            location: origin,
            content_size: Size::new(w, h),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpi_scale_transforms_and_snapping() {
        // Test 125% DPI scale + 1.15 user UI zoom = 1.4375 effective scale
        let ctx = DpiScaleContext::new(1.25, 1.15);
        let eff = ctx.effective_scale();
        assert!((eff - 1.4375).abs() < 1e-4);

        let pt = 100.0;
        let px = ctx.to_physical(pt);
        assert_eq!(px, 143.75);

        let back_pt = ctx.to_logical(px);
        assert!((back_pt - pt).abs() < 1e-4);

        // Test snapping at 150% (effective scale = 1.5)
        let ctx150 = DpiScaleContext::new(1.5, 1.0);
        let snapped = ctx150.snap_to_pixel(10.333);
        // 10.333 * 1.5 = 15.4995 -> round = 15.0 -> 15.0 / 1.5 = 10.0
        assert_eq!(snapped, 10.0);

        let rect = LayoutRect::new(10.0, 20.0, 100.0, 50.0);
        let (sx, sy, sw, sh) = ctx150.to_physical_scissor(rect, 1920, 1080);
        assert_eq!(sx, 15);
        assert_eq!(sy, 30);
        assert_eq!(sw, 150);
        assert_eq!(sh, 75);
    }

    #[test]
    fn test_responsive_window_shell_layout() {
        // Test wide desktop (> 1024pt)
        let bounds_wide = LayoutRect::new(0.0, 0.0, 1200.0, 800.0);
        let shell_wide = WindowShellLayout::solve(bounds_wide, 48.0, 36.0, 240.0, false);
        assert_eq!(shell_wide.breakpoint, ResponsiveBreakpoint::Expanded);
        assert_eq!(shell_wide.header.h, 48.0);
        assert_eq!(shell_wide.footer.h, 36.0);
        assert!(shell_wide.sidebar.is_some());
        assert_eq!(shell_wide.sidebar.unwrap().w, 240.0);
        assert_eq!(shell_wide.content.w, 1200.0 - 240.0);

        // Test compact screen (< 640pt)
        let bounds_compact = LayoutRect::new(0.0, 0.0, 500.0, 700.0);
        let shell_compact = WindowShellLayout::solve(bounds_compact, 44.0, 32.0, 200.0, false);
        assert_eq!(shell_compact.breakpoint, ResponsiveBreakpoint::Compact);
        assert!(shell_compact.sidebar.is_none()); // Collapsed in compact
        assert_eq!(shell_compact.content.w, 500.0);
    }

    #[test]
    fn test_bounded_typography_and_split_row() {
        let fit =
            BoundedTextRules::fit_single_line("Super Long Window Title Exceeding Width", 8.0, 80.0);
        assert!(fit.ends_with(BoundedTextRules::ELLIPSIS));
        assert!(fit.len() < "Super Long Window Title Exceeding Width".len());

        let row = LayoutRect::new(0.0, 0.0, 300.0, 40.0);
        let (left, right) = BoundedTextRules::solve_split_row(row, 60.0, 12.0, 10.0);
        assert_eq!(right.w, 60.0);
        assert_eq!(right.x, 300.0 - 10.0 - 60.0);
        assert_eq!(left.x, 10.0);
        assert_eq!(left.w, 280.0 - 60.0 - 12.0);
        assert!(left.right() + 12.0 <= right.x);
    }

    #[test]
    fn test_flexbox_row_solve_and_justify() {
        let flex = FlexContainer {
            direction: FlexDirection::Row,
            justify: JustifyContent::SpaceBetween,
            align: AlignItems::Center,
            gap: 10.0,
            padding: 0.0,
        };
        let bounds = LayoutRect::new(0.0, 0.0, 300.0, 100.0);
        let items: [(f32, f32); 3] = [(50.0, 40.0), (50.0, 40.0), (50.0, 40.0)];
        let rects = flex.solve(bounds, &items);

        assert_eq!(rects.len(), 3);
        assert_eq!(rects[0].x, 0.0);
        assert_eq!(rects[2].x + rects[2].w, 300.0);
        for r in &rects {
            assert_eq!(r.y, 30.0); // (100 - 40) / 2
        }
    }

    #[test]
    fn test_grid_container_solve() {
        let grid = GridContainer::new(3, 50.0, 10.0);
        let bounds = LayoutRect::new(0.0, 0.0, 320.0, 400.0);
        let rects = grid.solve(bounds, 5);

        assert_eq!(rects.len(), 5);
        assert_eq!(rects[0].w, 100.0);
        assert_eq!(rects[1].x, 110.0);
        assert_eq!(rects[3].y, 60.0); // Row 1
    }

    #[test]
    fn test_layout_cache_telemetry() {
        let mut lc = LayoutCache::new();
        let id = ElementId::from_raw(42);
        assert!(lc.get(id).is_none());
        assert_eq!(lc.misses, 1);

        lc.insert(id, LayoutRect::new(10.0, 20.0, 30.0, 40.0));
        assert!(lc.get(id).is_some());
        assert_eq!(lc.hits, 1);
        assert_eq!(lc.hit_ratio(), 0.5);
    }

    #[test]
    fn test_taffytree_zero_hardcoded_coordinates() {
        // Enforces Rule 1: Zero hardcoded coordinates - full window shell resolved through TaffyTree flexbox
        let mut tree = TaffyTree::new();

        // 1. Root window column container (1200x800)
        let root_style = Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            size: Size::new(length(1200.0), length(800.0)),
            gap: Size::new(0.0, 0.0),
            padding: Rect::all(0.0),
            ..Default::default()
        };

        // 2. Top Bar (36px high, full width)
        let top_bar = tree
            .new_leaf(Style {
                size: Size::new(Dimension::Auto, length(36.0)),
                flex_grow: 0.0,
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        // 3. Workspace Row (flex_grow: 1.0)
        let sidebar = tree
            .new_leaf(Style {
                size: Size::new(length(240.0), Dimension::Auto),
                flex_grow: 0.0,
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let main_content = tree
            .new_leaf(Style {
                size: Size::new(Dimension::Auto, Dimension::Auto),
                flex_grow: 1.0,
                flex_shrink: 1.0,
                min_size: Size::new(length(0.0), length(0.0)),
                ..Default::default()
            })
            .unwrap();

        let workspace_row = tree
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    size: Size::new(Dimension::Auto, Dimension::Auto),
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    ..Default::default()
                },
                &[sidebar, main_content],
            )
            .unwrap();

        // 4. Footer (48px high, full width)
        let footer = tree
            .new_leaf(Style {
                size: Size::new(Dimension::Auto, length(48.0)),
                flex_grow: 0.0,
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let root = tree
            .new_with_children(root_style, &[top_bar, workspace_row, footer])
            .unwrap();

        tree.compute_layout(
            root,
            Size::new(
                AvailableSpace::Definite(1200.0),
                AvailableSpace::Definite(800.0),
            ),
        )
        .unwrap();

        let top_bar_layout = tree.layout(top_bar).unwrap();
        assert_eq!(top_bar_layout.location.y, 0.0);
        assert_eq!(top_bar_layout.size.height, 36.0);
        assert_eq!(top_bar_layout.size.width, 1200.0);

        let ws_layout = tree.layout(workspace_row).unwrap();
        assert_eq!(ws_layout.location.y, 36.0);
        assert_eq!(ws_layout.size.height, 800.0 - 36.0 - 48.0); // 716.0

        let sb_layout = tree.layout(sidebar).unwrap();
        assert_eq!(sb_layout.location.x, 0.0);
        assert_eq!(sb_layout.size.width, 240.0);

        let cnt_layout = tree.layout(main_content).unwrap();
        assert_eq!(cnt_layout.location.x, 240.0);
        assert_eq!(cnt_layout.size.width, 1200.0 - 240.0); // 960.0

        let footer_layout = tree.layout(footer).unwrap();
        assert_eq!(footer_layout.location.y, 752.0); // 800 - 48
        assert_eq!(footer_layout.size.height, 48.0);
    }

    #[test]
    fn test_taffytree_rule2_shrinking_text_column_min_size_zero() {
        // Enforces Rule 2: Flex Min-Content Collision Prevention
        // Row: [Icon 36x36 (flex_shrink: 0)] + [Text Column (flex_grow: 1, flex_shrink: 1, min_size: 0, overflow: Hidden)] + [Amber Pill 132x32 (flex_shrink: 0)]
        let mut tree = TaffyTree::new();

        let icon = tree
            .new_leaf(Style {
                size: Size::new(length(36.0), length(36.0)),
                flex_shrink: 0.0,
                flex_grow: 0.0,
                ..Default::default()
            })
            .unwrap();

        // Text column using the Rule 2 factory method
        let text_col = tree.new_leaf(Style::shrinking_text_column()).unwrap();

        // Action pill using the Rule 2 factory method (flex_shrink: 0.0)
        let pill = tree.new_leaf(Style::action_pill(132.0, 32.0)).unwrap();

        let row_style = Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            gap: Size::new(8.0, 0.0),
            padding: Rect::all(10.0),
            ..Default::default()
        };

        let row = tree
            .new_with_children(row_style, &[icon, text_col, pill])
            .unwrap();

        // 1. Wide container (500px wide):
        // Inner width = 500 - 20 = 480px.
        // Gaps = 2 * 8 = 16px.
        // Remaining for text column = 480 - 36 - 132 - 16 = 296px.
        tree.compute_layout(
            row,
            Size::new(
                AvailableSpace::Definite(500.0),
                AvailableSpace::Definite(60.0),
            ),
        )
        .unwrap();

        let icon_layout = tree.layout(icon).unwrap();
        let text_layout = tree.layout(text_col).unwrap();
        let pill_layout = tree.layout(pill).unwrap();

        assert_eq!(icon_layout.size.width, 36.0);
        assert_eq!(pill_layout.size.width, 132.0);
        assert_eq!(text_layout.size.width, 296.0);
        assert_eq!(icon_layout.location.x, 10.0);
        assert_eq!(text_layout.location.x, 10.0 + 36.0 + 8.0); // 54.0
        assert_eq!(pill_layout.location.x, 54.0 + 296.0 + 8.0); // 358.0
        assert_eq!(pill_layout.location.x + pill_layout.size.width, 490.0); // 500 - 10 pad

        // 2. Narrow compressed container (250px wide):
        // Inner width = 250 - 20 = 230px.
        // Gaps = 16px.
        // Fixed children: Icon (36) + Pill (132) = 168px.
        // Action pill MUST NOT SHRINK (flex_shrink: 0.0).
        // Text column MUST shrink down to (230 - 168 - 16) = 46px.
        tree.compute_layout(
            row,
            Size::new(
                AvailableSpace::Definite(250.0),
                AvailableSpace::Definite(60.0),
            ),
        )
        .unwrap();

        let icon_compressed = tree.layout(icon).unwrap();
        let text_compressed = tree.layout(text_col).unwrap();
        let pill_compressed = tree.layout(pill).unwrap();

        // Invariant: Action pill never shrinks!
        assert_eq!(
            pill_compressed.size.width, 132.0,
            "Action pill must never shrink below 132px"
        );
        assert_eq!(icon_compressed.size.width, 36.0, "Icon must not shrink");
        // Invariant: Text column smoothly absorbs the shrink deficit down to min_size: 0.0 without colliding
        assert_eq!(
            text_compressed.size.width, 46.0,
            "Text column must absorb deficit down to 46px"
        );
        assert!(
            pill_compressed.location.x + pill_compressed.size.width <= 250.0,
            "No overflow allowed outside container"
        );
    }

    #[test]
    fn test_taffytree_text_measure_callback() {
        let mut tree = TaffyTree::new();

        // Intrinsic text measurement leaf callback simulating cosmic-text shaping
        let text_node = tree
            .new_leaf_with_measure(Style::default(), |_known, avail| {
                let max_w = avail.width.into_option().unwrap_or(200.0);
                // Simulate 120px text width bounded by max_w
                let text_w = 120.0f32.min(max_w);
                Size::new(text_w, 24.0)
            })
            .unwrap();

        tree.compute_layout(
            text_node,
            Size::new(
                AvailableSpace::Definite(300.0),
                AvailableSpace::Definite(50.0),
            ),
        )
        .unwrap();
        let layout = tree.layout(text_node).unwrap();
        assert_eq!(layout.size.width, 120.0);
        assert_eq!(layout.size.height, 24.0);
    }

    #[test]
    fn test_flex_container_solve_flex_items_rule2() {
        let flex = FlexContainer::row(8.0);
        let bounds = LayoutRect::new(0.0, 0.0, 240.0, 50.0);

        let items = [
            FlexItem::fixed(36.0, 36.0),
            FlexItem::shrinking_text(250.0, 20.0),
            FlexItem::action_pill(132.0, 32.0),
        ];

        let rects = flex.solve_flex_items(bounds, &items);
        assert_eq!(rects.len(), 3);

        // Fixed icon = 36px
        assert_eq!(rects[0].w, 36.0);
        // Action pill = 132px (strictly non-shrinking)
        assert_eq!(rects[2].w, 132.0);
        // Remaining space for shrinking text = 240 - 36 - 132 - (2 * 8) = 56px
        assert_eq!(rects[1].w, 56.0);
        // No collision
        assert!(rects[1].right() <= rects[2].x);
        assert!(rects[2].right() <= 240.0);
    }

    #[test]
    fn test_solve_row_shrinking_text_with_pills_telemetry_repair_card() {
        let row_rect = LayoutRect::new(0.0, 0.0, 300.0, 56.0);
        let (icon, text_col, pills) = FlexContainer::solve_row_shrinking_text_with_pills(
            row_rect,
            10.0,
            8.0,
            Some(36.0),
            &[132.0],
        );

        assert!(icon.is_some());
        let icon_r = icon.unwrap();
        assert_eq!(icon_r.w, 36.0);
        assert_eq!(icon_r.x, 8.0);

        assert_eq!(pills.len(), 1);
        let pill_r = pills[0];
        assert_eq!(pill_r.w, 132.0);
        assert_eq!(pill_r.right(), 300.0 - 8.0);

        // Text column is strictly bounded between icon and pill with 10px gaps
        assert_eq!(text_col.x, 8.0 + 36.0 + 10.0); // 54.0
        assert_eq!(text_col.right() + 10.0, pill_r.x);
        assert_eq!(text_col.w, (300.0 - 16.0) - 36.0 - 132.0 - 20.0); // 96.0
    }

    #[test]
    fn test_card_text_wrapping_and_strict_boundaries() {
        let card = LayoutRect::new(24.0, 68.0, 236.0, 138.0);
        let padding = 12.0;
        let avg_char_w = 5.8;

        let max_chars = BoundedTextRules::card_max_chars(card.w, padding, avg_char_w);
        let avail_w = card.w - padding * 2.0; // 212.0
        let expected_chars = (avail_w / avg_char_w).floor() as usize; // 36
        assert_eq!(max_chars, expected_chars);

        // Test long mission caption never bleeds outside card
        let mission_caption = "Micro-attribution unlocks dopamine upon completion.";
        let wrapped = BoundedTextRules::wrap_card_text(mission_caption, card, padding, avg_char_w);

        for line in wrapped.lines() {
            assert!(
                line.chars().count() <= max_chars,
                "Line '{}' exceeded card max chars {}",
                line,
                max_chars
            );
        }

        // Test LayoutRect convenience wrappers
        assert_eq!(card.max_chars_padded(padding, avg_char_w), max_chars);
        assert_eq!(
            card.wrap_text_padded(mission_caption, padding, avg_char_w),
            wrapped
        );

        let sc = card.to_scissor_rect();
        assert_eq!(sc.x, 24);
        assert_eq!(sc.y, 68);
        assert_eq!(sc.width, 236);
        assert_eq!(sc.height, 138);

        // Test multi-line vertical clamping to card height
        // Card of height 48px with 8px pad_y: available height = 32px.
        // With line_height = 14px, max 2 lines can fit.
        let small_card = LayoutRect::new(0.0, 0.0, 120.0, 48.0);
        let long_text = "Micro-attribution unlocks dopamine upon completion and drives persistent study momentum.";
        let lines = BoundedTextRules::fit_multiline_to_card(
            long_text, small_card, 8.0, 8.0, avg_char_w, 14.0,
        );
        assert_eq!(lines.len(), 2);
        assert!(lines[1].ends_with(BoundedTextRules::ELLIPSIS));

        // Test long unhyphenated word with narrow card width (avail 50px -> ~8 chars)
        let narrow_card = LayoutRect::new(0.0, 0.0, 74.0, 100.0);
        let narrow_max_chars = BoundedTextRules::card_max_chars(narrow_card.w, 12.0, avg_char_w);
        assert_eq!(narrow_max_chars, 8);
        let wrapped_word =
            BoundedTextRules::wrap_card_text("Micro-attribution", narrow_card, 12.0, avg_char_w);
        for line in wrapped_word.lines() {
            assert!(line.chars().count() <= 8);
        }
    }
}
