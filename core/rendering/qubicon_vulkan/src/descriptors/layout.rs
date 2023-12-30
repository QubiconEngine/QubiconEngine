use std::sync::Arc;
use smallvec::SmallVec;
use crate::{
    Error,
    error::VkError,
    device::inner::DeviceInner,
    shaders::PipelineShaderStageFlags
};
use ash::vk::{
    DescriptorSetLayout as VkDescriptorSetLayout,
    DescriptorSetLayoutBinding as VkDescriptorSetLayoutBinding,
    DescriptorSetLayoutCreateInfo as VkDescriptorSetLayoutCreateInfo
};
use super::DescriptorType;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DescriptorBinding {
    pub shader_stage_flags: PipelineShaderStageFlags,
    pub r#type: DescriptorType,
    pub count: u32,
    // TODO: ImmutableSamplers
}

impl DescriptorBinding {
    fn into_raw_binding_with_index(self, idx: u32) -> VkDescriptorSetLayoutBinding {
        VkDescriptorSetLayoutBinding {
            binding: idx,
            descriptor_type: self.r#type.into(),
            descriptor_count: self.count,
            stage_flags: self.shader_stage_flags.into(),
            
            p_immutable_samplers: core::ptr::null()
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DescriptorSetLayoutCreateInfo<T: Into<Box<[DescriptorBinding]>> = Vec<DescriptorBinding>> {
    pub bindings: T
}

pub struct DescriptorSetLayout {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) descriptor_set_layout: VkDescriptorSetLayout,

    pub(crate) bindings: Box<[DescriptorBinding]>
}

impl DescriptorSetLayout {
    // Wrapper for better ergonomics and smaller binary
    pub(crate) unsafe fn create_unchecked<T: Into<Box<[DescriptorBinding]>>>(
        device: Arc<DeviceInner>,
        create_info: DescriptorSetLayoutCreateInfo<T>
    ) -> Result<Arc<Self>, Error> {
        Self::create_with_vec_bindings_unchecked(device, create_info.bindings.into())
    }

    pub fn get_bindings(&self) -> &[DescriptorBinding] {
        &self.bindings
    }
}

impl DescriptorSetLayout {
    // TODO: Validation with device limits
    /// # Safety
    /// All descriptor bindings should match device limits
    unsafe fn create_with_vec_bindings_unchecked(
        device: Arc<DeviceInner>,
        bindings: Box<[DescriptorBinding]>
    ) -> Result<Arc<Self>, Error> {
        unsafe {
            let raw_bindings: SmallVec<[VkDescriptorSetLayoutBinding; 16]> = bindings.iter()
                .copied()
                .enumerate()
                .map(| (idx, b) | b.into_raw_binding_with_index(idx as u32))
                .collect();

            let descriptor_set_layout = device.create_descriptor_set_layout(
                &VkDescriptorSetLayoutCreateInfo {
                    binding_count: bindings.len() as u32,
                    p_bindings: raw_bindings.as_ptr(),
                    
                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            Ok(
                Arc::new(
                    Self {
                        device,
                        descriptor_set_layout,

                        bindings
                    }
                )
            )
        }
    }
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_descriptor_set_layout(
                self.descriptor_set_layout,
                None
            )
        }
    }
}