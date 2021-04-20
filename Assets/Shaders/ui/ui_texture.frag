precision highp float;

in vec2 tex_coord;
uniform sampler2D in_texture;
out vec4 out_color;

void main() {
    out_color = texture(in_texture, tex_coord);
}
