use std::sync::Arc;
use ash::vk::MemoryAllocateInfo as VkMemoryAllocateInfo;

use crate::{
    Error,
    error::VkError,
    device::{
        Device,
        inner::DeviceInner
    }
};
use super::{
    DeviceMemoryObject,
    DeviceMemoryAllocator,
    AllocatedDeviceMemoryFragment
};

pub struct StandartMemoryAllocator {
    device: Arc<DeviceInner>
}

impl StandartMemoryAllocator {
    pub fn new(device: &Device) -> Arc<StandartMemoryAllocator> {
        Arc::new(
            Self {
                device: Arc::clone(&device.inner)
            }
        )
    }
}

impl DeviceMemoryAllocator for StandartMemoryAllocator {
    type AllocError = Error;
    type MemoryFragmentType = StandartDeviceMemoryFragment;
    
    unsafe fn alloc(&self, memory_type_index: u32, size: u64, _align: u64) -> Result<Self::MemoryFragmentType, Self::AllocError> {
        let memory = self.device.allocate_memory(
            &VkMemoryAllocateInfo {
                memory_type_index,
                allocation_size: size,

                ..Default::default()
            },
            None
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        let memory = DeviceMemoryObject {
            dev: Arc::clone(&self.device),
            device_memory: memory,
            memory_type_index,
            size
        };

        Ok(
            StandartDeviceMemoryFragment {
                memory
            }
        )
    }

    unsafe fn dealloc(&self, fragment: Self::MemoryFragmentType) {
        core::mem::drop(fragment);
    }
}

pub struct StandartDeviceMemoryFragment {
    memory: DeviceMemoryObject
}

impl AllocatedDeviceMemoryFragment for StandartDeviceMemoryFragment {
    unsafe fn as_memory_object_and_offset(&self) -> (&DeviceMemoryObject, u64) {
        (&self.memory, 0)
    }
}