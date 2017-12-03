#version 130

#ifdef GL_ES
precision mediump float;
#else
precision highp float;
#endif

in vec3 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform float window_width;
uniform float window_height;

void main() {
    float x_pos = ((position[0] / window_width) * 2.0) - 1.0;
    float y_pos = ((position[1] / window_height) * 2.0) - 1.0;

    v_tex_coords = tex_coords;
    gl_Position = vec4(x_pos, y_pos, position[2], 1.0);
}
