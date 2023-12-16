pub use command_buffer::CommandBuffer;
pub use command_buffer_builder::CommandBufferBuilder;

pub mod command_buffer;
pub mod command_buffer_builder;


use bitflags::bitflags;
use ash::vk::CommandBufferUsageFlags as VkCommandBufferUsageFlags;

pub mod levels {
    #[derive(Default)]
    pub struct Primary;
    #[derive(Default)]
    pub struct Secondary;


    pub trait CommandBufferLevel: sealed::CommandBufferLevelInternal {}

    impl<T: sealed::CommandBufferLevelInternal> CommandBufferLevel for T {}


    mod sealed {
        use super::*;

        pub trait CommandBufferLevelInternal: Default {}

        impl CommandBufferLevelInternal for Primary {}
        impl CommandBufferLevelInternal for Secondary {}
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CommandBufferUsageFlags: u32 {
        const ONE_TIME_SUBMIT = 0b1;
        const RENDER_PASS_CONTINUE = 0b10;
        const SIMULTANEOUS_USE = 0b100;
    }
}

impl From<VkCommandBufferUsageFlags> for CommandBufferUsageFlags {
    fn from(value: VkCommandBufferUsageFlags) -> Self {
        Self(value.as_raw().into())
    }
}
impl Into<VkCommandBufferUsageFlags> for CommandBufferUsageFlags {
    fn into(self) -> VkCommandBufferUsageFlags {
        VkCommandBufferUsageFlags::from_raw(self.bits())
    }
}