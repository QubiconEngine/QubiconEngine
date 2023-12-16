use std::{
    sync::Arc,
    marker::PhantomData, cell::Cell
};
use ash::vk::{
    CommandBufferLevel as VkCommandBufferLevel,
    CommandBufferAllocateInfo as VkCommandBufferAllocateInfo
};

pub mod command_buffers;
pub(crate) mod command_pool_inner;

use command_buffers::{
    levels,
    CommandBufferBuilder,
    CommandBufferUsageFlags
};
use command_pool_inner::CommandPoolInner;

/// Internaly synchronized via Mutex
pub struct CommandPool {
    inner: Arc<CommandPoolInner>,
    _ph: PhantomData<Cell<()>>
}

impl From<Arc<CommandPoolInner>> for CommandPool {
    fn from(value: Arc<CommandPoolInner>) -> Self {
        Self { inner: value, _ph: Default::default() }
    }
}

impl CommandPool {
    pub fn create_primary_command_buffer(&self, usage: CommandBufferUsageFlags) -> CommandBufferBuilder<levels::Primary> {
        let lock = self.inner.lock.lock().unwrap();
        
        unsafe {
            let buffer = self.inner.device.allocate_command_buffers(
                &VkCommandBufferAllocateInfo {
                    command_pool: self.inner.pool,
                    level: VkCommandBufferLevel::PRIMARY,
                    command_buffer_count: 1,
                    
                    ..Default::default()
                }
            ).unwrap()[0];

            CommandBufferBuilder::new_primary(
                Arc::clone(&self.inner),
                buffer,
                usage,
                lock
            )
        }
    }

    pub fn create_secondary_command_buffer(&self, usage: CommandBufferUsageFlags) -> CommandBufferBuilder<levels::Secondary> {
        let lock = self.inner.lock.lock().unwrap();
        
        unsafe {
            let buffer = self.inner.device.allocate_command_buffers(
                &VkCommandBufferAllocateInfo {
                    command_pool: self.inner.pool,
                    level: VkCommandBufferLevel::SECONDARY,
                    command_buffer_count: 1,

                    ..Default::default()
                }
            ).unwrap()[0];
            
            CommandBufferBuilder::new_secondary(
                Arc::clone(&self.inner),
                buffer,
                usage,
                lock
            )
        }
    }
}