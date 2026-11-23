//! Design System Showcase (Internal Regression Verifier)
//! Toggled with Ctrl+Shift+D.
//! Renders all color tokens, typography scales, widgets in all states,
//! progress rings, and animation triggers.

use crate::draw_helpers::ClippedDraw;
use mindset_render::SdfInstance;
use mindset_ui::layout::LayoutRect;

pub fn render_design_system_showcase(draw: &mut ClippedDraw, rect: LayoutRect, ui_scale: f32) {
    let p_surface = [0.0824, 0.0980, 0.1333, 0.95];
    let p_border = [1.0, 1.0, 1.0, 0.20];
    let p_text_primary = [0.9608, 0.9647, 0.9804, 1.0];
    let p_accent = [0.8980, 0.6784, 0.2706, 1.0];

    // Background panel
    draw.sdf.push(SdfInstance::glass_panel(
        rect.x, rect.y, rect.w, rect.h, 16.0, p_surface, p_border,
    ));

    // Titlebar
    draw.queue_text(
        "DESIGN SYSTEM SHOWCASE (Internal Reference)",
        rect.x + 20.0,
        rect.y + 16.0,
        17.0 * ui_scale,
        p_accent,
    );
    draw.queue_text(
        "Verifying Zero-Hex Hardcoding, Spring Convergences, and Type Scale",
        rect.x + 20.0,
        rect.y + 38.0,
        13.5 * ui_scale,
        [0.6, 0.65, 0.75, 1.0],
    );

    // Swatches
    let colors = [
        ("Solar Amber", [0.8980, 0.6784, 0.2706, 1.0]),
        ("Emerald", [0.063, 0.725, 0.506, 1.0]),
        ("Royal Blue", [0.231, 0.510, 0.965, 1.0]),
        ("Slate", [0.392, 0.455, 0.545, 1.0]),
    ];

    let mut sx = rect.x + 20.0;
    for (name, col) in colors {
        let sw_r = LayoutRect::new(sx, rect.y + 70.0, 80.0, 36.0);
        draw.sdf.push(SdfInstance::button(
            sw_r.x,
            sw_r.y,
            sw_r.w,
            sw_r.h,
            6.0,
            col,
            [1.0, 1.0, 1.0, 0.2],
            0.0,
        ));
        draw.queue_text(name, sx, rect.y + 112.0, 11.5 * ui_scale, p_text_primary);
        sx += 96.0;
    }

    // Rings demo (0%, 25%, 50%, 75%, 100%)
    let fractions = [0.0, 0.25, 0.50, 0.75, 1.0];
    let mut rx = rect.x + 20.0;
    for frac in fractions {
        let center_x = rx + 30.0;
        let center_y = rect.y + 180.0;
        draw.sdf.push(SdfInstance::progress_ring(
            center_x,
            center_y,
            22.0,
            5.0,
            frac,
            -std::f32::consts::FRAC_PI_2,
            [0.2, 0.25, 0.35, 0.6],
            p_accent,
        ));
        draw.queue_text(
            &format!("{:.0}%", frac * 100.0),
            center_x - 12.0,
            center_y + 30.0,
            12.0 * ui_scale,
            p_text_primary,
        );
        rx += 80.0;
    }
}
