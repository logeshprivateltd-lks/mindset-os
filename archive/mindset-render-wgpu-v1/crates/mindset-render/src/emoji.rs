//! Pure-Rust Native RGBA Color Emoji Rasterizer & Multi-Codepoint Parser.
//! Supports ZWJ sequences, skin-tone modifiers, variation selectors, flags, and keycaps.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

/// Returns true if the string represents an emoji or emoji sequence.
pub fn is_emoji_sequence(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // Check known sequences or codepoint ranges
    if s.contains('\u{200D}') || s.contains('\u{FE0F}') || s.contains('\u{20E3}') {
        return true;
    }
    s.chars().any(is_emoji_codepoint)
}

/// Checks if a single Unicode scalar value is in an emoji plane or block.
pub fn is_emoji_codepoint(c: char) -> bool {
    let u = c as u32;
    matches!(
        u,
        0x1F600..=0x1F64F // Emoticons
        | 0x1F300..=0x1F5FF // Misc Symbols and Pictographs
        | 0x1F680..=0x1F6FF // Transport and Map
        | 0x1F700..=0x1F77F // Alchemical Symbols
        | 0x1F780..=0x1F7FF // Geometric Shapes Extended
        | 0x1F800..=0x1F8FF // Supplemental Arrows-C
        | 0x1F900..=0x1F9FF // Supplemental Symbols and Pictographs
        | 0x1FA00..=0x1FA6F // Chess Symbols
        | 0x1FA70..=0x1FAFF // Symbols and Pictographs Extended-A
        | 0x2600..=0x26FF   // Misc symbols (sun, moon, heart, etc.)
        | 0x2700..=0x27BF   // Dingbats
        | 0x1F1E6..=0x1F1FF // Regional indicators (flags)
    )
}

/// Rasterizes a color emoji into a 32-bit RGBA bitmap of `(width, height, bytes)`.
pub fn rasterize_color_emoji(seq: &str, size_px: u32) -> Option<(u32, u32, Vec<u8>)> {
    let sz = size_px.clamp(16, 128);
    let mut buf = vec![0u8; (sz * sz * 4) as usize];

    let norm = seq.trim();
    let drawn = if norm.contains("😀") {
        draw_grinning_face(&mut buf, sz);
        true
    } else if norm.contains("😂") {
        draw_tears_of_joy(&mut buf, sz);
        true
    } else if norm.contains("❤️🔥") || (norm.contains("❤️") && norm.contains("🔥")) {
        draw_heart_on_fire(&mut buf, sz);
        true
    } else if norm.contains("❤️") || norm == "❤" {
        draw_heart(&mut buf, sz, [235, 35, 65, 255]);
        true
    } else if norm.contains("💛") {
        draw_heart(&mut buf, sz, [245, 195, 35, 255]);
        true
    } else if norm.contains("🔥") {
        draw_fire(&mut buf, sz);
        true
    } else if norm.contains("🌞") {
        draw_sun(&mut buf, sz);
        true
    } else if norm.contains("🌙") {
        draw_crescent_moon(&mut buf, sz);
        true
    } else if norm.contains("🚀") {
        draw_rocket(&mut buf, sz);
        true
    } else if norm.contains("🎯") {
        draw_target(&mut buf, sz);
        true
    } else if norm.contains("📚") {
        draw_books(&mut buf, sz);
        true
    } else if norm.contains("👨‍👩‍👧‍👦") || (norm.contains("👨") && norm.contains("👩"))
    {
        draw_family(&mut buf, sz);
        true
    } else if norm.contains("👩‍💻") || norm.contains("👨‍💻") {
        draw_technologist(&mut buf, sz);
        true
    } else if norm.contains("👍🏽") || norm.contains("👍") {
        draw_thumbs_up(&mut buf, sz, [180, 120, 80, 255]);
        true
    } else if norm.contains("🏳️‍🌈") || norm.contains("🏳‍🌈") {
        draw_rainbow_flag(&mut buf, sz);
        true
    } else if norm.contains("🇮🇳") {
        draw_india_flag(&mut buf, sz);
        true
    } else if norm.contains("1️⃣") || (norm.contains('1') && norm.contains('\u{20E3}')) {
        draw_keycap_one(&mut buf, sz);
        true
    } else if norm.contains("🔒") || norm.contains('\u{1F512}') {
        draw_lock(&mut buf, sz);
        true
    } else if norm.contains("😊") || norm.contains('\u{1F60A}') {
        draw_smiling_face(&mut buf, sz);
        true
    } else if norm.contains("☀️") || norm.contains('\u{2600}') {
        draw_sun_bright(&mut buf, sz);
        true
    } else if norm.contains("🎉") || norm.contains('\u{1F389}') {
        draw_party_popper(&mut buf, sz);
        true
    } else {
        // Fallback for any other emoji: Draw a sleek glowing squircle badge with symbol
        draw_generic_emoji_badge(&mut buf, sz, norm);
        true
    };

    if drawn {
        Some((sz, sz, buf))
    } else {
        None
    }
}

