use ash::vk::DeviceQueueCreateInfo as VkDeviceQueueCreateInfo;
use crate::instance::physical_device::features::DeviceFeatures;

#[derive(Debug, Clone, Copy, Default, PartialEq, Hash)]
pub struct QueueFamilyUsage {
    pub family_index: u32,
    pub queue_count: u32,
    // TODO: Queue priorities
}

impl From<QueueFamilyUsage> for VkDeviceQueueCreateInfo {
    fn from(value: QueueFamilyUsage) -> Self {
        VkDeviceQueueCreateInfo {
            queue_family_index: value.family_index,
            queue_count: value.queue_count,
            p_queue_priorities: core::ptr::null(),

            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceCreateInfo<T: Into<Box<[QueueFamilyUsage]>> = Vec<QueueFamilyUsage>> {
    pub features: DeviceFeatures,
    pub queues: T,

    #[cfg(feature = "windowing")]
    pub enable_swapchain: bool
}