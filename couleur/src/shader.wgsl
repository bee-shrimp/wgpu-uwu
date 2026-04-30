struct Uniforms {
    model_matrix: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
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

    out.position = uniforms.model_matrix * vec4<f32>(model.position.xy, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.uv.x, in.uv.y, 1.0, 1.0);
}

//@vertex
//fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
//    let x = f32(i32(in_vertex_index) - 1);
//    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
//    return vec4<f32>(x, y, 0.0, 1.0);
//}
//
//struct Locals {
//    time: f32,
//}
//
//@group(0) @binding(0) var<uniform> locals: Locals;
//
//@fragment
//fn fs_main() -> @location(0) vec4<f32> {
//    return vec4<f32>((sin(locals.time) + 1) / 2, 0.3, 0.9, 1.0);
//}
