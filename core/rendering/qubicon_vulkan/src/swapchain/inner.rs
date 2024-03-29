use std::sync::Arc;
use ash::vk::SwapchainKHR as VkSwapchain;

use crate::{device::inner::DeviceInner, memory::alloc::{AllocatedDeviceMemoryFragment, DeviceMemoryAllocator}, surface::Surface};

pub struct SwapchainInner {
    // In option because may be obtained back
    pub(crate) surface: Option<Surface>,
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) swapchain: VkSwapchain,

    pub(crate) info: super::SwapchainCreateInfo
}

// For the first glance may be strange. This is done to make some kind of dependency.
// Images of swapchain depend on swapchain itself, and it will be destroyed only after
// all images stop being used
unsafe impl DeviceMemoryAllocator for SwapchainInner {
    type AllocError = crate::Error;
    type MemoryFragmentType = SwapchainMemoryFragment;

    unsafe fn alloc(&self, memory_type_index: u8, size: u64, align: u64) -> Result<Self::MemoryFragmentType, Self::AllocError> {
        unimplemented!("no allocation needed for swapchain images")
    }

    unsafe fn dealloc(&self, _fragment: Self::MemoryFragmentType) {}
}

impl Drop for SwapchainInner {
    fn drop(&mut self) {
        unsafe {
            self.device.swapchain.as_ref().unwrap_unchecked().destroy_swapchain(
                self.swapchain,
                None
            )
        }
    }
}

pub struct SwapchainMemoryFragment {
    pub(crate) image_index: u32
}

unsafe impl AllocatedDeviceMemoryFragment for SwapchainMemoryFragment {
    unsafe fn as_memory_object_and_offset(&self) -> (&crate::memory::alloc::DeviceMemoryObject, u64) {
        unimplemented!("no memory allocated for swapchain images")
    }
}