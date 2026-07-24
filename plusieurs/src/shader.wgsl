
// ------------------------------------------- bind for mid
struct Uniforms {
    model_matrix: mat4x4<f32>,
};

@group(0) @binding(0) 
var<uniform> uniforms: Uniforms;

// ------------------------------------------- bind for blend
@group(0) @binding(0)
var base_texture: texture_2d<f32>;
@group(0) @binding(1)
var mid_texture: texture_2d<f32>;
@group(0) @binding(2)
var blend_sampler: sampler;

// ------------------------------------------- bind for scaler
@group(0) @binding(0)
var blend_texture: texture_2d<f32>;
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
    @location(1) pos: vec2<f32>,
    @location(2) colour: vec3<f32>,
};

// ------------------------------------------- vs for base
@vertex
fn vs_base(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.colour = vec3<f32>(1.0, 0.0, 0.0);
    out.position = vec4<f32>(model.position.xy, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// ------------------------------------------- vs for mid
@vertex
fn vs_mid(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.colour = vec3<f32>(1.0, 1.0, 1.0);
    out.position = uniforms.model_matrix * vec4<f32>(model.position.xy, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// ------------------------------------------- vs for blend
@vertex
fn vs_blend(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;

    var pos = model.position.xy;

    out.position = vec4(pos, 0.0, 1.0);
    out.uv = model.uv;
    out.pos = pos;
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
    out.pos = pos;
    return out;
}

// ------------------------------------------- fs for base
@fragment
fn fs_base(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.colour, 1.0);
}

// ------------------------------------------- fs for mid
@fragment
fn fs_mid(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.colour, 1.0);
}

// ------------------------------------------- fs for blend
@fragment
fn fs_blend(in: VertexOutput) -> @location(0) vec4<f32> {
    var base_colour = textureSample(base_texture, blend_sampler, in.uv);
    var mid_colour = textureSample(mid_texture, blend_sampler, in.uv);
    var blend = (base_colour.rgb + mid_colour.rgb) / 2;
    return vec4<f32>(blend, 1.0);
}
// ------------------------------------------- fs for scaler
@fragment
fn fs_scaler(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(blend_texture, scaler_sampler, in.uv);
}


