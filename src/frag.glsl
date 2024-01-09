#version 330

uniform sampler2D tex_smp;

layout(location = 0) out vec4 frag_color;
in vec2 uv;
in vec4 color;
in vec3 pos_f;

float l;

void main()
{
    l = length(pos_f);
    frag_color = 0.5*texture(tex_smp, uv*l) + 0.2*color;
}