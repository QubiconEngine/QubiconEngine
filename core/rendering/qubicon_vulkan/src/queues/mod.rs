use std::sync::Arc;

use crate::device::{ Device, QueueFamilyIndex };

pub struct Queue {
    device: Arc<Device>,

    family_index: QueueFamilyIndex,
    queue_index: u32,

    queue: ash::vk::Queue
}

impl Queue {
    pub(crate) unsafe fn as_raw(&self) -> ash::vk::Queue {
        self.queue
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn family_index(&self) -> QueueFamilyIndex {
        self.family_index
    }

    pub fn queue_index(&self) -> u32 {
        self.queue_index
    }
}