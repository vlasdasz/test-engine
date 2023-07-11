precision highp float;

in vec2 tex_coord;

uniform sampler2D in_texture;

out vec4 out_color;

void main() {
    out_color.rgb = vec3(1.0, 1.0, 1.0);
    out_color.a = texture(in_texture, tex_coord).r;
}
