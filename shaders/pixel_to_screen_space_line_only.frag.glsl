#version 130

// GLSL shader to draw a texture

in vec2 v_tex_coords;
out vec4 color;

uniform vec4 in_color;

void main() {
    color = in_color;
}
