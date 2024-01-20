use std::sync::Arc;

use super::inner::SwapchainInner;
use crate::memory::alloc::{DeviceMemoryAllocator, AllocatedDeviceMemoryFragment};

pub struct SwapchainImageMemoryAllocator {
    pub(crate) _swapchain: Arc<SwapchainInner>
}

unsafe impl DeviceMemoryAllocator for SwapchainImageMemoryAllocator {
    type AllocError = crate::Error;
    type MemoryFragmentType = SwapchainImageMemoryFragment;

    unsafe fn alloc(&self, _memory_type_index: u8, _size: u64, _align: u64) -> Result<Self::MemoryFragmentType, Self::AllocError> {
        unreachable!("Swapchain images supplied with memory by default")
    }

    unsafe fn dealloc(&self, _fragment: Self::MemoryFragmentType) {
        //unreachable!("Swapchain images supplied with memory by default")
    }
}

pub struct SwapchainImageMemoryFragment;

unsafe impl AllocatedDeviceMemoryFragment for SwapchainImageMemoryFragment {
    unsafe fn as_memory_object_and_offset(&self) -> (&crate::memory::alloc::DeviceMemoryObject, u64) {
        unreachable!("Memory for swapchain images is not allocated by the program")
    }
}