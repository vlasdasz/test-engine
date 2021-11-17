
#include "transforms.glsl"

layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec2 in_tex_coord;

uniform bool flip;
uniform vec2 size;
uniform vec2 position;
uniform float rotation;
uniform float camera_rotation;
uniform vec2 camera_position;

uniform vec2 resolution;

out vec2 tex_coord;

void main() {

    gl_Position = vec4(vertex_position.xy, 0.0, 1.0);

    if (flip) {
        gl_Position.x *= -1.0;
    }

    gl_Position.x *= size.x;
    gl_Position.y *= size.y;

    gl_Position *= rotation_z_matrix(-rotation);

    gl_Position.xy += position - camera_position;

    gl_Position *= rotation_z_matrix(camera_rotation);

    gl_Position.x *= resolution.y / resolution.x;

    float scale = resolution.y / 10.0;
    gl_Position.xy /= scale;

    tex_coord = in_tex_coord;
}
