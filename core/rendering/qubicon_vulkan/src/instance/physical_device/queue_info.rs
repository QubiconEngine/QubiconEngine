use bitflags::bitflags;


bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct QueueFamilyCapabilities: u32 {
        const GRAPHICS = 0b1;
        const COMPUTE = 0b10;
        const TRANSFER = 0b100;
        const SPARSE_BINDING = 0b1000;
    }
}

impl From<ash::vk::QueueFlags> for QueueFamilyCapabilities {
    fn from(value: ash::vk::QueueFlags) -> Self {
        Self ( value.as_raw().into() )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueueFamily {
    pub capabilities: QueueFamilyCapabilities,
    pub queue_count: u32,
    // TODO: timestamp_valid_bits
    // TODO: min_image_tranfer_granularity
}

impl From<ash::vk::QueueFamilyProperties> for QueueFamily {
    fn from(value: ash::vk::QueueFamilyProperties) -> Self {
        Self {
            capabilities: value.queue_flags.into(),
            queue_count: value.queue_count
        }
    }
}