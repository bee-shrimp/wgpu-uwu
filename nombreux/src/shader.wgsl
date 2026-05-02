struct Uniforms {
    model_matrix: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) vertex_pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) pos: vec2<f32>};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput, @builtin(instance_index) instance_index: u32
) -> VertexOutput {
    var out: VertexOutput;

    var pos = model.vertex_pos.xy;

    pos = (uniforms.model_matrix * vec4<f32>(model.vertex_pos.xy, 0.0, 1.0)).xy;

    pos += model.pos;

    out.position = vec4(pos.xy, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.uv.xy, 1.0, 1.0);
}

