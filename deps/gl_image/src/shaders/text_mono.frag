precision highp float;

in vec2 tex_coord;

uniform vec4 color;
uniform sampler2D in_texture;

out vec4 out_color;

void main() {
    out_color = color;
    out_color.a = texture(in_texture, tex_coord).r;
//
//    out_color = texture(in_texture, tex_coord);
}
