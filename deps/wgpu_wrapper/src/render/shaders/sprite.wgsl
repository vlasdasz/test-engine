

@group(0) @binding(0) var<uniform> size:  vec2<f32>;
@group(0) @binding(1) var<uniform> position: vec2<f32>;
@group(0) @binding(2) var<uniform> rotation: f32;
@group(0) @binding(3) var<uniform> scale: f32;
@group(0) @binding(4) var<uniform> camera_rotation: f32;
@group(0) @binding(5) var<uniform> camera_position: vec2<f32>;


fn rotation_z_matrix(angle: f32) -> mat4x4<f32> {
    let cos_z: f32 = cos(angle);
    let sin_z: f32 = sin(angle);
    return mat4x4<f32>(
        vec4<f32>(cos_z, sin_z, 0.0, 0.0),
        vec4<f32>(-sin_z, cos_z, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );
}

@vertex
fn v_main(
    @location(0) position: vec2<f32>,
) -> @builtin(position) vec4<f32>  {
    var position: vec4<f32> = vec4<f32>(input.vertex_position, 0.0, 1.0);

    position.x *= size.x;
    position.y *= size.y;

    position = position * rotation_z_matrix(-rotation);

    position.xy += position.xy - camera_position;

    position = position * rotation_z_matrix(camera_rotation);

    position.x *= resolution.y / resolution.x;

    position.xy *= scale;
    let local_scale: f32 = resolution.y / 10.0;
    position.xy /= local_scale;

    return position;
}

@group(1) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn f_main() -> @location(0) vec4<f32> {
    return color;
}
