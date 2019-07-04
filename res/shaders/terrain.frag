#version 140

in vec4 v_data;
in vec2 v_tex_coords;
in vec3 v_normal;

uniform sampler2D tex;

out vec4 color;

float modI(float a,float b) {
    float m=a-floor((a+0.5)/b)*b;
    return floor(m+0.5);
}

void main() {
    color = texture(tex, v_tex_coords) + (v_data[2] - 1.0) * (modI(v_data[0] - v_data[1], 3) * vec4(0.1,0.1,0.1,0.1)) - v_data[2] * vec4(0.5,0.5,0.5,0.0);
}
