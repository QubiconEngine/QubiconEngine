use super::{AllocatedDeviceMemoryFragment, DeviceMemoryAllocator};

/// This allocator implementation is for cases where are no memory needed
pub struct HollowDeviceMemoryAllocator;

unsafe impl DeviceMemoryAllocator for HollowDeviceMemoryAllocator {
    type AllocError = crate::Error;
    type MemoryFragmentType = HollowMemoryFragment;

    unsafe fn alloc(&self, _memory_type_index: u8, size: u64, align: u64) -> Result<Self::MemoryFragmentType, Self::AllocError> {
        unimplemented!("hollow allocator")
    }

    unsafe fn dealloc(&self, fragment: HollowMemoryFragment) {
        unimplemented!("hollow allocator")
    }
}

pub struct HollowMemoryFragment;

unsafe impl AllocatedDeviceMemoryFragment for HollowMemoryFragment {
    unsafe fn as_memory_object_and_offset(&self) -> (&super::DeviceMemoryObject, u64) {
        unimplemented!("hollow allocator")
    }
}