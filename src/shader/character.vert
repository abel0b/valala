#version 140

in vec3 position;
uniform mat4 model;
uniform mat4 perspective;
uniform mat4 view;

void main() {
    gl_Position =  perspective * view * model * vec4(position, 1.0);
}
