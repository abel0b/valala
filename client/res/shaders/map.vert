#version 150

in uint id;
in vec4 data;
in vec3 position;
in vec2 tex_coords;
in vec3 normal;

uniform mat4 transform;

out vec2 v_tex_coords;
out vec4 v_data;
out vec3 v_normal;

void main() {
    v_data = data;
    v_tex_coords = tex_coords;
    v_normal = transpose(inverse(mat3(transform))) * normal;
    gl_Position = transform * vec4(position, 1.0);
}
