layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

uniform mat4 mvp_matrix;

out vec3 fragment_normal;
out vec3 fragment_position;

void main() {
    gl_Position = mvp_matrix * vec4(position, 1.0);
    fragment_normal = normal;
    fragment_position = gl_Position.xyz;
}
