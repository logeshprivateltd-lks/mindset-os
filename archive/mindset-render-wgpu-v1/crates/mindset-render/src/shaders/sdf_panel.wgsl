struct SdfUniforms {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: SdfUniforms;
@group(0) @binding(1) var blurred_texture: texture_2d<f32>;
@group(0) @binding(2) var blurred_sampler: sampler;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    // Instance attributes
    @location(0) rect: vec4<f32>,           // x, y, width, height (or cx, cy, radius, thickness for ring)
    @location(1) radii: vec4<f32>,          // tl, tr, br, bl (negative x for ring)
    @location(2) fill_color: vec4<f32>,     // rgba
    @location(3) border_color: vec4<f32>,   // rgba
    @location(4) shadow_color: vec4<f32>,   // rgba
    @location(5) params: vec4<f32>,         // border_width, shadow_blur, shadow_offset_y, glass_mix
    @location(6) clip_rect: vec4<f32>,      // min_x, min_y, max_x, max_y (scissor bounds)
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) frag_pixel: vec2<f32>,
    @location(1) rect: vec4<f32>,
    @location(2) radii: vec4<f32>,
    @location(3) fill_color: vec4<f32>,
    @location(4) border_color: vec4<f32>,
    @location(5) shadow_color: vec4<f32>,
    @location(6) params: vec4<f32>,
    @location(7) clip_rect: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Unit quad indices (0..6 -> 2 triangles)
    // 0: (0,0), 1: (1,0), 2: (0,1), 3: (0,1), 4: (1,0), 5: (1,1)
    var uv = vec2<f32>(0.0, 0.0);
    switch in.vertex_index {
        case 0u: { uv = vec2<f32>(0.0, 0.0); }
        case 1u: { uv = vec2<f32>(1.0, 0.0); }
        case 2u: { uv = vec2<f32>(0.0, 1.0); }
        case 3u: { uv = vec2<f32>(0.0, 1.0); }
        case 4u: { uv = vec2<f32>(1.0, 0.0); }
        default: { uv = vec2<f32>(1.0, 1.0); }
    }

    var min_x: f32;
    var max_x: f32;
    var min_y: f32;
    var max_y: f32;

    // Identify progress ring:
    // 1. in.radii.x < -0.5 (standard progress ring indicator)
    // 2. Compatibility fallback: radii == [1,1,1,1], params.z == 1.0, shadow_color.a == 0.0, params.w == 0.0
    let is_ring = in.radii.x < -0.5 || (in.radii.x == 1.0 && in.radii.y == 1.0 && in.radii.z == 1.0 && in.radii.w == 1.0 && in.params.z == 1.0 && in.shadow_color.w == 0.0 && in.params.w == 0.0);

    if (is_ring) {
        let r_extent = in.rect.z + 3.0;
        min_x = in.rect.x - r_extent;
        max_x = in.rect.x + r_extent;
        min_y = in.rect.y - r_extent;
        max_y = in.rect.y + r_extent;
    } else {
        // Expand bounding box to fully accommodate drop shadow blur and offset
        let blur_pad = max(in.params.y * 2.2, 4.0);
        min_x = in.rect.x - blur_pad;
        max_x = in.rect.x + in.rect.z + blur_pad;
        min_y = in.rect.y - blur_pad + min(0.0, in.params.z * 1.5);
        max_y = in.rect.y + in.rect.w + blur_pad + max(0.0, in.params.z * 1.5);
    }

    // Clip bounding quad to scissor clip_rect if specified
    let has_clip = in.clip_rect.z > in.clip_rect.x && in.clip_rect.w > in.clip_rect.y;
    if (has_clip) {
        min_x = max(min_x, in.clip_rect.x);
        max_x = min(max_x, in.clip_rect.z);
        min_y = max(min_y, in.clip_rect.y);
        max_y = min(max_y, in.clip_rect.w);
        max_x = max(min_x, max_x);
        max_y = max(min_y, max_y);
    }

    let world_pos = vec2<f32>(
        mix(min_x, max_x, uv.x),
        mix(min_y, max_y, uv.y)
    );

    // Project pixel coordinates to NDC [-1.0, 1.0]
    let ndc_x = (world_pos.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (world_pos.y / uniforms.screen_size.y) * 2.0;

    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.frag_pixel = world_pos;
    out.rect = in.rect;
    out.radii = in.radii;
    out.fill_color = in.fill_color;
    out.border_color = in.border_color;
    out.shadow_color = in.shadow_color;
    out.params = in.params;
    out.clip_rect = in.clip_rect;

    return out;
}

// Exact 2D Signed Distance Field for a rounded rectangle with variable corner radii
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

