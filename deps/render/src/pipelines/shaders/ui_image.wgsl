
struct RectView {
    resolution: vec2<f32>,
}

struct Vertex {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct UIRectInstance {
    @location(2) position:      vec2<f32>,
    @location(3) size:          vec2<f32>,
    @location(4) color:         vec4<f32>,
    @location(5) corner_radius: f32,
    @location(6) z_position:    f32,
}

@group(0) @binding(0)
var<uniform> view: RectView;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) corner_uv: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) corner_radius: f32,
}

@vertex
fn v_main(
    model: Vertex,
    instance: UIRectInstance,
) -> VertexOutput {
    var out_pos: vec4<f32> = vec4<f32>(model.pos, instance.z_position, 1.0);

    out_pos.y = -out_pos.y;

    out_pos.x /= 2.0;
    out_pos.y /= 2.0;

    out_pos.x += 0.5;
    out_pos.y += 0.5;

    out_pos.x /= view.resolution.x;
    out_pos.y /= view.resolution.y;

    out_pos.x *= instance.size.x;
    out_pos.y *= instance.size.y;

    out_pos.x += instance.position.x / view.resolution.x;
    out_pos.y += instance.position.y / view.resolution.y;

    out_pos.y *= -1.0;

    out_pos.x -= 0.5;
    out_pos.y += 0.5;

    out_pos.x *= 2.0;
    out_pos.y *= 2.0;

    var out: VertexOutput;
    out.pos = out_pos;
    out.uv  = model.uv;
    out.corner_uv = model.pos * 0.5;
    out.size = instance.size;
    out.corner_radius = instance.corner_radius;
    return out;
}

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let radius: f32 = in.corner_radius;

    if radius == 0.0 {
        return textureSample(t_diffuse, s_diffuse, in.uv);
    }

    let local_pos: vec2<f32> = in.corner_uv * in.size;

    let corner: vec2<f32> = in.size * 0.5 - vec2<f32>(radius, radius);
    let d: vec2<f32> = abs(local_pos) - corner;
    let dist: f32 = length(max(d, vec2<f32>(0.0, 0.0)));

    if (dist > radius) {
        discard;
    }

    return textureSample(t_diffuse, s_diffuse, in.uv);
}

