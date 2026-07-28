
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
var mid_texture: texture_2d<f32>;
@group(0) @binding(2)
var effect_sampler: sampler;

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

// ------------------------------------------- fs for effect 
@fragment
fn fs_effect(in: VertexOutput) -> @location(0) vec4<f32> {
    let colour = textureSample(mid_texture, effect_sampler, in.uv);
    let hsv = rgb_to_hsv(colour.rgb);

    // fract(x) means x - floor(x)
    let rotated_hue = fract(hsv.x + locals.time);

    let result = hsv_to_rgb(vec3<f32>(rotated_hue, hsv.y, hsv.z));

    return vec4<f32>(result, 1.0);
}

fn rgb_to_hsv(rgb: vec3<f32>) -> vec3<f32> {

    let max = max(max(rgb.r, rgb.g), rgb.b);
    let min = min(min(rgb.r, rgb.g), rgb.b);

    let delta = max - min;
    let saturation = select(0.0, delta / max, delta > 0.0);

    var hue: f32;

    var hue_if_max_red = 60 * ((rgb.g - rgb.b) / delta);
    hue_if_max_red = fract(hue_if_max_red);
    let hue_if_max_green = 60 * ((rgb.b - rgb.r) / delta + 2.0);
    let hue_if_max_blue = 60 * ((rgb.r - rgb.g) / delta + 4.0);

    hue = select(hue_if_max_green, hue_if_max_red, rgb.r >= rgb.g && rgb.r >= rgb.b); // see if red is max
    hue = select(hue, hue_if_max_blue, rgb.b >= rgb.r && rgb.b >= rgb.g); // see if blue is max

    return vec3<f32>(hue / 360, saturation, max);
}

fn hsv_to_rgb(hsv: vec3<f32>) -> vec3<f32> {
    let hue = hsv.x;
    let saturation = hsv.y;
    let value = hsv.z;

    let h6 = hue * 6;

    let i = floor(h6);
    let f = fract(h6);

    let max = value;
    let min = max * (1.0 - saturation);
    let inc = max * (1.0 - saturation * (1.0 - f));
    let dec = max * (1.0 - saturation * f);

    var r: f32;
    var g: f32;
    var b: f32;

    switch i32(i) {
        case 0: { r = max; g = inc; b = min; }
        case 1: { r = dec; g = max; b = min; }
        case 2: { r = min; g = max; b = inc; }
        case 3: { r = min; g = dec; b = max; }
        case 4: { r = inc; g = min; b = max; }
        case 5: { r = max; g = min; b = dec; }
        default: { r = max; g = min; b = min; }
    }

    var rgb = vec3<f32>(r, g, b);
    rgb = saturate(rgb);

    return vec3<f32>(r, g, b);
}
