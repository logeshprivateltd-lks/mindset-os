//! MindSet OS — Drawing Helpers & Clipped Draw Primitive Context (`draw_helpers.rs`)
//! Pure Rust hardware-accelerated drawing wrappers with bounded typography,
//! occlusion culling, and interactive press states.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub use mindset_render::optimal_text_color_for_fill;
use mindset_render::{SdfInstance, SdfPipeline, TextRenderer};
use mindset_ui::LayoutRect;

pub fn push_panel(sdf: &mut SdfPipeline, rect: LayoutRect, fill: [f32; 4], radius: f32) {
    sdf.push(SdfInstance::glass_panel(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        radius,
        fill,
        [1.0, 1.0, 1.0, 0.40],
    ));
}

/// Push a frosted glass panel with custom directional specular rim
pub fn push_panel_rim(
    sdf: &mut SdfPipeline,
    rect: LayoutRect,
    fill: [f32; 4],
    border_rim: [f32; 4],
    radius: f32,
) {
    sdf.push(SdfInstance::glass_panel(
        rect.x, rect.y, rect.w, rect.h, radius, fill, border_rim,
    ));
}

/// Push a frosted glass card using active GlassTokens
pub fn push_glass_card(
    sdf: &mut SdfPipeline,
    rect: LayoutRect,
    tokens: &mindset_ui::theme::GlassTokens,
    radius: f32,
) {
    push_panel_rim(
        sdf,
        rect,
        tokens.card_scrim,
        tokens.specular_rim_top,
        radius,
    );
}

/// Push a frosted glass window using active GlassTokens
pub fn push_glass_window(
    sdf: &mut SdfPipeline,
    rect: LayoutRect,
    tokens: &mindset_ui::theme::GlassTokens,
    radius: f32,
) {
    push_panel_rim(
        sdf,
        rect,
        tokens.window_scrim,
        tokens.specular_rim_top,
        radius,
    );
}

/// Push a frosted glass overlay using active GlassTokens
pub fn push_glass_overlay(
    sdf: &mut SdfPipeline,
    rect: LayoutRect,
    tokens: &mindset_ui::theme::GlassTokens,
    radius: f32,
) {
    push_panel_rim(
        sdf,
        rect,
        tokens.overlay_scrim,
        tokens.specular_rim_top,
        radius,
    );
}

pub fn push_btn(sdf: &mut SdfPipeline, rect: LayoutRect, fill: [f32; 4], border: [f32; 4]) {
    sdf.push(SdfInstance::button(
        rect.x, rect.y, rect.w, rect.h, 6.0, fill, border, 0.15,
    ));
}

pub fn push_squircle_badge_standalone(
    sdf: &mut SdfPipeline,
    txt: &mut TextRenderer,
    queue: &wgpu::Queue,
    rect: LayoutRect,
    badge_color: [f32; 4],
    symbol: &str,
    text_color: [f32; 4],
) {
    let rad = (rect.h * 0.28).min(12.0);
    sdf.push(SdfInstance::button(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        rad,
        badge_color,
        [
            badge_color[0] * 1.3,
            badge_color[1] * 1.3,
            badge_color[2] * 1.3,
            0.4,
        ],
        0.2,
    ));
    sdf.push(SdfInstance::button(
        rect.x + 2.0,
        rect.y + 1.0,
        rect.w - 4.0,
        (rect.h * 0.35).min(14.0),
        rad * 0.6,
        [1.0, 1.0, 1.0, 0.15],
        [0.0; 4],
        0.0,
    ));
    let fs = (rect.h * 0.40).clamp(9.0, 15.0);
    let (sym_w, _) = txt.measure_line_px(symbol, fs);
    let sx = rect.x + (rect.w - sym_w) * 0.5;
    let sy = rect.y + (rect.h - fs) * 0.5 - 1.0;
    let final_tc = if text_color[3] > 0.0 {
        optimal_text_color_for_fill(badge_color)
    } else {
        text_color
    };
    txt.queue_text(queue, symbol, sx, sy, fs, final_tc);
}

