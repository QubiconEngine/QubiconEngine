use std::{
    ops::Deref,
    sync::OnceLock
};
use ash::{
    Device,
    prelude::VkResult,
    vk::DeviceCreateInfo
};
use crate::instance::physical_device::{
    PhysicalDevice,
    features::DeviceFeatures,
    properties::DeviceProperties,
    memory_properties::DeviceMemoryProperties
};

pub(crate) struct DeviceInner {
    pub(crate) features: DeviceFeatures,
    pub(crate) properties: DeviceProperties,
    pub(crate) memory_properties: DeviceMemoryProperties,

    pub(crate) queue_usage: OnceLock<Vec<super::create_info::QueueFamilyUsage>>,

    //pub(crate) memory_allocator: (),

    pub(crate) physical_device: PhysicalDevice,
    pub(crate) device: Device
}

impl DeviceInner {
    pub(crate) fn create_from_physical_device(
        create_info: super::create_info::DeviceCreateInfo,
        physical_device: PhysicalDevice
    ) -> VkResult<Self> {
        unsafe {
            let vk_features = create_info.features.into();
            let vk_queues_info: Vec<_> = create_info.queues
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
            )?;

            let properties = physical_device.get_properties();
            let memory_properties = physical_device.get_memory_properties();

            Ok(
                Self {
                    features: create_info.features,
                    properties,
                    memory_properties,

                    queue_usage: OnceLock::from(create_info.queues),
                    //memory_allocator: (),

                    physical_device,
                    device
                }
            )
        }
    }

    #[inline]
    pub(crate) fn get_queue_usage(&self) -> &[super::create_info::QueueFamilyUsage] {
        unsafe { self.queue_usage.get().unwrap_unchecked() }
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