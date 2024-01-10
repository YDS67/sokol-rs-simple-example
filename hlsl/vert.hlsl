cbuffer vs_params : register(b0)
{
    row_major float4x4 _19_mvp : packoffset(c0);
};

static float4 gl_Position;
static float4 pos;
static float4 color;
static float4 color0;
static float2 uv;
static float2 texcoord0;
static float3 l;

struct SPIRV_Cross_Input
{
    float4 pos : TEXCOORD0;
    float4 color0 : TEXCOORD1;
    float2 texcoord0 : TEXCOORD2;
};

struct SPIRV_Cross_Output
{
    float4 color : TEXCOORD0;
    float2 uv : TEXCOORD1;
    float3 l : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    l = pos.xyz;
    gl_Position = mul(pos, _19_mvp);
    color = color0;
    uv = texcoord0 * 5.0f;
}

SPIRV_Cross_Output main(SPIRV_Cross_Input stage_input)
{
    pos = stage_input.pos;
    color0 = stage_input.color0;
    texcoord0 = stage_input.texcoord0;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.color = color;
    stage_output.uv = uv;
    stage_output.l = l;
    return stage_output;
}