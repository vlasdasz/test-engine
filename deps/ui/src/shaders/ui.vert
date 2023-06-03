
layout(location = 0) in vec2 position;

uniform float z_position;

void main() {
    gl_Position = vec4(position, z_position, 1.0);
}
