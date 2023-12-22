use std::sync::Arc;
use ash::vk::CommandPoolCreateInfo as VkCommandPoolCreateInfo;
use crate::{
    commands::{
        CommandPool,
        command_pool_inner::CommandPoolInner
    },
    Error,
    error::VkError,
    instance::physical_device::queue_info::QueueFamilyCapabilities
};


pub(crate) mod inner;

pub struct Queue {
    inner: Arc<inner::QueueInner>
}

impl From<Arc<inner::QueueInner>> for Queue {
    fn from(value: Arc<inner::QueueInner>) -> Self {
        Self { inner: value }
    }
}

impl Queue {
    pub fn get_family_index(&self) -> u32 {
        self.inner.family_index
    }
    pub fn get_queue_index(&self) -> u32 {
        self.inner.queue_index
    }
    pub fn get_capabilities(&self) -> QueueFamilyCapabilities {
        self.inner.capabilities
    }

    pub fn create_command_pool(&self) -> Result<CommandPool, Error> {
        unsafe {
            let command_pool = self.inner.device.create_command_pool(
                &VkCommandPoolCreateInfo {
                    queue_family_index: self.inner.family_index,
                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            Ok(
                Arc::new(
                    CommandPoolInner::new(
                        Arc::clone(&self.inner.device),
                        Arc::clone(&self.inner),
                        command_pool
                    )
                ).into()
            )
        }
    }
}