#version 450 core

layout(location = 0) in vec2 out_position;
layout(location = 0) out vec4 fragment_color;

void main() {
    //fragment_color = vec4(0.5, 0.5, 1.0, 1.0);
    fragment_color = vec4(out_position, 1.0, 1.0);
}
