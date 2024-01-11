use std::error::Error;

pub use device_memory::DeviceMemoryObject;

pub mod error;
pub mod device_memory;
pub mod standart_device_memory_allocator;

pub unsafe trait MapGuard<'a> {
    unsafe fn as_ptr(&self) -> *const ();
    unsafe fn as_mut_ptr(&mut self) -> *mut ();
}

pub unsafe trait AllocatedDeviceMemoryFragment: Send {
    unsafe fn as_memory_object_and_offset(&self) -> (&DeviceMemoryObject, u64);
}

pub unsafe trait MappableAllocatedDeviceMemoryFragment<'a>: AllocatedDeviceMemoryFragment {
    type MapError: Error;
    type MapGuard: MapGuard<'a>;

    fn map(&'a self) -> Result<Self::MapGuard, Self::MapError>;
}

pub unsafe trait DeviceMemoryAllocator: Send + Sync {
    type AllocError: Error;
    type MemoryFragmentType: AllocatedDeviceMemoryFragment;

    unsafe fn alloc(&self, memory_type_index: u8, size: u64, align: u64) -> Result<Self::MemoryFragmentType, Self::AllocError>;
    unsafe fn dealloc(&self, fragment: Self::MemoryFragmentType);
}