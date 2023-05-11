#version 450 core

layout(location = 0) in vec2 out_position;
layout(location = 0) out vec4 fragment_color;

layout(binding = 0, std140) uniform Globals {
    mat4 view;
    vec4 color;
} globals;

/*
layout(std140, binding = 0) uniform Globals {
    //mat4 view;
    vec4 color;
} globals;
*/

void main() {
    //fragment_color = vec4(0.5, 0.5, 1.0, 1.0);
    //fragment_color = vec4(out_position, 1.0, 1.0);
    //fragment_color = vec4(0.0, 1.0, 1.0, 1.0);
    fragment_color = globals.color;
}
