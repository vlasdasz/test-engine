
struct SpriteVertexData {
    size: vec2<f32>,
    position: vec2<f32>,
    camera_position: vec2<f32>,
    resolution: vec2<f32>,
    rotation: f32,
    scale: f32,
    camera_rotation: f32,
}

@group(0) @binding(0)
var<uniform> data: SpriteVertexData;

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
    var out_pos: vec4<f32> = vec4<f32>(vertex_pos, 0.0, 1.0); //    gl_Position = vec4(vertex_position.xy, 1.0, 1.0);

    out_pos.x *= data.size.x; //gl_Position.x *= size.x;
    out_pos.y *= data.size.y; //gl_Position.y *= size.y;

    out_pos *= rotation_z_matrix(-data.rotation);//gl_Position *= rotation_z_matrix(-rotation);

    out_pos.x += data.position.x;// - data.camera_position.x; //    gl_Position.xy += position - camera_position;
    out_pos.y += data.position.y;// - data.camera_position.y; //    gl_Position.xy += position - camera_position;

    out_pos *=  rotation_z_matrix(data.camera_rotation);//    gl_Position *= rotation_z_matrix(camera_rotation);

    out_pos *= data.resolution.y / data.resolution.x;  //    gl_Position.x *= resolution.y / resolution.x;

    out_pos.x *= data.scale;  //    gl_Position.xy *= scale;
    out_pos.y *= data.scale;  //    gl_Position.xy *= scale;

    let scale: f32 = data.resolution.y / 10.0; //    float scale = resolution.y / 10.0;

    out_pos.x /= scale; //    gl_Position.xy /= scale;
    out_pos.y /= scale; //    gl_Position.xy /= scale;

    return out_pos;
}

@group(1) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn f_main() -> @location(0) vec4<f32> {
    return color;
}
