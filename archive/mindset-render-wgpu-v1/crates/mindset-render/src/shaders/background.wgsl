struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index == 1u) * 4 - 1);
    let y = f32(i32(in_vertex_index == 2u) * 4 - 1);
    out.position = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>(x * 0.5 + 0.5, -y * 0.5 + 0.5);
    return out;
}

struct SceneGlobals {
    resolution: vec2<f32>,
    time: f32,
    focus_depth: f32,
    wallpaper_style: f32,
    daily_progress: f32,
    pad1: f32,
    pad2: f32,
};

@group(0) @binding(0) var<uniform> globals: SceneGlobals;

// Pseudo-random triangular dither to prevent color banding
fn dither(pos: vec2<f32>) -> f32 {
    let noise = fract(sin(dot(pos, vec2<f32>(12.9898, 78.233))) * 43758.5453);
    return (noise - 0.5) / 255.0;
}

// Procedural hash for stars / organic terrain
fn hash21(p: vec2<f32>) -> f32 {
    let q = fract(p * vec2<f32>(234.34, 435.345));
    return fract(dot(q, q + 34.23));
}

// Multi-frequency sinusoidal noise for mountains
fn mountain_height(x: f32, seed: f32) -> f32 {
    let w1 = sin(x * 3.5 + seed) * 0.10;
    let w2 = sin(x * 8.2 + seed * 2.3) * 0.05;
    let w3 = abs(cos(x * 16.4 + seed * 1.7)) * 0.025;
    let w4 = sin(x * 32.0 + seed * 3.1) * 0.012;
    return w1 + w2 + w3 + w4;
}

// 0: Solar Amber Glow (Slow-moving softly blurred golden-yellow and warm amber-cream light spheres)
fn render_solar_amber_glow(uv: vec2<f32>, time: f32) -> vec3<f32> {
    // Colors: #FFFBEB background, #FDE68A soft gold, #F59E0B solar amber, #FEF3C7 warm cream
    let bg_base = vec3<f32>(1.000, 0.984, 0.922);     // #FFFBEB soft cream
    let amber_gold = vec3<f32>(0.961, 0.620, 0.043);  // #F59E0B warm solar amber
    let soft_gold = vec3<f32>(0.992, 0.902, 0.541);   // #FDE68A delicate gold
    let warm_cream = vec3<f32>(0.996, 0.953, 0.780);  // #FEF3C7 warm yellow-cream
    let golden_glow = vec3<f32>(0.984, 0.749, 0.141); // #FBBF24 sun highlight

    let t = time * 0.20;

    // Orb 1: Primary slow drifting golden sun sphere (top-center)
    let orb1_pos = vec2<f32>(0.48 + sin(t * 0.6) * 0.22, 0.35 + cos(t * 0.4) * 0.16);
    let orb1_dist = length(uv - orb1_pos);
    let orb1_glow = exp(-orb1_dist * 2.6);

    // Orb 2: Ambient warm cream secondary sphere (left)
    let orb2_pos = vec2<f32>(0.22 + cos(t * 0.5 + 1.2) * 0.18, 0.68 + sin(t * 0.7 + 0.4) * 0.14);
    let orb2_dist = length(uv - orb2_pos);
    let orb2_glow = exp(-orb2_dist * 3.0);

    // Orb 3: Rich solar amber depth sphere (lower right)
    let orb3_pos = vec2<f32>(0.76 + sin(t * 0.35 + 2.5) * 0.16, 0.74 + cos(t * 0.45 + 1.8) * 0.15);
    let orb3_dist = length(uv - orb3_pos);
    let orb3_glow = exp(-orb3_dist * 2.4);

    // Base subtle vertical gradient
    var col = mix(bg_base, warm_cream, uv.y * 0.7 + sin(uv.x * 2.5 + t * 0.3) * 0.04);

    // Smooth Gaussian & smoothstep falloff for atmospheric golden light
    col = mix(col, soft_gold, smoothstep(0.75, 0.0, orb1_dist) * 0.55 + orb1_glow * 0.30);
    col = mix(col, warm_cream, smoothstep(0.85, 0.0, orb2_dist) * 0.45 + orb2_glow * 0.22);
    col = mix(col, amber_gold, smoothstep(0.90, 0.0, orb3_dist) * 0.38 + orb3_glow * 0.28);
    col += golden_glow * (orb1_glow * 0.20);

    return col;
}

