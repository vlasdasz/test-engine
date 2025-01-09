
struct RectView {
    resolution: vec2<f32>,
}

struct RectInstance {
    @location(2) origin:     vec2<f32>,
    @location(3) size:       vec2<f32>,
    @location(4) color:      vec4<f32>,
    @location(5) z_position: f32,
    @location(6) padding:    f32,
}

@group(0) @binding(0)
var<uniform> view: RectView;

struct VertexOutput {
    @builtin(position)   pos: vec4<f32>,
          @location(0) color: vec4<f32>,
}

@vertex
fn v_main(
    @location(0) model: vec2<f32>,
    instance: RectInstance,
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

    out_pos.x += instance.origin.x / view.resolution.x;
    out_pos.y += instance.origin.y / view.resolution.y;

    out_pos.y *= -1.0;

    out_pos.x -= 0.5;
    out_pos.y += 0.5;

    out_pos.x *= 2.0;
    out_pos.y *= 2.0;

    var out: VertexOutput;
    out.pos   = out_pos;
    out.color = instance.color;
    return out;
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
