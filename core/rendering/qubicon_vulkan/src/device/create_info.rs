use std::collections::HashMap;
use crate::instance::physical_device::DeviceFeatures;

pub type QueueFamilyIndex = u32;

pub struct DeviceCreateInfo {
    pub queue_families: HashMap<QueueFamilyIndex, QueueFamilyUsage>,
    pub features: DeviceFeatures,
}


pub struct QueueFamilyUsage {
    pub queues: Vec<f32>,

    // TODO: flags
}