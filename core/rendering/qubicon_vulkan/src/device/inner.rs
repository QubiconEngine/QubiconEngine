use std::ops::Deref;
use arrayvec::ArrayVec;
use ash::{
    Device,
    vk::DeviceCreateInfo,
    extensions::khr::Swapchain
};
use crate::{
    instance::physical_device::{
        PhysicalDevice,
        features::DeviceFeatures,
        properties::DeviceProperties,
        memory_properties::DeviceMemoryProperties
    },
    Error,
    error::VkError
};

use super::create_info::QueueFamilyUsage;

pub(crate) struct DeviceInner {
    pub(crate) features: DeviceFeatures,
    pub(crate) properties: DeviceProperties,
    pub(crate) memory_properties: DeviceMemoryProperties,

    pub(crate) queue_usage: Box<[QueueFamilyUsage]>,

    #[cfg(feature = "windowing")]
    pub(crate) swapchain: Option<Swapchain>,

    pub(crate) physical_device: PhysicalDevice,
    pub(crate) device: Device
}

impl DeviceInner {
    pub(crate) fn create_from_physical_device<T: Into<Box<[QueueFamilyUsage]>>>(
        create_info: super::create_info::DeviceCreateInfo<T>,
        physical_device: PhysicalDevice
    ) -> Result<Self, Error> {
        let queue_usage: Box<[QueueFamilyUsage]> = create_info.queues.into();

        let mut enabled_extensions: ArrayVec<*const u8, 4> = ArrayVec::new();

        #[cfg(feature = "windowing")]
        if create_info.enable_swapchain {
            enabled_extensions.push("VK_KHR_swapchain\0".as_ptr());
        }
        
        unsafe {
            let vk_features = create_info.features.into();
            let vk_queues_info: Vec<_> = queue_usage
                .iter()
                .copied()
                .map(Into::into)
                .collect(); 

            let device = physical_device.instance.create_device(
                physical_device.dev,
                &DeviceCreateInfo {
                    queue_create_info_count: vk_queues_info.len() as u32,
                    p_queue_create_infos: vk_queues_info.as_ptr(),
                    //enabled_layer_count: (),
                    //pp_enabled_layer_names: (),
                    enabled_extension_count: enabled_extensions.len() as u32,
                    pp_enabled_extension_names: enabled_extensions.as_ptr().cast(),
                    p_enabled_features: &vk_features,

                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            let properties = physical_device.get_properties();
            let memory_properties = physical_device.get_memory_properties();

            #[cfg(feature = "windowing")]
            let swapchain = match create_info.enable_swapchain {
                true => Some(Swapchain::new(&physical_device.instance, &device)),
                false => None
            };

            Ok(
                Self {
                    features: create_info.features,
                    properties,
                    memory_properties,

                    queue_usage,
                    #[cfg(feature = "windowing")]
                    swapchain,

                    physical_device,
                    device
                }
            )
        }
    }

    #[inline]
    pub(crate) fn get_queue_usage(&self) -> &[super::create_info::QueueFamilyUsage] {
        &self.queue_usage
    }
}

impl PartialEq for DeviceInner {
    fn eq(&self, other: &Self) -> bool {
        self.physical_device == other.physical_device &&
        self.device.handle() == other.device.handle()
    }
}

impl Deref for DeviceInner {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl Drop for DeviceInner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}