// 2D Signed Distance Field for an anti-aliased ring / donut
fn sd_ring(p: vec2<f32>, outer_r: f32, thickness: f32) -> f32 {
    return abs(length(p) - (outer_r - thickness * 0.5)) - thickness * 0.5;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Scissor / boundary clipping test
    let has_clip = in.clip_rect.z > in.clip_rect.x && in.clip_rect.w > in.clip_rect.y;
    if (has_clip) {
        if (in.frag_pixel.x < in.clip_rect.x || in.frag_pixel.x > in.clip_rect.z ||
            in.frag_pixel.y < in.clip_rect.y || in.frag_pixel.y > in.clip_rect.w) {
            discard;
        }
    }

    let is_ring = in.radii.x < -0.5 || (in.radii.x == 1.0 && in.radii.y == 1.0 && in.radii.z == 1.0 && in.radii.w == 1.0 && in.params.z == 1.0 && in.shadow_color.w == 0.0 && in.params.w == 0.0);

    // Progress Ring rendering branch
    if (is_ring) {
        let center = in.rect.xy;
        let local_p = in.frag_pixel - center;
        let outer_r = in.rect.z;
        let thickness = in.rect.w;
        let d_ring = sd_ring(local_p, outer_r, thickness);
        if (d_ring > 1.5) {
            discard;
        }

        let ang = atan2(local_p.y, local_p.x);
        let start = in.params.y;
        let progress = clamp(in.params.x, 0.0, 1.0);
        let sweep = progress * 6.2831853;

        var a = ang - start;
        if (a < 0.0) {
            a = a + 6.2831853;
        }

        let in_sweep = step(a, sweep);
        let track_alpha = smoothstep(1.0, -1.0, d_ring) * (1.0 - in_sweep);
        let fill_alpha  = smoothstep(1.0, -1.0, d_ring) * in_sweep;

        let col = in.fill_color.rgb * in.fill_color.a * track_alpha + in.border_color.rgb * in.border_color.a * fill_alpha;
        let total_alpha = in.fill_color.a * track_alpha + in.border_color.a * fill_alpha;
        if (total_alpha < 0.002) {
            discard;
        }
        return vec4<f32>(col / max(total_alpha, 0.0001), total_alpha);
    }

    // Standard rounded box / panel rendering branch
    let center = in.rect.xy + in.rect.zw * 0.5;
    let half_size = in.rect.zw * 0.5;
    let local_p = in.frag_pixel - center;

    let dist = sd_rounded_box(local_p, half_size, in.radii);

    // Drop shadow computation (offset along Y axis)
    let shadow_blur = in.params.y;
    let shadow_offset_y = in.params.z;
    let local_p_shadow = in.frag_pixel - (center + vec2<f32>(0.0, shadow_offset_y));
    let dist_shadow = sd_rounded_box(local_p_shadow, half_size, in.radii);

    // Outside the panel surface
    if (dist > 0.0) {
        if (shadow_blur > 0.0 && in.shadow_color.a > 0.0) {
            let shadow_factor = 1.0 - smoothstep(0.0, shadow_blur, dist_shadow);
            let alpha = shadow_factor * in.shadow_color.a;
            if (alpha < 0.002) {
                discard;
            }
            return vec4<f32>(in.shadow_color.rgb, alpha);
        } else {
            discard;
        }
    }

    // Inside the panel surface (dist <= 0.0)
    let glass_mix = in.params.w;
    var surface_rgb = in.fill_color.rgb;
    var surface_alpha = in.fill_color.a;

    if (glass_mix > 0.0) {
        let screen_uv = in.frag_pixel / uniforms.screen_size;

        // Subtle edge normal refraction
        let normal = normalize(local_p + vec2<f32>(0.0001, 0.0001));
        let edge_proximity = 1.0 - smoothstep(0.0, 16.0, -dist);
        let refract_uv = clamp(screen_uv + normal * 0.004 * edge_proximity, vec2<f32>(0.0), vec2<f32>(1.0));

        let glass_color = textureSample(blurred_texture, blurred_sampler, refract_uv);

        // Blend surface tint with sampled blurred glass
        surface_rgb = mix(in.fill_color.rgb, mix(glass_color.rgb, in.fill_color.rgb, 0.28), glass_mix);
        surface_alpha = mix(in.fill_color.a, 1.0, glass_mix);
    }

    // Top-lit specular rim highlight along border
    let border_width = in.params.x;
    if (border_width > 0.0 && in.border_color.a > 0.0) {
        let border_dist = abs(dist + border_width * 0.5) - border_width * 0.5;
        let border_alpha = 1.0 - smoothstep(0.0, 1.2, border_dist);

        // Directional optical meniscus rim with sun vector alignment [-0.6, -0.8]
        let normal = normalize(local_p + vec2<f32>(0.0001, 0.0001));
        let sun_align = clamp(dot(normal, normalize(vec2<f32>(-0.6, -0.8))) * 0.5 + 0.5, 0.0, 1.0);
        let top_specular = 0.70 + 0.55 * sun_align;
        let border_rgb = in.border_color.rgb * top_specular;

        surface_rgb = mix(surface_rgb, border_rgb, border_alpha * in.border_color.a);
    }

    // Anti-alias exterior perimeter
    let edge_aa = 1.0 - smoothstep(-1.0, 0.0, dist);

    return vec4<f32>(surface_rgb, surface_alpha * edge_aa);
}
