struct TextUniforms {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: TextUniforms;
@group(0) @binding(1) var glyph_atlas: texture_2d<f32>;
@group(0) @binding(2) var glyph_sampler: sampler;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    // Instance attributes
    @location(0) pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) uv_min: vec2<f32>,
    @location(3) uv_max: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) is_color: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) is_color: u32,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Unit quad (0..6 -> 2 triangles)
    var quad_uv = vec2<f32>(0.0, 0.0);
    switch in.vertex_index {
        case 0u: { quad_uv = vec2<f32>(0.0, 0.0); }
        case 1u: { quad_uv = vec2<f32>(1.0, 0.0); }
        case 2u: { quad_uv = vec2<f32>(0.0, 1.0); }
        case 3u: { quad_uv = vec2<f32>(0.0, 1.0); }
        case 4u: { quad_uv = vec2<f32>(1.0, 0.0); }
        default: { quad_uv = vec2<f32>(1.0, 1.0); }
    }

    let world_pos = in.pos + in.size * quad_uv;
    let ndc_x = (world_pos.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (world_pos.y / uniforms.screen_size.y) * 2.0;

    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.uv = mix(in.uv_min, in.uv_max, quad_uv);
    out.color = in.color;
    out.is_color = in.is_color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(glyph_atlas, glyph_sampler, in.uv);
    if (in.is_color == 1u) {
        // Render full-color emoji bitmap directly without tinting it yellow
        if (tex_color.a <= 0.01) {
            discard;
        }
        return vec4<f32>(tex_color.rgb, tex_color.a * in.color.a);
    }
    let final_color = in.color * tex_color;
    if (final_color.a <= 0.01) {
        discard;
    }
    return final_color;
}

