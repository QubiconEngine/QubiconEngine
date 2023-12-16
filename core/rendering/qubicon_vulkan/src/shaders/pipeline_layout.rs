use std::sync::Arc;
use thiserror::Error;
use smallvec::SmallVec;
use crate::{
    device::inner::DeviceInner,
    descriptors::DescriptorSetLayout
};
use ash::vk::{
    PipelineLayout as VkPipelineLayout,
    DescriptorSetLayout as VkDescriptorSetLayout,
    PipelineLayoutCreateInfo as VkPipelineLayoutCreateInfo
};

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipelineLayoutCreationError {
    #[error("out of host memory")]
    OutOfHostMemory,
    #[error("out of device memory")]
    OutOfDeviceMemory,
    #[error("device in descriptor sets dont match")]
    InvalidDevice
}

pub struct PipelineLayout {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) pipeline_layout: VkPipelineLayout,

    // TODO: Push constants

    pub(crate) descriptor_sets: Box<[Arc<DescriptorSetLayout>]>
}

impl PipelineLayout {
    pub(crate) unsafe fn create_unchecked(
        device: Arc<DeviceInner>,
        descriptor_sets: impl Into<Box<[Arc<DescriptorSetLayout>]>>
    ) -> Result<Arc<Self>, PipelineLayoutCreationError> {
        Self::create_unchecked_with_vec_descriptor_sets(device, descriptor_sets.into())
    }

    pub(crate) fn create(
        device: Arc<DeviceInner>,
        descriptor_sets: impl Into<Box<[Arc<DescriptorSetLayout>]>>
    ) -> Result<Arc<Self>, PipelineLayoutCreationError> {
        Self::create_with_vec_descriptor_sets(device, descriptor_sets.into())
    }

    pub fn get_descriptor_sets(&self) -> &[Arc<DescriptorSetLayout>] {
        &self.descriptor_sets
    }
}

impl PipelineLayout {
    fn create_with_vec_descriptor_sets(
        device: Arc<DeviceInner>,
        descriptor_sets: Box<[Arc<DescriptorSetLayout>]>
    ) -> Result<Arc<Self>, PipelineLayoutCreationError> {
        let is_valid = descriptor_sets
            .iter()
            .all(| s | s.device == device);

        match is_valid {
            false => Err(PipelineLayoutCreationError::InvalidDevice),
            true => unsafe {
                Self::create_unchecked_with_vec_descriptor_sets(device, descriptor_sets)
            }
        }
    }

    unsafe fn create_unchecked_with_vec_descriptor_sets(
        device: Arc<DeviceInner>,
        descriptor_sets: Box<[Arc<DescriptorSetLayout>]>
    ) -> Result<Arc<Self>, PipelineLayoutCreationError> {
        let raw_descriptor_sets: SmallVec<[VkDescriptorSetLayout; 4]> = descriptor_sets.iter()
            .map(| s | s.descriptor_set_layout)
            .collect();

        let pipeline_layout = device.create_pipeline_layout(
            &VkPipelineLayoutCreateInfo {
                set_layout_count: descriptor_sets.len() as u32,
                p_set_layouts: raw_descriptor_sets.as_ptr(),

                // TODO: Push constants

                ..Default::default()
            },
            None
        ).unwrap();

        Ok(
            Arc::new(
                Self {
                    device,
                    pipeline_layout,
                    descriptor_sets
                }
            )
        )
    }
}

impl PartialEq for PipelineLayout {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device &&
        self.pipeline_layout == other.pipeline_layout
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_pipeline_layout(
                self.pipeline_layout,
                None
            )
        }
    }
}