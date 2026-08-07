struct Locals {
    time: f32,
    input: f32}

@group(0) @binding(0) 
var<uniform> locals: Locals;
@group(0) @binding(1)
var resource_texture: texture_2d<f32>;
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

    let start = 0.8;
    let end = 0.15;
    let mapped_uv_y = start + uv.y * (end - start);

    let base_frequency = 11.0;
    let base_speed = 0.1;
    let amplitude = 0.007;

    let inverse_y = 1.0 - uv.y;
    let inverse_x = 1.0 - uv.x;

    let frequency_y = inverse_y * base_frequency;
    let frequency_x = inverse_x * base_frequency;

    let speed = time * base_speed;

    let offset_x1 = sin(frequency_y + speed) * amplitude * 0.5;
    let offset_x2 = sin((frequency_y * 5.0) + (speed * 1.7)) * (amplitude * 0.4);
    let offset_x3 = sin((frequency_y * 11.0) + (speed * 2.1)) * (amplitude * 0.3);
    let offset_y1 = sin((frequency_x * 0.2) + (speed * 0.3)) * (amplitude * 0.08);
    let offset_y2 = sin((frequency_x * 1.1) + (speed * 1.3)) * (amplitude * 0.07);
    let offset_y3 = sin((frequency_x * 1.8) + (speed * 1.9)) * (amplitude * 0.06);

    uv.x = uv.x + (offset_x1 + offset_x2 + offset_x3) * intensity;
    uv.y = mapped_uv_y + (offset_y1 + offset_y2 + offset_y3) * intensity;

    let colour = textureSample(resource_texture, effect_sampler, uv);
    return vec4(colour.rgb, 0.6);
}