pub fn push_maindy_ai_orb_standalone(sdf: &mut SdfPipeline, center_x: f32, center_y: f32) {
    sdf.push(SdfInstance::button(
        center_x - 16.0,
        center_y - 16.0,
        32.0,
        32.0,
        16.0,
        [0.388, 0.400, 0.945, 0.28],
        [0.388, 0.400, 0.945, 0.4],
        0.0,
    ));
    sdf.push(SdfInstance::button(
        center_x - 12.0,
        center_y - 12.0,
        24.0,
        24.0,
        12.0,
        [0.310, 0.350, 0.920, 1.0],
        [0.55, 0.65, 1.0, 0.8],
        0.25,
    ));
    sdf.push(SdfInstance::button(
        center_x - 5.0,
        center_y - 3.0,
        3.0,
        4.0,
        1.5,
        [1.0, 1.0, 1.0, 1.0],
        [0.0; 4],
        0.0,
    ));
    sdf.push(SdfInstance::button(
        center_x + 2.0,
        center_y - 3.0,
        3.0,
        4.0,
        1.5,
        [1.0, 1.0, 1.0, 1.0],
        [0.0; 4],
        0.0,
    ));
}

/// Z-order clipping and occlusion context preventing background window text
/// and controls from bleeding through foreground windows and modal overlays.
pub struct ClippedDraw<'a> {
    pub sdf: &'a mut SdfPipeline,
    pub txt: &'a mut TextRenderer,
    pub queue: &'a wgpu::Queue,
    pub clip_bounds: Option<LayoutRect>,
    pub occlusions: &'a [LayoutRect],
    pub is_mouse_down: bool,
    pub mouse_pos: (f32, f32),
}

impl<'a> ClippedDraw<'a> {
    #[inline]
    pub fn is_occluded(&self, cx: f32, cy: f32) -> bool {
        for occ in self.occlusions {
            if occ.contains(cx, cy) {
                return true;
            }
        }
        false
    }

    #[inline]
    pub fn is_clipped(&self, cx: f32, cy: f32) -> bool {
        if let Some(clip) = self.clip_bounds {
            !clip.contains(cx, cy)
        } else {
            false
        }
    }

    #[inline]
    pub fn push_panel(&mut self, rect: LayoutRect, fill: [f32; 4], radius: f32) {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        if self.is_clipped(cx, cy) || self.is_occluded(cx, cy) {
            return;
        }
        self.sdf.push(SdfInstance::glass_panel(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            radius,
            fill,
            [1.0, 1.0, 1.0, 0.40],
        ));
    }

    #[inline]
    pub fn push_panel_rim(
        &mut self,
        rect: LayoutRect,
        fill: [f32; 4],
        border_rim: [f32; 4],
        radius: f32,
    ) {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        if self.is_clipped(cx, cy) || self.is_occluded(cx, cy) {
            return;
        }
        self.sdf.push(SdfInstance::glass_panel(
            rect.x, rect.y, rect.w, rect.h, radius, fill, border_rim,
        ));
    }

    #[inline]
    pub fn push_glass_card(
        &mut self,
        rect: LayoutRect,
        tokens: &mindset_ui::theme::GlassTokens,
        radius: f32,
    ) {
        self.push_panel_rim(rect, tokens.card_scrim, tokens.specular_rim_top, radius);
    }

    #[inline]
    pub fn push_glass_window(
        &mut self,
        rect: LayoutRect,
        tokens: &mindset_ui::theme::GlassTokens,
        radius: f32,
    ) {
        self.push_panel_rim(rect, tokens.window_scrim, tokens.specular_rim_top, radius);
    }

    #[inline]
    pub fn push_glass_overlay(
        &mut self,
        rect: LayoutRect,
        tokens: &mindset_ui::theme::GlassTokens,
        radius: f32,
    ) {
        self.push_panel_rim(rect, tokens.overlay_scrim, tokens.specular_rim_top, radius);
    }

