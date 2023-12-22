use std::sync::Arc;
use ash::vk::MemoryAllocateInfo;
use crate::device::inner::DeviceInner;

pub mod error;
pub mod device_memory;

pub struct Allocator {
    device: Arc<DeviceInner>
}

impl Allocator {
    pub(crate) fn new(device: Arc<DeviceInner>) -> Self {
        Self { device }
    }

    pub(crate) fn get_device(&self) -> &Arc<DeviceInner> {
        &self.device
    }

    pub(crate) unsafe fn allocate(
        self: &Arc<Self>,
        memory_type_index: u32,
        size: u64, // TODO: DeviceAddress
        _align: u64 // TODO: DeviceAddress
    ) -> Result<device_memory::AllocatedMemory, error::AllocationError> {
        let memory = self.device.allocate_memory(
            &MemoryAllocateInfo {
                allocation_size: size,
                memory_type_index,

                ..Default::default()
            },
            None
        ).map_err(error::AllocationError::from)?;

        let memory = Arc::new(
            device_memory::DeviceMemoryObject {
                dev: Arc::clone(&self.device),
                device_memory: memory,
                memory_type_index,
                size
            }
        );

        Ok(
            device_memory::AllocatedMemory {
                allocator: Arc::clone(&self),
                memory,
                size,
                offset: 0,
                allocation_index: 0
            }
        )
    }

    pub(crate) unsafe fn deallocate(
        &self,
        _allocated_memory: &device_memory::AllocatedMemory
    ) {
        //core::mem::drop(allocated_memory);
    }
}