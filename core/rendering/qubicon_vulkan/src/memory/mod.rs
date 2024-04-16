use std::sync::Arc;

use crate::{ device::Device, instance::physical_device::{ DeviceSize, PhysicalDevice } };

pub struct AllocationInfo {
    pub size: DeviceSize,
    pub memory_type: u32
}

impl AllocationInfo {
    pub fn validate(&self, device: &PhysicalDevice) {
        if self.size == 0 {
            panic!("allocation size shouldnt be zero");
        }

        let memory_types = &device.memory_properties().memory_types;

        if memory_types.len() < self.memory_type {
            panic!("no memory type with index {} exist for this device", self.memory_type);
        }
    }
}

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