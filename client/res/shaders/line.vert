#version 140

in vec3 position;
uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

void main() {
    gl_Position =  perspective * view * model * vec4(position + vec3(0.0, 0.01, 0.0), 1.0);
}
