
@group(0) @binding(0)
var base1_texture: texture_2d<f32>;
@group(0) @binding(1)
var base2_texture: texture_2d<f32>;
@group(0) @binding(2)
var effect_texture: texture_2d<f32>;
@group(0) @binding(3)
var sampler_nearest: sampler;
@group(0) @binding(4)
var sampler_linear: sampler;

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
    let base1_colour = textureSample(base1_texture, sampler_nearest, in.uv);
    let bloom_colour = textureSample(base2_texture, sampler_linear, in.uv);

    let base_colour = (base1_colour + bloom_colour).rgb;

    let is_water_zone = in.uv.y >= 0.67;

    let effect_uv = vec2<f32>(in.uv.x, (in.uv.y - 0.5) * 2.0);
    let effect_colour = textureSample(effect_texture, sampler_linear, effect_uv);

    let blend = (base1_colour.rgb * (1.0 - effect_colour.a) + effect_colour.rgb * effect_colour.a);

    let result = select(base_colour, blend, is_water_zone);
    return vec4<f32>(result, 1.0);
}
