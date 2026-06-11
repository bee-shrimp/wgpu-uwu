struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) pos: vec2<f32>};

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;

    var pos = model.position.xy;

    out.position = vec4(pos, 0.0, 1.0);
    out.uv = model.uv;
    out.pos = pos;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let base_uv = in.uv;
    let size = textureDimensions(t_diffuse, 0);
    let texel_size = 1.0 / vec2<f32>(f32(size.x), f32(size.y));
    let center_uv = base_uv + texel_size * 0.5;

    let weight0 = 4.0 / 16.0;
    let weight1 = 2.0 / 16.0;
    let weight2 = 1.0 / 16.0;

    var colour = textureSample(t_diffuse, s_diffuse, center_uv) * weight0;

    colour += textureSample(t_diffuse, s_diffuse, center_uv + vec2(1.0, 0.0) * texel_size) * weight1;
    colour += textureSample(t_diffuse, s_diffuse, center_uv + vec2(-1.0, 0.0) * texel_size) * weight1;
    colour += textureSample(t_diffuse, s_diffuse, center_uv + vec2(0.0, 1.0) * texel_size) * weight1;
    colour += textureSample(t_diffuse, s_diffuse, center_uv + vec2(0.0, -1.0) * texel_size) * weight1;
    colour += textureSample(t_diffuse, s_diffuse, center_uv + vec2(1.0, 1.0) * texel_size) * weight2;
    colour += textureSample(t_diffuse, s_diffuse, center_uv + vec2(1.0, -1.0) * texel_size) * weight2;
    colour += textureSample(t_diffuse, s_diffuse, center_uv + vec2(-1.0, 1.0) * texel_size) * weight2;
    colour += textureSample(t_diffuse, s_diffuse, center_uv + vec2(-1.0, -1.0) * texel_size) * weight2;

    return colour;
}

