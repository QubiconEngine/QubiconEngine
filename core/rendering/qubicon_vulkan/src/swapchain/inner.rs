use std::sync::Arc;
use ash::vk::SwapchainKHR as VkSwapchain;

use crate::{surface::Surface, device::inner::DeviceInner};

pub(crate) struct SwapchainInner {
    // In option because may be obtained back
    pub(crate) surface: Option<Surface>,
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) swapchain: VkSwapchain
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