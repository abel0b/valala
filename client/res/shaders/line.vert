#version 140

in vec3 position;
uniform mat4 transform;

void main() {
    gl_Position =  transform * vec4(position + vec3(0.0, 0.01, 0.0), 1.0);
}
