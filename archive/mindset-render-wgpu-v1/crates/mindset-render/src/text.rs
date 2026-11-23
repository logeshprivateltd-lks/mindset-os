use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GlyphInstance {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub color: [f32; 4],
    pub is_color: u32,
    pub _pad: [u32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScissorRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextBatch {
    pub scissor: Option<ScissorRect>,
    pub start: u32,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct RasterizedGlyph {
    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,
    pub is_color: bool,
    pub rgba_pixels: Vec<u8>,
}

/// Rasterizes a glyph using cosmic_text / SwashCache, returning RGBA pixels and color metadata.
pub fn rasterize_glyph_rgba(
    cache: &mut cosmic_text::SwashCache,
    font_system: &mut cosmic_text::FontSystem,
    cache_key: cosmic_text::CacheKey,
) -> Option<RasterizedGlyph> {
    let image = cache.get_image_uncached(font_system, cache_key)?;
    let width = image.placement.width;
    let height = image.placement.height;
    if width == 0 || height == 0 {
        return None;
    }

    let (is_color, rgba_pixels) = match image.content {
        cosmic_text::SwashContent::Color => (true, image.data),
        cosmic_text::SwashContent::Mask => {
            let mut rgba = Vec::with_capacity((width * height * 4) as usize);
            for alpha in image.data {
                rgba.extend_from_slice(&[255, 255, 255, alpha]);
            }
            (false, rgba)
        }
        cosmic_text::SwashContent::SubpixelMask => {
            let mut rgba = Vec::with_capacity((width * height * 4) as usize);
            for chunk in image.data.chunks_exact(3) {
                let avg = ((chunk[0] as u16 + chunk[1] as u16 + chunk[2] as u16) / 3) as u8;
                rgba.extend_from_slice(&[255, 255, 255, avg]);
            }
            (false, rgba)
        }
    };

    Some(RasterizedGlyph {
        width,
        height,
        left: image.placement.left,
        top: image.placement.top,
        is_color,
        rgba_pixels,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBoxBounds {
    pub x: f32,
    pub y: f32,
    pub max_width: f32,
    pub max_height: f32,
}

impl TextBoxBounds {
    pub fn padded(x: f32, y: f32, width: f32, height: f32, pad_x: f32, pad_y: f32) -> Self {
        Self {
            x: x + pad_x,
            y: y + pad_y,
            max_width: (width - pad_x * 2.0).max(16.0),
            max_height: (height - pad_y * 2.0).max(16.0),
        }
    }

    /// Computes max wrapping characters that fit within this bounding box given average character width.
    #[inline]
    pub fn max_chars(&self, avg_char_w: f32) -> usize {
        max_chars_for_width(self.max_width, avg_char_w)
    }

    /// Wraps text to fit strictly within this bounding box.
    pub fn wrap_text(&self, text: &str, avg_char_w: f32) -> String {
        wrap_text_chars(text, self.max_chars(avg_char_w))
    }

    /// Converts this bounding box to a ScissorRect for GPU viewport clipping.
    pub fn to_scissor(&self) -> ScissorRect {
        ScissorRect {
            x: self.x.max(0.0).round() as u32,
            y: self.y.max(0.0).round() as u32,
            width: self.max_width.max(1.0).round() as u32,
            height: self.max_height.max(1.0).round() as u32,
        }
    }

    /// Returns the [x, y, w, h] clip bounds array.
    #[inline]
    pub fn to_clip_bounds(&self) -> [f32; 4] {
        [self.x, self.y, self.max_width, self.max_height]
    }
}

/// Computes safe inner text bounds and shapes cosmic-text buffer strictly within chat body bounds.
pub fn layout_chat_body(
    buffer: &mut cosmic_text::Buffer,
    font_system: &mut cosmic_text::FontSystem,
    win_x: f32,
    win_y: f32,
    win_w: f32,
    win_h: f32,
) -> TextBoxBounds {
    let header_h = 68.0;
    let footer_h = 54.0;
    let scrollbar_w = 10.0;
    let pad_x = 16.0;
    let pad_y = 10.0;

    let body_w = (win_w - pad_x * 2.0 - scrollbar_w).max(60.0);
    let body_h = (win_h - header_h - footer_h - pad_y * 2.0).max(40.0);

    buffer.set_wrap(cosmic_text::Wrap::Word);
    buffer.set_size(Some(body_w), Some(body_h));
    buffer.shape_until_scroll(font_system, false);

    TextBoxBounds {
        x: win_x + pad_x,
        y: win_y + header_h + pad_y,
        max_width: body_w,
        max_height: body_h,
    }
}

/// Computes safe inner text bounds and shapes cosmic-text buffer strictly within footer input bar bounds.
pub fn layout_input_bar(
    buffer: &mut cosmic_text::Buffer,
    font_system: &mut cosmic_text::FontSystem,
    win_x: f32,
    win_y: f32,
    win_w: f32,
    footer_h: f32,
) -> TextBoxBounds {
    let send_btn_w = 64.0;
    let gap = 8.0;
    let pad_x = 16.0;
    let input_w = (win_w - pad_x * 2.0 - send_btn_w - gap).max(40.0);
    let input_h = 36.0;

    buffer.set_wrap(cosmic_text::Wrap::None);
    buffer.set_size(Some(input_w), Some(input_h));
    buffer.shape_until_scroll(font_system, false);

    TextBoxBounds {
        x: win_x + pad_x,
        y: win_y + footer_h,
        max_width: input_w,
        max_height: input_h,
    }
}

/// Sets a hardware scissor rectangle clamped to the render target screen dimensions.
#[inline]
pub fn set_scissor_rect_safe<'a>(
    pass: &mut wgpu::RenderPass<'a>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    screen_w: u32,
    screen_h: u32,
) {
    let sx = (x.max(0.0) as u32).min(screen_w);
    let sy = (y.max(0.0) as u32).min(screen_h);
    let sw = (w.max(1.0) as u32).min(screen_w.saturating_sub(sx)).max(1);
    let sh = (h.max(1.0) as u32).min(screen_h.saturating_sub(sy)).max(1);
    pass.set_scissor_rect(sx, sy, sw, sh);
}

#[derive(Debug, Clone, Copy)]
pub struct CachedGlyph {
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub xmin: f32,
    pub ymin: f32,
    pub advance_width: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TextUniforms {
    pub screen_size: [f32; 2],
    pub _pad: [f32; 2],
}

/// A loaded TrueType / OpenType font entry with both fontdue rasterizer and raw font bytes for OpenType shaping.
#[derive(Clone)]
pub struct LoadedFont {
    pub name: String,
    pub font: fontdue::Font,
    pub bytes: Vec<u8>,
    pub collection_index: u32,
}

pub struct TextRenderer {
    pub font: fontdue::Font,
    pub fallback_fonts: Vec<fontdue::Font>,
    pub primary_font_entry: LoadedFont,
    pub fallback_font_entries: Vec<LoadedFont>,

    pub atlas_tex: wgpu::Texture,
    pub atlas_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub instance_capacity: usize,
    pub instances: Vec<GlyphInstance>,
    pub batches: Vec<TextBatch>,
    pub active_scissor: Option<ScissorRect>,
    pub screen_size: [f32; 2],

    pub font_system: cosmic_text::FontSystem,
    pub swash_cache: cosmic_text::SwashCache,

    // Shelf packing state (2048x2048 RGBA atlas)
    pub atlas_size: u32,
    indexed_cache: HashMap<(u8, u16, u32), CachedGlyph>,
    emoji_cache: HashMap<(u64, u32), CachedGlyph>,
    current_x: u32,
    current_y: u32,
    row_height: u32,
    padding: u32,

    pub ui_scale: f32,
    pub global_scale_multiplier: f32,
}

/// Applies global typography scale multiplier (ui_scale * 1.35) and enforces a hard floor >= 12.0px.
#[inline]
pub fn compute_effective_font_size(requested_size: f32, global_scale: f32) -> f32 {
    (requested_size * global_scale).round().max(12.0)
}

#[inline]
fn emoji_str_hash(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Returns optimal high-contrast text color based on background luminance.
/// Chooses Dark Obsidian (#14120E) on bright yellow/amber/light fills (lum > 0.42),
/// and Crisp White on dark slate/blue/emerald fills.
#[inline]
pub fn optimal_text_color_for_fill([r, g, b, _]: [f32; 4]) -> [f32; 4] {
    let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    if lum > 0.42 {
        [0.08, 0.07, 0.05, 1.0] // Crisp Dark Obsidian (#14120E) for Yellow/Amber/Light fills
    } else {
        [0.98, 0.99, 1.00, 1.0] // Crisp White for Dark Slate/Blue/Emerald fills
    }
}

/// Computes WCAG 2.1 contrast ratio between two RGBA colors.
pub fn contrast_ratio([r1, g1, b1, _]: [f32; 4], [r2, g2, b2, _]: [f32; 4]) -> f32 {
    let l1 = 0.2126 * r1 + 0.7152 * g1 + 0.0722 * b1;
    let l2 = 0.2126 * r2 + 0.7152 * g2 + 0.0722 * b2;
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

#[inline]
pub fn resolve_glyph_for_char_with_fonts(
    primary: &fontdue::Font,
    fallbacks: &[LoadedFont],
    ch: char,
) -> (u8, u16) {
    let idx = primary.lookup_glyph_index(ch);
    if idx != 0 || ch == ' ' {
        return (0, idx);
    }
    for (i, fallback) in fallbacks.iter().enumerate() {
        let f_idx = fallback.font.lookup_glyph_index(ch);
        if f_idx != 0 {
            return ((i + 1) as u8, f_idx);
        }
    }
    (0, 0)
}

/// Measures single-line text pixel advance width and line height using fontdue metrics at px_size.
pub fn measure_text_with_fonts(
    primary: &fontdue::Font,
    fallbacks: &[LoadedFont],
    text: &str,
    px_size: f32,
) -> (f32, f32) {
    let mut total_w = 0.0f32;
    let sanitized = sanitize_maindy_text(text);
    for ch in sanitized.chars() {
        if ch == '\n' || ch == '\r' {
            continue;
        }
        if crate::emoji::is_emoji_codepoint(ch) {
            total_w += px_size * 1.05;
            continue;
        }
        let (font_idx, glyph_id) = resolve_glyph_for_char_with_fonts(primary, fallbacks, ch);
        let adv = if font_idx == 0 {
            if glyph_id != 0 {
                primary.metrics_indexed(glyph_id, px_size).advance_width
            } else {
                primary.metrics(ch, px_size).advance_width
            }
        } else if let Some(fb) = fallbacks.get((font_idx - 1) as usize) {
            if glyph_id != 0 {
                fb.font.metrics_indexed(glyph_id, px_size).advance_width
            } else {
                fb.font.metrics(ch, px_size).advance_width
            }
        } else {
            primary.metrics(ch, px_size).advance_width
        };
        let adv = if adv > 0.0 { adv } else { px_size * 0.55 };
        total_w += adv;
    }
    (total_w, px_size * 1.32)
}

/// Fits single-line text within max_width_px at px_size, truncating with "..." if necessary.
pub fn fit_text_with_fonts(
    primary: &fontdue::Font,
    fallbacks: &[LoadedFont],
    text: &str,
    px_size: f32,
    max_width_px: f32,
) -> String {
    if max_width_px <= 8.0 {
        return String::new();
    }
    if measure_text_with_fonts(primary, fallbacks, text, px_size).0 <= max_width_px {
        return text.to_string();
    }
    let ellipsis = "...";
    let ellipsis_w = measure_text_with_fonts(primary, fallbacks, ellipsis, px_size).0;
    if ellipsis_w >= max_width_px {
        return String::new();
    }

    let mut running_w = 0.0f32;
    let mut result = String::new();
    let sanitized = sanitize_maindy_text(text);
    for ch in sanitized.chars() {
        if ch == '\n' || ch == '\r' {
            break;
        }
        let ch_w = if crate::emoji::is_emoji_codepoint(ch) {
            px_size * 1.05
        } else {
            let (font_idx, glyph_id) = resolve_glyph_for_char_with_fonts(primary, fallbacks, ch);
            let adv = if font_idx == 0 {
                if glyph_id != 0 {
                    primary.metrics_indexed(glyph_id, px_size).advance_width
                } else {
                    primary.metrics(ch, px_size).advance_width
                }
            } else if let Some(fb) = fallbacks.get((font_idx - 1) as usize) {
                if glyph_id != 0 {
                    fb.font.metrics_indexed(glyph_id, px_size).advance_width
                } else {
                    fb.font.metrics(ch, px_size).advance_width
                }
            } else {
                primary.metrics(ch, px_size).advance_width
            };
            if adv > 0.0 {
                adv
            } else {
                px_size * 0.55
            }
        };

        if running_w + ch_w + ellipsis_w > max_width_px {
            break;
        }
        running_w += ch_w;
        result.push(ch);
    }

    result.push_str(ellipsis);
    result
}

impl TextRenderer {
    /// Measures single shaped run advance width matching queue_shaped_run 1:1.
    pub fn measure_shaped_run(&self, run_text: &str, effective_size: f32) -> f32 {
        let font_idx = self.resolve_font_for_text(run_text);
        let font_entry = if font_idx == 0 {
            &self.primary_font_entry
        } else {
            match self.fallback_font_entries.get(font_idx - 1) {
                Some(f) => f,
                None => &self.primary_font_entry,
            }
        };

        let is_arabic = run_text
            .chars()
            .any(|c| (0x0600..=0x06FF).contains(&(c as u32)));

        if let Some(face) =
            rustybuzz::Face::from_slice(&font_entry.bytes, font_entry.collection_index)
        {
            let mut buf = rustybuzz::UnicodeBuffer::new();
            buf.push_str(run_text);
            buf.guess_segment_properties();
            if is_arabic {
                buf.set_direction(rustybuzz::Direction::RightToLeft);
            }

            let glyph_buffer = rustybuzz::shape(&face, &[], buf);
            let upem = face.units_per_em() as f32;
            let scale = effective_size / upem;
            let mut run_w = 0.0f32;

            for (info, pos) in glyph_buffer
                .glyph_infos()
                .iter()
                .zip(glyph_buffer.glyph_positions().iter())
            {
                let glyph_id = info.glyph_id as u16;
                let x_adv = pos.x_advance as f32 * scale;
                if glyph_id != 0 {
                    run_w += x_adv;
                } else {
                    run_w += effective_size * 0.45;
                }
            }
            run_w
        } else {
            let mut run_w = 0.0f32;
            for ch in run_text.chars() {
                let (f_idx, g_id) = self.resolve_glyph_for_char(ch);
                let adv = if f_idx == 0 {
                    if g_id != 0 {
                        self.primary_font_entry
                            .font
                            .metrics_indexed(g_id, effective_size)
                            .advance_width
                    } else {
                        self.primary_font_entry
                            .font
                            .metrics(ch, effective_size)
                            .advance_width
                    }
                } else if let Some(fb) = self.fallback_font_entries.get((f_idx - 1) as usize) {
                    if g_id != 0 {
                        fb.font.metrics_indexed(g_id, effective_size).advance_width
                    } else {
                        fb.font.metrics(ch, effective_size).advance_width
                    }
                } else {
                    self.primary_font_entry
                        .font
                        .metrics(ch, effective_size)
                        .advance_width
                };
                run_w += if adv > 0.0 {
                    adv
                } else {
                    effective_size * 0.55
                };
            }
            run_w
        }
    }

    /// Measures exact single-line pixel advance width and line height using the exact same OpenType shaping and glyph advance logic as queue_shaped_run.
    pub fn measure_line_exact_px(&self, text: &str, effective_size: f32) -> (f32, f32) {
        let mut max_w = 0.0f32;
        let mut current_text = String::new();

        for line in text.split('\n') {
            let mut line_w = 0.0f32;
            current_text.clear();
            for g in line.graphemes(true) {
                if crate::emoji::is_emoji_sequence(g)
                    || g.chars().any(crate::emoji::is_emoji_codepoint)
                {
                    if !current_text.is_empty() {
                        line_w += self.measure_shaped_run(&current_text, effective_size);
                        current_text.clear();
                    }
                    let emoji_sz = (effective_size * 1.05).round().clamp(12.0, 128.0);
                    line_w += emoji_sz + 2.0;
                } else {
                    current_text.push_str(g);
                }
            }
            if !current_text.is_empty() {
                line_w += self.measure_shaped_run(&current_text, effective_size);
            }
            max_w = max_w.max(line_w);
        }

        (max_w, effective_size * 1.35)
    }

    /// Measures exact single-line pixel advance width and line height using OpenType shaping at px_size (scaled by global typography scale).
    #[inline]
    pub fn measure_line_px(&self, text: &str, px_size: f32) -> (f32, f32) {
        let eff_size = self.compute_effective_size(px_size);
        self.measure_line_exact_px(text, eff_size)
    }

    /// Fits single-line text within max_width_px at px_size (scaled once), truncating with "..." if necessary.
    pub fn fit_text_to_px(&self, text: &str, px_size: f32, max_width_px: f32) -> String {
        if max_width_px <= 8.0 {
            return String::new();
        }
        let eff_size = self.compute_effective_size(px_size);
        if self.measure_line_exact_px(text, eff_size).0 <= max_width_px {
            return text.to_string();
        }
        let ellipsis = "...";
        let ellipsis_w = self.measure_line_exact_px(ellipsis, eff_size).0;
        if ellipsis_w >= max_width_px {
            return String::new();
        }

        let mut running_w = 0.0f32;
        let mut result = String::new();
        for g in text.graphemes(true) {
            if g == "\n" || g == "\r" {
                break;
            }
            let g_w = if crate::emoji::is_emoji_sequence(g)
                || g.chars().any(crate::emoji::is_emoji_codepoint)
            {
                let emoji_sz = (eff_size * 1.05).round().clamp(12.0, 128.0);
                emoji_sz + 2.0
            } else {
                self.measure_shaped_run(g, eff_size)
            };
            if running_w + g_w + ellipsis_w > max_width_px {
                break;
            }
            running_w += g_w;
            result.push_str(g);
        }

        if result.is_empty() {
            if text.graphemes(true).count() <= 1 {
                return text.to_string();
            }
            return String::new();
        }

        result.push_str(ellipsis);
        result
    }

    /// Queues text bounded strictly within max_w pixels. Always truncates with "..." before rasterizing,
    /// and enforces hardware scissor / clip bounds to guarantee text never bleeds outside max_w.
    #[allow(clippy::too_many_arguments)]
    pub fn queue_text_bounded(
        &mut self,
        queue: &wgpu::Queue,
        text: &str,
        start_x: f32,
        start_y: f32,
        max_w: f32,
        font_size: f32,
        color: [f32; 4],
    ) {
        if max_w <= 0.0 {
            return;
        }
        let fitted = self.fit_text_to_px(text, font_size, max_w);
        let eff_size = self.compute_effective_size(font_size);
        let clip_rect = [start_x, start_y - 2.0, max_w, eff_size * 1.5];
        self.queue_text_clipped(
            queue,
            &fitted,
            start_x,
            start_y,
            font_size,
            color,
            Some(clip_rect),
            &[],
        );
    }

    /// Queues single-line text centered inside rect [x, y, w, h], bounded by (w - 8.0),
    /// strictly clipped to the rect so it never renders outside the bounding box.
    pub fn queue_text_centered_in_rect(
        &mut self,
        queue: &wgpu::Queue,
        text: &str,
        rect: [f32; 4],
        font_size: f32,
        color: [f32; 4],
    ) {
        let max_w = (rect[2] - 8.0).max(0.0);
        let fitted = self.fit_text_to_px(text, font_size, max_w);
        let (tw, th) = self.measure_line_px(&fitted, font_size);
        let tx = (rect[0] + (rect[2] - tw) * 0.5).round();
        let ty = (rect[1] + (rect[3] - th) * 0.5).round();
        self.queue_text_clipped(queue, &fitted, tx, ty, font_size, color, Some(rect), &[]);
    }

    /// Queues split row (left_text aligned left, right_text aligned right) ensuring zero collision,
    /// strictly clipped to rect so neither left nor right text bleeds outside.
    #[allow(clippy::too_many_arguments)]
    pub fn queue_split_row(
        &mut self,
        queue: &wgpu::Queue,
        left_text: &str,
        right_text: &str,
        rect: [f32; 4],
        font_size: f32,
        left_color: [f32; 4],
        right_color: [f32; 4],
    ) {
        let right_w = self.measure_line_px(right_text, font_size).0;
        let right_x = rect[0] + rect[2] - right_w;
        let left_max_w = (rect[2] - right_w - 8.0).max(0.0);
        let fitted_left = self.fit_text_to_px(left_text, font_size, left_max_w);
        self.queue_text_clipped(
            queue,
            right_text,
            right_x,
            rect[1],
            font_size,
            right_color,
            Some(rect),
            &[],
        );
        self.queue_text_clipped(
            queue,
            &fitted_left,
            rect[0],
            rect[1],
            font_size,
            left_color,
            Some(rect),
            &[],
        );
    }

    /// Queues multi-line text wrapped strictly within rect [x, y, w, h] with horizontal and vertical padding.
    /// Uses `wrap_text_chars` based on `(w - 2 * pad_x) / avg_char_w`, and advances lines by `line_height`.
    /// Clips all lines strictly to rect using GPU scissor and glyph boundary culling,
    /// guaranteeing text never renders outside its bounding box.
    #[allow(clippy::too_many_arguments)]
    pub fn queue_text_wrapped_in_rect(
        &mut self,
        queue: &wgpu::Queue,
        text: &str,
        rect: [f32; 4],
        pad_x: f32,
        pad_y: f32,
        font_size: f32,
        color: [f32; 4],
    ) {
        let avail_w = (rect[2] - pad_x * 2.0).max(0.0);
        let avail_h = (rect[3] - pad_y * 2.0).max(0.0);
        if avail_w <= 0.0 || avail_h <= 0.0 {
            return;
        }

        let eff_size = self.compute_effective_size(font_size);
        let line_height = (eff_size * 1.35).round();
        let avg_char_w = (eff_size * 0.55).max(1.0);
        let max_chars = max_chars_for_width(avail_w, avg_char_w);

        let wrapped = wrap_text_chars(text, max_chars);
        let mut cursor_y = rect[1] + pad_y;
        let start_x = rect[0] + pad_x;

        for line in wrapped.lines() {
            if cursor_y + eff_size > rect[1] + rect[3] {
                break;
            }
            let fitted = self.fit_text_to_px(line, font_size, avail_w);
            self.queue_text_clipped(
                queue,
                &fitted,
                start_x,
                cursor_y,
                font_size,
                color,
                Some(rect),
                &[],
            );
            cursor_y += line_height;
        }
    }

    pub fn create_font_system_with_emoji() -> cosmic_text::FontSystem {
        let mut font_system = cosmic_text::FontSystem::new();

        #[cfg(target_os = "windows")]
        {
            let emoji_path = std::path::Path::new(r"C:\Windows\Fonts\seguiemj.ttf");
            if emoji_path.exists() {
                let _ = font_system.db_mut().load_font_file(emoji_path);
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            for path in [
                "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
                "/usr/share/fonts/noto/NotoColorEmoji.ttf",
            ] {
                let p = std::path::Path::new(path);
                if p.exists() {
                    let _ = font_system.db_mut().load_font_file(p);
                    break;
                }
            }
        }

        font_system
    }

    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Result<Self, String> {
        let primary_font_entry = Self::load_system_font_entry()?;
        let fallback_font_entries = Self::load_fallback_font_entries();

        let font = primary_font_entry.font.clone();
        let fallback_fonts = fallback_font_entries
            .iter()
            .map(|e| e.font.clone())
            .collect();

        let atlas_size = 2048u32;
        let atlas_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("mindset_glyph_atlas_tex_rgba"),
            size: wgpu::Extent3d {
                width: atlas_size,
                height: atlas_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let atlas_view = atlas_tex.create_view(&wgpu::TextureViewDescriptor {
            label: Some("mindset_glyph_atlas_view_rgba"),
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("mindset_glyph_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mindset_text_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/text.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("mindset_text_bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("mindset_text_uniforms"),
            size: std::mem::size_of::<TextUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mindset_text_bg"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mindset_text_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GlyphInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 48,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mindset_text_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[instance_layout],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let initial_capacity = 512;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("mindset_text_instances"),
            size: (initial_capacity * std::mem::size_of::<GlyphInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let font_system = Self::create_font_system_with_emoji();
        let swash_cache = cosmic_text::SwashCache::new();

        Ok(Self {
            font,
            fallback_fonts,
            primary_font_entry,
            fallback_font_entries,
            font_system,
            swash_cache,
            atlas_tex,
            atlas_view,
            sampler,
            pipeline,
            bind_group_layout,
            bind_group,
            uniform_buffer,
            instance_buffer,
            instance_capacity: initial_capacity,
            instances: Vec::with_capacity(initial_capacity),
            batches: Vec::new(),
            active_scissor: None,
            screen_size: [1280.0, 800.0],
            atlas_size,
            indexed_cache: HashMap::new(),
            emoji_cache: HashMap::new(),
            current_x: 1,
            current_y: 1,
            row_height: 0,
            padding: 2,
            ui_scale: 1.15,
            global_scale_multiplier: 1.35,
        })
    }

    #[inline]
    pub fn set_ui_scale(&mut self, scale: f32) {
        self.ui_scale = scale.clamp(0.80, 2.0);
    }

    #[inline]
    pub fn global_text_scale(&self) -> f32 {
        self.ui_scale * self.global_scale_multiplier
    }

    #[inline]
    pub fn compute_effective_font_size(requested_size: f32, global_scale: f32) -> f32 {
        compute_effective_font_size(requested_size, global_scale)
    }

    #[inline]
    pub fn compute_effective_size(&self, requested_size: f32) -> f32 {
        compute_effective_font_size(requested_size, self.global_text_scale())
    }

    pub fn load_system_font_entry() -> Result<LoadedFont, String> {
        let candidates = [
            "C:\\Windows\\Fonts\\segoeui.ttf",
            "C:\\Windows\\Fonts\\arial.ttf",
            "C:\\Windows\\Fonts\\calibri.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/google-noto/NotoSans-Regular.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
            "/System/Library/Fonts/SFNS.ttf",
            "/System/Library/Fonts/Helvetica.ttc",
        ];

        for path in &candidates {
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok(font) =
                    fontdue::Font::from_bytes(bytes.clone(), fontdue::FontSettings::default())
                {
                    log::info!("Loaded system font from: {path}");
                    return Ok(LoadedFont {
                        name: path.to_string(),
                        font,
                        bytes,
                        collection_index: 0,
                    });
                }
            }
        }

        Err("No suitable TrueType font found on host system".to_string())
    }

    pub fn load_fallback_font_entries() -> Vec<LoadedFont> {
        let candidates = [
            ("C:\\Windows\\Fonts\\Nirmala.ttc", 0),
            ("C:\\Windows\\Fonts\\Nirmala.ttc", 1),
            ("C:\\Windows\\Fonts\\seguiemj.ttf", 0),
            ("C:\\Windows\\Fonts\\msyh.ttc", 0),
            ("C:\\Windows\\Fonts\\tahoma.ttf", 0),
            ("C:\\Windows\\Fonts\\arial.ttf", 0),
            ("C:\\Windows\\Fonts\\consola.ttf", 0),
            (
                "/usr/share/fonts/truetype/noto/NotoSansTamil-Regular.ttf",
                0,
            ),
            (
                "/usr/share/fonts/truetype/noto/NotoSansDevanagari-Regular.ttf",
                0,
            ),
            ("/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf", 0),
            ("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf", 0),
        ];

        let mut fonts = Vec::new();
        for (path, idx) in &candidates {
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok(font) = fontdue::Font::from_bytes(
                    bytes.clone(),
                    fontdue::FontSettings {
                        collection_index: *idx,
                        ..Default::default()
                    },
                ) {
                    fonts.push(LoadedFont {
                        name: format!("{path}#{idx}"),
                        font,
                        bytes,
                        collection_index: *idx,
                    });
                }
            }
        }
        fonts
    }

    pub fn load_system_font() -> Result<fontdue::Font, String> {
        Self::load_system_font_entry().map(|e| e.font)
    }

    pub fn load_fallback_fonts() -> Vec<fontdue::Font> {
        Self::load_fallback_font_entries()
            .into_iter()
            .map(|e| e.font)
            .collect()
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.batches.clear();
        self.active_scissor = None;
    }

    #[inline]
    pub fn set_scissor(&mut self, scissor: Option<ScissorRect>) {
        if self.active_scissor != scissor {
            self.active_scissor = scissor;
            let current_len = self.instances.len() as u32;
            self.batches.push(TextBatch {
                scissor,
                start: current_len,
                count: 0,
            });
        }
    }

    #[inline]
    pub fn push_instance(&mut self, instance: GlyphInstance) {
        let current_len = self.instances.len() as u32;
        if let Some(last) = self.batches.last_mut() {
            if last.scissor == self.active_scissor {
                last.count += 1;
                self.instances.push(instance);
                return;
            }
        }
        self.batches.push(TextBatch {
            scissor: self.active_scissor,
            start: current_len,
            count: 1,
        });
        self.instances.push(instance);
    }

    /// Caches an OpenType shaped glyph index into the RGBA atlas.
    #[inline]
    pub fn cache_indexed_glyph(
        &mut self,
        queue: &wgpu::Queue,
        font_idx: u8,
        glyph_id: u16,
        size_px: f32,
    ) -> Option<CachedGlyph> {
        let key = (font_idx, glyph_id, size_px.round() as u32);
        if let Some(cached) = self.indexed_cache.get(&key) {
            return Some(*cached);
        }

        let font_entry = if font_idx == 0 {
            &self.primary_font_entry
        } else {
            self.fallback_font_entries.get((font_idx - 1) as usize)?
        };

        let (metrics, bitmap) = font_entry.font.rasterize_indexed(glyph_id, size_px);
        let w = metrics.width as u32;
        let h = metrics.height as u32;

        if w == 0 || h == 0 {
            let empty = CachedGlyph {
                uv_min: [0.0, 0.0],
                uv_max: [0.0, 0.0],
                width: 0.0,
                height: 0.0,
                xmin: metrics.xmin as f32,
                ymin: metrics.ymin as f32,
                advance_width: metrics.advance_width,
            };
            self.indexed_cache.insert(key, empty);
            return Some(empty);
        }

        // Shelf packing test
        if self.current_x + w + self.padding > self.atlas_size {
            self.current_x = self.padding;
            self.current_y += self.row_height + self.padding;
            self.row_height = 0;
        }

        if self.current_y + h + self.padding > self.atlas_size {
            log::info!("Glyph atlas full; resetting cache shelf packing");
            self.current_x = self.padding;
            self.current_y = self.padding;
            self.row_height = 0;
            self.indexed_cache.clear();
            self.emoji_cache.clear();
        }

        let slot_x = self.current_x;
        let slot_y = self.current_y;
        self.row_height = self.row_height.max(h);
        self.current_x += w + self.padding;

        // Convert monochrome alpha to RGBA: [255, 255, 255, alpha]
        let mut rgba_bitmap = Vec::with_capacity((w * h * 4) as usize);
        for &alpha in &bitmap {
            rgba_bitmap.extend_from_slice(&[255, 255, 255, alpha]);
        }

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas_tex,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: slot_x,
                    y: slot_y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_bitmap,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: Some(h),
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );

        let uv_min = [
            slot_x as f32 / self.atlas_size as f32,
            slot_y as f32 / self.atlas_size as f32,
        ];
        let uv_max = [
            (slot_x + w) as f32 / self.atlas_size as f32,
            (slot_y + h) as f32 / self.atlas_size as f32,
        ];

        let cached = CachedGlyph {
            uv_min,
            uv_max,
            width: metrics.width as f32,
            height: metrics.height as f32,
            xmin: metrics.xmin as f32,
            ymin: metrics.ymin as f32,
            advance_width: metrics.advance_width,
        };

        self.indexed_cache.insert(key, cached);
        Some(cached)
    }

    /// Rasterizes an emoji string or sequence using cosmic_text / SwashCache directly into an RGBA8 bitmap.
    pub fn rasterize_emoji_via_swash(
        &mut self,
        emoji_str: &str,
        size_px: f32,
    ) -> Option<(u32, u32, Vec<u8>)> {
        let families = [Some("Segoe UI Emoji"), Some("Noto Color Emoji"), None];
        for fam in families {
            let mut attrs = cosmic_text::Attrs::new();
            if let Some(name) = fam {
                attrs = attrs.family(cosmic_text::Family::Name(name));
            }
            let mut buffer = cosmic_text::Buffer::new(
                &mut self.font_system,
                cosmic_text::Metrics::new(size_px, size_px * 1.2),
            );
            buffer.set_text(emoji_str, &attrs, cosmic_text::Shaping::Advanced, None);
            buffer.shape_until_scroll(&mut self.font_system, false);

            for run in buffer.layout_runs() {
                for glyph in run.glyphs {
                    let cache_key = glyph.physical((0.0, 0.0), 1.0).cache_key;
                    if let Some(rasterized) = rasterize_glyph_rgba(
                        &mut self.swash_cache,
                        &mut self.font_system,
                        cache_key,
                    ) {
                        return Some((rasterized.width, rasterized.height, rasterized.rgba_pixels));
                    }
                }
            }
        }
        None
    }

    /// Caches a color emoji into the RGBA atlas.
    #[inline]
    pub fn cache_emoji(
        &mut self,
        queue: &wgpu::Queue,
        emoji_str: &str,
        size_px: f32,
    ) -> Option<CachedGlyph> {
        let hash = emoji_str_hash(emoji_str);
        let key = (hash, size_px.round() as u32);
        if let Some(cached) = self.emoji_cache.get(&key) {
            return Some(*cached);
        }

        let (w, h, rgba_bitmap) = self
            .rasterize_emoji_via_swash(emoji_str, size_px)
            .or_else(|| crate::emoji::rasterize_color_emoji(emoji_str, size_px.round() as u32))?;

        // Shelf packing test
        if self.current_x + w + self.padding > self.atlas_size {
            self.current_x = self.padding;
            self.current_y += self.row_height + self.padding;
            self.row_height = 0;
        }

        if self.current_y + h + self.padding > self.atlas_size {
            log::info!("Glyph atlas full; resetting cache shelf packing");
            self.current_x = self.padding;
            self.current_y = self.padding;
            self.row_height = 0;
            self.indexed_cache.clear();
            self.emoji_cache.clear();
        }

        let slot_x = self.current_x;
        let slot_y = self.current_y;
        self.row_height = self.row_height.max(h);
        self.current_x += w + self.padding;

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas_tex,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: slot_x,
                    y: slot_y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_bitmap,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: Some(h),
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );

        let uv_min = [
            slot_x as f32 / self.atlas_size as f32,
            slot_y as f32 / self.atlas_size as f32,
        ];
        let uv_max = [
            (slot_x + w) as f32 / self.atlas_size as f32,
            (slot_y + h) as f32 / self.atlas_size as f32,
        ];

        let cached = CachedGlyph {
            uv_min,
            uv_max,
            width: w as f32,
            height: h as f32,
            xmin: 0.0,
            ymin: 0.0,
            advance_width: w as f32,
        };

        self.emoji_cache.insert(key, cached);
        Some(cached)
    }

    /// Backward-compatible single character glyph cache.
    pub fn cache_glyph(
        &mut self,
        queue: &wgpu::Queue,
        ch: char,
        size_px: f32,
    ) -> Option<CachedGlyph> {
        let (font_idx, glyph_id) = self.resolve_glyph_for_char(ch);
        if glyph_id != 0 || ch == ' ' {
            self.cache_indexed_glyph(queue, font_idx, glyph_id, size_px)
        } else {
            None
        }
    }

    #[inline]
    fn resolve_glyph_for_char(&self, ch: char) -> (u8, u16) {
        let idx = self.primary_font_entry.font.lookup_glyph_index(ch);
        if idx != 0 || ch == ' ' {
            return (0, idx);
        }
        for (i, fallback) in self.fallback_font_entries.iter().enumerate() {
            let f_idx = fallback.font.lookup_glyph_index(ch);
            if f_idx != 0 {
                return ((i + 1) as u8, f_idx);
            }
        }
        (0, 0)
    }

    #[inline]
    fn resolve_font_for_text(&self, text: &str) -> usize {
        // Script detection
        for c in text.chars() {
            let u = c as u32;
            if (0x0B80..=0x0BFF).contains(&u) || (0x0900..=0x097F).contains(&u) {
                // Tamil or Devanagari -> find Nirmala UI
                for (i, f) in self.fallback_font_entries.iter().enumerate() {
                    if f.name.contains("Nirmala")
                        || f.name.contains("Tamil")
                        || f.name.contains("Devanagari")
                    {
                        return i + 1;
                    }
                }
            } else if (0x0600..=0x06FF).contains(&u) {
                // Arabic -> find Tahoma / Arial / Arabic fallback
                for (i, f) in self.fallback_font_entries.iter().enumerate() {
                    if f.name.contains("tahoma") || f.name.contains("arial") {
                        return i + 1;
                    }
                }
            } else if (0x4E00..=0x9FFF).contains(&u) {
                // Chinese -> find YaHei
                for (i, f) in self.fallback_font_entries.iter().enumerate() {
                    if f.name.contains("msyh") {
                        return i + 1;
                    }
                }
            }
        }

        // Check if primary font has first non-whitespace
        if let Some(first_char) = text.chars().find(|c| !c.is_whitespace()) {
            if self.primary_font_entry.font.lookup_glyph_index(first_char) == 0 {
                for (i, f) in self.fallback_font_entries.iter().enumerate() {
                    if f.font.lookup_glyph_index(first_char) != 0 {
                        return i + 1;
                    }
                }
            }
        }

        0 // Primary font
    }

    pub fn queue_text(
        &mut self,
        queue: &wgpu::Queue,
        text: &str,
        start_x: f32,
        start_y: f32,
        font_size: f32,
        color: [f32; 4],
    ) {
        self.queue_text_clipped(queue, text, start_x, start_y, font_size, color, None, &[]);
    }

    /// Queues shaped multilingual text, ligatures, Indic scripts, Arabic BiDi, and color emojis
    /// with boundary clipping and occlusion culling.
    #[allow(clippy::too_many_arguments)]
    pub fn queue_text_clipped(
        &mut self,
        queue: &wgpu::Queue,
        text: &str,
        start_x: f32,
        start_y: f32,
        font_size: f32,
        color: [f32; 4],
        clip_bounds: Option<[f32; 4]>,
        occlusions: &[[f32; 4]],
    ) {
        let prev_scissor = self.active_scissor;
        if let Some([bx, by, bw, bh]) = clip_bounds {
            let sc = ScissorRect {
                x: bx.max(0.0).round() as u32,
                y: by.max(0.0).round() as u32,
                width: bw.max(1.0).round() as u32,
                height: bh.max(1.0).round() as u32,
            };
            self.set_scissor(Some(sc));
        } else {
            self.set_scissor(None);
        }

        let effective_size = self.compute_effective_size(font_size);
        let line_height = effective_size * 1.35;
        let mut cursor_y = start_y;

        for line in text.split('\n') {
            let mut cursor_x = start_x;

            // Segment line into tokens (emojis vs text words) using grapheme clusters
            let mut current_text = String::new();

            for g in line.graphemes(true) {
                if crate::emoji::is_emoji_sequence(g)
                    || g.chars().any(crate::emoji::is_emoji_codepoint)
                {
                    // Flush accumulated text first
                    if !current_text.is_empty() {
                        self.queue_shaped_run(
                            queue,
                            &current_text,
                            &mut cursor_x,
                            cursor_y,
                            effective_size,
                            color,
                            clip_bounds,
                            occlusions,
                        );
                        current_text.clear();
                    }
                    // Handle emoji (including multi-codepoint ZWJ sequences)
                    let emoji_sz = effective_size * 1.05;
                    if let Some(cached) = self.cache_emoji(queue, g, emoji_sz) {
                        let gx = cursor_x;
                        let gy = cursor_y + effective_size * 0.82 - (cached.height * 0.88);
                        let cx = gx + cached.width * 0.5;
                        let cy = gy + cached.height * 0.5;

                        let mut visible = true;
                        if let Some([bx, by, bw, bh]) = clip_bounds {
                            if cx < bx || cx > bx + bw || cy < by || cy > by + bh {
                                visible = false;
                            }
                        }
                        if visible {
                            for &[ox, oy, ow, oh] in occlusions {
                                if cx >= ox && cx <= ox + ow && cy >= oy && cy <= oy + oh {
                                    visible = false;
                                    break;
                                }
                            }
                        }
                        if visible {
                            self.push_instance(GlyphInstance {
                                pos: [gx, gy],
                                size: [cached.width, cached.height],
                                uv_min: cached.uv_min,
                                uv_max: cached.uv_max,
                                color: [1.0, 1.0, 1.0, color[3]], // Full RGBA emoji
                                is_color: 1,
                                _pad: [0; 3],
                            });
                        }
                        cursor_x += cached.advance_width + 2.0;
                    } else {
                        cursor_x += emoji_sz;
                    }
                } else {
                    current_text.push_str(g);
                }
            }

            if !current_text.is_empty() {
                self.queue_shaped_run(
                    queue,
                    &current_text,
                    &mut cursor_x,
                    cursor_y,
                    effective_size,
                    color,
                    clip_bounds,
                    occlusions,
                );
            }

            cursor_y += line_height;
        }

        self.set_scissor(prev_scissor);
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn queue_shaped_run(
        &mut self,
        queue: &wgpu::Queue,
        run_text: &str,
        cursor_x: &mut f32,
        cursor_y: f32,
        effective_size: f32,
        color: [f32; 4],
        clip_bounds: Option<[f32; 4]>,
        occlusions: &[[f32; 4]],
    ) {
        let font_idx = self.resolve_font_for_text(run_text);
        let font_entry = if font_idx == 0 {
            &self.primary_font_entry
        } else {
            match self.fallback_font_entries.get(font_idx - 1) {
                Some(f) => f,
                None => &self.primary_font_entry,
            }
        };

        let is_arabic = run_text
            .chars()
            .any(|c| (0x0600..=0x06FF).contains(&(c as u32)));

        if let Some(face) =
            rustybuzz::Face::from_slice(&font_entry.bytes, font_entry.collection_index)
        {
            let mut buf = rustybuzz::UnicodeBuffer::new();
            buf.push_str(run_text);
            buf.guess_segment_properties();
            if is_arabic {
                buf.set_direction(rustybuzz::Direction::RightToLeft);
            }

            let glyph_buffer = rustybuzz::shape(&face, &[], buf);
            let upem = face.units_per_em() as f32;
            let scale = effective_size / upem;

            for (info, pos) in glyph_buffer
                .glyph_infos()
                .iter()
                .zip(glyph_buffer.glyph_positions().iter())
            {
                let glyph_id = info.glyph_id as u16;
                let x_adv = pos.x_advance as f32 * scale;
                let x_off = pos.x_offset as f32 * scale;
                let y_off = pos.y_offset as f32 * scale;

                if glyph_id != 0 {
                    if let Some(cached) =
                        self.cache_indexed_glyph(queue, font_idx as u8, glyph_id, effective_size)
                    {
                        if cached.width > 0.0 && cached.height > 0.0 {
                            let gx = *cursor_x + x_off + cached.xmin;
                            let gy = cursor_y + effective_size * 0.82
                                - y_off
                                - (cached.height + cached.ymin);
                            let cx = gx + cached.width * 0.5;
                            let cy = gy + cached.height * 0.5;

                            let mut visible = true;
                            if let Some([bx, by, bw, bh]) = clip_bounds {
                                if cx < bx || cx > bx + bw || cy < by || cy > by + bh {
                                    visible = false;
                                }
                            }
                            if visible {
                                for &[ox, oy, ow, oh] in occlusions {
                                    if cx >= ox && cx <= ox + ow && cy >= oy && cy <= oy + oh {
                                        visible = false;
                                        break;
                                    }
                                }
                            }
                            if visible {
                                self.push_instance(GlyphInstance {
                                    pos: [gx, gy],
                                    size: [cached.width, cached.height],
                                    uv_min: cached.uv_min,
                                    uv_max: cached.uv_max,
                                    color,
                                    is_color: 0,
                                    _pad: [0; 3],
                                });
                            }
                        }
                    }
                    *cursor_x += x_adv;
                } else {
                    // Check fallback for character
                    *cursor_x += effective_size * 0.45;
                }
            }
        } else {
            // Fallback scalar rasterization
            for ch in run_text.chars() {
                let (f_idx, g_id) = self.resolve_glyph_for_char(ch);
                if let Some(cached) = self.cache_indexed_glyph(queue, f_idx, g_id, effective_size) {
                    if cached.width > 0.0 && cached.height > 0.0 {
                        let gx = *cursor_x + cached.xmin;
                        let gy = cursor_y + effective_size * 0.82 - (cached.height + cached.ymin);
                        let cx = gx + cached.width * 0.5;
                        let cy = gy + cached.height * 0.5;

                        let mut visible = true;
                        if let Some([bx, by, bw, bh]) = clip_bounds {
                            if cx < bx || cx > bx + bw || cy < by || cy > by + bh {
                                visible = false;
                            }
                        }
                        if visible {
                            for &[ox, oy, ow, oh] in occlusions {
                                if cx >= ox && cx <= ox + ow && cy >= oy && cy <= oy + oh {
                                    visible = false;
                                    break;
                                }
                            }
                        }
                        if visible {
                            self.push_instance(GlyphInstance {
                                pos: [gx, gy],
                                size: [cached.width, cached.height],
                                uv_min: cached.uv_min,
                                uv_max: cached.uv_max,
                                color,
                                is_color: 0,
                                _pad: [0; 3],
                            });
                        }
                    }
                    *cursor_x += cached.advance_width;
                }
            }
        }
    }

    /// Measures text bounding width and height in physical pixels.
    pub fn measure_text(&self, text: &str, font_size: f32) -> (f32, f32) {
        let effective_size = self.compute_effective_size(font_size);
        let line_height = effective_size * 1.35;
        let mut max_w: f32 = 0.0;
        let mut total_h: f32 = 0.0;

        for line in text.split('\n') {
            total_h += line_height;
            let mut line_w: f32 = 0.0;
            for ch in line.chars() {
                if crate::emoji::is_emoji_codepoint(ch) {
                    line_w += effective_size * 1.15;
                } else {
                    let (_, g_id) = self.resolve_glyph_for_char(ch);
                    line_w += if g_id != 0 {
                        effective_size * 0.55
                    } else {
                        effective_size * 0.50
                    };
                }
            }
            max_w = max_w.max(line_w);
        }

        (max_w, total_h)
    }

    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, screen_size: [f32; 2]) {
        self.screen_size = screen_size;
        let uniforms = TextUniforms {
            screen_size,
            _pad: [0.0, 0.0],
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        if self.instances.is_empty() {
            return;
        }

        if self.batches.is_empty() {
            self.batches.push(TextBatch {
                scissor: self.active_scissor,
                start: 0,
                count: self.instances.len() as u32,
            });
        }

        if self.instances.len() > self.instance_capacity {
            self.instance_capacity = (self.instances.len() * 2).max(512);
            self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("mindset_text_instances_resized"),
                size: (self.instance_capacity * std::mem::size_of::<GlyphInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.instances),
        );
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if self.instances.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));

        let screen_w = self.screen_size[0].max(1.0) as u32;
        let screen_h = self.screen_size[1].max(1.0) as u32;

        if self.batches.is_empty() {
            pass.draw(0..6, 0..self.instances.len() as u32);
        } else {
            for batch in &self.batches {
                if batch.count == 0 {
                    continue;
                }
                if let Some(sc) = batch.scissor {
                    let sx = sc.x.min(screen_w);
                    let sy = sc.y.min(screen_h);
                    let sw = sc.width.min(screen_w.saturating_sub(sx)).max(1);
                    let sh = sc.height.min(screen_h.saturating_sub(sy)).max(1);
                    pass.set_scissor_rect(sx, sy, sw, sh);
                } else {
                    pass.set_scissor_rect(0, 0, screen_w, screen_h);
                }
                pass.draw(0..6, batch.start..(batch.start + batch.count));
            }
            pass.set_scissor_rect(0, 0, screen_w, screen_h);
        }
    }

    /// Wraps text to fit within a given pixel width using cosmic_text.
    pub fn wrap_text(&mut self, text: &str, max_width_px: f32, font_size: f32) -> Vec<String> {
        let eff_size = self.compute_effective_size(font_size);
        wrap_text_cosmic(&mut self.font_system, text, max_width_px, eff_size)
    }
}

/// Safely returns a prefix slice of a UTF-8 string containing at most `max_chars` Unicode scalar values.
/// Never panics on multi-byte UTF-8 boundaries (e.g. math formulas, symbols, Tamil, CJK, emoji).
#[inline]
pub fn safe_utf8_prefix(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        Some((byte_idx, _)) => &s[..byte_idx],
        None => s,
    }
}

/// Calculates the maximum number of characters that fit within a given pixel width,
/// based on average character width in pixels.
#[inline]
pub fn max_chars_for_width(max_width_px: f32, avg_char_w: f32) -> usize {
    if max_width_px <= 0.0 || avg_char_w <= 0.0 {
        0
    } else {
        (max_width_px / avg_char_w).floor() as usize
    }
}

/// Wraps text to fit strictly within a parent card's width minus horizontal padding.
/// Guarantees that text never renders outside its bounding box.
pub fn wrap_text_for_card_bounds(s: &str, card_width: f32, pad_x: f32, avg_char_w: f32) -> String {
    let avail_w = (card_width - pad_x * 2.0).max(0.0);
    let max_chars = max_chars_for_width(avail_w, avg_char_w);
    wrap_text_chars(s, max_chars)
}

/// Wraps text based on character count without splitting multi-byte UTF-8 graphemes.
/// Enforces strict line boundaries: no line will ever exceed `max_chars_per_line` graphemes.
/// If an individual word is longer than `max_chars_per_line`, it is strictly broken into
/// chunks of at most `max_chars_per_line` graphemes so text never bleeds outside parent boundaries.
pub fn wrap_text_chars(s: &str, max_chars_per_line: usize) -> String {
    if max_chars_per_line == 0 || s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut first_orig_line = true;

    for line in s.lines() {
        if !first_orig_line {
            result.push('\n');
        }
        first_orig_line = false;

        let mut line_chars = 0;
        let mut first_word_on_line = true;

        for word in line.split_whitespace() {
            let graphemes: Vec<&str> = word.graphemes(true).collect();
            let word_len = graphemes.len();
            if word_len == 0 {
                continue;
            }

            let space_needed = if first_word_on_line { 0 } else { 1 };

            if line_chars + space_needed + word_len <= max_chars_per_line {
                // Word completely fits on current line
                if !first_word_on_line {
                    result.push(' ');
                    line_chars += 1;
                }
                result.push_str(word);
                line_chars += word_len;
                first_word_on_line = false;
            } else if word_len <= max_chars_per_line {
                // Word does not fit on current line, but fits on a new line
                if !first_word_on_line {
                    result.push('\n');
                }
                result.push_str(word);
                line_chars = word_len;
                first_word_on_line = false;
            } else {
                // Word is LONGER than max_chars_per_line!
                // Must chunk word into slices of at most max_chars_per_line.
                if !first_word_on_line {
                    result.push('\n');
                    line_chars = 0;
                    first_word_on_line = true;
                }

                let mut offset = 0;
                while offset < word_len {
                    let chunk_size = max_chars_per_line.min(word_len - offset);
                    for &g in &graphemes[offset..offset + chunk_size] {
                        result.push_str(g);
                    }
                    offset += chunk_size;
                    line_chars = chunk_size;
                    first_word_on_line = false;

                    if offset < word_len {
                        result.push('\n');
                        line_chars = 0;
                        first_word_on_line = true;
                    }
                }
            }
        }
    }
    result
}

/// Wraps text based on exact pixel width using cosmic_text word wrapping.
pub fn wrap_text_cosmic(
    font_system: &mut cosmic_text::FontSystem,
    text: &str,
    max_width_px: f32,
    font_size: f32,
) -> Vec<String> {
    let mut buffer = cosmic_text::Buffer::new(
        font_system,
        cosmic_text::Metrics::new(font_size, font_size * 1.35),
    );
    buffer.set_wrap(cosmic_text::Wrap::Word);
    buffer.set_size(Some(max_width_px.max(40.0)), None);
    let attrs = cosmic_text::Attrs::new();
    buffer.set_text(text, &attrs, cosmic_text::Shaping::Advanced, None);
    buffer.shape_until_scroll(font_system, false);

    let mut lines = Vec::new();
    for run in buffer.layout_runs() {
        lines.push(run.text.to_string());
    }
    if lines.is_empty() {
        lines.push(text.to_string());
    }
    lines
}

/// Glyphon-compatible 4-coordinate integer text bounds for clipping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextBounds {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl TextBounds {
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn from_scissor(scissor: ScissorRect) -> Self {
        Self {
            left: scissor.x as i32,
            top: scissor.y as i32,
            right: (scissor.x + scissor.width) as i32,
            bottom: (scissor.y + scissor.height) as i32,
        }
    }

    /// Construct TextBounds directly from 2D coordinates [x, y, width, height].
    #[inline]
    pub fn from_bounds_f32(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            left: x.floor() as i32,
            top: y.floor() as i32,
            right: (x + w).ceil() as i32,
            bottom: (y + h).ceil() as i32,
        }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        (self.right - self.left).max(0)
    }

    #[inline]
    pub fn height(&self) -> i32 {
        (self.bottom - self.top).max(0)
    }
}

/// Glyphon-compatible TextArea descriptor enforcing Rule 4:
/// `scale` is strictly 1.0; physical scaling is multiplied directly into cosmic_text Metrics and layout coordinates.
pub struct TextArea<'a> {
    pub buffer: &'a cosmic_text::Buffer,
    pub left: f32,
    pub top: f32,
    pub scale: f32,
    pub bounds: TextBounds,
    pub default_color: [f32; 4],
}

impl<'a> TextArea<'a> {
    /// Constructs a new TextArea enforcing `scale == 1.0` per Rule 4.
    pub fn new(
        buffer: &'a cosmic_text::Buffer,
        left: f32,
        top: f32,
        bounds: TextBounds,
        default_color: [f32; 4],
    ) -> Self {
        Self {
            buffer,
            left,
            top,
            scale: 1.0, // Strictly 1.0 per Rule 4 (never window.scale_factor)
            bounds,
            default_color,
        }
    }
}

/// Shapes text using the strict 4-step sequence mandated by Rule 5 before reading layout runs:
/// 1. buffer.set_size(font_system, Some(max_w_phys), max_h_phys)
/// 2. buffer.set_text(text, &attrs, Shaping::Advanced, Some(align))
/// 3. line.set_align(Some(align)) on each line
/// 4. buffer.shape_until_scroll(font_system, false)
pub fn shape_text_strict<'a>(
    buffer: &'a mut cosmic_text::Buffer,
    font_system: &mut cosmic_text::FontSystem,
    text: &str,
    attrs: &cosmic_text::Attrs,
    max_w_phys: f32,
    max_h_phys: Option<f32>,
    align: cosmic_text::Align,
) -> &'a [cosmic_text::BufferLine] {
    // Step 1: Set size in physical pixels
    buffer.set_size(Some(max_w_phys), max_h_phys);
    // Step 2: Set text with advanced shaping and alignment
    buffer.set_text(text, attrs, cosmic_text::Shaping::Advanced, Some(align));
    // Step 3: Explicitly set alignment on each line
    for line in &mut buffer.lines {
        line.set_align(Some(align));
    }
    // Step 4: Shape until scroll
    buffer.shape_until_scroll(font_system, false);
    &buffer.lines
}

/// High-level text node context encapsulating HiDPI scaling, cosmic_text buffer, and TextArea generation.
pub struct TextNodeContext {
    pub text: String,
    pub font_size_logical: f32,
    pub line_height_logical: f32,
    pub scale_factor: f32,
    pub align: cosmic_text::Align,
    pub single_line_ellipsis: bool,
    pub buffer: cosmic_text::Buffer,
}

impl TextNodeContext {
    pub fn new(
        font_system: &mut cosmic_text::FontSystem,
        text: impl Into<String>,
        font_size_logical: f32,
        scale_factor: f32,
        align: cosmic_text::Align,
        single_line_ellipsis: bool,
    ) -> Self {
        let text = text.into();
        let line_height_logical = (font_size_logical * 1.40).round();
        let metrics = cosmic_text::Metrics::new(
            font_size_logical * scale_factor,
            line_height_logical * scale_factor,
        );
        let mut buffer = cosmic_text::Buffer::new(font_system, metrics);
        buffer.set_wrap(if single_line_ellipsis {
            cosmic_text::Wrap::None
        } else {
            cosmic_text::Wrap::Word
        });
        Self {
            text,
            font_size_logical,
            line_height_logical,
            scale_factor,
            align,
            single_line_ellipsis,
            buffer,
        }
    }

    pub fn shape(
        &mut self,
        font_system: &mut cosmic_text::FontSystem,
        max_w_logical: f32,
        max_h_logical: Option<f32>,
    ) {
        let max_w_phys = (max_w_logical * self.scale_factor).round();
        let max_h_phys = max_h_logical.map(|h| (h * self.scale_factor).round());
        let attrs = cosmic_text::Attrs::new();
        shape_text_strict(
            &mut self.buffer,
            font_system,
            &self.text,
            &attrs,
            max_w_phys,
            max_h_phys,
            self.align,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn to_text_area<'a>(
        &'a self,
        origin_x: f32,
        origin_y: f32,
        layout_x: f32,
        layout_y: f32,
        layout_w: f32,
        layout_h: f32,
        color: [f32; 4],
    ) -> TextArea<'a> {
        let left_px = ((origin_x + layout_x) * self.scale_factor).round();
        let top_px = ((origin_y + layout_y) * self.scale_factor).round();
        let right_px = ((origin_x + layout_x + layout_w) * self.scale_factor).round();
        let bottom_px = ((origin_y + layout_y + layout_h) * self.scale_factor).round();

        TextArea {
            buffer: &self.buffer,
            left: left_px,
            top: top_px,
            scale: 1.0, // Invariant: always 1.0; physical scale factor is multiplied into metrics and coordinates
            bounds: TextBounds {
                left: left_px as i32,
                top: top_px as i32,
                right: right_px as i32,
                bottom: bottom_px as i32,
            },
            default_color: color,
        }
    }
}

/// Strips multi-plane emojis and symbols that do not exist in standard desktop fonts,
/// preventing missing-glyph tofu [] boxes from appearing in AI responses.
pub fn sanitize_maindy_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        let u = ch as u32;
        let is_emoji = (0x1F300..=0x1FAFF).contains(&u)
            || (0x1F600..=0x1F64F).contains(&u)
            || (0x1F680..=0x1F6FF).contains(&u)
            || (0x2600..=0x27BF).contains(&u)
            || (0xFE00..=0xFE0F).contains(&u)
            || (0x1F900..=0x1F9FF).contains(&u)
            || (0x1F000..=0x1F02F).contains(&u);
        if !is_emoji {
            out.push(ch);
        }
    }
    out
}

/// Maps characters unsupported by host system TrueType fonts (like segoeui.ttf)
/// into clean ASCII or standard TrueType mathematical representations.
pub fn map_unsupported_char(ch: char) -> Option<&'static str> {
    match ch {
        '∮' => Some("Int "),
        '∫' => Some("Int "),
        '·' | '⋅' => Some("*"),
        '•' => Some("*"),
        'ε' => Some("e"),
        '₀' => Some("0"),
        '₁' => Some("1"),
        '₂' => Some("2"),
        '₃' => Some("3"),
        '₄' => Some("4"),
        '₅' => Some("5"),
        '₆' => Some("6"),
        '₇' => Some("7"),
        '₈' => Some("8"),
        '₉' => Some("9"),
        'Φ' => Some("Phi"),
        'Δ' => Some("Delta"),
        'λ' => Some("lambda"),
        'π' => Some("pi"),
        'θ' => Some("theta"),
        'ω' => Some("omega"),
        'μ' => Some("mu"),
        'σ' => Some("sigma"),
        'α' => Some("alpha"),
        'β' => Some("beta"),
        'γ' => Some("gamma"),
        _ => None,
    }
}

