#version 150

const vec2 lightBias = vec2(0.7, 0.6);

in vec3 v_normal;
in vec2 v_tex_coords;

uniform vec3 u_light;
uniform sampler2D tex;

out vec4 color;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.0, 0.0, 0.0);
    vec3 regular_color = vec3(0.1, 0.1, 0.1);
    vec4 dark = vec4(mix(dark_color, regular_color, brightness), 1.0);
    color = 2.0*dark + texture(tex, v_tex_coords);
}
