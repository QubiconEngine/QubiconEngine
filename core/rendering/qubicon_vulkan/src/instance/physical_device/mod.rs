use ash::prelude::VkResult;
use ash::vk::PhysicalDevice as VkPhysicalDevice;
use std::sync::{
    Arc,
    OnceLock
};

pub mod features;
pub mod properties;
pub mod queue_info;
pub mod memory_properties;

#[derive(Clone)]
pub struct PhysicalDevice {
    pub(crate) instance: Arc<super::inner::InstanceInner>,
    pub(crate) dev: VkPhysicalDevice,
    queues: OnceLock<Vec<queue_info::QueueFamily>>
}

impl PhysicalDevice {
    pub(crate) unsafe fn from_instance_and_raw_physical_device(
        instance: Arc<super::inner::InstanceInner>,
        dev: VkPhysicalDevice
    ) -> Self {
        Self {
            instance,
            dev,
            queues: OnceLock::new()
        }
    }
}

impl PhysicalDevice {
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

    pub fn get_queue_family_infos(&self) -> &[queue_info::QueueFamily] {
        self.queues.get_or_init(||
            unsafe { self.instance.get_physical_device_queue_family_properties(self.dev) }
                .into_iter()
                .map(Into::into)
                .collect()
        )
    }

    /// Shortcut
    #[inline]
    pub fn create_logical_device(
        self,
        create_info: crate::device::create_info::DeviceCreateInfo
    ) -> VkResult<crate::device::Device> {
        crate::device::Device::create_from_physical_device(create_info, self)
    }
}

impl PartialEq for PhysicalDevice {
    fn eq(&self, other: &Self) -> bool {
        self.instance == other.instance &&
        self.dev == other.dev
    }
}