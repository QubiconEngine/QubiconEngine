use std::{
    sync::Arc,
    marker::PhantomData
};
use super::{
    levels,
    CommandBufferUsageFlags,
    super::command_pool_inner::CommandPoolInner
};
use ash::vk::CommandBuffer as VkCommandBuffer;

pub struct CommandBuffer<L: levels::CommandBufferLevel> {
    pub(crate) command_pool: Arc<CommandPoolInner>,
    pub(crate) command_buffer: VkCommandBuffer,

    pub(crate) usage: CommandBufferUsageFlags,
    
    pub(crate) _level: PhantomData<L>
}

impl<L: levels::CommandBufferLevel> Drop for CommandBuffer<L> {
    fn drop(&mut self) {
        let _lock = self.command_pool.lock.lock().unwrap();

        unsafe {
            self.command_pool.device.free_command_buffers(
                self.command_pool.pool,
                &[self.command_buffer]
            )
        }
    }
}