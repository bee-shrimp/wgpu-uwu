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
    model: VertexInput, @builtin(instance_index) instance_index: u32
) -> VertexOutput {
    var out: VertexOutput;

    let row = instance_index / 5u;
    let col = instance_index % 5u;
    let x = f32(col) * 0.4 - 0.8;
    let y = f32(row) * 0.4 - 0.8;

    var pos = model.position.xy;
    pos.x += x;
    pos.y += y;

    pos = (uniforms.model_matrix * vec4<f32>(pos, 0.0, 1.0)).xy;

    out.position = vec4(pos, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.uv.x, in.uv.y, 1.0, 1.0);
}

