#version 330

in vec3 position;
in vec4 color;
uniform mat4 transform;

out vec4 v_color;

void main() {
    v_color = color;
    gl_Position =  transform * vec4(position, 1.0);
}
