#version 150

in uint id;
in vec4 data;
in vec3 position;
in vec2 tex_coords;
in vec3 normal;

uniform mat4 model;
uniform mat4 perspective;
uniform mat4 view;

out vec2 v_tex_coords;
out vec4 v_data;
out vec3 v_normal;

void main() {
    v_data = data;
    v_tex_coords = tex_coords;
    mat4 matrix = perspective * view * model;
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = matrix * vec4(position, 1.0);
}
