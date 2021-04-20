precision highp float;

in vec4 v_color;
in vec3 v_light_position;
in vec3 v_normal;
in vec3 v_position;

out vec4 out_color;

void main() {
    vec3 light_vector = normalize(v_light_position - v_position);
    out_color = v_color;
    vec4 ambient = v_color * 0.2;
    vec3 diffuse = out_color.rgb * dot(light_vector, v_normal) * (1.0 / distance(v_position, v_light_position));

    out_color.a = 1.0;
    //color.rgb = color.rgb;// ambient.rgb + diffuse.rgb;
}
