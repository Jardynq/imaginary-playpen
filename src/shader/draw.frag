#version 300 es
precision highp float;
precision highp int;

uniform bool draw_state;
uniform vec2 mouse_pos;

in vec2 f_uv;
in vec4 f_color;
out vec4 color;

const float epsilon = 0.0001;


void main() {
    color = f_uv;
}