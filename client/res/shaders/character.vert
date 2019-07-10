#version 150

in vec3 position;
in vec3 normal;

uniform mat4 model;
uniform mat4 perspective;
uniform mat4 view;

out vec3 v_normal;

void main() {
    mat4 matrix = perspective * view * model;
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position =  matrix * vec4(position, 1.0);
}
