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
    let intensity = locals.input;
    var uv = in.uv;

    let start = 0.5;
    let end = 0.0;
    let mapped_uv_y = start + uv.y * (end - start);

    var offset = sin((1.0 - uv.y) * time * uv.y * 2) / 100;

    uv.x += offset;
    uv.y = mapped_uv_y;

    let colour = textureSample(mid_texture, effect_sampler, uv);
    return vec4(colour.rgb, 0.5);
}

fn rand(time: f32, uv_y: f32) -> f32 {
    let seeds = vec2<f32>(time, uv_y);
    let randf = fract(sin(dot(seeds, vec2(12.9898, 78.233))) * 43758.5453);

    return randf;
}


