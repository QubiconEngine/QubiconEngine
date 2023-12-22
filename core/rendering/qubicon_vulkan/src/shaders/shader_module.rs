use std::sync::Arc;
use crate::{
    Error,
    error::VkError,
    device::inner::DeviceInner
};
use ash::vk::{
    ShaderModule as VkShaderModule,
    ShaderModuleCreateInfo as VkShaderModuleCreateInfo
};

pub struct ShaderModule {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) shader_module: VkShaderModule
}

impl ShaderModule {
    /// # Safety
    /// Shader binary should contain valid **SPIR-V** binary
    pub(crate) unsafe fn create_from_binary(device: Arc<DeviceInner>, shader_binary: &[u32]) -> Result<Self, Error> {
        Self::create_from_binary(
            device,
            core::slice::from_raw_parts(
                shader_binary.as_ptr().cast(),
                shader_binary.len() * 4
            )
        )
    }

    // TODO: Add GLSL support
}

impl ShaderModule {
    unsafe fn create(device: Arc<DeviceInner>, sources: &[u8]) -> Result<Self, Error> {
        let shader_module = device.create_shader_module(
            &VkShaderModuleCreateInfo {
                code_size: core::mem::size_of_val(sources),
                p_code: sources.as_ptr().cast(),

                ..Default::default()
            },
            None
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        Ok(
            Self {
                device,
                shader_module
            }
        )
    }
}

impl PartialEq for ShaderModule {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device &&
        self.shader_module == other.shader_module
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_shader_module(
                self.shader_module,
                None
            )
        }
    }
}