// 1: Alpine Lake (Daylight mountain scenery with crystal reflection)
fn render_alpine_lake(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let sky_top = vec3<f32>(0.20, 0.46, 0.90);
    let sky_horizon = vec3<f32>(0.74, 0.86, 0.96);
    let sun_color = vec3<f32>(1.0, 0.98, 0.90);

    let sun_pos = vec2<f32>(0.68, 0.20);
    let sun_dist = length(uv - sun_pos);
    let sun_glow = exp(-sun_dist * 3.2) * 0.38;

    let sky = mix(sky_top, sky_horizon, clamp(uv.y * 1.7, 0.0, 1.0)) + sun_color * sun_glow;
    let horizon = 0.54;

    // Distant snow-capped peaks
    let peak_base = 0.44;
    let peaks = peak_base + mountain_height(uv.x * 2.4, 1.3);
    let rock_col = vec3<f32>(0.28, 0.35, 0.46);
    let snow_col = vec3<f32>(0.92, 0.96, 1.0);
    let peak_col = mix(rock_col, snow_col, clamp((peaks + 0.07 - uv.y) * 14.0, 0.0, 1.0));

    // Mid-range pine ridge
    let ridge_base = 0.49;
    let ridge = ridge_base + mountain_height(uv.x * 4.2, 5.8) * 0.65;
    let ridge_col = vec3<f32>(0.11, 0.23, 0.19);

    var land = sky;
    if (uv.y > peaks) {
        land = peak_col;
    }
    if (uv.y > ridge) {
        land = ridge_col;
    }

    // Lake surface below horizon
    if (uv.y >= horizon) {
        let ripple = sin(uv.y * 160.0 + time * 2.5) * cos(uv.x * 35.0) * 0.003;
        let refl_y = horizon - (uv.y - horizon) * 0.85 + ripple;

        let water_deep = vec3<f32>(0.07, 0.28, 0.40);
        let water_bright = vec3<f32>(0.20, 0.58, 0.66);

        var refl_col = mix(sky_top, sky_horizon, clamp(refl_y * 1.7, 0.0, 1.0));
        let r_peaks = peak_base + mountain_height((uv.x + ripple * 2.0) * 2.4, 1.3);
        if (refl_y > r_peaks) {
            refl_col = peak_col * 0.75;
        }
        let r_ridge = ridge_base + mountain_height((uv.x + ripple * 2.0) * 4.2, 5.8) * 0.65;
        if (refl_y > r_ridge) {
            refl_col = ridge_col * 0.8;
        }

        let water_grad = (uv.y - horizon) / (1.0 - horizon);
        let base_water = mix(water_bright, water_deep, water_grad);
        let spec_sun = exp(-abs(uv.x - sun_pos.x) * 18.0) * exp(-abs(uv.y - horizon - 0.06) * 10.0) * 0.32;

        land = mix(refl_col * 0.65 + base_water * 0.35, base_water, water_grad * 0.42) + vec3<f32>(spec_sun);
    }

    return land;
}

// 1: Aurora Midnight (Arctic dark sky with flowing emerald & violet aurora ribbons)
fn render_aurora_midnight(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let obsidian = vec3<f32>(0.015, 0.022, 0.045);
    let midnight_blue = vec3<f32>(0.04, 0.06, 0.14);
    let neon_emerald = vec3<f32>(0.08, 0.88, 0.55);
    let violet_magenta = vec3<f32>(0.58, 0.18, 0.82);

    var sky = mix(midnight_blue, obsidian, uv.y);

    // Sub-pixel radial twinkling pinpoint starfield (Zero blocky squares)
    let grid_scale = 120.0;
    let star_cell = floor(uv * grid_scale);
    let star_hash = hash21(star_cell);
    if (star_hash > 0.978 && uv.y < 0.72) {
        let star_offset = vec2<f32>(hash21(star_cell + 1.3), hash21(star_cell + 4.7)) * 0.6 + 0.2;
        let cell_uv = fract(uv * grid_scale) - star_offset;
        let star_shape = smoothstep(0.075, 0.008, length(cell_uv));
        let star_twinkle = sin(time * 2.5 + star_hash * 6.28) * 0.35 + 0.65;
        sky += vec3<f32>(star_shape * star_twinkle * 0.95);
    }

    // Flowing aurora ribbons
    let t = time * 0.4;
    let wave1 = sin(uv.x * 3.2 + t * 0.7) * 0.15 + cos(uv.x * 6.5 - t * 0.4) * 0.06;
    let dist1 = abs(uv.y - 0.35 - wave1);
    let ribbon1 = exp(-dist1 * 14.0) * (sin(uv.x * 12.0 + t) * 0.2 + 0.8);

    let wave2 = cos(uv.x * 2.8 - t * 0.5) * 0.18 + sin(uv.x * 5.2 + t * 0.6) * 0.07;
    let dist2 = abs(uv.y - 0.48 - wave2);
    let ribbon2 = exp(-dist2 * 12.0) * (cos(uv.x * 8.0 - t * 0.8) * 0.2 + 0.8);

    sky += neon_emerald * ribbon1 * 0.75;
    sky += violet_magenta * ribbon2 * 0.65;

    // Distant mountain silhouette at bottom
    let horiz = 0.82 + sin(uv.x * 8.0) * 0.04 + cos(uv.x * 18.0) * 0.02;
    if (uv.y > horiz) {
        sky = vec3<f32>(0.008, 0.012, 0.022);
    }

    return sky;
}

