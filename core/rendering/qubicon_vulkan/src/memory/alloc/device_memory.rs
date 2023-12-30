use std::sync::Arc;
use crate::device::inner::DeviceInner;
use ash::vk::DeviceMemory as VkDeviceMemory;

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