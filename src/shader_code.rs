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
        sg::Backend::Glcore => {
            desc.vertex_func.source = &VS_SOURCE_GLSL330 as *const _ as *const _;
            desc.vertex_func.entry = c"main".as_ptr();
            desc.fragment_func.source = &FS_SOURCE_GLSL330 as *const _ as *const _;
            desc.fragment_func.entry = c"main".as_ptr();
            desc.attrs[0].base_type = sg::ShaderAttrBaseType::Float;
            desc.attrs[0].glsl_name = c"pos".as_ptr();
            desc.attrs[1].base_type = sg::ShaderAttrBaseType::Float;
            desc.attrs[1].glsl_name = c"color0".as_ptr();
            desc.attrs[2].base_type = sg::ShaderAttrBaseType::Float;
            desc.attrs[2].glsl_name = c"texcoord0".as_ptr();
            desc.uniform_blocks[0].stage = sg::ShaderStage::Vertex;
            desc.uniform_blocks[0].layout = sg::UniformLayout::Std140;
            desc.uniform_blocks[0].size = 64;
            desc.uniform_blocks[0].glsl_uniforms[0]._type = sg::UniformType::Float4;
            desc.uniform_blocks[0].glsl_uniforms[0].array_count = 4;
            desc.uniform_blocks[0].glsl_uniforms[0].glsl_name = c"vs_params".as_ptr();
            desc.views[0].texture.stage = sg::ShaderStage::Fragment;
            desc.views[0].texture.multisampled = false;
            desc.views[0].texture.image_type = sg::ImageType::Dim2;
            desc.views[0].texture.sample_type = sg::ImageSampleType::Float;
            desc.samplers[0].stage = sg::ShaderStage::Fragment;
            desc.samplers[0].sampler_type = sg::SamplerType::Filtering;
            desc.texture_sampler_pairs[0].stage = sg::ShaderStage::Fragment;
            desc.texture_sampler_pairs[0].view_slot = 0;
            desc.texture_sampler_pairs[0].sampler_slot = 0;
            desc.texture_sampler_pairs[0].glsl_name = c"tex_smp".as_ptr();
        },
        _ => {},
    }
    desc
}