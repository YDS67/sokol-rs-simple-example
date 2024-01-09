#version 330

uniform vec4 vs_params[4];
layout(location = 0) in vec4 pos;
out vec4 color;
layout(location = 1) in vec4 color0;
out vec2 uv;
layout(location = 2) in vec2 texcoord0;

void main()
{
    gl_Position = mat4(vs_params[0], vs_params[1], vs_params[2], vs_params[3]) * pos;
    color = color0;
    uv = texcoord0 * 5.0;
}