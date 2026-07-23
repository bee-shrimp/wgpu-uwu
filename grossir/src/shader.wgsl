struct Uniforms {
    model_matrix: mat4x4<f32>,
};

@group(0) @binding(0) 
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var mid_t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var mid_s_diffuse: sampler;

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

struct MidVertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct ScalerVertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_mid(
    model: MidVertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.position = uniforms.model_matrix * vec4<f32>(model.position.xy, 0.0, 1.0);
    return out;
}

@vertex
fn vs_scaler(
    model: ScalerVertexInput
) -> VertexOutput {
    var out: VertexOutput;

    var pos = model.position.xy;

    out.position = vec4(pos, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

@fragment
fn fs_mid(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(mid_t_diffuse, mid_s_diffuse, in.uv);
}

@fragment
fn fs_scaler(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv);
}