/// Formatted markdown line structure for UI rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct FormattedLine {
    pub text: String,
    pub is_header: bool,
    pub is_bullet: bool,
    pub level: usize,
}

/// Formats raw markdown into structured header and bullet lines.
pub fn format_markdown_lines(md: &str) -> Vec<FormattedLine> {
    let mut lines = Vec::new();
    for raw_line in md.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("### ") {
            lines.push(FormattedLine {
                text: rest.trim().to_string(),
                is_header: true,
                is_bullet: false,
                level: 3,
            });
        } else if let Some(rest) = trimmed.strip_prefix("## ") {
            lines.push(FormattedLine {
                text: rest.trim().to_string(),
                is_header: true,
                is_bullet: false,
                level: 2,
            });
        } else if let Some(rest) = trimmed.strip_prefix("# ") {
            lines.push(FormattedLine {
                text: rest.trim().to_string(),
                is_header: true,
                is_bullet: false,
                level: 1,
            });
        } else if let Some(rest) = trimmed
            .strip_prefix("* ")
            .or_else(|| trimmed.strip_prefix("- "))
        {
            let clean_bullet = rest.trim().replace("**", "");
            lines.push(FormattedLine {
                text: format!("• {}", clean_bullet),
                is_header: false,
                is_bullet: true,
                level: 0,
            });
        } else if trimmed.starts_with("**") && trimmed.contains(":**") {
            let clean = trimmed.replace("**", "");
            lines.push(FormattedLine {
                text: clean,
                is_header: true,
                is_bullet: false,
                level: 4,
            });
        } else {
            lines.push(FormattedLine {
                text: trimmed.replace("**", ""),
                is_header: false,
                is_bullet: false,
                level: 0,
            });
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_maindy_text_strips_emojis() {
        let input = "Hello 🚀 Brain 🧠 Rocket 💡!";
        let clean = sanitize_maindy_text(input);
        assert_eq!(clean, "Hello  Brain  Rocket !");
    }

    #[test]
    fn test_format_markdown_lines_headers_and_bullets() {
        let md = "### Key Principles\n**Newton's Second Law:** F = m * a\n* Conserved energy\n- Zero friction\nNormal text";
        let lines = format_markdown_lines(md);
        assert_eq!(lines.len(), 5);
        assert!(lines[0].is_header);
        assert_eq!(lines[0].text, "Key Principles");
        assert!(lines[1].is_header);
        assert_eq!(lines[1].text, "Newton's Second Law: F = m * a");
        assert!(lines[2].is_bullet);
        assert_eq!(lines[2].text, "• Conserved energy");
        assert!(lines[3].is_bullet);
        assert_eq!(lines[3].text, "• Zero friction");
        assert!(!lines[4].is_header && !lines[4].is_bullet);
        assert_eq!(lines[4].text, "Normal text");
    }

    #[test]
    fn test_text_uniforms_size_and_wgsl_match() {
        assert_eq!(std::mem::size_of::<TextUniforms>(), 16);
        let shader_source = include_str!("shaders/text.wgsl");
        let module =
            wgpu::naga::front::wgsl::parse_str(shader_source).expect("Failed to parse text.wgsl");
        let mut found = false;
        for (_, ty) in module.types.iter() {
            if ty.name.as_deref() == Some("TextUniforms") {
                if let wgpu::naga::TypeInner::Struct { span, .. } = &ty.inner {
                    assert_eq!(*span, 16, "TextUniforms in WGSL must have size 16");
                    found = true;
                }
            }
        }
        assert!(found, "TextUniforms struct not found in text.wgsl");
    }

    #[test]
    fn test_map_unsupported_char_zero_tofu() {
        assert_eq!(map_unsupported_char('∮'), Some("Int "));
        assert_eq!(map_unsupported_char('·'), Some("*"));
        assert_eq!(map_unsupported_char('ε'), Some("e"));
        assert_eq!(map_unsupported_char('₀'), Some("0"));
        assert_eq!(map_unsupported_char('Φ'), Some("Phi"));
        assert_eq!(map_unsupported_char('🚀'), None);
    }

    #[test]
    fn test_multilingual_font_fallback_and_safe_shaping() {
        let english = "MindSet OS helps me focus on my goals.";
        let tamil = "MindSet OS எனக்கு கவனம் செலுத்த உதவுகிறது.";
        let hindi = "MindSet OS मुझे सीखने में मदद करता है।";

        assert_eq!(safe_utf8_prefix(english, 10), "MindSet OS");
        assert!(!safe_utf8_prefix(tamil, 15).is_empty());
        assert!(!safe_utf8_prefix(hindi, 15).is_empty());

        let wrapped_en = wrap_text_chars(english, 15);
        let wrapped_ta = wrap_text_chars(tamil, 15);
        let wrapped_hi = wrap_text_chars(hindi, 15);

        assert!(wrapped_en.lines().count() >= 2);
        assert!(wrapped_ta.lines().count() >= 2);
        assert!(wrapped_hi.lines().count() >= 2);

        let fallbacks = TextRenderer::load_fallback_fonts();
        let _ = fallbacks.len();
    }

    #[test]
    fn test_global_font_scale_floor_and_hierarchy() {
        let global_scale = 1.15 * 1.35; // 1.5525
        assert!(compute_effective_font_size(10.0, global_scale) >= 12.0);
        assert_eq!(compute_effective_font_size(10.0, global_scale), 16.0);
        assert_eq!(compute_effective_font_size(7.0, global_scale), 12.0);
        assert_eq!(compute_effective_font_size(11.0, global_scale), 17.0);
        assert_eq!(compute_effective_font_size(12.5, global_scale), 19.0);
        assert_eq!(compute_effective_font_size(15.0, global_scale), 23.0);
        assert_eq!(compute_effective_font_size(36.0, global_scale), 56.0);
    }

    #[test]
    fn test_measurement_vs_rasterization_exact_match() {
        let primary = TextRenderer::load_system_font_entry().unwrap();

        let test_strings = [
            "MindSet OS",
            "[WS 1]",
            "[Alerts (3)]",
            "[Search]",
            "[Controls]",
            "[Lock]",
            "[Power]",
            "Wave Optics Numericals",
            "Target: 590 / 600 (98.3%)",
        ];

        let face = rustybuzz::Face::from_slice(&primary.bytes, primary.collection_index).unwrap();
        let upem = face.units_per_em() as f32;

        let font_sizes = [10.0, 11.0, 12.5, 14.0, 18.0];
        let global_scales = [1.00 * 1.35, 1.15 * 1.35, 1.30 * 1.35, 1.45 * 1.35];

        for &s in &test_strings {
            for &fs in &font_sizes {
                for &gs in &global_scales {
                    let eff_size = compute_effective_font_size(fs, gs);
                    let scale = eff_size / upem;

                    let mut buf = rustybuzz::UnicodeBuffer::new();
                    buf.push_str(s);
                    buf.guess_segment_properties();
                    let glyph_buf = rustybuzz::shape(&face, &[], buf);

                    let mut expected_adv = 0.0f32;
                    for pos in glyph_buf.glyph_positions() {
                        expected_adv += pos.x_advance as f32 * scale;
                    }

                    // Total advance from shaping matches exactly
                    let mut m_adv = 0.0f32;
                    let mut buf2 = rustybuzz::UnicodeBuffer::new();
                    buf2.push_str(s);
                    buf2.guess_segment_properties();
                    let g_buf2 = rustybuzz::shape(&face, &[], buf2);
                    for pos in g_buf2.glyph_positions() {
                        m_adv += pos.x_advance as f32 * scale;
                    }

                    assert!(
                        (m_adv - expected_adv).abs() < 0.001,
                        "Measurement mismatch for string '{}' at eff_size {}: measured {}, expected {}",
                        s, eff_size, m_adv, expected_adv
                    );
                }
            }
        }
    }

    #[test]
    fn test_color_emoji_rasterization_suite() {
        let test_emojis = [
            "😀",
            "😂",
            "❤️",
            "🔥",
            "🌞",
            "🌙",
            "💛",
            "🚀",
            "🎯",
            "📚",
            "👨‍👩‍👧‍👦",
            "👩‍💻",
            "👍🏽",
            "🏳️‍🌈",
            "🇮🇳",
            "1️⃣",
            "❤️🔥",
        ];
        for emoji in test_emojis {
            assert!(
                crate::emoji::is_emoji_sequence(emoji),
                "Failed to detect emoji: {}",
                emoji
            );
            let result = crate::emoji::rasterize_color_emoji(emoji, 32);
            assert!(result.is_some(), "Failed to rasterize emoji: {}", emoji);
            let (w, h, rgba) = result.unwrap();
            assert_eq!(w, 32);
            assert_eq!(h, 32);
            assert_eq!(rgba.len(), 32 * 32 * 4);
            // Verify there is non-zero alpha and color
            let non_zero_pixels = rgba.chunks(4).filter(|p| p[3] > 0).count();
            assert!(non_zero_pixels > 10, "Emoji {} rasterized empty!", emoji);
        }
    }

    #[test]
    fn test_opentype_tamil_and_indic_shaping() {
        let tamil_str = "தமிழ் கணினி இயக்க முறைமை. மனதை ஒருமுகப்படுத்துங்கள்.";
        let mixed_str = "MindSet OS — தமிழ் Learning Environment.";
        let hindi_str = "नमस्ते दुनिया।";
        let arabic_str = "مرحبا بالعالم";

        assert!(!tamil_str.is_empty());
        assert!(!mixed_str.is_empty());
        assert!(!hindi_str.is_empty());
        assert!(!arabic_str.is_empty());

        let font_entries = TextRenderer::load_fallback_font_entries();
        if let Some(nirmala) = font_entries.iter().find(|e| e.name.contains("Nirmala")) {
            let face = rustybuzz::Face::from_slice(&nirmala.bytes, nirmala.collection_index);
            assert!(face.is_some());
            let face = face.unwrap();

            let mut buf = rustybuzz::UnicodeBuffer::new();
            buf.push_str(tamil_str);
            buf.guess_segment_properties();

            let glyph_buf = rustybuzz::shape(&face, &[], buf);
            let infos = glyph_buf.glyph_infos();
            assert!(!infos.is_empty());
            // Verify shaped glyph IDs are non-zero (valid shaped glyphs, not .notdef)
            let valid_glyphs = infos.iter().filter(|g| g.glyph_id != 0).count();
            assert!(valid_glyphs > 5, "Shaping failed to resolve Tamil glyphs!");
        }
    }

    #[test]
    fn test_cosmic_font_system_with_emoji_creation() {
        let fs = TextRenderer::create_font_system_with_emoji();
        let db = fs.db();
        assert!(
            !db.is_empty(),
            "FontSystem database should contain system fonts"
        );
    }

    #[test]
    fn test_textbox_bounds_inner_padding_and_chat_layout() {
        let mut fs = TextRenderer::create_font_system_with_emoji();
        let mut buffer = cosmic_text::Buffer::new(&mut fs, cosmic_text::Metrics::new(14.0, 18.0));
        let bounds = layout_chat_body(&mut buffer, &mut fs, 100.0, 100.0, 400.0, 500.0);
        assert_eq!(bounds.x, 116.0); // 100 + 16 pad_x
        assert_eq!(bounds.y, 178.0); // 100 + 68 header + 10 pad_y
        assert_eq!(bounds.max_width, 400.0 - 32.0 - 10.0); // 358.0
        assert_eq!(bounds.max_height, 500.0 - 68.0 - 54.0 - 20.0); // 358.0

        let input_bounds = layout_input_bar(&mut buffer, &mut fs, 100.0, 600.0, 400.0, 54.0);
        assert_eq!(input_bounds.x, 116.0);
        assert_eq!(input_bounds.max_width, 400.0 - 32.0 - 64.0 - 8.0); // 296.0
    }

    #[test]
    fn test_text_batch_scissor_rect_batching() {
        let sc1 = ScissorRect {
            x: 10,
            y: 20,
            width: 100,
            height: 200,
        };
        let sc2 = ScissorRect {
            x: 50,
            y: 60,
            width: 80,
            height: 120,
        };

        assert_eq!(sc1.x, 10);
        assert_eq!(sc1.width, 100);
        assert_ne!(sc1, sc2);

        let b1 = TextBatch {
            scissor: Some(sc1),
            start: 0,
            count: 5,
        };
        let b2 = TextBatch {
            scissor: None,
            start: 5,
            count: 10,
        };
        assert_eq!(b1.count, 5);
        assert_eq!(b2.start, 5);
    }

    #[test]
    fn test_fontdue_pixel_measurement_and_proportional_width() {
        let primary = TextRenderer::load_system_font_entry().unwrap();
        let fallbacks = TextRenderer::load_fallback_font_entries();

        let (w_12, h_12) = measure_text_with_fonts(
            &primary.font,
            &fallbacks,
            "MindSet OS Sovereign Desktop",
            12.0,
        );
        let (w_24, h_24) = measure_text_with_fonts(
            &primary.font,
            &fallbacks,
            "MindSet OS Sovereign Desktop",
            24.0,
        );

        assert!(w_12 > 50.0, "Width should be strictly positive");
        assert!(
            w_24 > w_12 * 1.8,
            "Width should scale proportionally with font size: w12={}, w24={}",
            w_12,
            w_24
        );
        assert!(h_24 > h_12 * 1.8, "Height should scale proportionally");
    }

    #[test]
    fn test_fit_text_to_px_guarantees_bounded_width_at_all_scales() {
        let primary = TextRenderer::load_system_font_entry().unwrap();
        let fallbacks = TextRenderer::load_fallback_font_entries();

        let long_files = [
            "02_Electrochemistry_Formulas.md",
            "TP-Link Archer (12ms)",
            "Class 12 Physics • Wave Optics and Induction Master Notes.pdf",
            "Personal_Encrypted_Journal_2026_Final.enc",
        ];

        let scales = [1.00, 1.15, 1.30, 1.45];
        let max_widths = [80.0, 120.0, 160.0, 200.0];

        for &filename in &long_files {
            for &s in &scales {
                let px_size = 12.0 * s;
                for &max_w in &max_widths {
                    let fitted =
                        fit_text_with_fonts(&primary.font, &fallbacks, filename, px_size, max_w);
                    let (measured_w, _) =
                        measure_text_with_fonts(&primary.font, &fallbacks, &fitted, px_size);
                    assert!(
                        measured_w <= max_w + 0.5,
                        "Fitted text '{}' (width {}) exceeded max_width {} at scale {}",
                        fitted,
                        measured_w,
                        max_w,
                        s
                    );
                    if measure_text_with_fonts(&primary.font, &fallbacks, filename, px_size).0
                        > max_w
                    {
                        assert!(
                            fitted.ends_with("..."),
                            "Truncated string must end with '...': {}",
                            fitted
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_optimal_text_color_for_solar_amber_and_contrast() {
        let solar_amber = [0.984, 0.749, 0.141, 1.0]; // #FBBF24
        let rich_amber = [0.961, 0.620, 0.043, 1.0]; // #F59E0B
        let dark_slate = [0.086, 0.098, 0.149, 1.0]; // #161926
        let deep_blue = [0.145, 0.388, 0.922, 1.0]; // #2563EB

        let tc_amber = optimal_text_color_for_fill(solar_amber);
        let tc_rich = optimal_text_color_for_fill(rich_amber);
        let tc_slate = optimal_text_color_for_fill(dark_slate);
        let tc_blue = optimal_text_color_for_fill(deep_blue);

        // Dark obsidian for bright amber
        assert_eq!(tc_amber, [0.08, 0.07, 0.05, 1.0]);
        assert_eq!(tc_rich, [0.08, 0.07, 0.05, 1.0]);

        // Crisp white for dark backgrounds
        assert_eq!(tc_slate, [0.98, 0.99, 1.00, 1.0]);
        assert_eq!(tc_blue, [0.98, 0.99, 1.00, 1.0]);

        // Contrast ratio check (> 6:1 linear, > 9:1 perceived)
        let cr = contrast_ratio(solar_amber, tc_amber);
        assert!(
            cr > 6.0,
            "Contrast ratio on solar amber must be > 6.0:1 (got {})",
            cr
        );
    }

    #[test]
    fn test_glyphon_hidpi_text_area_scale_one_invariant() {
        let mut fs = TextRenderer::create_font_system_with_emoji();
        let scale_factors = [1.0, 1.25, 1.5, 2.0];

        for &scale in &scale_factors {
            let mut ctx = TextNodeContext::new(
                &mut fs,
                "MindSet HiDPI Alignment Test",
                14.0,
                scale,
                cosmic_text::Align::Center,
                true,
            );
            ctx.shape(&mut fs, 200.0, Some(30.0));

            let area = ctx.to_text_area(0.0, 0.0, 10.0, 20.0, 200.0, 30.0, [1.0, 1.0, 1.0, 1.0]);

            // Rule 4 Invariant: TextArea.scale MUST strictly be 1.0
            assert_eq!(
                area.scale, 1.0,
                "TextArea.scale must strictly be 1.0 (got {})",
                area.scale
            );

            // Physical coordinates must match logical * scale_factor
            let expected_left = (10.0 * scale).round();
            let expected_top = (20.0 * scale).round();
            let expected_right = (210.0 * scale).round();
            let expected_bottom = (50.0 * scale).round();

            assert_eq!(area.left, expected_left);
            assert_eq!(area.top, expected_top);
            assert_eq!(area.bounds.left, expected_left as i32);
            assert_eq!(area.bounds.top, expected_top as i32);
            assert_eq!(area.bounds.right, expected_right as i32);
            assert_eq!(area.bounds.bottom, expected_bottom as i32);
        }
    }

    #[test]
    fn test_strict_text_measurement_sequence() {
        let mut fs = TextRenderer::create_font_system_with_emoji();
        let mut buffer = cosmic_text::Buffer::new(&mut fs, cosmic_text::Metrics::new(16.0, 22.0));
        let attrs = cosmic_text::Attrs::new();

        // Execute Rule 5 strict 4-step sequence
        let lines = shape_text_strict(
            &mut buffer,
            &mut fs,
            "Autonomous Intelligence Sovereign OS",
            &attrs,
            300.0,
            Some(44.0),
            cosmic_text::Align::Center,
        );

        assert!(!lines.is_empty());
        for line in lines {
            assert_eq!(line.align(), Some(cosmic_text::Align::Center));
        }

        // Reading layout runs after strict shaping sequence
        let mut run_count = 0;
        for run in buffer.layout_runs() {
            assert!(run.line_w > 0.0);
            run_count += 1;
        }
        assert!(run_count >= 1);
    }

    #[test]
    fn test_text_bounds_scissor_clamping() {
        let sc = ScissorRect {
            x: 50,
            y: 100,
            width: 300,
            height: 150,
        };
        let bounds = TextBounds::from_scissor(sc);
        assert_eq!(bounds.left, 50);
        assert_eq!(bounds.top, 100);
        assert_eq!(bounds.right, 350);
        assert_eq!(bounds.bottom, 250);
        assert_eq!(bounds.width(), 300);
        assert_eq!(bounds.height(), 150);
    }

    #[test]
    fn test_wrap_text_chars_strict_boundaries_no_bleeding() {
        let text = "Micro-attribution unlocks dopamine upon completion.";
        let max_chars = 15;
        let wrapped = wrap_text_chars(text, max_chars);

        // Every line must strictly have <= 15 graphemes
        for line in wrapped.lines() {
            let grapheme_count = line.graphemes(true).count();
            assert!(
                grapheme_count <= max_chars,
                "Line '{}' exceeded max_chars {} (got {})",
                line,
                max_chars,
                grapheme_count
            );
        }

        // Test long single word (17 chars) with max_chars = 10
        let long_word = "Micro-attribution";
        let wrapped_word = wrap_text_chars(long_word, 10);
        let lines: Vec<&str> = wrapped_word.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "Micro-attr");
        assert_eq!(lines[1], "ibution");
        assert!(lines[0].graphemes(true).count() <= 10);
        assert!(lines[1].graphemes(true).count() <= 10);

        // Test max_chars = 0
        assert_eq!(wrap_text_chars(text, 0), "");

        // Test empty string
        assert_eq!(wrap_text_chars("", 20), "");

        // Multilingual Tamil test: aksharas are not split mid-cluster
        let tamil = "MindSet OS எனக்கு கவனம் செலுத்த உதவுகிறது.";
        let wrapped_ta = wrap_text_chars(tamil, 15);
        for line in wrapped_ta.lines() {
            assert!(line.graphemes(true).count() <= 15);
        }
        assert!(wrapped_ta.lines().count() >= 2);
    }

    #[test]
    fn test_wrap_text_for_card_bounds_and_textbox_bounds() {
        let card_w: f32 = 236.0;
        let pad_x: f32 = 12.0;
        let avg_char_w: f32 = 5.8;

        let avail_w: f32 = card_w - pad_x * 2.0; // 212.0
        let expected_chars = (avail_w / avg_char_w).floor() as usize; // 36
        assert_eq!(max_chars_for_width(avail_w, avg_char_w), expected_chars);

        let mission_caption = "Micro-attribution unlocks dopamine upon completion.";
        let wrapped = wrap_text_for_card_bounds(mission_caption, card_w, pad_x, avg_char_w);
        for line in wrapped.lines() {
            assert!(
                line.graphemes(true).count() <= expected_chars,
                "Card text line '{}' bled outside max chars {}",
                line,
                expected_chars
            );
        }

        // Test TextBoxBounds helper methods
        let tb = TextBoxBounds::padded(24.0, 68.0, 236.0, 138.0, 12.0, 12.0);
        assert_eq!(tb.x, 36.0);
        assert_eq!(tb.y, 80.0);
        assert_eq!(tb.max_width, 212.0);
        assert_eq!(tb.max_height, 114.0);

        let tb_chars = tb.max_chars(avg_char_w);
        assert_eq!(tb_chars, expected_chars);
        let tb_wrapped = tb.wrap_text(mission_caption, avg_char_w);
        assert_eq!(tb_wrapped, wrapped);

        let sc = tb.to_scissor();
        assert_eq!(sc.x, 36);
        assert_eq!(sc.y, 80);
        assert_eq!(sc.width, 212);
        assert_eq!(sc.height, 114);

        assert_eq!(tb.to_clip_bounds(), [36.0, 80.0, 212.0, 114.0]);
    }
}
