#version 140

in vec2 v_tex_coords;
uniform sampler2D tex;
in vec3 v_coordinates;

out vec4 color;

float modI(float a,float b) {
    float m=a-floor((a+0.5)/b)*b;
    return floor(m+0.5);
}

void main() {
    if (v_coordinates[2] == 0.0) {
        color = texture(tex, v_tex_coords) - modI(v_coordinates[0] - v_coordinates[1], 3) * vec4(0.1,0.1,0.1,0.0);
    }
    else {
        color = texture(tex, v_tex_coords) - vec4(0.5,0.5,0.5,0.0);
    }
}