    #[inline]
    pub fn push_btn(&mut self, rect: LayoutRect, fill: [f32; 4], border: [f32; 4]) {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        if self.is_clipped(cx, cy) || self.is_occluded(cx, cy) {
            return;
        }
        let is_pressed = self.is_mouse_down && rect.contains(self.mouse_pos.0, self.mouse_pos.1);
        let (draw_rect, draw_fill) = if is_pressed {
            (
                rect.inset(2.0),
                [
                    (fill[0] + 0.18).min(1.0),
                    (fill[1] + 0.18).min(1.0),
                    (fill[2] + 0.18).min(1.0),
                    fill[3],
                ],
            )
        } else {
            (rect, fill)
        };
        self.sdf.push(SdfInstance::button(
            draw_rect.x,
            draw_rect.y,
            draw_rect.w,
            draw_rect.h,
            6.0,
            draw_fill,
            border,
            0.15,
        ));
    }

    #[inline]
    pub fn push_squircle_badge(
        &mut self,
        rect: LayoutRect,
        badge_color: [f32; 4],
        symbol: &str,
        text_color: [f32; 4],
    ) {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        if self.is_clipped(cx, cy) || self.is_occluded(cx, cy) {
            return;
        }
        let is_pressed = self.is_mouse_down && rect.contains(self.mouse_pos.0, self.mouse_pos.1);
        let r = if is_pressed { rect.inset(1.5) } else { rect };
        let rad = (r.h * 0.28).min(12.0);
        self.sdf.push(SdfInstance::button(
            r.x,
            r.y,
            r.w,
            r.h,
            rad,
            badge_color,
            [
                badge_color[0] * 1.3,
                badge_color[1] * 1.3,
                badge_color[2] * 1.3,
                0.4,
            ],
            0.2,
        ));
        self.sdf.push(SdfInstance::button(
            r.x + 2.0,
            r.y + 1.0,
            r.w - 4.0,
            (r.h * 0.35).min(14.0),
            rad * 0.6,
            [1.0, 1.0, 1.0, 0.15],
            [0.0; 4],
            0.0,
        ));
        let fs = (r.h * 0.40).clamp(9.0, 15.0);
        let (sym_w, _) = self.txt.measure_line_px(symbol, fs);
        let sx = r.x + (r.w - sym_w) * 0.5;
        let sy = r.y + (r.h - fs) * 0.5 - 1.0;
        let final_tc = if text_color[3] > 0.0 {
            optimal_text_color_for_fill(badge_color)
        } else {
            text_color
        };
        self.queue_text(symbol, sx, sy, fs, final_tc);
    }

    #[inline]
    pub fn push_progress_bar(
        &mut self,
        rect: LayoutRect,
        value: f32,
        track_col: [f32; 4],
        fill_col: [f32; 4],
    ) {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        if self.is_clipped(cx, cy) || self.is_occluded(cx, cy) {
            return;
        }
        self.sdf.push(SdfInstance::button(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            rect.h * 0.5,
            track_col,
            [0.0, 0.0, 0.0, 0.0],
            0.0,
        ));
        let fw = (rect.w * value.clamp(0.0, 1.0)).max(0.0);
        if fw > 0.5 {
            self.sdf.push(SdfInstance::button(
                rect.x,
                rect.y,
                fw,
                rect.h,
                rect.h * 0.5,
                fill_col,
                [0.0, 0.0, 0.0, 0.0],
                0.0,
            ));
        }
    }

