//! Procedural Cosmic Wallpaper & Downsampled Backdrop Cache Engine for MindSet OS
//!
//! Provides zero-overhead GPU-accelerated canvas fallback and low-RAM texture downsampling
//! (target <= 60 FPS on 8GB RAM hardware). Blends Pop!_OS geometric facets with Garuda Linux
//! aurora light-trails and caches ambient backdrop tiles so static frames use 0 GPU recomputation.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::theme::ThemePalette;
use iced::widget::canvas::{Cache, Geometry, Path, Program, Stroke};
use iced::{mouse, Color, Point, Rectangle, Renderer, Theme};

/// Wallpaper archetype used for automated multi-wallpaper readability stress verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperProfile {
    BrightMinimalist,
    DarkNeon,
    HighDetailBusy,
}

impl WallpaperProfile {
    /// Representative worst-case sample background RGB color for contrast validation.
    pub fn sample_backdrop_color(self) -> Color {
        match self {
            Self::BrightMinimalist => Color::from_rgb(0.95, 0.97, 0.99), // Near-white sky/minimalist
            Self::DarkNeon => Color::from_rgb(0.03, 0.05, 0.11), // Deep obsidian cosmic neon
            Self::HighDetailBusy => Color::from_rgb(0.46, 0.52, 0.60), // High-frequency midtone busy
        }
    }
}

/// Downsamples an RGBA8 image buffer via lightweight integer box filtering so 4K/8K custom
/// wallpapers never exceed 1080p memory budgets on 8 GB RAM hardware.
pub fn downsample_rgba_box(
    src: &[u8],
    src_w: u32,
    src_h: u32,
    max_w: u32,
    max_h: u32,
) -> (Vec<u8>, u32, u32) {
    if src_w == 0 || src_h == 0 || max_w == 0 || max_h == 0 {
        return (Vec::new(), 0, 0);
    }
    let expected_len = (src_w as usize)
        .saturating_mul(src_h as usize)
        .saturating_mul(4);
    if src.len() < expected_len {
        return (src.to_vec(), src_w, src_h);
    }
    if src_w <= max_w && src_h <= max_h {
        return (src[..expected_len].to_vec(), src_w, src_h);
    }

    let scale_x = src_w as f32 / max_w as f32;
    let scale_y = src_h as f32 / max_h as f32;
    let scale = scale_x.max(scale_y).max(1.0);
    let dst_w = ((src_w as f32 / scale).floor() as u32).max(1);
    let dst_h = ((src_h as f32 / scale).floor() as u32).max(1);

    let mut dst = vec![0u8; (dst_w as usize) * (dst_h as usize) * 4];
    for dy in 0..dst_h {
        let sy_start = ((dy as f32 * scale) as u32).min(src_h - 1);
        let sy_end = (((dy + 1) as f32 * scale).ceil() as u32).clamp(sy_start + 1, src_h);
        for dx in 0..dst_w {
            let sx_start = ((dx as f32 * scale) as u32).min(src_w - 1);
            let sx_end = (((dx + 1) as f32 * scale).ceil() as u32).clamp(sx_start + 1, src_w);
            let mut acc = [0u32; 4];
            let mut count = 0u32;
            for sy in sy_start..sy_end {
                let row_off = (sy as usize) * (src_w as usize) * 4;
                for sx in sx_start..sx_end {
                    let idx = row_off + (sx as usize) * 4;
                    acc[0] += src[idx] as u32;
                    acc[1] += src[idx + 1] as u32;
                    acc[2] += src[idx + 2] as u32;
                    acc[3] += src[idx + 3] as u32;
                    count += 1;
                }
            }
            let out_idx = ((dy as usize) * (dst_w as usize) + (dx as usize)) * 4;
            let div = count.max(1);
            dst[out_idx] = (acc[0] / div) as u8;
            dst[out_idx + 1] = (acc[1] / div) as u8;
            dst[out_idx + 2] = (acc[2] / div) as u8;
            dst[out_idx + 3] = (acc[3] / div) as u8;
        }
    }

    (dst, dst_w, dst_h)
}

#[derive(Debug)]
pub struct ProceduralCosmicWallpaper<'a> {
    pub cache: &'a Cache,
    pub palette: ThemePalette,
}

