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

struct BlurUniforms {
    texel_size: vec2<f32>,
    offset: f32,
    luminance_clamp: f32,
};

@group(0) @binding(0) var<uniform> uniforms: BlurUniforms;
@group(0) @binding(1) var input_texture: texture_2d<f32>;
@group(0) @binding(2) var input_sampler: sampler;

@fragment
fn fs_downsample(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let d = uniforms.texel_size * uniforms.offset;

    // 4 bilinear tap offsets
    let c0 = textureSample(input_texture, input_sampler, uv + vec2<f32>(-d.x, -d.y));
    let c1 = textureSample(input_texture, input_sampler, uv + vec2<f32>( d.x, -d.y));
    let c2 = textureSample(input_texture, input_sampler, uv + vec2<f32>(-d.x,  d.y));
    let c3 = textureSample(input_texture, input_sampler, uv + vec2<f32>( d.x,  d.y));

    return (c0 + c1 + c2 + c3) * 0.25;
}

@fragment
fn fs_upsample(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let d = uniforms.texel_size * uniforms.offset;

    // Tent filter (8 perimeter taps)
    let c0 = textureSample(input_texture, input_sampler, uv + vec2<f32>(-d.x * 2.0, 0.0));
    let c1 = textureSample(input_texture, input_sampler, uv + vec2<f32>( d.x * 2.0, 0.0));
    let c2 = textureSample(input_texture, input_sampler, uv + vec2<f32>(0.0, -d.y * 2.0));
    let c3 = textureSample(input_texture, input_sampler, uv + vec2<f32>(0.0,  d.y * 2.0));

    let c4 = textureSample(input_texture, input_sampler, uv + vec2<f32>(-d.x, -d.y));
    let c5 = textureSample(input_texture, input_sampler, uv + vec2<f32>( d.x, -d.y));
    let c6 = textureSample(input_texture, input_sampler, uv + vec2<f32>(-d.x,  d.y));
    let c7 = textureSample(input_texture, input_sampler, uv + vec2<f32>( d.x,  d.y));

    let color = (c0 + c1 + c2 + c3 + (c4 + c5 + c6 + c7) * 2.0) / 12.0;

    // Subtle luminance compression to guarantee text contrast
    let luma = dot(color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    var compressed_rgb = color.rgb;
    if (luma > uniforms.luminance_clamp && uniforms.luminance_clamp > 0.0) {
        let factor = uniforms.luminance_clamp / luma;
        compressed_rgb = mix(color.rgb, color.rgb * factor, 0.45);
    }

    return vec4<f32>(compressed_rgb, color.a);
}

