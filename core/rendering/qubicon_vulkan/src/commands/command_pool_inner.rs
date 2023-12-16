use std::sync::{Arc, Mutex};
use crate::{
    queue::inner::QueueInner,
    device::inner::DeviceInner
};
use ash::vk::CommandPool as VkCommandPool;


pub(crate) struct CommandPoolInner {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) queue: Arc<QueueInner>,
    pub(crate) pool: VkCommandPool,

    pub(crate) lock: Mutex<()>
}

impl CommandPoolInner {
    pub(crate) unsafe fn new(device: Arc<DeviceInner>, queue: Arc<QueueInner>, pool: VkCommandPool) -> Self {
        Self {
            device,
            queue,
            pool,
            lock: Mutex::new(())
        }
    }
}

impl Drop for CommandPoolInner {
    fn drop(&mut self) {
        let _lock = self.lock.lock().unwrap();

        unsafe {
            self.device.destroy_command_pool(
                self.pool,
                None
            )
        }
    }
}