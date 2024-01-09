#![allow(dead_code)]

use sokol::gfx as sg;

use crate::math as m;
pub const ATTR_VS_POS: usize = 0;
pub const ATTR_VS_COLOR0: usize = 1;
pub const ATTR_VS_TEXCOORD0: usize = 2;
pub const SLOT_TEX: usize = 0;
pub const SLOT_SMP: usize = 0;
pub const SLOT_VS_PARAMS: usize = 0;
#[repr(C)]
pub struct VsParams {
    pub mvp: m::Mat4,
}

pub fn texcube_shader_desc(backend: sg::Backend) -> sg::ShaderDesc {
    let mut desc = sg::ShaderDesc::new();
    match backend {
        sg::Backend::Glcore33 => {
            desc.attrs[0].name = b"pos\0".as_ptr() as *const _;
            desc.attrs[1].name = b"color0\0".as_ptr() as *const _;
            desc.attrs[2].name = b"texcoord0\0".as_ptr() as *const _;
            desc.vs.source = &VS_SOURCE_GLSL330 as *const _ as *const _;
            desc.vs.entry = b"main\0".as_ptr() as *const _;
            desc.vs.uniform_blocks[0].size = 64;
            desc.vs.uniform_blocks[0].layout = sg::UniformLayout::Std140;
            desc.vs.uniform_blocks[0].uniforms[0].name = b"vs_params\0".as_ptr() as *const _;
            desc.vs.uniform_blocks[0].uniforms[0]._type = sg::UniformType::Float4;
            desc.vs.uniform_blocks[0].uniforms[0].array_count = 4;
            desc.fs.source = &FS_SOURCE_GLSL330 as *const _ as *const _;
            desc.fs.entry = b"main\0".as_ptr() as *const _;
            desc.fs.images[0].used = true;
            desc.fs.images[0].multisampled = true;
            desc.fs.images[0].image_type = sg::ImageType::Dim2;
            desc.fs.images[0].sample_type = sg::ImageSampleType::Float;
            desc.fs.samplers[0].used = true;
            desc.fs.samplers[0].sampler_type = sg::SamplerType::Filtering;
            desc.fs.image_sampler_pairs[0].used = true;
            desc.fs.image_sampler_pairs[0].image_slot = 0;
            desc.fs.image_sampler_pairs[0].sampler_slot = 0;
            desc.fs.image_sampler_pairs[0].glsl_name = b"tex_smp\0".as_ptr() as *const _;
            desc.label = b"texcube_shader\0".as_ptr() as *const _;
        },
        sg::Backend::D3d11 => {
            desc.attrs[0].sem_name = b"TEXCOORD\0".as_ptr() as *const _;
            desc.attrs[0].sem_index = 0;
            desc.attrs[1].sem_name = b"TEXCOORD\0".as_ptr() as *const _;
            desc.attrs[1].sem_index = 1;
            desc.attrs[2].sem_name = b"TEXCOORD\0".as_ptr() as *const _;
            desc.attrs[2].sem_index = 2;
            desc.vs.source = &VS_SOURCE_HLSL4 as *const _ as *const _;
            desc.vs.d3d11_target = b"vs_4_0\0".as_ptr() as *const _;
            desc.vs.entry = b"main\0".as_ptr() as *const _;
            desc.vs.uniform_blocks[0].size = 64;
            desc.vs.uniform_blocks[0].layout = sg::UniformLayout::Std140;
            desc.fs.source = &FS_SOURCE_HLSL4 as *const _ as *const _;
            desc.fs.d3d11_target = b"ps_4_0\0".as_ptr() as *const _;
            desc.fs.entry = b"main\0".as_ptr() as *const _;
            desc.fs.images[0].used = true;
            desc.fs.images[0].multisampled = true;
            desc.fs.images[0].image_type = sg::ImageType::Dim2;
            desc.fs.images[0].sample_type = sg::ImageSampleType::Float;
            desc.fs.samplers[0].used = true;
            desc.fs.samplers[0].sampler_type = sg::SamplerType::Filtering;
            desc.fs.image_sampler_pairs[0].used = true;
            desc.fs.image_sampler_pairs[0].image_slot = 0;
            desc.fs.image_sampler_pairs[0].sampler_slot = 0;
            desc.label = b"texcube_shader\0".as_ptr() as *const _;
        },
        _ => {},
    }
    desc
}
 
pub const VS_SOURCE_GLSL330: [u8; 369] = [
0x23, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x33, 0x33, 0x30, 0xa, 0xa, 0x75, 0x6e, 0x69, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x34, 0x5d, 0x3b, 0xa, 0x6f, 0x75, 0x74, 0x20, 0x76, 0x65, 0x63, 0x33, 0x20, 0x70, 0x6f, 0x73, 0x5f, 0x66, 0x3b, 0xa, 0x6c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x30, 0x29, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x70, 0x6f, 0x73, 0x3b, 0xa, 0x6f, 0x75, 0x74, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x6c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x31, 0x29, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0xa, 0x6f, 0x75, 0x74, 0x20, 0x76, 0x65, 0x63, 0x32, 0x20, 0x75, 0x76, 0x3b, 0xa, 0x6c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x32, 0x29, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x32, 0x20, 0x74, 0x65, 0x78, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x30, 0x3b, 0xa, 0xa, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x70, 0x6f, 0x73, 0x5f, 0x66, 0x20, 0x3d, 0x20, 0x70, 0x6f, 0x73, 0x2e, 0x78, 0x79, 0x7a, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x6d, 0x61, 0x74, 0x34, 0x28, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x30, 0x5d, 0x2c, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x31, 0x5d, 0x2c, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x32, 0x5d, 0x2c, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x33, 0x5d, 0x29, 0x20, 0x2a, 0x20, 0x70, 0x6f, 0x73, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x75, 0x76, 0x20, 0x3d, 0x20, 0x74, 0x65, 0x78, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x30, 0x20, 0x2a, 0x20, 0x35, 0x2e, 0x30, 0x3b, 0xa, 0x7d, 0x0a, 0x0a, 0x00,
];
 
