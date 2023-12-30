use std::error::Error;

pub use device_memory::DeviceMemoryObject;

pub mod error;
pub mod device_memory;
pub mod standart_device_memory_allocator;

pub trait AllocatedDeviceMemoryFragment: Send {
    unsafe fn as_memory_object_and_offset(&self) -> (&DeviceMemoryObject, u64);
}

pub trait DeviceMemoryAllocator: Send + Sync {
    type AllocError: Error;
    type MemoryFragmentType: AllocatedDeviceMemoryFragment;

    unsafe fn alloc(&self, memory_type_index: u32, size: u64, align: u64) -> Result<Self::MemoryFragmentType, Self::AllocError>;
    unsafe fn dealloc(&self, fragment: Self::MemoryFragmentType);
}