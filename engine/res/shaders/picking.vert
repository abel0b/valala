#version 140

in uint id;
in vec3 position;
uniform mat4 transform;

flat out uint v_id;

void main() {
    v_id = id;
    gl_Position =  transform * vec4(position, 1.0);
}
