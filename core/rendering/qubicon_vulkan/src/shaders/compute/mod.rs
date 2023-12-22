use std::sync::Arc;
use smallstr::SmallString;
use super::pipeline_layout::PipelineLayout;
use crate::{
    Error,
    error::VkError,
    device::inner::DeviceInner
};
use ash::vk::{
    Pipeline as VkPipeline,
    ComputePipelineCreateInfo as VkComputePipelineCreateInfo,
    PipelineShaderStageCreateInfo as VkPipelineShaderStageCreateInfo
};

#[derive(Clone, PartialEq)]
pub struct ComputePipelineCreateInfo<'a> {
    pub create_flags: super::PipelineCreateFlags,

    pub stage: super::PipelineShaderStageCreateInfo<'a>,
    
    pub layout: Arc<PipelineLayout>,
    pub base_pipeline: Option<Arc<ComputePipeline>>
}

pub struct ComputePipeline {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) pipeline: VkPipeline,

    pub(crate) layout: Arc<PipelineLayout>,
    pub(crate) create_flags: super::PipelineCreateFlags,
    pub(crate) base_pipeline: Option<Arc<ComputePipeline>>
}

impl ComputePipeline {
    /// # Safety
    /// * layout and shader module should be owned by given device
    /// * entry with *entry_name* must be located in shader module
    pub(crate) unsafe fn create_unchecked(
        device: Arc<DeviceInner>,
        create_info: ComputePipelineCreateInfo
    ) -> Result<Arc<Self>, Error> {
        // Convert Rust str to C str
        let mut entry_name = SmallString::<[u8; 64]>::from_str(
            create_info.stage.entry_name
        );

        entry_name.push('\0');

        let raw_create_info = VkComputePipelineCreateInfo {
            flags: create_info.create_flags.into(),
            layout: create_info.layout.pipeline_layout,
            base_pipeline_handle: create_info.base_pipeline
                .as_ref()
                .map(| p | p.pipeline)
                .unwrap_or_default(),

            stage: VkPipelineShaderStageCreateInfo {
                // flags
                stage: create_info.stage.stage.into(),
                module: create_info.stage.module.shader_module,
                p_name: entry_name.as_ptr().cast(),
                //p_Specialization

                ..Default::default()
            },

            ..Default::default()
        };

        unsafe {
            let pipeline = device.create_compute_pipelines(
                Default::default(),
                &[raw_create_info],
                None
            )
                .map(| v | v[0])
                .map_err(| (_, r) | VkError::try_from(r).unwrap_unchecked())?;

            Ok(
                Arc::new(
                    Self {
                        device,
                        pipeline,

                        layout: create_info.layout,
                        create_flags: create_info.create_flags,
                        base_pipeline: create_info.base_pipeline
                    }
                )
            )
        }
    }

    pub fn get_layout(&self) -> &Arc<PipelineLayout> {
        &self.layout
    }

    pub fn get_base_pipeline(&self) -> Option<&Arc<ComputePipeline>> {
        self.base_pipeline.as_ref()
    }
}

impl PartialEq for ComputePipeline {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device &&
        self.pipeline == other.pipeline
    }
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_pipeline(
                self.pipeline,
                None
            )
        }
    }
}