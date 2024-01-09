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