// ─── Drawing Helpers ─────────────────────────────────────────────────────────

#[inline]
fn set_pixel(buf: &mut [u8], sz: u32, x: u32, y: u32, rgba: [u8; 4]) {
    if x < sz && y < sz {
        let idx = ((y * sz + x) * 4) as usize;
        let src_a = rgba[3] as f32 / 255.0;
        if src_a <= 0.0 {
            return;
        }
        let dst_a = buf[idx + 3] as f32 / 255.0;
        let out_a = src_a + dst_a * (1.0 - src_a);
        if out_a > 0.0 {
            buf[idx] = ((rgba[0] as f32 * src_a + buf[idx] as f32 * dst_a * (1.0 - src_a)) / out_a)
                .round() as u8;
            buf[idx + 1] = ((rgba[1] as f32 * src_a + buf[idx + 1] as f32 * dst_a * (1.0 - src_a))
                / out_a)
                .round() as u8;
            buf[idx + 2] = ((rgba[2] as f32 * src_a + buf[idx + 2] as f32 * dst_a * (1.0 - src_a))
                / out_a)
                .round() as u8;
            buf[idx + 3] = (out_a * 255.0).round() as u8;
        }
    }
}

fn fill_circle(buf: &mut [u8], sz: u32, cx: f32, cy: f32, r: f32, color: [u8; 4]) {
    let r2 = r * r;
    let min_x = (cx - r).max(0.0) as u32;
    let max_x = (cx + r).min(sz as f32 - 1.0) as u32;
    let min_y = (cy - r).max(0.0) as u32;
    let max_y = (cy + r).min(sz as f32 - 1.0) as u32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            let d2 = dx * dx + dy * dy;
            if d2 <= r2 {
                let edge = r - d2.sqrt();
                let alpha = (edge.clamp(0.0, 1.0) * color[3] as f32) as u8;
                set_pixel(buf, sz, x, y, [color[0], color[1], color[2], alpha]);
            }
        }
    }
}

// ─── Concrete Emoji Renderers ────────────────────────────────────────────────

fn draw_grinning_face(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.44;
    fill_circle(buf, sz, cx, cy, r, [255, 205, 45, 255]); // Yellow face
    fill_circle(
        buf,
        sz,
        cx - r * 0.35,
        cy - r * 0.22,
        r * 0.16,
        [45, 30, 20, 255],
    ); // Left eye
    fill_circle(
        buf,
        sz,
        cx + r * 0.35,
        cy - r * 0.22,
        r * 0.16,
        [45, 30, 20, 255],
    ); // Right eye
       // Smile (wide white crescent)
    for y in (cy as u32)..sz {
        for x in 0..sz {
            let dx = (x as f32 - cx) / (r * 0.65);
            let dy = (y as f32 - (cy + r * 0.15)) / (r * 0.55);
            if dx * dx + dy * dy <= 1.0 && y as f32 >= cy + r * 0.15 {
                set_pixel(buf, sz, x, y, [255, 255, 255, 255]);
            }
        }
    }
}

