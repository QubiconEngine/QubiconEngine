use bitflags::bitflags;
use ash::vk::{
    ShaderStageFlags as VkShaderStageFlags,
    PipelineStageFlags as VkPipelineStageFlags,
    PipelineCreateFlags as VkPipelineCreateFlags
};

pub mod compute;
pub mod graphics;
pub mod shader_module;
pub mod pipeline_layout;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ShaderStageFlags: u32 {
        const VERTEX = 0b1;
        const TESSELLATION_CONTROL = 0b10;
        const TESSELLATION_EVALUATION = 0b100;
        const GEOMETRY = 0b1000;
        const FRAGMENT = 0b1_0000;
        const COMPUTE = 0b10_0000;
    }
}

impl From<VkShaderStageFlags> for ShaderStageFlags {
    fn from(value: VkShaderStageFlags) -> Self {
        Self(value.as_raw().into())
    }
}

impl Into<VkShaderStageFlags> for ShaderStageFlags {
    fn into(self) -> VkShaderStageFlags {
        VkShaderStageFlags::from_raw(self.bits().into())
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

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PipelineStageFlags: u32 {
        const TOP_OF_PIPE = 0b1;
        const DRAW_INDIRECT = 0b10;
        const VERTEX_INPUT = 0b100;
        const VERTEX_SHADER = 0b1000;
        const TESSELLATION_CONTROL_SHADER = 0b1_0000;
        const TESSELLATION_EVALUATION_SHADER = 0b10_0000;
        const GEOMETRY_SHADER = 0b100_0000;
        const FRAGMENT_SHADER = 0b1000_0000;
        const EARLY_FRAGMENT_TESTS = 0b1_0000_0000;
        const LATE_FRAGMENT_TESTS = 0b10_0000_0000;
        const COLOR_ATTACHMENT_OUTPUT = 0b100_0000_0000;
        const COMPUTE_SHADER = 0b1000_0000_0000;
        const TRANSFER = 0b1_0000_0000_0000;
        const BOTTOM_OF_PIPE = 0b10_0000_0000_0000;
        const HOST = 0b100_0000_0000_0000;
        const ALL_GRAPHICS = 0b1000_0000_0000_0000;
        const ALL_COMMANDS = 0b1_0000_0000_0000_0000;
    }
}

impl From<VkPipelineStageFlags> for PipelineStageFlags {
    fn from(value: VkPipelineStageFlags) -> Self {
        Self (value.as_raw().into())
    }
}

impl Into<VkPipelineStageFlags> for PipelineStageFlags {
    fn into(self) -> VkPipelineStageFlags {
        VkPipelineStageFlags::from_raw(self.bits())
    }
}


#[derive(Clone, Copy, PartialEq)]
pub struct PipelineShaderStageCreateInfo<'a> {
    // TODO: flags
    pub stage: ShaderStageFlags,
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