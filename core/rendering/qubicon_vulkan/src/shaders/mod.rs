use bitflags::bitflags;
use ash::vk::{
    ShaderStageFlags as VkShaderStageFlags,
    PipelineCreateFlags as VkPipelineCreateFlags
};

pub mod compute;
pub mod graphics;
pub mod shader_module;
pub mod pipeline_layout;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PipelineShaderStageFlags: u32 {
        const VERTEX = 0b1;
        const TESSELLATION_CONTROL = 0b10;
        const TESSELLATION_EVALUATION = 0b100;
        const GEOMETRY = 0b1000;
        const FRAGMENT = 0b1_0000;
        const COMPUTE = 0b10_0000;
    }
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PipelineCreateFlags: u32 {
        const DISABLE_OPTIMIZATION = 0b1;
        const ALLOW_DERIVATIVES = 0b10;
        const DERIVATIVE = 0b100;

        // TODO: All other flags
    }
}

impl From<VkShaderStageFlags> for PipelineShaderStageFlags {
    fn from(value: VkShaderStageFlags) -> Self {
        Self(value.as_raw().into())
    }
}

impl Into<VkShaderStageFlags> for PipelineShaderStageFlags {
    fn into(self) -> VkShaderStageFlags {
        VkShaderStageFlags::from_raw(self.bits().into())
    }
}

impl From<VkPipelineCreateFlags> for PipelineCreateFlags {
    fn from(value: VkPipelineCreateFlags) -> Self {
        Self(value.as_raw().into())
    }
}

impl Into<VkPipelineCreateFlags> for PipelineCreateFlags {
    fn into(self) -> VkPipelineCreateFlags {
        VkPipelineCreateFlags::from_raw(self.bits().into())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct PipelineShaderStageCreateInfo<'a> {
    // TODO: flags
    pub stage: PipelineShaderStageFlags,
    pub module: &'a shader_module::ShaderModule,
    pub entry_name: &'a str,
    // TODO: Specialization
}

impl<'a> std::fmt::Debug for PipelineShaderStageCreateInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineShaderStageCreateInfo")
            .field("stage", &self.stage)
            .field("entry_name", &self.entry_name)
            .finish()
    }
}