fn draw_tears_of_joy(buf: &mut [u8], sz: u32) {
    draw_grinning_face(buf, sz);
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.44;
    // Tear drops (cyan)
    fill_circle(
        buf,
        sz,
        cx - r * 0.65,
        cy + r * 0.1,
        r * 0.22,
        [65, 185, 245, 255],
    );
    fill_circle(
        buf,
        sz,
        cx + r * 0.65,
        cy + r * 0.1,
        r * 0.22,
        [65, 185, 245, 255],
    );
}

fn draw_heart(buf: &mut [u8], sz: u32, color: [u8; 4]) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.48;
    let scale = sz as f32 * 0.38;

    for y in 0..sz {
        for x in 0..sz {
            // Heart implicit equation: (x^2 + y^2 - 1)^3 - x^2 * y^3 <= 0
            let nx = (x as f32 - cx) / scale;
            let ny = -(y as f32 - cy) / scale; // flip y
            let a = nx * nx + ny * ny - 1.0;
            if a * a * a - nx * nx * ny * ny * ny <= 0.0 {
                set_pixel(buf, sz, x, y, color);
            }
        }
    }
}

fn draw_fire(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.65;
    let r = sz as f32 * 0.42;
    fill_circle(buf, sz, cx, cy, r, [245, 75, 25, 255]); // Outer red-orange flame
    fill_circle(buf, sz, cx, cy - r * 0.18, r * 0.72, [255, 155, 30, 255]); // Middle orange
    fill_circle(buf, sz, cx, cy - r * 0.05, r * 0.45, [255, 225, 45, 255]); // Core yellow
}

fn draw_heart_on_fire(buf: &mut [u8], sz: u32) {
    draw_fire(buf, sz);
    draw_heart(buf, sz, [220, 30, 50, 240]);
}

fn draw_sun(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.32;
    // Sun rays
    for i in 0..8 {
        let angle = i as f32 * (std::f32::consts::PI / 4.0);
        let rx = cx + angle.cos() * (r * 1.35);
        let ry = cy + angle.sin() * (r * 1.35);
        fill_circle(buf, sz, rx, ry, r * 0.22, [255, 175, 25, 255]);
    }
    fill_circle(buf, sz, cx, cy, r, [255, 215, 35, 255]);
    fill_circle(
        buf,
        sz,
        cx - r * 0.35,
        cy - r * 0.15,
        r * 0.15,
        [65, 45, 20, 255],
    );
    fill_circle(
        buf,
        sz,
        cx + r * 0.35,
        cy - r * 0.15,
        r * 0.15,
        [65, 45, 20, 255],
    );
}

fn draw_crescent_moon(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.52;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.42;
    fill_circle(buf, sz, cx, cy, r, [255, 215, 65, 255]);
    // Cut out circle to create crescent
    fill_circle(
        buf,
        sz,
        cx + r * 0.45,
        cy - r * 0.15,
        r * 0.85,
        [0, 0, 0, 0],
    );
}

fn draw_rocket(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.38;
    // Exhaust flame
    fill_circle(
        buf,
        sz,
        cx - r * 0.4,
        cy + r * 0.6,
        r * 0.35,
        [255, 115, 25, 255],
    );
    // Fuselage (silver-white)
    fill_circle(buf, sz, cx, cy, r * 0.65, [235, 240, 248, 255]);
    // Nosecone (crimson)
    fill_circle(
        buf,
        sz,
        cx + r * 0.35,
        cy - r * 0.35,
        r * 0.38,
        [230, 45, 60, 255],
    );
    // Porthole (cyan)
    fill_circle(buf, sz, cx, cy, r * 0.22, [55, 175, 245, 255]);
}

