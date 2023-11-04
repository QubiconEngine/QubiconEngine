use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationError {
    #[error("No memory with given parameters found")]
    NoMemoryTypeFound,
    #[error("no device memory left")]
    OutOfDeviceMemory,
    #[error("no host memory left")]
    OutOfHostMemory
}

impl From<ash::vk::Result> for AllocationError {
    fn from(value: ash::vk::Result) -> Self {
        match value {
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => Self::OutOfDeviceMemory,
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => Self::OutOfHostMemory,
            _ => unreachable!()
        }
    }
}