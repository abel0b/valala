#version 140

in vec3 v_normal;

uniform vec3 u_light;

out vec4 color;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.05, 0.11, 0.11);
    vec3 regular_color = vec3(0.07, 0.21, 0.22);
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
