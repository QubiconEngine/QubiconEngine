use std::sync::{ Arc, atomic::Ordering };

use super::DeviceSize;
use crate::{ error::VkError, device::Device, instance::physical_device::PhysicalDevice };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

        let heap_idx = memory_types[self.memory_type as usize].heap_index;
    
        if self.size > memory_heaps[heap_idx as usize].size {
            panic!("allocation size should be less than heap size");
        }

        // TODO: more checks
    }
}

impl From<AllocationInfo> for ash::vk::MemoryAllocateInfo {
    fn from(value: AllocationInfo) -> Self {
        Self::builder()
            .allocation_size(value.size)
            .memory_type_index(value.memory_type)
            .build()
    }
}



pub struct MemoryObject {
    device: Arc<Device>,
    
    size: DeviceSize,
    memory_type: u32,
    
    memory: ash::vk::DeviceMemory
}

impl MemoryObject {
    pub(crate) unsafe fn as_raw(&self) -> ash::vk::DeviceMemory {
        self.memory
    }

    pub fn allocate_from(device: Arc<Device>, allocation_info: AllocationInfo) -> Result<Self, VkError> {
        allocation_info.validate(&device.physical_device());

        
        { // check memory objects count, and, if too much, return VkError::TooManyObjects
            let max_memory_objects_count = device.physical_device().properties().limits.max_memory_allocation_count;
            let memory_objects_count = device.memory_objects_count();

            if memory_objects_count >= max_memory_objects_count {
                return Err( VkError::TooManyObjects );
            }
        }

        let memory = unsafe {
            let allocate_info = allocation_info.into();

            device.as_raw().allocate_memory(&allocate_info, None)
        }?;


        unsafe { device.edit_memory_objects_count().fetch_add(1, Ordering::SeqCst) }


        let result = Self {
            device,

            size: allocation_info.size,
            memory_type: allocation_info.memory_type,

            memory
        };

        Ok ( result )
    }

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
        unsafe {
            self.device.edit_memory_objects_count().fetch_sub(1, Ordering::SeqCst);

            self.device.as_raw().free_memory( self.memory, None )
        }
    }
}