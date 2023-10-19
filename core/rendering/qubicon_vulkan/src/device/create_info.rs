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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeviceCreateInfo {
    pub features: DeviceFeatures,
    pub queues: Vec<QueueFamilyUsage>
}