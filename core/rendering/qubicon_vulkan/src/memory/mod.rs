use std::sync::Arc;

use crate::{ device::Device, instance::physical_device::DeviceSize };

pub struct MemoryObject {
    device: Arc<Device>,
    
    size: DeviceSize,
    memory_type: u32,
    
    memory: ash::vk::DeviceMemory
}

impl Drop for MemoryObject {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().free_memory( self.memory, None ) }
    }
}