    #[inline]
    pub fn push_slider(
        &mut self,
        track_rect: LayoutRect,
        value: f32,
        track_col: [f32; 4],
        fill_col: [f32; 4],
    ) {
        let cx = track_rect.x + track_rect.w * 0.5;
        let cy = track_rect.y + track_rect.h * 0.5;
        if self.is_clipped(cx, cy) || self.is_occluded(cx, cy) {
            return;
        }
        let rad = track_rect.h * 0.5;
        self.sdf.push(SdfInstance::button(
            track_rect.x,
            track_rect.y,
            track_rect.w,
            track_rect.h,
            rad,
            track_col,
            [0.0; 4],
            0.0,
        ));
        let fw = (track_rect.w * value.clamp(0.0, 1.0)).max(0.0);
        if fw > 0.5 {
            self.sdf.push(SdfInstance::button(
                track_rect.x,
                track_rect.y,
                fw,
                track_rect.h,
                rad,
                fill_col,
                [0.0; 4],
                0.0,
            ));
        }
        let knob_r = 9.0;
        let kx = track_rect.x + fw - knob_r;
        let ky = track_rect.y + track_rect.h * 0.5 - knob_r;
        self.sdf.push(SdfInstance::button(
            kx,
            ky,
            knob_r * 2.0,
            knob_r * 2.0,
            knob_r,
            [1.0, 1.0, 1.0, 1.0],
            [0.8, 0.85, 0.9, 0.5],
            0.15,
        ));
    }

    #[inline]
    pub fn push_pill_toggle(&mut self, rect: LayoutRect, is_on: bool) {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        if self.is_clipped(cx, cy) || self.is_occluded(cx, cy) {
            return;
        }
        let pill_bg = if is_on {
            [0.145, 0.388, 0.922, 1.0] // Rich blue #2563EB
        } else {
            [0.278, 0.333, 0.412, 1.0] // Slate #475569
        };
        let rad = rect.h * 0.5;
        self.sdf.push(SdfInstance::button(
            rect.x, rect.y, rect.w, rect.h, rad, pill_bg, [0.0; 4], 0.0,
        ));
        let knob_d = rect.h - 6.0;
        let knob_r = knob_d * 0.5;
        let kx = if is_on {
            rect.x + rect.w - knob_d - 3.0
        } else {
            rect.x + 3.0
        };
        let ky = rect.y + 3.0;
        self.sdf.push(SdfInstance::button(
            kx,
            ky,
            knob_d,
            knob_d,
            knob_r,
            [1.0, 1.0, 1.0, 1.0],
            [0.7, 0.75, 0.85, 0.5],
            0.15,
        ));
    }

    #[inline]
    pub fn push_maindy_ai_orb(&mut self, center_x: f32, center_y: f32) {
        if self.is_clipped(center_x, center_y) || self.is_occluded(center_x, center_y) {
            return;
        }
        self.sdf.push(SdfInstance::button(
            center_x - 16.0,
            center_y - 16.0,
            32.0,
            32.0,
            16.0,
            [0.388, 0.400, 0.945, 0.28],
            [0.388, 0.400, 0.945, 0.4],
            0.0,
        ));
        self.sdf.push(SdfInstance::button(
            center_x - 12.0,
            center_y - 12.0,
            24.0,
            24.0,
            12.0,
            [0.310, 0.350, 0.920, 1.0],
            [0.55, 0.65, 1.0, 0.8],
            0.25,
        ));
        self.sdf.push(SdfInstance::button(
            center_x - 5.0,
            center_y - 3.0,
            3.0,
            4.0,
            1.5,
            [1.0, 1.0, 1.0, 1.0],
            [0.0; 4],
            0.0,
        ));
        self.sdf.push(SdfInstance::button(
            center_x + 2.0,
            center_y - 3.0,
            3.0,
            4.0,
            1.5,
            [1.0, 1.0, 1.0, 1.0],
            [0.0; 4],
            0.0,
        ));
    }

    #[inline]
    pub fn push_labeled_btn(
        &mut self,
        rect: LayoutRect,
        label: &str,
        font_size: f32,
        fill: [f32; 4],
        border: [f32; 4],
    ) {
        self.push_btn(rect, fill, border);
        let tc = optimal_text_color_for_fill(fill);
        self.queue_text_centered_in_rect(label, rect, font_size, tc);
    }

