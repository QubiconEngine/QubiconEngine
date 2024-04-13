use crate::instance::physical_device::features::DeviceFeatures;

pub struct DeviceCreateInfo {
    pub queue_families: Vec<QueueFamilyUsage>,
    pub features: DeviceFeatures,

}


pub struct QueueFamilyUsage {
    pub family_idx: u32,
    pub queues: Vec<f32>,

    // TODO: flags
}

impl From<&QueueFamilyUsage> for ash::vk::DeviceQueueCreateInfo {
    fn from(value: &QueueFamilyUsage) -> Self {
        ash::vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(value.family_idx)
            .queue_priorities(&value.queues)
            // TODO: .flags(flags)
            .build()
    }
}