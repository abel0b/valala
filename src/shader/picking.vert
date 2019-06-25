#version 140

in uint id;
in vec3 position;
uniform mat4 model;
uniform mat4 perspective;
uniform mat4 view;

flat out uint v_id;

void main() {
    v_id = id;
    gl_Position =  perspective * view * model * vec4(position, 1.0);
}
