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
    let intensity = locals.input * 10.0;
    var uv = in.uv;

    let start = 0.5;
    let end = 0.0;
    let mapped_uv_y = start + uv.y * (end - start);

    let frequency = 11.0; // 1.0-5.0
    let amplitude = 0.007; // 0.01-0.2
    let speed = 0.2; // 0.5-5.0

    let inverse_y = 1.0 - uv.y;
    let inverse_x = 1.0 - uv.x;

    let offset_x1 = sin(inverse_y * frequency + time * speed) * amplitude * 0.5;
    let offset_x2 = sin(inverse_y * (frequency * 5.0) + time * (speed * 1.7)) * (amplitude * 0.4);
    let offset_x3 = sin(inverse_y * (frequency * 11.0) + time * (speed * 2.1)) * (amplitude * 0.3);
    let offset_y1 = sin(inverse_x * (frequency * 4.0) + time * (speed * 1.1)) * (amplitude * 0.2);
    let offset_y2 = sin(inverse_x * (frequency * 8.0) + time * (speed * 1.3)) * (amplitude * 0.1);

    uv.x = uv.x + (offset_x1 + offset_x2 + offset_x3) * intensity;
    uv.y = mapped_uv_y + (offset_y1 + offset_y2) * intensity;

    let colour = textureSample(mid_texture, effect_sampler, uv);
    return vec4(colour.rgb, 0.6);
}

fn rand(time: f32, uv_y: f32) -> f32 {
    let seeds = vec2<f32>(time, uv_y);
    let randf = fract(sin(dot(seeds, vec2(12.9898, 78.233))) * 43758.5453);

    return randf;
}


