#version 150

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 transform;

out vec3 v_normal;
out vec2 v_tex_coords;

void main() {
    v_normal = transpose(inverse(mat3(transform))) * normal;
    v_tex_coords = tex_coords;
    gl_Position =  transform * vec4(position, 1.0);
}
