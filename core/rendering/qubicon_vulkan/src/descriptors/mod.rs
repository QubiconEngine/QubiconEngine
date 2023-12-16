pub use alloc::{
    DescriptorPool,
    DescriptorPoolCreateInfo
};
pub use layout::{
    DescriptorBinding,
    DescriptorSetLayout,
    DescriptorSetLayoutCreateInfo
};

pub mod alloc;
pub mod layout;


use ash::vk::DescriptorType as VkDescriptorType;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DescriptorType {
    // 1.0
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
    
    // TODO: Other versions
}

impl From<VkDescriptorType> for DescriptorType {
    fn from(value: VkDescriptorType) -> Self {
        match value {
            VkDescriptorType::SAMPLER => Self::Sampler,
            VkDescriptorType::COMBINED_IMAGE_SAMPLER => Self::CombinedImageSampler,
            VkDescriptorType::SAMPLED_IMAGE => Self::SampledImage,
            VkDescriptorType::STORAGE_IMAGE => Self::StorageImage,
            VkDescriptorType::UNIFORM_TEXEL_BUFFER => Self::UniformTexelBuffer,
            VkDescriptorType::STORAGE_TEXEL_BUFFER => Self::StorageTexelBuffer,
            VkDescriptorType::UNIFORM_BUFFER => Self::UniformBuffer,
            VkDescriptorType::STORAGE_BUFFER => Self::StorageBuffer,
            VkDescriptorType::UNIFORM_BUFFER_DYNAMIC => Self::UniformBufferDynamic,
            VkDescriptorType::STORAGE_BUFFER_DYNAMIC => Self::StorageBufferDynamic,
            VkDescriptorType::INPUT_ATTACHMENT => Self::InputAttachment,

            _ => unimplemented!("descripotr types for higher vulkan versions")
        }
    }
}

impl Into<VkDescriptorType> for DescriptorType {
    fn into(self) -> VkDescriptorType {
        match self {
            Self::Sampler => VkDescriptorType::SAMPLER,
            Self::CombinedImageSampler => VkDescriptorType::COMBINED_IMAGE_SAMPLER,
            Self::SampledImage => VkDescriptorType::SAMPLED_IMAGE,
            Self::StorageImage => VkDescriptorType::STORAGE_IMAGE,
            Self::UniformTexelBuffer => VkDescriptorType::UNIFORM_TEXEL_BUFFER,
            Self::StorageTexelBuffer => VkDescriptorType::STORAGE_TEXEL_BUFFER,
            Self::UniformBuffer => VkDescriptorType::UNIFORM_BUFFER,
            Self::StorageBuffer => VkDescriptorType::STORAGE_BUFFER,
            Self::UniformBufferDynamic => VkDescriptorType::UNIFORM_BUFFER_DYNAMIC,
            Self::StorageBufferDynamic => VkDescriptorType::STORAGE_BUFFER_DYNAMIC,
            Self::InputAttachment => VkDescriptorType::INPUT_ATTACHMENT
        }
    }
}