pub const FS_SOURCE_GLSL330: [u8; 235] = [
0x23, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x33, 0x33, 0x30, 0xa, 0xa, 0x75, 0x6e, 0x69, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x72, 0x32, 0x44, 0x20, 0x74, 0x65, 0x78, 0x5f, 0x73, 0x6d, 0x70, 0x3b, 0xa, 0xa, 0x6c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x30, 0x29, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x32, 0x20, 0x75, 0x76, 0x3b, 0xa, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x33, 0x20, 0x70, 0x6f, 0x73, 0x5f, 0x66, 0x3b, 0xa, 0xa, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x20, 0x6c, 0x3b, 0xa, 0xa, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x6c, 0x20, 0x3d, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x28, 0x70, 0x6f, 0x73, 0x5f, 0x66, 0x29, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x30, 0x2e, 0x35, 0x2a, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x28, 0x74, 0x65, 0x78, 0x5f, 0x73, 0x6d, 0x70, 0x2c, 0x20, 0x75, 0x76, 0x2a, 0x6c, 0x29, 0x20, 0x2b, 0x20, 0x30, 0x2e, 0x32, 0x2a, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x7d, 0x0a, 0x0a, 0x00,
];
 
pub const VS_SOURCE_HLSL4: [u8; 1003] = [
0x63, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x20, 0x3a, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x28, 0x62, 0x30, 0x29, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x72, 0x6f, 0x77, 0x5f, 0x6d, 0x61, 0x6a, 0x6f, 0x72, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x78, 0x34, 0x20, 0x5f, 0x31, 0x39, 0x5f, 0x6d, 0x76, 0x70, 0x20, 0x3a, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x28, 0x63, 0x30, 0x29, 0x3b, 0xa, 0x7d, 0x3b, 0xa, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x70, 0x6f, 0x73, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x32, 0x20, 0x75, 0x76, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x32, 0x20, 0x74, 0x65, 0x78, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x30, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x6c, 0x3b, 0xa, 0xa, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x49, 0x6e, 0x70, 0x75, 0x74, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x70, 0x6f, 0x73, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x30, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x31, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x32, 0x20, 0x74, 0x65, 0x78, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x30, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x32, 0x3b, 0xa, 0x7d, 0x3b, 0xa, 0xa, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x30, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x32, 0x20, 0x75, 0x76, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x31, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x6c, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x32, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3a, 0x20, 0x53, 0x56, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3b, 0xa, 0x7d, 0x3b, 0xa, 0xa, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x76, 0x65, 0x72, 0x74, 0x5f, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x6c, 0x20, 0x3d, 0x20, 0x70, 0x6f, 0x73, 0x2e, 0x78, 0x79, 0x7a, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x6d, 0x75, 0x6c, 0x28, 0x70, 0x6f, 0x73, 0x2c, 0x20, 0x5f, 0x31, 0x39, 0x5f, 0x6d, 0x76, 0x70, 0x29, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x75, 0x76, 0x20, 0x3d, 0x20, 0x74, 0x65, 0x78, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x30, 0x20, 0x2a, 0x20, 0x35, 0x2e, 0x30, 0x66, 0x3b, 0xa, 0x7d, 0xa, 0xa, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x29, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x70, 0x6f, 0x73, 0x20, 0x3d, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x70, 0x6f, 0x73, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x20, 0x3d, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x74, 0x65, 0x78, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x30, 0x20, 0x3d, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x74, 0x65, 0x78, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x30, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x76, 0x65, 0x72, 0x74, 0x5f, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x75, 0x76, 0x20, 0x3d, 0x20, 0x75, 0x76, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x6c, 0x20, 0x3d, 0x20, 0x6c, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x3b, 0xa, 0x7d, 0x0a, 0x0a, 0x00,
];
 
pub const FS_SOURCE_HLSL4: [u8; 684] = [
0x54, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x32, 0x44, 0x3c, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x3e, 0x20, 0x74, 0x65, 0x78, 0x20, 0x3a, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x28, 0x74, 0x30, 0x29, 0x3b, 0xa, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x73, 0x6d, 0x70, 0x20, 0x3a, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x28, 0x73, 0x30, 0x29, 0x3b, 0xa, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x32, 0x20, 0x75, 0x76, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x6c, 0x3b, 0xa, 0xa, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x49, 0x6e, 0x70, 0x75, 0x74, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x30, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x32, 0x20, 0x75, 0x76, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x31, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x6c, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x32, 0x3b, 0xa, 0x7d, 0x3b, 0xa, 0xa, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3a, 0x20, 0x53, 0x56, 0x5f, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x30, 0x3b, 0xa, 0x7d, 0x3b, 0xa, 0xa, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x30, 0x2e, 0x35, 0x2a, 0x74, 0x65, 0x78, 0x2e, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x28, 0x73, 0x6d, 0x70, 0x2c, 0x20, 0x75, 0x76, 0x2a, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x28, 0x6c, 0x29, 0x29, 0x20, 0x2b, 0x20, 0x30, 0x2e, 0x32, 0x2a, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x7d, 0xa, 0xa, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x29, 0xa, 0x7b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x75, 0x76, 0x20, 0x3d, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x75, 0x76, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x6c, 0x20, 0x3d, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x6c, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0xa, 0x20, 0x20, 0x20, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x3b, 0xa, 0x7d, 0x0a, 0x0a, 0x00,
];