use std::sync::{Arc, Mutex, OnceLock};
use ash::vk::DescriptorPool as VkDescriptorPool;

use super::DescriptorPoolSize;
use crate::device::inner::DeviceInner;

pub(crate) struct Tracker {
    pub(crate) sets_tracker: u32,
    pub(crate) pool_sizes_tracker: Vec<u32>
}

pub(crate) struct DescriptorPoolInner {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) descriptor_pool: VkDescriptorPool,

    pub(crate) max_sets: u32,
    pub(crate) pool_sizes: OnceLock<Vec<DescriptorPoolSize>>,

    pub(crate) tracker: Mutex<Tracker> // Also used to sync all write access to descriptor pool
}

impl Drop for DescriptorPoolInner {
    fn drop(&mut self) {
        let _lock = self.tracker.lock().unwrap();
        
        unsafe {
            self.device.destroy_descriptor_pool(
                self.descriptor_pool,
                None
            )
        }
    }
}