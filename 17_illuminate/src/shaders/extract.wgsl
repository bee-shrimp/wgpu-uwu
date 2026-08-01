@group(0) @binding(0)
var base_texture: texture_2d<f32>;
@group(0) @binding(1)
var extract_sampler: sampler;

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
    let colour = textureSample(base_texture, extract_sampler, in.uv);
    let value = max(colour.r, max(colour.g, colour.b));

    let is_bright = value >= 0.5;

    let black = vec3<f32>(0.0);
    let result = select(black, colour.rgb, is_bright);

    return vec4<f32>(result, 1.0);
}
