use std::sync::Arc;
use crate::device::inner::DeviceInner;
use ash::vk::DeviceMemory as VkDeviceMemory;

use super::Allocator;

pub struct DeviceMemoryObject {
    pub(crate) dev: Arc<DeviceInner>,
    pub(crate) device_memory: VkDeviceMemory,

    pub(crate) memory_type_index: u32,
    pub(crate) size: u64 // TODO: DeviceAddress
}

impl Drop for DeviceMemoryObject {
    fn drop(&mut self) {
        unsafe {
            self.dev.free_memory(
                self.device_memory,
                None
            )
        }
    }
}

// TODO: Automatic memory deallocation
pub struct AllocatedMemory {
    pub(crate) allocator: Arc<Allocator>,
    pub(crate) memory: Arc<DeviceMemoryObject>,
    pub(crate) offset: u64, // TODO: DeviceAddress
    pub(crate) size: u64,
    pub(crate) allocation_index: u64
}

impl Drop for AllocatedMemory {
    fn drop(&mut self) {
        unsafe {
            self.allocator.deallocate(&self);
        }
    }
}