use std::{sync::Arc, fmt::Debug};
use physical_device::PhysicalDevice;

pub mod error;
pub mod creation_info;
pub mod physical_device;
pub(crate) mod inner;

pub struct Instance {
    inner: Arc<inner::InstanceInner>
}

impl Instance {
    pub fn create(create_info: &creation_info::InstanceCreateInfo) -> Result<Self, error::InstanceError> {
        Ok(
            Self {
                inner: Arc::new(inner::InstanceInner::load(create_info)?)
            }
        )
    }

    pub fn enumerate_devices(&self) -> ash::prelude::VkResult<impl Iterator<Item = PhysicalDevice>> {
        let inner = Arc::clone(&self.inner);
        let iter = unsafe { self.inner.enumerate_physical_devices()? }
            .into_iter()
            .map(move | dev | PhysicalDevice {
                instance: Arc::clone(&inner),
                dev
            });

        Ok(iter)
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instance")
    }
}