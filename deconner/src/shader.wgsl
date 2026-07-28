
struct Locals {
    time: f32,
}

// ------------------------------------------- bind for mid
@group(0) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(0) @binding(1)
var mid_sampler: sampler;

// ------------------------------------------- bind for effect
@group(0) @binding(0) var<uniform> locals: Locals;
@group(0) @binding(1)
var mid_texture: texture_2d<f32>;
@group(0) @binding(2)
var effect_sampler: sampler;

// ------------------------------------------- vertex struct for both
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// ------------------------------------------- vs for mid and effect
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(model.position.xy, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// ------------------------------------------- fs for mid
@fragment
fn fs_mid(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(diffuse_texture, mid_sampler, in.uv);
}

// ------------------------------------------- fs for effect 
@fragment
fn fs_effect(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = in.position;
    var uv = in.uv;

    let rand = rand(locals.time, pos.x * uv.y);

    let offset = select(0.0, rand, uv.y >= 0.5 && uv.y <= 0.52);
    uv.x += offset;
    var colour = textureSample(mid_texture, effect_sampler, uv);

    return vec4<f32>(rand, rand, rand, 1.0);
}

fn rand(seed: f32, seedy: f32) -> f32 {
    let seeds = vec2<f32>(seed, seedy);
    return fract(sin(dot(seeds, vec2(12.9898, 78.233))) * 43758.5453);
}
