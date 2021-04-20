precision highp float;

uniform vec4 color;
uniform bool selected;

in vec3 fragment_normal;

out vec4 out_color;

void main() {

    vec3 light_vector = normalize(vec3(1.0, 1.0, 1.0));

    vec3 ambient = color.rgb * 0.5;
    vec3 diffuse = color.rgb * dot(light_vector, fragment_normal);

    out_color.a = color.a;
    out_color.rgb = ambient + diffuse * 0.4;

    if (selected) {
        out_color.rgb *= 2.0;
    }
}