impl<'a, Message> Program<Message, Theme, Renderer> for ProceduralCosmicWallpaper<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let w = bounds.width;
            let h = bounds.height;

            // 1. OS Cosmic Void Base tuned to active theme (elementary / Pop!_OS clarity)
            let base_fill = match self.palette.mode {
                mindset_core::ThemeMode::AlpineGlass => Color::from_rgb(0.08, 0.14, 0.24),
                mindset_core::ThemeMode::DarkSpace => Color::from_rgb8(0x06, 0x0A, 0x16),
                mindset_core::ThemeMode::VioletNebula => Color::from_rgb(0.07, 0.04, 0.14),
                mindset_core::ThemeMode::SolarAmber => Color::from_rgb(0.08, 0.06, 0.05),
            };
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), base_fill);

            // Soft horizon nebula radial wash (cached once in canvas::Cache — 0 per-frame blur cost)
            frame.fill(
                &Path::circle(Point::new(w * 0.72, h * 0.30), w * 0.34),
                Color::from_rgba(0.14, 0.44, 0.90, 0.14),
            );
            frame.fill(
                &Path::circle(Point::new(w * 0.26, h * 0.72), w * 0.30),
                Color::from_rgba(0.54, 0.26, 0.88, 0.12),
            );

            // 2. Pop!_OS Geometric Facet Mesh (Subtle low-frequency facets)
            let facet_stroke = Stroke::default()
                .with_color(Color::from_rgba(0.32, 0.56, 0.92, 0.09))
                .with_width(1.0);
            let mesh_path = Path::new(|b| {
                b.move_to(Point::new(0.0, h * 0.4));
                b.line_to(Point::new(w * 0.35, 0.0));
                b.line_to(Point::new(w * 0.7, h * 0.35));
                b.line_to(Point::new(w, h * 0.1));
            });
            frame.stroke(&mesh_path, facet_stroke);

            let mesh_path2 = Path::new(|b| {
                b.move_to(Point::new(0.0, h * 0.7));
                b.line_to(Point::new(w * 0.25, h));
                b.line_to(Point::new(w * 0.65, h * 0.75));
                b.line_to(Point::new(w, h * 0.85));
            });
            frame.stroke(&mesh_path2, facet_stroke);

            // Subtle geometric horizon ridge polygon
            let horizon_facet = Path::new(|b| {
                b.move_to(Point::new(0.0, h));
                b.line_to(Point::new(0.0, h * 0.78));
                b.line_to(Point::new(w * 0.38, h * 0.62));
                b.line_to(Point::new(w * 0.74, h * 0.76));
                b.line_to(Point::new(w, h * 0.58));
                b.line_to(Point::new(w, h));
                b.close();
            });
            frame.fill(&horizon_facet, Color::from_rgba(0.02, 0.04, 0.09, 0.36));

            // 3. Garuda Cyber-Neon Light Trails (Cyan, Violet, Amber Bezier Splines)
            let cyan_stroke = Stroke::default()
                .with_color(Color::from_rgba(0.0, 0.90, 1.0, 0.38))
                .with_width(2.5);
            let cyan_curve = Path::new(|b| {
                b.move_to(Point::new(0.0, h * 0.85));
                b.bezier_curve_to(
                    Point::new(w * 0.25, h * 0.95),
                    Point::new(w * 0.55, h * 0.40),
                    Point::new(w, h * 0.25),
                );
            });
            frame.stroke(&cyan_curve, cyan_stroke);

            let violet_stroke = Stroke::default()
                .with_color(Color::from_rgba(0.66, 0.33, 0.97, 0.30))
                .with_width(3.5);
            let violet_curve = Path::new(|b| {
                b.move_to(Point::new(0.0, h * 0.90));
                b.bezier_curve_to(
                    Point::new(w * 0.30, h * 0.80),
                    Point::new(w * 0.65, h * 0.50),
                    Point::new(w, h * 0.35),
                );
            });
            frame.stroke(&violet_curve, violet_stroke);

            let amber_stroke = Stroke::default()
                .with_color(Color::from_rgba(0.96, 0.62, 0.04, 0.24))
                .with_width(2.0);
            let amber_curve = Path::new(|b| {
                b.move_to(Point::new(w * 0.1, h));
                b.bezier_curve_to(
                    Point::new(w * 0.45, h * 0.75),
                    Point::new(w * 0.70, h * 0.60),
                    Point::new(w, h * 0.45),
                );
            });
            frame.stroke(&amber_curve, amber_stroke);

            // Minimal crisp focal anchor stars
            let stars = [
                (0.18, 0.22, 1.4),
                (0.46, 0.16, 1.2),
                (0.84, 0.18, 1.6),
                (0.62, 0.32, 1.2),
                (0.29, 0.44, 1.0),
                (0.76, 0.62, 1.3),
            ];
            for (sx, sy, sr) in stars {
                frame.fill(
                    &Path::circle(Point::new(w * sx, h * sy), sr),
                    Color::from_rgba(0.95, 0.98, 1.0, 0.42),
                );
            }
        });

        vec![geometry]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_wallpaper_asset_file_presence() {
        let p1 = Path::new("assets/wallpaper.jpg");
        let p2 =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/wallpaper.jpg");
        assert!(
            p1.exists() || p2.exists(),
            "assets/wallpaper.jpg must exist on disk"
        );
        let target = if p1.exists() { p1 } else { p2.as_path() };
        let metadata = std::fs::metadata(target).expect("metadata");
        assert!(
            metadata.len() > 10_000,
            "wallpaper.jpg should be a real high-res image asset"
        );
    }

    #[test]
    fn test_procedural_cosmic_wallpaper_struct() {
        let cache = Cache::default();
        let palette = ThemePalette::for_mode(mindset_core::ThemeMode::DarkSpace);
        let wallpaper = ProceduralCosmicWallpaper {
            cache: &cache,
            palette,
        };
        assert_eq!(wallpaper.palette.mode, mindset_core::ThemeMode::DarkSpace);
    }

    #[test]
    fn test_wallpaper_downsample_box_respects_max_dimensions() {
        // 4K synthetic texture (3840x2160 scaled proxy 160x90 -> downsampled to <= 40x30)
        let src_w = 160u32;
        let src_h = 90u32;
        let src = vec![128u8; (src_w * src_h * 4) as usize];
        let (dst, dw, dh) = downsample_rgba_box(&src, src_w, src_h, 40, 30);
        assert!(dw <= 40 && dh <= 30);
        assert_eq!(dst.len(), (dw * dh * 4) as usize);
        assert_eq!(dst[0], 128);
    }
}
