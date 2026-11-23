// MindSet OS — Liquid Glass & Frosted Glassmorphism Fragment Shader (WGSL)
// iOS 18 / visionOS Liquidmorphism Pipeline for LKS PVT LTD.
// Features: Dual-Kawase blurred background sampling, variable corner radii SDF,
// specular refraction inner border gradient, glossy liquid pill buttons.

struct LiquidGlassUniforms {
    screen_size: vec2<f32>,
    blur_radius: f32,
    time: f32,
};

@group(0) @binding(0) var<uniform> uniforms: LiquidGlassUniforms;
@group(0) @binding(1) var blurred_texture: texture_2d<f32>;
@group(0) @binding(2) var blurred_sampler: sampler;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) rect: vec4<f32>,           // x, y, width, height
    @location(1) radii: vec4<f32>,          // tl, tr, br, bl
    @location(2) fill_color: vec4<f32>,     // base tint rgba
    @location(3) border_top: vec4<f32>,     // top specular border rgba
    @location(4) border_bottom: vec4<f32>,  // bottom specular border rgba
    @location(5) params: vec4<f32>,         // border_width, shadow_blur, shadow_offset_y, glass_mix
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) frag_pixel: vec2<f32>,
    @location(1) rect: vec4<f32>,
    @location(2) radii: vec4<f32>,
    @location(3) fill_color: vec4<f32>,
    @location(4) border_top: vec4<f32>,
    @location(5) border_bottom: vec4<f32>,
    @location(6) params: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Unit quad (0..6 -> 2 triangles)
    var uv = vec2<f32>(0.0, 0.0);
    switch in.vertex_index {
        case 0u: { uv = vec2<f32>(0.0, 0.0); }
        case 1u: { uv = vec2<f32>(1.0, 0.0); }
        case 2u: { uv = vec2<f32>(0.0, 1.0); }
        case 3u: { uv = vec2<f32>(0.0, 1.0); }
        case 4u: { uv = vec2<f32>(1.0, 0.0); }
        default: { uv = vec2<f32>(1.0, 1.0); }
    }

    let shadow_pad = max(in.params.y * 2.5, 6.0);
    let min_x = in.rect.x - shadow_pad;
    let max_x = in.rect.x + in.rect.z + shadow_pad;
    let min_y = in.rect.y - shadow_pad + min(0.0, in.params.z * 1.5);
    let max_y = in.rect.y + in.rect.w + shadow_pad + max(0.0, in.params.z * 1.5);

    let world_pos = vec2<f32>(
        mix(min_x, max_x, uv.x),
        mix(min_y, max_y, uv.y)
    );

    let ndc_x = (world_pos.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (world_pos.y / uniforms.screen_size.y) * 2.0;

    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.frag_pixel = world_pos;
    out.rect = in.rect;
    out.radii = in.radii;
    out.fill_color = in.fill_color;
    out.border_top = in.border_top;
    out.border_bottom = in.border_bottom;
    out.params = in.params;

    return out;
}

// Signed Distance Field for rounded rectangle with per-corner radii
fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    var rad: f32;
    if (p.x > 0.0) {
        if (p.y > 0.0) { rad = r.z; } else { rad = r.y; }
    } else {
        if (p.y > 0.0) { rad = r.w; } else { rad = r.x; }
    }
    let q = abs(p) - b + vec2<f32>(rad, rad);
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0, 0.0))) - rad;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let center = in.rect.xy + in.rect.zw * 0.5;
    let half_size = in.rect.zw * 0.5;
    let local_p = in.frag_pixel - center;

    let dist = sd_rounded_box(local_p, half_size, in.radii);

    // Drop shadow calculation
    let shadow_blur = in.params.y;
    let shadow_offset_y = in.params.z;
    if (dist > 0.0) {
        if (shadow_blur > 0.0) {
            let shadow_p = in.frag_pixel - (center + vec2<f32>(0.0, shadow_offset_y));
            let shadow_dist = sd_rounded_box(shadow_p, half_size, in.radii);
            let shadow_alpha = (1.0 - smoothstep(0.0, shadow_blur, shadow_dist)) * 0.35;
            if (shadow_alpha < 0.003) {
                discard;
            }
            return vec4<f32>(0.0, 0.0, 0.0, shadow_alpha);
        } else {
            discard;
        }
    }

    // Interior frosted liquid glass refraction
    let glass_mix = in.params.w;
    var surface_rgb = in.fill_color.rgb;
    var surface_alpha = in.fill_color.a;

    if (glass_mix > 0.0) {
        let screen_uv = in.frag_pixel / uniforms.screen_size;

        // Subtle curved glass edge normal refraction
        let normal = normalize(local_p + vec2<f32>(0.0001, 0.0001));
        let edge_proximity = 1.0 - smoothstep(0.0, 20.0, -dist);
        let refract_uv = clamp(screen_uv + normal * 0.005 * edge_proximity, vec2<f32>(0.0), vec2<f32>(1.0));

        let glass_color = textureSample(blurred_texture, blurred_sampler, refract_uv);

        // Mix scenic wallpaper background with surface tint
        surface_rgb = mix(in.fill_color.rgb, mix(glass_color.rgb, in.fill_color.rgb, 0.30), glass_mix);
        surface_alpha = mix(in.fill_color.a, 0.90, glass_mix);
    }

    // Top-to-bottom inner specular border gradient with optical meniscus rim
    let border_width = in.params.x;
    if (border_width > 0.0) {
        let border_dist = abs(dist + border_width * 0.5) - border_width * 0.5;
        let border_factor = 1.0 - smoothstep(0.0, 1.2, border_dist);

        // Directional optical meniscus rim aligned with sun vector [-0.6, -0.8]
        let normal = normalize(local_p + vec2<f32>(0.0001, 0.0001));
        let sun_dir = normalize(vec2<f32>(-0.6, -0.8));
        let sun_align = clamp(dot(normal, sun_dir) * 0.5 + 0.5, 0.0, 1.0);
        let border_color = mix(in.border_bottom, in.border_top, sun_align);

        surface_rgb = mix(surface_rgb, border_color.rgb, border_factor * border_color.a);
    }

    // Anti-aliased outer perimeter
    let edge_aa = 1.0 - smoothstep(-1.0, 0.0, dist);

    return vec4<f32>(surface_rgb, surface_alpha * edge_aa);
}
