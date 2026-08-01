
@group(0) @binding(0)
var sampler_nearest: sampler;
@group(0) @binding(1)
var sampler_linear: sampler;
@group(0) @binding(2)
var base_texture: texture_2d<f32>;
@group(0) @binding(3)
var l1_texture: texture_2d<f32>;
@group(0) @binding(4)
var l2_texture: texture_2d<f32>;

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
    let base_colour = textureSample(base_texture, sampler_nearest, in.uv);

    let l1_colour = textureSample(l1_texture, sampler_linear, in.uv);
    let l2_colour = textureSample(l2_texture, sampler_linear, in.uv);

    //let blend = (base_colour.rgb * (1.0 - effect_colour.a) + effect_colour.rgb * effect_colour.a);
    var blend = base_colour + l1_colour;
    blend += l2_colour;

    return vec4<f32>(blend);
}
