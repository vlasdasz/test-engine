
struct RectView {
    resolution: vec2<f32>,
    _padding: vec2<u32>,
}

struct Vertex {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct UIImageInstance {
    @location(2) position:      vec2<f32>,
    @location(3) size:          vec2<f32>,
    @location(4) border_color:  vec4<f32>,
    @location(5) border_width:  f32,
    @location(6) corner_radius: f32,
    @location(7) z_position:    f32,
    @location(8) flags:         u32,
    @location(9) scale:         f32,
}

@group(0) @binding(0)
var<uniform> view: RectView;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) corner_uv: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) corner_radius: f32,
    @location(5) border_width: f32,
}

@vertex
fn v_main(
    model: Vertex,
    instance: UIImageInstance,
) -> VertexOutput {
    let flip_x: bool = ((instance.flags >> 0u) & 1u) != 0u;
    let flip_y: bool = ((instance.flags >> 1u) & 1u) != 0u;

    var pos = model.pos;

    if flip_x {
        pos.x *= -1.0;
    }

    if flip_y {
        pos.y *= -1.0;
    }

    var out_pos: vec4<f32> = vec4<f32>(pos, instance.z_position, 1.0);

    out_pos.y = -out_pos.y;

    out_pos.x /= 2.0;
    out_pos.y /= 2.0;

    out_pos.x += 0.5;
    out_pos.y += 0.5;

    out_pos.x /= view.resolution.x;
    out_pos.y /= view.resolution.y;

    out_pos.x *= instance.size.x * instance.scale;
    out_pos.y *= instance.size.y * instance.scale;

    out_pos.x += instance.position.x * instance.scale / view.resolution.x;
    out_pos.y += instance.position.y * instance.scale / view.resolution.y;

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
    out.border_color = instance.border_color;
    out.border_width = instance.border_width;
    return out;
}

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureSample(t_diffuse, s_diffuse, in.uv);
    let radius: f32 = in.corner_radius;
    let border: f32 = in.border_width;
    let local_pos: vec2<f32> = in.corner_uv * in.size;

    if radius == 0.0 {
        if border > 0.0 {
            let half_size: vec2<f32> = in.size * 0.5;
            let dist_to_edge: vec2<f32> = half_size - abs(local_pos);
            let min_dist: f32 = min(dist_to_edge.x, dist_to_edge.y);

            if min_dist < border {
                return in.border_color;
            }
        }
        if tex.a == 0.0 {
            discard;
        } else {
            return tex;
        }
    }
    let corner: vec2<f32> = in.size * 0.5 - vec2<f32>(radius, radius);
    let d: vec2<f32> = abs(local_pos) - corner;
    let dist_outer: f32 = length(max(d, vec2<f32>(0.0, 0.0)));

    if (dist_outer > radius) {
        discard;
    }

    if border > 0.0 {
        let inner_radius: f32 = max(radius - border, 0.0);
        let inner_corner: vec2<f32> = in.size * 0.5 - vec2<f32>(radius, radius);
        let inner_d: vec2<f32> = abs(local_pos) - inner_corner;
        let dist_inner: f32 = length(max(inner_d, vec2<f32>(0.0, 0.0)));

        if dist_inner > inner_radius {
            return in.border_color;
        }
    }

    if tex.a == 0.0 {
        discard;
    } else {
        return tex;
    }
}
