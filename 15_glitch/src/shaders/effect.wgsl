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

    let scanline_id = floor(uv.y * 100.0);

    let freq_rand = rand(time, scanline_id);
    let threshold = 1.0 - intensity * 0.8;

    let is_glitch = freq_rand >= threshold;

    var base_uv = uv;

    let spatial_rand = rand(time, scanline_id + 1.0);
    let spatial_shift = (spatial_rand - 0.5) * intensity * 0.05;  // <= ±2.5%
    let offset = select(0.0, spatial_shift, is_glitch);
    base_uv.x += offset;

    if is_glitch {
        let chroma_r = rand(time, scanline_id + 2.0);
        let chroma_g = rand(time, scanline_id + 3.0);
        let chroma_b = rand(time, scanline_id + 4.0);

        let shift_r = (chroma_r - 0.5) * intensity * 0.2;  // <= ±10%
        let shift_g = (chroma_g - 0.5) * intensity * 0.2;
        let shift_b = (chroma_b - 0.5) * intensity * 0.2;

        let uv_r = vec2(base_uv.x + shift_r, uv.y);
        let uv_g = vec2(base_uv.x + shift_g, uv.y);
        let uv_b = vec2(base_uv.x + shift_b, uv.y);

        let col_r = textureSample(mid_texture, effect_sampler, uv_r);
        let col_g = textureSample(mid_texture, effect_sampler, uv_g);
        let col_b = textureSample(mid_texture, effect_sampler, uv_b);
        let alpha = min(col_r.a, min(col_g.a, col_b.a));

        return vec4(col_r.r, col_g.g, col_b.b, alpha);
    } else {
        let colour = textureSample(mid_texture, effect_sampler, base_uv);
        return vec4(colour);
    }
}

fn rand(time: f32, uv_y: f32) -> f32 {
    let seeds = vec2<f32>(time, uv_y);
    let rand_p = fract(sin(dot(seeds, vec2(12.9898, 78.233))) * 43758.5453);
    let rand_np = (rand_p * 2.0) - 1.0;
    let randf = select(rand_p, rand_np, rand_p <= 0.02);

    return randf;
}


