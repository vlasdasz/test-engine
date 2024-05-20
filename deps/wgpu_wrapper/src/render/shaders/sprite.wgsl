
struct SpriteView {
    camera_pos: vec2<f32>,
    resolution: vec2<f32>,
    camera_rotation: f32,
    scale: f32,
}

struct SpriteInstance {
    @location(1) size:     vec2<f32>,
    @location(2) position: vec2<f32>,
    @location(3) color:    vec4<f32>,
    @location(4) rotation: f32,
    @location(5) paddind:  u32, // TODO: remove?
}

@group(0) @binding(0)
var<uniform> view: SpriteView;

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

struct VertexOutput {
    @builtin(position)   pos: vec4<f32>,
          @location(0) color: vec4<f32>,
}

@vertex
fn v_main(
    @location(0) model: vec2<f32>,
    instance: SpriteInstance,
) -> VertexOutput {
    var out_pos: vec4<f32> = vec4<f32>(model, 0.0, 0.5); //    gl_Position = vec4(vertex_position.xy, 1.0, 1.0);

    out_pos.x *= instance.size.x; //gl_Position.x *= size.x;
    out_pos.y *= instance.size.y; //gl_Position.y *= size.y;

    out_pos *= rotation_z_matrix(-instance.rotation);//gl_Position *= rotation_z_matrix(-rotation);

    out_pos.x += instance.position.x;// - data.camera_position.x; //    gl_Position.xy += position - camera_position;
    out_pos.y += instance.position.y;// - data.camera_position.y; //    gl_Position.xy += position - camera_position;

    out_pos *=  rotation_z_matrix(view.camera_rotation);//    gl_Position *= rotation_z_matrix(camera_rotation);

    out_pos *= view.resolution.y / view.resolution.x;  //    gl_Position.x *= resolution.y / resolution.x;

    out_pos.x *= view.scale;  //    gl_Position.xy *= scale;
    out_pos.y *= view.scale;  //    gl_Position.xy *= scale;

    let scale: f32 = view.resolution.y / 10.0; //    float scale = resolution.y / 10.0;

    out_pos.x /= scale; //    gl_Position.xy /= scale;
    out_pos.y /= scale; //    gl_Position.xy /= scale;

    var out: VertexOutput;
    out.pos   = out_pos;
    out.color = instance.color;
    return out;
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
