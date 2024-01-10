#version 330

uniform sampler2D tex_smp;

layout(location = 0) out vec4 frag_color;
in vec2 uv;
in vec4 color;
in vec3 pos_f;

float l = length(pos_f*pos_f)/sqrt(2.0);

void main()
{
    if (l < 1.0) {
        frag_color = 0.7 * texture(tex_smp, uv) + 0.3 * color;
    } else {
        discard;
    }
}