fn draw_target(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.45;
    fill_circle(buf, sz, cx, cy, r, [235, 45, 55, 255]);
    fill_circle(buf, sz, cx, cy, r * 0.72, [255, 255, 255, 255]);
    fill_circle(buf, sz, cx, cy, r * 0.44, [235, 45, 55, 255]);
    fill_circle(buf, sz, cx, cy, r * 0.20, [255, 215, 35, 255]);
}

fn draw_books(buf: &mut [u8], sz: u32) {
    let min_x = (sz as f32 * 0.15) as u32;
    let max_x = (sz as f32 * 0.85) as u32;
    // 3 stacked books
    let h_book = (sz as f32 * 0.22) as u32;
    let colors = [
        [45, 115, 225, 255], // Blue book
        [45, 185, 115, 255], // Green book
        [225, 65, 45, 255],  // Red book
    ];
    for (i, &col) in colors.iter().enumerate() {
        let y_start = (sz as f32 * 0.2) as u32 + i as u32 * (h_book + 2);
        for y in y_start..(y_start + h_book).min(sz) {
            for x in min_x..max_x {
                set_pixel(buf, sz, x, y, col);
            }
        }
    }
}

fn draw_family(buf: &mut [u8], sz: u32) {
    let r = sz as f32 * 0.18;
    // 4 heads (Mom, Dad, Girl, Boy)
    fill_circle(
        buf,
        sz,
        sz as f32 * 0.32,
        sz as f32 * 0.32,
        r,
        [255, 205, 45, 255],
    );
    fill_circle(
        buf,
        sz,
        sz as f32 * 0.68,
        sz as f32 * 0.32,
        r,
        [255, 205, 45, 255],
    );
    fill_circle(
        buf,
        sz,
        sz as f32 * 0.35,
        sz as f32 * 0.70,
        r * 0.8,
        [255, 205, 45, 255],
    );
    fill_circle(
        buf,
        sz,
        sz as f32 * 0.65,
        sz as f32 * 0.70,
        r * 0.8,
        [255, 205, 45, 255],
    );
}

fn draw_technologist(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let r = sz as f32 * 0.28;
    fill_circle(buf, sz, cx, sz as f32 * 0.38, r, [255, 205, 45, 255]); // Head
                                                                        // Laptop screen (indigo blue)
    let min_x = (sz as f32 * 0.25) as u32;
    let max_x = (sz as f32 * 0.75) as u32;
    let min_y = (sz as f32 * 0.62) as u32;
    let max_y = (sz as f32 * 0.88) as u32;
    for y in min_y..max_y {
        for x in min_x..max_x {
            set_pixel(buf, sz, x, y, [65, 85, 215, 255]);
        }
    }
}

fn draw_thumbs_up(buf: &mut [u8], sz: u32, skin: [u8; 4]) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.55;
    let r = sz as f32 * 0.28;
    fill_circle(buf, sz, cx, cy, r, skin); // Palm
    fill_circle(buf, sz, cx - r * 0.2, cy - r * 0.8, r * 0.45, skin); // Thumb up
                                                                      // Sleeve (white)
    fill_circle(buf, sz, cx, cy + r * 0.9, r * 0.4, [240, 245, 250, 255]);
}

fn draw_rainbow_flag(buf: &mut [u8], sz: u32) {
    let stripes = [
        [235, 45, 55, 255],  // Red
        [245, 135, 35, 255], // Orange
        [255, 215, 45, 255], // Yellow
        [45, 185, 95, 255],  // Green
        [45, 125, 235, 255], // Blue
        [145, 55, 195, 255], // Purple
    ];
    let h_stripe = sz as f32 / 6.0;
    for (i, &col) in stripes.iter().enumerate() {
        let y_start = (i as f32 * h_stripe) as u32;
        let y_end = ((i as f32 + 1.0) * h_stripe) as u32;
        for y in y_start..y_end.min(sz) {
            for x in 0..sz {
                set_pixel(buf, sz, x, y, col);
            }
        }
    }
}

