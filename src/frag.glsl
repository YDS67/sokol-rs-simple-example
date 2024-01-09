#version 330

uniform sampler2D tex_smp;

layout(location = 0) out vec4 frag_color;
in vec2 uv;
in vec4 color;

void main()
{
    frag_color = 0.5*texture(tex_smp, uv) + 0.5*color;
}