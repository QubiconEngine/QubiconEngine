use std::sync::Arc;
use thiserror::Error;
use crate::device::inner::DeviceInner;
use ash::vk::{
    Result as VkResult,
    ShaderModule as VkShaderModule,
    ShaderModuleCreateInfo as VkShaderModuleCreateInfo
};

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderModuleCreationError {
    #[error("out of host memory")]
    OutOfHostMemory,
    #[error("out of device memory")]
    OutOfDeviceMemory,
    #[error("invalid shader")]
    InvalidShader
}

impl From<VkResult> for ShaderModuleCreationError {
    fn from(value: VkResult) -> Self {
        match value {
            VkResult::ERROR_OUT_OF_HOST_MEMORY => Self::OutOfHostMemory,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY => Self::OutOfDeviceMemory,
            VkResult::ERROR_INVALID_SHADER_NV => Self::InvalidShader,

            _ => unreachable!()
        }
    }
}

pub struct ShaderModule {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) shader_module: VkShaderModule
}

impl ShaderModule {
    pub(crate) fn from_binary(device: Arc<DeviceInner>, binary: &[u32]) -> Result<Self, ShaderModuleCreationError> {
        unsafe {
            let shader_module = device.create_shader_module(
                &VkShaderModuleCreateInfo {
                    code_size: core::mem::size_of_val(binary),
                    p_code: binary.as_ptr(),

                    ..Default::default()
                },
                None
            )?;

            Ok(
                Self {
                    device,
                    shader_module
                }
            )
        }
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