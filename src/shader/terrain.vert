#version 140

in uint id;
in vec3 position;
in vec2 tex_coords;
in vec2 coordinates;
uniform mat4 model;
uniform mat4 perspective;
uniform mat4 view;

out vec2 v_tex_coords;
out vec2 v_coordinates;

void main() {
    v_tex_coords = tex_coords;
    v_coordinates = coordinates;
    gl_Position =  perspective * view * model * vec4(position, 1.0);
}
