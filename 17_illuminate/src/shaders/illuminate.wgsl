struct Locals {
    time: f32,
    input: f32}

@group(0) @binding(0) 
var<uniform> locals: Locals;
@group(0) @binding(1)
var mid_texture: texture_2d<f32>;
@group(0) @binding(2)
var effect_sampler: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(model.position.xy, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let time = locals.time;
    let intensity = locals.input * 10.0;
    var uv = in.uv;

    let base_uv = in.uv;
    let size = textureDimensions(mid_texture, 0);
    let texel_size = 1.0 / vec2<f32>(f32(size.x), f32(size.y));
    let center_uv = base_uv + texel_size * 0.5;

    let weight0 = 4.0 / 16.0;
    let weight1 = 2.0 / 16.0;
    let weight2 = 1.0 / 16.0;

    var bright_colour = textureSample(mid_texture, effect_sampler, center_uv) * weight0;

    bright_colour += textureSample(mid_texture, effect_sampler, center_uv + vec2(1.0, 0.0) * texel_size) * weight1;
    bright_colour += textureSample(mid_texture, effect_sampler, center_uv + vec2(-1.0, 0.0) * texel_size) * weight1;
    bright_colour += textureSample(mid_texture, effect_sampler, center_uv + vec2(0.0, 1.0) * texel_size) * weight1;
    bright_colour += textureSample(mid_texture, effect_sampler, center_uv + vec2(0.0, -1.0) * texel_size) * weight1;
    bright_colour += textureSample(mid_texture, effect_sampler, center_uv + vec2(1.0, 1.0) * texel_size) * weight2;
    bright_colour += textureSample(mid_texture, effect_sampler, center_uv + vec2(1.0, -1.0) * texel_size) * weight2;
    bright_colour += textureSample(mid_texture, effect_sampler, center_uv + vec2(-1.0, 1.0) * texel_size) * weight2;
    bright_colour += textureSample(mid_texture, effect_sampler, center_uv + vec2(-1.0, -1.0) * texel_size) * weight2;

    return vec4(bright_colour.rgb, 0.8);
}

