use std::ops::Deref;
use ash::{
    Device,
    vk::DeviceCreateInfo
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

    //pub(crate) memory_allocator: (),

    pub(crate) physical_device: PhysicalDevice,
    pub(crate) device: Device
}

impl DeviceInner {
    pub(crate) fn create_from_physical_device<T: Into<Box<[QueueFamilyUsage]>>>(
        create_info: super::create_info::DeviceCreateInfo<T>,
        physical_device: PhysicalDevice
    ) -> Result<Self, Error> {
        let queue_usage: Box<[QueueFamilyUsage]> = create_info.queues.into();
        
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
                    //enabled_extension_count: (),
                    //pp_enabled_extension_names: (),
                    p_enabled_features: &vk_features,

                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            let properties = physical_device.get_properties();
            let memory_properties = physical_device.get_memory_properties();

            Ok(
                Self {
                    features: create_info.features,
                    properties,
                    memory_properties,

                    queue_usage,
                    //memory_allocator: (),

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