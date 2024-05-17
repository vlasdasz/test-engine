
mat4 rotation_x_matrix(in float angle) {
    float cos_x = cos(angle);
    float sin_x = sin(angle);
    return mat4 (
        1,      0,     0, 0,
        0,  cos_x, sin_x, 0,
        0, -sin_x, cos_x, 0,
        0,      0,     0, 1
    );
}

mat4 rotation_y_matrix(in float angle) {
    float cos_y = cos(angle);
    float sin_y = sin(angle);
    return mat4 (
        cos_y, 0, -sin_y, 0,
            0, 1,      0, 0,
        sin_y, 0,  cos_y, 0,
            0, 0,      0, 1
    );
}

mat4 rotation_z_matrix(in float angle) {
    float cos_z = cos(angle);
    float sin_z = sin(angle);
    return mat4 (
        cos_z, sin_z, 0, 0,
       -sin_z, cos_z, 0, 0,
            0,     0, 1, 0,
            0,     0, 0, 1
    );
}

mat4 scale_matrix(in vec3 scale) {
    return mat4 (
        scale.x,       0,       0, 0,
              0, scale.y,       0, 0,
              0,       0, scale.z, 0,
              0,       0,       0, 1
    );
}

mat4 scale_matrix(in float x, in float y, in float z) {
    return mat4 (
        x, 0, 0, 0,
        0, y, 0, 0,
        0, 0, z, 0,
        0, 0, 0, 1
    );
}

mat4 scale_matrix(in float x, in float y) {
    return mat4 (
        x, 0, 0, 0,
        0, y, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    );
}

mat4 translation_matrix(in vec3 location) {
    return mat4 (
        1, 0, 0, location.x,
        0, 1, 0, location.y,
        0, 0, 1, location.z,
        0, 0, 0, 1
    );
}

mat4 translation_matrix(in float x, in float y, in float z) {
    return mat4 (
        1, 0, 0, x,
        0, 1, 0, y,
        0, 0, 1, z,
        0, 0, 0, 1
    );
}

mat4 translation_matrix(in float x, in float y) {
    return mat4 (
        1, 0, 0, x,
        0, 1, 0, y,
        0, 0, 1, 0,
        0, 0, 0, 1
    );
}
