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
        let memory_heaps = &device.memory_properties().memory_heaps;

        if memory_types.len() < self.memory_type {
            panic!("no memory type with index {} exist for this device", self.memory_type);
        }

        let heap_idx = memory_types[self.memory_type].heap_index;
    
        if self.size > memory_heaps[heap_idx].size {
            panic!("allocation size should be less than heap size");
        }

        // TODO: more checks
    }
}

pub struct MemoryObject {
    device: Arc<Device>,
    
    size: DeviceSize,
    memory_type: u32,
    
    memory: ash::vk::DeviceMemory
}

impl MemoryObject {
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    
    pub fn size(&self) -> DeviceSize {
        self.size
    }

    pub fn memory_type(&self) -> u32 {
        self.memory_type
    }
}

impl Drop for MemoryObject {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().free_memory( self.memory, None ) }
    }
}