use bitflags::bitflags;
use ash::vk::{
    QueueFlags as VkQueueFlags,
    QueueFamilyProperties as VkQueueFamilyProperties
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct QueueFamilyCapabilities: u32 {
        const GRAPHICS = 0b1;
        const COMPUTE = 0b10;
        const TRANSFER = 0b100;
        const SPARSE_BINDING = 0b1000;
    }
}

impl Into<QueueFamilyCapabilities> for VkQueueFlags {
    fn into(self) -> QueueFamilyCapabilities {
        QueueFamilyCapabilities(self.as_raw().into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueueFamily {
    pub capabilities: QueueFamilyCapabilities,
    pub queue_count: u32,
    // TODO: timestamp_valid_bits
    // TODO: min_image_tranfer_granularity
}

impl Into<QueueFamily> for VkQueueFamilyProperties {
    fn into(self) -> QueueFamily {
        QueueFamily {
            capabilities: self.queue_flags.into(),
            queue_count: self.queue_count
        }
    }
}