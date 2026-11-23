// MindSet OS — Frosted Glass Shader with WCAG AAA Adaptive Scrims & Meniscus Rims
// Compliant with `/mindset-os-ui-rust` skill.

struct GlassUniforms {
    quad_rect_px: vec4<f32>,
    viewport_px: vec2<f32>,
    corner_radius_px: f32,
    noise_dither_amp: f32,
    scrim_color: vec4<f32>,
    rim_top_left: vec4<f32>,
    rim_bottom_right: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: GlassUniforms;
@group(0) @binding(1) var blurred_backdrop: texture_2d<f32>;
@group(0) @binding(2) var backdrop_sampler: sampler;

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) local_px: vec2<f32>,
    @location(1) screen_uv: vec2<f32>,
};

fn rounded_box_sdf(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn hash12(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let half_size = u.quad_rect_px.zw * 0.5;
    let center_rel = in.local_px - half_size;
    let dist = rounded_box_sdf(center_rel, half_size, u.corner_radius_px);

    let shape_alpha = 1.0 - smoothstep(-0.75, 0.75, dist);
    if (shape_alpha <= 0.001) {
        discard;
    }

    let grad = normalize(vec2<f32>(dpdx(dist), dpdy(dist)) + vec2<f32>(1e-5));
    let bevel_mask = smoothstep(-6.0, 0.0, dist);
    let refracted_uv = clamp(in.screen_uv - grad * bevel_mask * 0.003, vec2<f32>(0.0), vec2<f32>(1.0));
    let backdrop_rgb = textureSample(blurred_backdrop, backdrop_sampler, refracted_uv).rgb;

    let dither = (hash12(in.local_px) - 0.5) * u.noise_dither_amp;
    let base_glass = mix(backdrop_rgb, u.scrim_color.rgb, u.scrim_color.a) + vec3<f32>(dither);

    let rim_band = smoothstep(-1.5, -0.25, dist) * (1.0 - smoothstep(-0.25, 0.5, dist));
    let sun_align = clamp(dot(grad, normalize(vec2<f32>(-0.6, -0.8))) * 0.5 + 0.5, 0.0, 1.0);
    let rim_color = mix(u.rim_bottom_right, u.rim_top_left, sun_align);

    let final_rgb = mix(base_glass, rim_color.rgb, rim_band * rim_color.a);
    return vec4<f32>(final_rgb, shape_alpha);
}
