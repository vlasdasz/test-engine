precision highp float;

uniform vec4 color;
uniform bool selected;

in vec3 fragment_normal;
in vec3 fragment_position;

out vec4 out_color;

void main() {

    vec3 light_vector = normalize(vec3(1.0, 1.0, 1.0));

    vec3 ambient = color.rgb * 0.5;
    vec3 diffuse = color.rgb * dot(light_vector, fragment_normal);

    out_color.a = color.a;
    out_color.rgb = ambient + diffuse * 0.4;

    float fog_factor = distance(fragment_position / 50.0, vec3(0, 0, 0));

    fog_factor = clamp(fog_factor, 0.0, 1.0);

    out_color = mix(out_color, vec4(0.8, 0.8, 0.8, 1.0), fog_factor);

    if (selected)
    out_color.rgb *= 4.0;
}
