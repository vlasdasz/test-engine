struct Vertex {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var<uniform> z_pos: f32;

@vertex
fn v_main(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex.uv;
    out.pos = vec4<f32>(vertex.pos, z_pos, 1.0);
    return out;
}

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv);
}
