//! # mindset-render
//!
//! High-performance native GPU rendering subsystem for MindSet OS:
//! - WGPU Context & surface manager
//! - Procedural circadian aurora backdrop with dither & vignette
//! - Instanced Signed Distance Field (SDF) liquid glass panels with drop shadows
//! - Shelf-packed GPU glyph texture atlas & anti-aliased text renderer

pub mod background;
pub mod celebration;
pub mod context;
pub mod emoji;
pub mod sdf;
pub mod text;

pub use celebration::{CelebrationBurst, Particle};

pub use background::{BackgroundPipeline, SceneGlobals, WallpaperPreset};
pub use context::RenderContext;
pub use sdf::{SdfInstance, SdfPipeline, SdfUniforms};
pub use text::{
    contrast_ratio, fit_text_with_fonts, format_markdown_lines, layout_chat_body, layout_input_bar,
    map_unsupported_char, max_chars_for_width, measure_text_with_fonts,
    optimal_text_color_for_fill, rasterize_glyph_rgba, safe_utf8_prefix, sanitize_maindy_text,
    set_scissor_rect_safe, wrap_text_chars, wrap_text_cosmic, wrap_text_for_card_bounds,
    CachedGlyph, FormattedLine, GlyphInstance, RasterizedGlyph, ScissorRect, TextBatch,
    TextBoxBounds, TextRenderer,
};