// 2: Sunset Horizon (Twilight gradient with fiery amber, coral pink, and calm reflection)
fn render_sunset_horizon(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let dusk_violet = vec3<f32>(0.14, 0.08, 0.28);
    let coral_pink  = vec3<f32>(0.85, 0.26, 0.46);
    let amber_gold  = vec3<f32>(0.98, 0.62, 0.15);
    let ocean_dark  = vec3<f32>(0.08, 0.06, 0.16);

    let horizon = 0.58;
    var col: vec3<f32>;

    if (uv.y < horizon) {
        let norm_y = uv.y / horizon;
        let c1 = mix(dusk_violet, coral_pink, smoothstep(0.0, 0.6, norm_y));
        let c2 = mix(c1, amber_gold, smoothstep(0.5, 1.0, norm_y));

        let sun_dist = length(uv - vec2<f32>(0.5, horizon - 0.04));
        let sun_glow = exp(-sun_dist * 4.5) * 0.45;
        col = c2 + amber_gold * sun_glow;
    } else {
        let norm_y = (uv.y - horizon) / (1.0 - horizon);
        let ripple = sin(uv.y * 140.0 + time * 2.5) * 0.004;
        let sun_col_dist = abs(uv.x - 0.5 + ripple * 2.0);
        let sun_trail = exp(-sun_col_dist * 8.0) * exp(-norm_y * 3.0) * 0.65;

        col = mix(coral_pink * 0.35 + ocean_dark, ocean_dark, norm_y) + amber_gold * sun_trail;
    }

    return col;
}

// 3: Emerald Zen (Misty forest canopy with sage green, teal and diffused light rays)
fn render_emerald_zen(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let deep_moss = vec3<f32>(0.03, 0.12, 0.08);
    let sage_teal = vec3<f32>(0.12, 0.46, 0.38);
    let mist_gold = vec3<f32>(0.82, 0.88, 0.65);

    let t = time * 0.3;
    let wave1 = sin(uv.x * 2.5 + t * 0.4) * cos(uv.y * 2.0 - t * 0.3);
    let wave2 = cos(uv.x * 4.0 - t * 0.5 + sin(uv.y * 3.0));

    let ray_angle = (uv.x + uv.y * 0.7);
    let rays = (sin(ray_angle * 12.0 + t * 0.5) * 0.5 + 0.5) * exp(-uv.y * 1.5) * 0.25;

    var col = mix(deep_moss, sage_teal, clamp(uv.y * 0.8 + wave1 * 0.2, 0.0, 1.0));
    col += mist_gold * rays;
    col += vec3<f32>(0.04, 0.18, 0.14) * (wave2 * 0.2 + 0.2);

    return col;
}

// 4: Minimal Studio (Refined dark graphite/slate radial lighting)
fn render_minimal_studio(uv: vec2<f32>) -> vec3<f32> {
    let graphite_deep = vec3<f32>(0.055, 0.075, 0.110);
    let slate_mid     = vec3<f32>(0.115, 0.155, 0.220);
    let warm_accent   = vec3<f32>(0.180, 0.200, 0.250);

    let center_dist = length(uv - vec2<f32>(0.5, 0.35));
    let vignette = smoothstep(0.9, 0.1, center_dist);

    var col = mix(graphite_deep, slate_mid, vignette);
    col += warm_accent * (1.0 - smoothstep(0.0, 0.5, center_dist)) * 0.25;
    return col;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let t = globals.time;
    let style = u32(round(globals.wallpaper_style));

    var color: vec3<f32>;
    if (style == 0u) {
        color = render_solar_amber_glow(uv, t);
    } else if (style == 1u) {
        color = render_alpine_lake(uv, t);
    } else if (style == 2u) {
        color = render_aurora_midnight(uv, t);
    } else if (style == 3u) {
        color = render_sunset_horizon(uv, t);
    } else if (style == 4u) {
        color = render_emerald_zen(uv, t);
    } else {
        color = render_minimal_studio(uv);
    }

    // Focus mode vignette: dims periphery during deep study focus
    let center_dist = length(uv - vec2<f32>(0.5, 0.5));
    let vignette = smoothstep(0.85, 0.15, center_dist);
    color = color * mix(1.0, 0.55 + vignette * 0.45, globals.focus_depth);

    // Ambient wallpaper daily progress modulation
    let warmth = clamp(globals.daily_progress, 0.0, 1.0);
    let luma = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    let sat_boost = mix(0.85, 1.15, warmth);
    color = mix(vec3<f32>(luma), color, sat_boost);
    color = color * mix(0.92, 1.06, warmth);

    // Subtle dither to eliminate 8-bit banding
    color += vec3<f32>(dither(in.position.xy));

    return vec4<f32>(color, 1.0);
}
