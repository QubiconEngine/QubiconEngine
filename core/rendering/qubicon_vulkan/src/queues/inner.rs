use std::sync::Arc;
use ash::vk::Queue as VkQueue;
use crate::{
    device::inner::DeviceInner,
    instance::physical_device::queue_info::QueueFamilyCapabilities
};

pub(crate) struct QueueInner {
    pub(crate) queue_index: u32,
    pub(crate) family_index: u32,
    pub(crate) capabilities: QueueFamilyCapabilities,
    pub(crate) device: Arc<DeviceInner>,

    pub(crate) queue: VkQueue
}