#version 140

in uint id;
in vec4 data;
in vec3 position;
in vec2 tex_coords;
uniform mat4 model;
uniform mat4 perspective;
uniform mat4 view;

out vec2 v_tex_coords;
out vec4 v_data;

void main() {
    v_data = data;
    v_tex_coords = tex_coords;
    gl_Position =  perspective * view * model * vec4(position, 1.0);
}
