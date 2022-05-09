#version 300 es
precision highp float;
precision highp int;

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 tex_coord;


out vec2 f_uv;
out vec3 f_vertex;


void main() {
    gl_Position = position;

    // Webgl has flipped y-axis compared to Canvas2d, 
    // So we flip it back
    // We also need to center the uv at the center instead of top right,
    // though we do that in the fragment shader.
    f_uv = vec2(tex_coord.x, 1.0 - tex_coord.y);
    f_vertex = position.xyz;
}
