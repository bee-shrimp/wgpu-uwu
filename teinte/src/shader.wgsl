
struct Locals {
    time: f32,
}

// ------------------------------------------- bind for mid
@group(0) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(0) @binding(1)
var mid_sampler: sampler;

// ------------------------------------------- bind for effect
@group(0) @binding(0) var<uniform> locals: Locals;
@group(0) @binding(1)
var scaler_texture: texture_2d<f32>;
@group(0) @binding(2)
var effect_sampler: sampler;

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
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(model.position.xy, 0.0, 1.0);
    out.uv = model.uv;
    return out;
}

// ------------------------------------------- fs for mid
@fragment
fn fs_mid(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(diffuse_texture, mid_sampler, in.uv);
}

// ------------------------------------------- fs for scaler
@fragment
fn fs_scaler(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(mid_texture, scaler_sampler, in.uv);
}

// ------------------------------------------- fs for effect 
@fragment
fn fs_effect(in: VertexOutput) -> @location(0) vec4<f32> {
    let colour = textureSample(scaler_texture, effect_sampler, in.uv);
    let hsv = rgb_to_hsv(colour.rgb);
    let rotated_hue = fract(hsv.x + locals.time); // fract(x) means x - floor(x)
    let result = hsv_to_rgb(vec3<f32>(rotated_hue, hsv.y, hsv.z));

    return vec4<f32>(colour);
}

fn rgb_to_hsv(rgb: vec3<f32>) -> vec3<f32> {

    let max = max(max(rgb.r, rgb.g), rgb.b);
    let min = min(min(rgb.r, rgb.g), rgb.b);

    let delta = max - min;
    let saturation = select(0.0, delta / max, delta > 0.0);

    var hue: f32;

    let hue_if_max_red = 60 * (fract(rgb.g - rgb.b) / delta);
    let hue_if_max_green = 60 * ((rgb.b - rgb.r) / delta + 2.0);
    let hue_if_max_blue = 60 * ((rgb.r - rgb.g) / delta + 4.0);

    hue = select(hue_if_max_green, hue_if_max_red, rgb.r >= rgb.g && rgb.r >= rgb.b); // see if red is max
    hue = select(hue, hue_if_max_blue, rgb.b >= rgb.r && rgb.b >= rgb.g); // see if blue is max

    return vec3<f32>(hue, saturation, max);
}

fn hsv_to_rgb(hsv: vec3<f32>) -> vec3<f32> {
    let hue = hsv.x;
    let saturation = hsv.y;
    let value = hsv.z;

    return vec3<f32>();
}
