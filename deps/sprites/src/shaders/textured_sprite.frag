precision highp float;

in vec2 tex_coord;

uniform sampler2D in_texture;
uniform bool selected;

out vec4 out_color;

void main() {
    out_color = texture(in_texture, tex_coord);
    if (selected) {
        out_color.rgb *= 2.0;
    }
}
