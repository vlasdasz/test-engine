precision highp float;

uniform vec4 color;
uniform bool selected;

out vec4 out_color;

void main() {

    out_color = color;

    if (selected) {
        out_color.rgb *= 4.0;
    }
}
