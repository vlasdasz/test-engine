
@group(0) @binding(0) var<uniform> z_position: f32;
@group(1) @binding(0) var<uniform> size: vec2<f32>;

@vertex
fn v_main(
    @location(0) position: vec2<f32>,
) -> @builtin(position) vec4<f32>  {
    var x: f32 = 2.0 * (position.x / size.x);
    var y: f32 = 2.0 * (position.y / size.y);
    return vec4<f32>(-1.0 + x, 1.0 - y, z_position, 1.0);
}

@group(1) @binding(1) var<uniform> color: vec4<f32>;

@fragment
fn f_main() -> @location(0) vec4<f32> {
    return color;
}
