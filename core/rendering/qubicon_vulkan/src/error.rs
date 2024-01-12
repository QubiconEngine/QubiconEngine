use thiserror::Error;
use ash::vk::Result as VkResult;

#[derive(Error, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum VkError {
    #[error("not ready")]
    NotReady,
    #[error("operation timeout")]
    Timeout,
    #[error("an event is signaled")]
    EventSet,
    #[error("an event is unsignaled")]
    EventReset,
    // #[error("return array is too small")]
    // Incomplete,
    #[error("not enought of host memory")]
    OutOfHostMemory,
    #[error("not enought of device memory")]
    OutOfDeviceMemory,
    #[error("initialization of object failed")]
    InitializationFailed,
    #[error("logical device has been lost")]
    DeviceLost,
    #[error("mapping of memory object failed")]
    MemoryMapFailed,
    // #[error("some of specified layers not exist")]
    // LayerNotPresent,
    // #[error("some of specified extensions not presented")]
    // ExtensionNotPresent,
    #[error("some of requested features not available on this device")]
    FeatureNotPresent,
    #[error("unable to find vulkan driver")]
    IncompatibleDriver,
    #[error("too many objects of this type have already been created")]
    TooManyObjects,
    #[error("requested format is not supported on this device")]
    FormatNotSupported,
    #[error("pool allocation failed because of pool memory fragmentation")]
    FragmentedPool,
    #[error("unknown error")]
    Unknown,

    #[error("invalid shader")]
    InvalidShader
}

impl TryFrom<VkResult> for VkError {
    type Error = ();
    
    fn try_from(value: VkResult) -> Result<Self, Self::Error> {
        Ok(
            match value {
                VkResult::NOT_READY => Self::NotReady,
                VkResult::TIMEOUT => Self::Timeout,
                VkResult::EVENT_SET => Self::EventSet,
                VkResult::EVENT_RESET => Self::EventReset,
                // VkResult::INCOMPLETE => Self::Incomplete,
                VkResult::ERROR_OUT_OF_HOST_MEMORY => Self::OutOfHostMemory,
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY => Self::OutOfDeviceMemory,
                VkResult::ERROR_INITIALIZATION_FAILED => Self::InitializationFailed,
                VkResult::ERROR_DEVICE_LOST => Self::DeviceLost,
                VkResult::ERROR_MEMORY_MAP_FAILED => Self::MemoryMapFailed,
                // VkResult::ERROR_LAYER_NOT_PRESENT => Self::LayerNotPresent,
                // VkResult::ERROR_EXTENSION_NOT_PRESENT => Self::ExtensionNotPresent,
                VkResult::ERROR_FEATURE_NOT_PRESENT => Self::FeatureNotPresent,
                VkResult::ERROR_INCOMPATIBLE_DRIVER => Self::IncompatibleDriver,
                VkResult::ERROR_TOO_MANY_OBJECTS => Self::TooManyObjects,
                VkResult::ERROR_FORMAT_NOT_SUPPORTED => Self::FormatNotSupported,
                VkResult::ERROR_FRAGMENTED_POOL => Self::FragmentedPool,
                VkResult::ERROR_UNKNOWN => Self::Unknown,


                _ => return Err(())
            }
        )
    }
}

#[derive(Error, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ValidationError {
    #[error("objects owned by different devices")]
    InvalidDevice,
    #[error("device dont have requested memory type")]
    NoValidMemoryTypeFound,
    #[error("size of allocation is zero or greater than heap size")]
    InvalidAllocationSize,
    #[error("memory object dont support mapping")]
    MemoryMappingNotSupported,
    #[error("provided invalid queue family index")]
    InvalidQueueFamilyIndex
}

#[derive(Error, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Error {
    // TODO: Add description
    #[error("")]
    Vulkan(#[from] VkError),
    #[error("invalid usage")]
    Validation(#[from] ValidationError)
}