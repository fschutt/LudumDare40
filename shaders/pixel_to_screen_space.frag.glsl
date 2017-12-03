#version 130

// GLSL shader to draw a texture

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform float transparency;

void main() {
    vec4 cur_color = texture(tex, v_tex_coords);
    cur_color.a *= transparency;

    color = cur_color;
}
