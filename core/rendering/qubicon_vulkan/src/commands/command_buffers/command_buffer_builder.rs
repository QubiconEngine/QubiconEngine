use std::{
    sync::{Arc, MutexGuard},
    marker::PhantomData
};
use ash::vk::{
    CommandBuffer as VkCommandBuffer,
    CommandBufferBeginInfo as VkCommandBufferBeginInfo,

    IndexType as VkIndexType,
    PipelineBindPoint as VkPipelineBindPoint
};
use super::{
    levels,
    CommandBufferUsageFlags,
    super::CommandPoolInner,
    command_buffer::CommandBuffer
};
use crate::{
    Error,
    error::VkError,
    shaders::{compute::ComputePipeline, pipeline_layout::PipelineLayout},
    memory::{
        alloc::DeviceMemoryAllocator,
        resources::buffer::Buffer
    }, descriptors::alloc::descriptor_set::DescriptorSet
};



pub struct CommandBufferBuilder<'a, L: levels::CommandBufferLevel> {
    pub(crate) command_pool: Option<Arc<CommandPoolInner>>,
    pub(crate) command_buffer: VkCommandBuffer,

    pub(crate) usage: CommandBufferUsageFlags,
    
    _lock: MutexGuard<'a, ()>,

    _ph: PhantomData<L>,
    _ph2: PhantomData<*const ()>
}

impl<'a> CommandBufferBuilder<'a, levels::Primary> {
    pub(crate) unsafe fn new_primary(
        command_pool: Arc<CommandPoolInner>,
        command_buffer: VkCommandBuffer,
        usage: CommandBufferUsageFlags,
        lock: MutexGuard<'a, ()>
    ) -> Result<Self, Error> {
        command_pool.device.begin_command_buffer(
            command_buffer,
            &VkCommandBufferBeginInfo {
                //flags: 
                p_inheritance_info: core::ptr::null(),

                ..Default::default()
            }
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        Ok(
            Self {
                command_pool: Some(command_pool),
                command_buffer,

                usage,
            
                _lock: lock,

                _ph: Default::default(),
                _ph2: Default::default()
            }
        )
    }
}

impl<'a> CommandBufferBuilder<'a, levels::Secondary> {
    pub(crate) unsafe fn new_secondary(
        command_pool: Arc<CommandPoolInner>,
        command_buffer: VkCommandBuffer,
        usage: CommandBufferUsageFlags,
        lock: MutexGuard<'a, ()>
    ) -> Result<Self, Error> {
        command_pool.device.begin_command_buffer(
            command_buffer,
            &VkCommandBufferBeginInfo {
                //flags:
                p_inheritance_info: core::ptr::null(),

                ..Default::default()
            }
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        Ok(
            Self {
                command_pool: Some(command_pool),
                command_buffer,

                usage,

                _lock: lock,

                _ph: Default::default(),
                _ph2: Default::default()
            }
        )
    }
}

impl<'a, L: levels::CommandBufferLevel> CommandBufferBuilder<'a, L> {
    pub fn build(mut self) -> Result<CommandBuffer<L>, Error> {
        unsafe {
            self.command_pool.as_ref().unwrap_unchecked()
                .device.end_command_buffer(self.command_buffer)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            Ok(
                CommandBuffer {
                    command_pool: self.command_pool.take().unwrap_unchecked(),
                    command_buffer: self.command_buffer,

                    usage: self.usage,

                    _level: Default::default()
                }
            )
        }
    }
}

impl<'a, L: levels::CommandBufferLevel> Drop for CommandBufferBuilder<'a, L> {
    fn drop(&mut self) {
        if let Some(pool) = self.command_pool.as_ref() {
            unsafe {
                pool.device.free_command_buffers(
                    pool.pool,
                    &[self.command_buffer]
                );
            }
        }
    }
}



#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipelineBindPoint {
    Graphics = 0,
    Compute = 1
}

impl From<VkPipelineBindPoint> for PipelineBindPoint {
    fn from(value: VkPipelineBindPoint) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl Into<VkPipelineBindPoint> for PipelineBindPoint {
    fn into(self) -> VkPipelineBindPoint {
        unsafe { core::mem::transmute(self) }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexType {
    U16,
    U32
}

impl From<VkIndexType> for IndexType {
    fn from(value: VkIndexType) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl Into<VkIndexType> for IndexType {
    fn into(self) -> VkIndexType {
        unsafe { core::mem::transmute(self) }
    }
}



// Unvalidated commands
impl<'a, L: levels::CommandBufferLevel> CommandBufferBuilder<'a, L> {
    pub unsafe fn cmd_bind_descriptor_set_unchecked(self, bind_point: PipelineBindPoint, set_id: u32, layout: &PipelineLayout, descriptor_set: &DescriptorSet) -> Self {
        self.command_pool.as_ref().unwrap_unchecked().device
            .cmd_bind_descriptor_sets(
                self.command_buffer,
                bind_point.into(),
                layout.pipeline_layout,
                set_id,
                &[descriptor_set.as_raw()],
                &[]
            );

        self
    }

    pub unsafe fn cmd_bind_compute_pipeline_unchecked(self, pipeline: &ComputePipeline) -> Self {
        self.command_pool.as_ref().unwrap_unchecked().device
            .cmd_bind_pipeline(
                self.command_buffer,
                VkPipelineBindPoint::COMPUTE,
                pipeline.pipeline
            );

        self
    }

    pub unsafe fn cmd_dispatch_unchecked(self, group_count_x: u32, group_count_y: u32, group_count_z: u32) -> Self {
        self.command_pool.as_ref().unwrap_unchecked().device
            .cmd_dispatch(
                self.command_buffer,
                group_count_x,
                group_count_y,
                group_count_z
            );

        self
    }

    //unsafe fn cmd_bind_graphics_pipeline_unchecked(self)

    pub unsafe fn cmd_bind_vertex_buffer_unchecked(self, binding_idx: u32, buffer: &Buffer<impl DeviceMemoryAllocator>, offset: u64) -> Self {
        self.command_pool.as_ref().unwrap_unchecked().device
            .cmd_bind_vertex_buffers(
                self.command_buffer,
                binding_idx,
                &[buffer.as_inner().buffer],
                &[offset]
            );

        self
    }

    pub unsafe fn cmd_bind_index_buffer_unchecked(self, buffer: &Buffer<impl DeviceMemoryAllocator>, offset: u64, index_type: IndexType) -> Self {
        self.command_pool.as_ref().unwrap_unchecked().device
            .cmd_bind_index_buffer(
                self.command_buffer,
                buffer.as_inner().buffer,
                offset,
                index_type.into()
            );
        
        self
    }
}