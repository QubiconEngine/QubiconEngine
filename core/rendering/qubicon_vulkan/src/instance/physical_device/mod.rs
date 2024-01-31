use ash::vk::PhysicalDevice as VkPhysicalDevice;
use std::sync::{
    Arc,
    OnceLock
};
use crate::{
    Error,
    device::create_info::QueueFamilyUsage
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
    pub fn create_logical_device<T: Into<Box<[QueueFamilyUsage]>>>(
        self,
        create_info: crate::device::create_info::DeviceCreateInfo<T>
    ) -> Result<crate::device::Device, Error> {
        crate::device::Device::create_from_physical_device(create_info, self)
    }
}

impl PhysicalDevice {
    #[cfg(feature = "x11")]
    /// # Safety
    /// * *display* must be valid X object, *visual_id* must be valid value
    pub unsafe fn get_x_presentation_support(
        &self,
        queue_family_index: u32,
        display: *mut x11::xlib::Display,
        visual_id: x11::xlib::VisualID
    ) -> Result<bool, crate::error::ValidationError> {
        if queue_family_index as usize > self.get_queue_family_infos().len() {
            return Err(crate::error::ValidationError::InvalidQueueFamilyIndex);
        }

        if let Some(x_surface_ext_calls) = self.instance.x_surface.as_ref() {
            let res = unsafe {
                x_surface_ext_calls.get_physical_device_xlib_presentation_support(
                    self.dev,
                    queue_family_index,
                    // Bruh
                    core::mem::transmute(display),
                    visual_id as u32
                )
            };

            return Ok(res);
        }

        return Err(crate::error::ValidationError::NoWindowingEnabled);
    }
}

impl PartialEq for PhysicalDevice {
    fn eq(&self, other: &Self) -> bool {
        self.instance == other.instance &&
        self.dev == other.dev
    }
}