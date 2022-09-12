layout (location = 0) in vec2 position;
layout (location = 1) in vec2 in_tex_coord;

out vec2 tex_coord;
uniform bool flipped;
uniform bool flipped_y;

void main() {
  gl_Position = vec4(position, 1.0, 1.0);
  tex_coord = in_tex_coord;

  if (flipped) {
    gl_Position.x *= -1.0;
  }

  if (flipped_y) {
    gl_Position.y *= -1.0;
  }
}
