
struct RectView {
    resolution: vec2<f32>,
}

struct UIGradientInstance {
    @location(2) position:      vec2<f32>,
    @location(3) size:          vec2<f32>,
    @location(4) start_color:   vec4<f32>,
    @location(5) end_color:     vec4<f32>,
    @location(6) corner_radius: f32,
    @location(7) z_position:    f32,
}

@group(0) @binding(0)
var<uniform> view: RectView;

struct VertexOutput {
    @builtin(position) pos:   vec4<f32>,
          @location(0) uv:   vec2<f32>,
          @location(1) size: vec2<f32>,
          @location(2) corner_radius: f32,
          @location(3) gradient_pos:  f32,
          @location(4) start_color: vec4<f32>,
          @location(5) end_color:   vec4<f32>,
}

@vertex
fn v_main(
    @location(0) model: vec2<f32>,
    instance: UIGradientInstance,
) -> VertexOutput {
    var out_pos: vec4<f32> = vec4<f32>(model, instance.z_position, 1.0);

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
    out.pos   = out_pos;

    out.uv = model * 0.5;
    out.gradient_pos = (model.y + 1.0) / 2.0;
    out.size = instance.size;
    out.corner_radius = instance.corner_radius;
    out.start_color = instance.start_color;
    out.end_color = instance.end_color;

    return out;
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let radius: f32 = in.corner_radius;

    if radius == 0.0 {
        return mix(in.start_color, in.end_color, in.gradient_pos);
    }

    let local_pos: vec2<f32> = in.uv * in.size;

    let corner: vec2<f32> = in.size * 0.5 - vec2<f32>(radius, radius);
    let d: vec2<f32> = abs(local_pos) - corner;
    let dist: f32 = length(max(d, vec2<f32>(0.0, 0.0)));

    if (dist > radius) {
        discard;
    }

    return mix(in.start_color, in.end_color, in.gradient_pos);
}
