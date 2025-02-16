
@group(0) @binding(0) var<uniform> z_position: f32;
@group(1) @binding(0) var<uniform> position: vec2<f32>;
@group(1) @binding(1) var<uniform> resolution: vec2<f32>;

@vertex
fn v_main(
    @location(0) vertex: vec2<f32>,
) -> @builtin(position) vec4<f32>  {
    var x: f32 = (vertex.x + position.x) * 2.0;
    var y: f32 = (vertex.y + position.y) * 2.0;

    x /= resolution.x;
    y /= resolution.y;

    return vec4<f32>(-1.0 + x, 1.0 - y, z_position, 1.0);
}

@group(1) @binding(2) var<uniform> color: vec4<f32>;

@fragment
fn f_main() -> @location(0) vec4<f32> {
    return color;
}