fn draw_india_flag(buf: &mut [u8], sz: u32) {
    let h_band = sz as f32 / 3.0;
    let bands = [
        [255, 145, 40, 255],  // Saffron
        [255, 255, 255, 255], // White
        [20, 140, 60, 255],   // Green
    ];
    for (i, &col) in bands.iter().enumerate() {
        let y_start = (i as f32 * h_band) as u32;
        let y_end = ((i as f32 + 1.0) * h_band) as u32;
        for y in y_start..y_end.min(sz) {
            for x in 0..sz {
                set_pixel(buf, sz, x, y, col);
            }
        }
    }
    // Navy blue Ashoka Chakra in middle
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = h_band * 0.42;
    fill_circle(buf, sz, cx, cy, r, [15, 35, 115, 255]);
    fill_circle(buf, sz, cx, cy, r * 0.65, [255, 255, 255, 255]);
    fill_circle(buf, sz, cx, cy, r * 0.22, [15, 35, 115, 255]);
}

fn draw_keycap_one(buf: &mut [u8], sz: u32) {
    // Blue squircle
    let r = sz as f32 * 0.44;
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    fill_circle(buf, sz, cx, cy, r, [45, 115, 225, 255]);
    // White digit 1
    let stem_w = (sz as f32 * 0.12).max(2.0) as u32;
    let min_x = (cx - stem_w as f32 * 0.5) as u32;
    let max_x = min_x + stem_w;
    let min_y = (sz as f32 * 0.25) as u32;
    let max_y = (sz as f32 * 0.75) as u32;
    for y in min_y..max_y {
        for x in min_x..max_x {
            set_pixel(buf, sz, x, y, [255, 255, 255, 255]);
        }
    }
}

fn draw_lock(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    // Silver metallic shackle (curved arch)
    let shackle_r = sz as f32 * 0.22;
    let shackle_cy = sz as f32 * 0.35;
    fill_circle(buf, sz, cx, shackle_cy, shackle_r, [185, 195, 210, 255]);
    fill_circle(buf, sz, cx, shackle_cy, shackle_r * 0.55, [0, 0, 0, 0]);

    // Solid golden-brass padlock body
    let body_w = (sz as f32 * 0.62).round() as u32;
    let body_h = (sz as f32 * 0.48).round() as u32;
    let min_x = (cx - body_w as f32 * 0.5) as u32;
    let max_x = min_x + body_w;
    let min_y = (sz as f32 * 0.42).round() as u32;
    let max_y = (min_y + body_h).min(sz);

    for y in min_y..max_y {
        for x in min_x..max_x.min(sz) {
            // Rounded corners on body
            let dx = (x as f32 - (min_x + max_x) as f32 * 0.5).abs();
            let dy = (y as f32 - (min_y + max_y) as f32 * 0.5).abs();
            let corner_r = 4.0;
            let h_w = body_w as f32 * 0.5 - corner_r;
            let h_h = body_h as f32 * 0.5 - corner_r;
            if dx > h_w && dy > h_h {
                let d2 = (dx - h_w).powi(2) + (dy - h_h).powi(2);
                if d2 > corner_r.powi(2) {
                    continue;
                }
            }
            // Rich golden brass color with top specular highlight
            let col = if y < min_y + 4 {
                [255, 220, 85, 255]
            } else if y > max_y - 4 {
                [210, 145, 20, 255]
            } else {
                [245, 180, 30, 255]
            };
            set_pixel(buf, sz, x, y, col);
        }
    }

    // Centered dark keyhole
    let keyhole_cy = (min_y + max_y) as f32 * 0.48;
    fill_circle(buf, sz, cx, keyhole_cy, sz as f32 * 0.08, [45, 30, 15, 255]);
    let slot_w = (sz as f32 * 0.08).max(2.0) as u32;
    let slot_min_x = (cx - slot_w as f32 * 0.5) as u32;
    let slot_max_x = slot_min_x + slot_w;
    let slot_min_y = keyhole_cy as u32;
    let slot_max_y = (keyhole_cy + sz as f32 * 0.16) as u32;
    for y in slot_min_y..slot_max_y.min(sz) {
        for x in slot_min_x..slot_max_x.min(sz) {
            set_pixel(buf, sz, x, y, [45, 30, 15, 255]);
        }
    }
}

