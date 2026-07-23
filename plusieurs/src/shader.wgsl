
// ------------------------------------------- bind for mid
struct Uniforms {
    model_matrix: mat4x4<f32>,
};

@group(0) @binding(0) 
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_sampler: sampler;

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
fn vs_mid(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.position = uniforms.model_matrix * vec4<f32>(model.position.xy, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// ------------------------------------------- vs for scaler
@vertex
fn vs_scaler(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;

    var pos = model.position.xy;

    out.position = vec4(pos, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// ------------------------------------------- fs for mid
@fragment
fn fs_mid(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(diffuse_texture, diffuse_sampler, in.uv);
}

// ------------------------------------------- fs for scaler
@fragment
fn fs_scaler(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(mid_texture, scaler_sampler, in.uv);
}

