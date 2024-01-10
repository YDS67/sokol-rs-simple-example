Texture2D<float4> tex : register(t0);
SamplerState smp : register(s0);

static float4 frag_color;
static float2 uv;
static float4 color;
static float3 l;

struct SPIRV_Cross_Input
{
    float4 color : TEXCOORD0;
    float2 uv : TEXCOORD1;
    float3 l : TEXCOORD2;
};

struct SPIRV_Cross_Output
{
    float4 frag_color : SV_Target0;
};

void frag_main()
{
    frag_color = 0.5*tex.Sample(smp, uv*length(l)) + 0.2*color;
}

SPIRV_Cross_Output main(SPIRV_Cross_Input stage_input)
{
    uv = stage_input.uv;
    color = stage_input.color;
    l = stage_input.l;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.frag_color = frag_color;
    return stage_output;
}