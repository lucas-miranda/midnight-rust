#version 450

//const vec2 positions[3] = vec2[](vec2(0.0, -0.5), vec2(-0.5, 0.5), vec2(0.5, 0.5));

/*
struct VertexInput {
    float4 position;
    //flaot4 color : COLOR;
}
*/

layout(location = 0) in vec2 position;
layout(location = 0) out vec2 out_position;

/*
layout(std140, binding = 0) uniform Globals {
    //mat4 view;
    vec4 color;
} globals;
*/

layout(binding = 0, std140) uniform Globals {
    mat4 view;
    vec4 color;
} globals;

void main() {
    //vec2 pos;

    /*
    if (gl_VertexIndex == 0) {
        pos = vec2(0.0, -0.5);
    } else if (gl_VertexIndex == 1) {
        pos = vec2(-0.5, 0.5);
    } else if (gl_VertexIndex == 2) {
        pos = vec2(0.5, 0.5);
    }
    */

    //gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    //gl_Position = vec4(pos, 0.0, 1.0);
    vec4 pos = vec4(position, 0.0, 1.0) * globals.view;
    out_position = pos.xy;
    //out_position = pos * globals.view;
    gl_Position = pos;
}
