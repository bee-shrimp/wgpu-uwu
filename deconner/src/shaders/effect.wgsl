struct Locals {
    time: f32,
    input: f32}

@group(0) @binding(0) var<uniform> locals: Locals;
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
    let input = locals.input;
    let pos = in.position;
    let uv = in.uv;

    let scanline_id = floor(uv.y * 100);

    let line_rnd = rand(floor(time), scanline_id);

    let is_glitch = line_rnd >= 1.0 - input * 10;

    let offset = select(0.0, rand(time, scanline_id + 1.0), is_glitch);

    var base_uv = uv;
    base_uv.x += offset * input;

    var colour = textureSample(mid_texture, effect_sampler, base_uv);

    return vec4<f32>(colour);
}

fn rand(time: f32, uv_y: f32) -> f32 {
    let seeds = vec2<f32>(time, uv_y);
    let rand_p = fract(sin(dot(seeds, vec2(12.9898, 78.233))) * 43758.5453);
    let rand_np = (rand_p * 2.0) - 1.0;
    let randf = select(rand_p, rand_np, rand_p <= 0.02);

    return randf;
}


