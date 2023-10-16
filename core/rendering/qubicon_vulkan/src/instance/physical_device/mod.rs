use std::sync::Arc;
use ash::vk::PhysicalDevice as VkPhysicalDevice;

pub mod features;
pub mod properties;
pub mod memory_properties;

#[derive(Clone)]
pub struct PhysicalDevice {
    pub(crate) instance: Arc<super::inner::InstanceInner>,
    pub(crate) dev: VkPhysicalDevice
}

impl PhysicalDevice {
    //fn ttt(&self) {
    //    self.instance.get_physical_device_
    //}

    #[inline]
    pub fn get_features(&self) -> features::DeviceFeatures {
        unsafe {
            self.instance.get_physical_device_features(self.dev).into()
        }
    }

    #[inline]
    pub fn get_properties(&self) -> properties::DeviceProperties {
        unsafe {
            self.instance.get_physical_device_properties(self.dev).into()
        }
    }

    #[inline]
    pub fn get_memory_properties(&self) -> memory_properties::DeviceMemoryProperties {
        unsafe {
            self.instance.get_physical_device_memory_properties(self.dev).into()
        }
    }
}