    #[inline]
    pub fn queue_text(
        &mut self,
        text: &str,
        start_x: f32,
        start_y: f32,
        font_size: f32,
        color: [f32; 4],
    ) {
        if self.is_occluded(start_x, start_y) {
            return;
        }
        let clip_raw = self.clip_bounds.map(|c| [c.x, c.y, c.w, c.h]);
        let occ_raw: Vec<[f32; 4]> = self
            .occlusions
            .iter()
            .map(|o| [o.x, o.y, o.w, o.h])
            .collect();
        let bounded_text = if let Some(clip) = self.clip_bounds {
            let max_w = (clip.x + clip.w - start_x - 2.0).max(0.0);
            self.txt.fit_text_to_px(text, font_size, max_w)
        } else {
            text.to_string()
        };
        self.txt.queue_text_clipped(
            self.queue,
            &bounded_text,
            start_x,
            start_y,
            font_size,
            color,
            clip_raw,
            &occ_raw,
        );
    }

    #[inline]
    pub fn measure_line_px(&self, text: &str, px_size: f32) -> (f32, f32) {
        self.txt.measure_line_px(text, px_size)
    }

    #[inline]
    pub fn fit_text_to_px(&self, text: &str, px_size: f32, max_width_px: f32) -> String {
        self.txt.fit_text_to_px(text, px_size, max_width_px)
    }

    #[inline]
    pub fn queue_text_bounded(
        &mut self,
        text: &str,
        start_x: f32,
        start_y: f32,
        max_width_px: f32,
        font_size: f32,
        color: [f32; 4],
    ) -> f32 {
        if self.is_occluded(start_x, start_y) {
            return 0.0;
        }
        let fitted = self.txt.fit_text_to_px(text, font_size, max_width_px);
        let (w, _) = self.txt.measure_line_px(&fitted, font_size);
        let clip_raw = self.clip_bounds.map(|c| [c.x, c.y, c.w, c.h]);
        let occ_raw: Vec<[f32; 4]> = self
            .occlusions
            .iter()
            .map(|o| [o.x, o.y, o.w, o.h])
            .collect();
        self.txt.queue_text_clipped(
            self.queue, &fitted, start_x, start_y, font_size, color, clip_raw, &occ_raw,
        );
        w
    }

    #[inline]
    pub fn queue_text_centered_in_rect(
        &mut self,
        text: &str,
        rect: LayoutRect,
        font_size: f32,
        color: [f32; 4],
    ) {
        let is_single_symbol = text.chars().count() <= 2;
        let pad = if is_single_symbol || rect.w <= 36.0 {
            2.0
        } else {
            8.0
        };
        let max_w = (rect.w - pad).max(10.0);
        let fitted = if is_single_symbol {
            text.to_string()
        } else {
            self.txt.fit_text_to_px(text, font_size, max_w)
        };
        let (w, h) = self.txt.measure_line_px(&fitted, font_size);
        let start_x = rect.x + (rect.w - w) * 0.5;
        let start_y = rect.y + (rect.h - h) * 0.5;
        self.queue_text(&fitted, start_x, start_y, font_size, color);
    }

    #[inline]
    pub fn queue_split_row(
        &mut self,
        left: &str,
        right: &str,
        rect: LayoutRect,
        font_size: f32,
        left_color: [f32; 4],
        right_color: [f32; 4],
    ) {
        let (right_w, _) = self.txt.measure_line_px(right, font_size);
        let right_x = rect.x + rect.w - right_w;
        let max_left_w = (right_x - rect.x - 8.0).max(0.0);
        let fitted_left = self.txt.fit_text_to_px(left, font_size, max_left_w);
        self.queue_text(&fitted_left, rect.x, rect.y, font_size, left_color);
        self.queue_text(right, right_x, rect.y, font_size, right_color);
    }
}
