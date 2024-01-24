layout(location = 0) in vec2 position;

uniform vec2 size;
uniform float z_position;

void main() {
    float x = 2.0 * (position.x / size.x);
    float y = 2.0 * (position.y / size.y);

    gl_Position = vec4(-1.0 + x, 1.0 - y, z_position, 1.0);
}