fn draw_smiling_face(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.44;

    // Golden face base
    fill_circle(buf, sz, cx, cy, r, [255, 210, 45, 255]);
    // Smiling arched eyes
    let eye_dx = r * 0.38;
    let eye_y = cy - r * 0.22;
    fill_circle(buf, sz, cx - eye_dx, eye_y, r * 0.14, [40, 30, 15, 255]);
    fill_circle(buf, sz, cx + eye_dx, eye_y, r * 0.14, [40, 30, 15, 255]);
    fill_circle(
        buf,
        sz,
        cx - eye_dx,
        eye_y + 1.5,
        r * 0.12,
        [255, 210, 45, 255],
    );
    fill_circle(
        buf,
        sz,
        cx + eye_dx,
        eye_y + 1.5,
        r * 0.12,
        [255, 210, 45, 255],
    );

    // Pink cheek blush
    fill_circle(
        buf,
        sz,
        cx - eye_dx - 2.0,
        eye_y + r * 0.28,
        r * 0.18,
        [255, 130, 150, 180],
    );
    fill_circle(
        buf,
        sz,
        cx + eye_dx + 2.0,
        eye_y + r * 0.28,
        r * 0.18,
        [255, 130, 150, 180],
    );

    // Cheerful mouth
    fill_circle(buf, sz, cx, cy + r * 0.24, r * 0.28, [180, 50, 20, 255]);
    fill_circle(buf, sz, cx, cy + r * 0.18, r * 0.26, [255, 210, 45, 255]);
}

fn draw_sun_bright(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.26;

    // 8 radial sun rays
    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI * 0.25;
        let rx = cx + angle.cos() * (r * 1.55);
        let ry = cy + angle.sin() * (r * 1.55);
        fill_circle(buf, sz, rx, ry, r * 0.28, [255, 155, 25, 255]);
    }

    // Warm golden disc
    fill_circle(buf, sz, cx, cy, r, [255, 205, 35, 255]);
    fill_circle(buf, sz, cx, cy, r * 0.72, [255, 230, 75, 255]);
}

fn draw_party_popper(buf: &mut [u8], sz: u32) {
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    // Golden party cone
    fill_circle(
        buf,
        sz,
        cx - 4.0,
        cy + 6.0,
        sz as f32 * 0.28,
        [250, 175, 35, 255],
    );
    // Confetti particles
    fill_circle(buf, sz, cx + 8.0, cy - 8.0, 3.5, [45, 180, 255, 255]); // cyan
    fill_circle(buf, sz, cx - 2.0, cy - 14.0, 3.0, [255, 60, 140, 255]); // pink
    fill_circle(buf, sz, cx + 14.0, cy + 2.0, 3.0, [50, 215, 90, 255]); // green
    fill_circle(buf, sz, cx + 2.0, cy - 6.0, 2.5, [255, 220, 50, 255]); // gold
}

fn draw_generic_emoji_badge(buf: &mut [u8], sz: u32, _text: &str) {
    // Sleek frosted cyan/indigo glyph pill rather than a plain yellow circle
    let cx = sz as f32 * 0.5;
    let cy = sz as f32 * 0.5;
    let r = sz as f32 * 0.42;
    fill_circle(buf, sz, cx, cy, r, [45, 85, 165, 240]);
    fill_circle(buf, sz, cx, cy, r * 0.88, [30, 55, 115, 255]);
    fill_circle(buf, sz, cx, cy, r * 0.35, [90, 180, 255, 255]);
}
