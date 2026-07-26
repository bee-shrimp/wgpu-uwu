
struct Locals {
    direction: f32,
}

// ------------------------------------------- bind for mid
@group(0) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(0) @binding(1)
var mid_sampler: sampler;

// ------------------------------------------- bind for effect
@group(0) @binding(0) var<uniform> locals: Locals;
@group(0) @binding(1)
var scaler_texture: texture_2d<f32>;
@group(0) @binding(2)
var effect_sampler: sampler;

// ------------------------------------------- bind for scaler
@group(0) @binding(0)
var mid_texture: texture_2d<f32>;
@group(0) @binding(1)
var scaler_sampler: sampler;

// ------------------------------------------- vertex struct for both
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// ------------------------------------------- vs for mid
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

// ------------------------------------------- fs for scaler
@fragment
fn fs_scaler(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(mid_texture, scaler_sampler, in.uv);
}

// ------------------------------------------- fs for effect 
@fragment
fn fs_effect(in: VertexOutput) -> @location(0) vec4<f32> {
    var colour = textureSample(scaler_texture, effect_sampler, in.uv);
    var blend = colour.rgb + locals.direction;
    var result = clamp(blend, vec3<f32>(0.0), vec3<f32>(1.0));
    return vec4<f32>(result, 1.0);
}
