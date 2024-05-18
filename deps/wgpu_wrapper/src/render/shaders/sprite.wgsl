

@group(0) @binding(0) var<uniform> size:  vec2<f32>;
@group(0) @binding(1) var<uniform> position: vec2<f32>;
@group(0) @binding(2) var<uniform> rotation: f32;
@group(0) @binding(3) var<uniform> scale: f32;
@group(0) @binding(4) var<uniform> camera_rotation: f32;
@group(0) @binding(5) var<uniform> camera_position: vec2<f32>;
@group(0) @binding(6) var<uniform> resolution: vec2<f32>;


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
    @location(0) vertex_pos: vec2<f32>,
) -> @builtin(position) vec4<f32>  {


    var out_pos: vec4<f32> = vec4<f32>(vertex_pos, 1.0, 1.0); //    gl_Position = vec4(vertex_position.xy, 1.0, 1.0);

    out_pos.x *= size.x; //gl_Position.x *= size.x;
    out_pos.y *= size.y; //gl_Position.y *= size.y;

    out_pos *= rotation_z_matrix(-rotation);//gl_Position *= rotation_z_matrix(-rotation);

    out_pos.x += position.x - camera_position.x; //    gl_Position.xy += position - camera_position;
    out_pos.y += position.y - camera_position.y; //    gl_Position.xy += position - camera_position;

    out_pos *=  rotation_z_matrix(camera_rotation);//    gl_Position *= rotation_z_matrix(camera_rotation);

    out_pos *= resolution.y / resolution.x;  //    gl_Position.x *= resolution.y / resolution.x;

    out_pos.x *= scale;  //    gl_Position.xy *= scale;
    out_pos.y *= scale;  //    gl_Position.xy *= scale;

    let scale: f32 = resolution.y / 10.0; //    float scale = resolution.y / 10.0;

    out_pos.x /= scale; //    gl_Position.xy /= scale;
    out_pos.y /= scale; //    gl_Position.xy /= scale;

    return out_pos;
}

@group(1) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn f_main() -> @location(0) vec4<f32> {
    return color;
}
