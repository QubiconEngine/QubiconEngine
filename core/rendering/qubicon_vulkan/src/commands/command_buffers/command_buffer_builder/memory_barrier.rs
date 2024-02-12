use bitflags::bitflags;
use ash::vk::{
    AccessFlags as VkAccessFlags,
    DependencyFlags as VkDependencyFlags,
    
    MemoryBarrier as VkMemoryBarrier,
    ImageMemoryBarrier as VkImageMemoryBarrier,
    BufferMemoryBarrier as VkBufferMemoryBarrier
};
use smallvec::SmallVec;

use crate::{commands::command_buffers::levels::CommandBufferLevel, memory::{alloc::DeviceMemoryAllocator, resources::{buffer::Buffer, image::{Image, ImageLayout}, image_view::ImageSubresourceRange}}, shaders::PipelineStageFlags};

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PipelineBarrierDependencyFlags: u8 {
        const BY_REGION = 0b1;
        //const BY_DEVICE_GROUB = 0b10;
        const VIEW_LOCAL = 0b100;
    }

    // TODO: Move to another place
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AccessFlags: u32 {
        const INDIRECT_COMMAND_READ = 0b1;
        const INDEX_READ = 0b10;
        const VERTEX_ATTRIBUTE_READ = 0b100;
        const UNIFORM_READ = 0b1000;
        const INPUT_ATTACHMENT_READ = 0b1_0000;
        const SHADER_READ = 0b10_0000;
        const SHADER_WRITE = 0b100_0000;
        const COLOR_ATTACHMENT_READ = 0b1000_0000;
        const COLOR_ATTACHMENT_WRITE = 0b1_0000_0000;
        const DEPTH_STENCIL_ATTACHMENT_READ = 0b10_0000_0000;
        const DEPTH_STENCIL_ATTACHMENT_WRITE = 0b100_0000_0000;
        const TRANSFER_READ = 0b1000_0000_0000;
        const TRANSFER_WRITE = 0b1_0000_0000_0000;
        const HOST_READ = 0b10_0000_0000_0000;
        const HOST_WRITE = 0b100_0000_0000_0000;
        const MEMORY_READ = 0b1000_0000_0000_0000;
        const MEMORY_WRITE = 0b1_0000_0000_0000_0000;
    }
}

impl Into<VkDependencyFlags> for PipelineBarrierDependencyFlags {
    fn into(self) -> VkDependencyFlags {
        VkDependencyFlags::from_raw(self.bits() as u32)
    }
}
impl Into<VkAccessFlags> for AccessFlags {
    fn into(self) -> VkAccessFlags {
        VkAccessFlags::from_raw(self.bits() as u32)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags
}
// TODO: Add Default
#[derive(/* Default, */)]
pub struct ImageMemoryBarrier<'a, A: DeviceMemoryAllocator> {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: &'a Image<A>,
    pub subresource_range: ImageSubresourceRange
}
// TODO: Add Default
#[derive(/* Default, */)]
pub struct BufferMemoryBarrier<'a, A: DeviceMemoryAllocator> {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: &'a Buffer<A>,
    
    // maybe use std range ?
    pub offset: u64,
    pub size: u64
}

impl<'a, A: DeviceMemoryAllocator> Copy for BufferMemoryBarrier<'a, A> {}
impl<'a, A: DeviceMemoryAllocator> Clone for ImageMemoryBarrier<'a, A> {
    fn clone(&self) -> Self {
        Self {
            subresource_range: self.subresource_range.clone(),

            ..(*self)
        }
    }
}
impl<'a, A: DeviceMemoryAllocator> Clone for BufferMemoryBarrier<'a, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Into<VkMemoryBarrier> for MemoryBarrier {
    fn into(self) -> VkMemoryBarrier {
        VkMemoryBarrier {
            src_access_mask: self.src_access_mask.into(),
            dst_access_mask: self.dst_access_mask.into(),

            ..Default::default()
        }
    }
}
impl<'a, A: DeviceMemoryAllocator> Into<VkImageMemoryBarrier> for ImageMemoryBarrier<'a, A> {
    fn into(self) -> VkImageMemoryBarrier {
        VkImageMemoryBarrier {
            src_access_mask: self.src_access_mask.into(),
            dst_access_mask: self.dst_access_mask.into(),
            old_layout: self.old_layout.into(),
            new_layout: self.new_layout.into(),
            src_queue_family_index: self.src_queue_family_index,
            dst_queue_family_index: self.dst_queue_family_index,
            image: self.image.as_inner().image,
            subresource_range: self.subresource_range.into(),

            ..Default::default()
        }
    }
}
impl<'a, A: DeviceMemoryAllocator> Into<VkBufferMemoryBarrier> for BufferMemoryBarrier<'a, A> {
    fn into(self) -> VkBufferMemoryBarrier {
        VkBufferMemoryBarrier {
            src_access_mask: self.src_access_mask.into(),
            dst_access_mask: self.dst_access_mask.into(),
            src_queue_family_index: self.src_queue_family_index,
            dst_queue_family_index: self.dst_queue_family_index,
            buffer: self.buffer.as_inner().buffer,
            offset: self.offset,
            size: self.size,

            ..Default::default()
        }
    }
}

impl<'a, A: DeviceMemoryAllocator> core::fmt::Debug for ImageMemoryBarrier<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageMemoryBarrier")
            .field("src_access_mask", &self.src_access_mask)
            .field("dst_access_mask", &self.dst_access_mask)
            .field("old_laybout", &self.old_layout)
            .field("new_layout", &self.new_layout)
            .field("src_queue_family_index", &self.src_queue_family_index)
            .field("dst_queue_family_index", &self.dst_queue_family_index)
            .field("subresource_range", &self.subresource_range)
            .finish()
    }
}
impl<'a, A: DeviceMemoryAllocator> core::fmt::Debug for BufferMemoryBarrier<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferMemoryBarrier")
            .field("src_access_mask", &self.src_access_mask)
            .field("dst_access_mask", &self.dst_access_mask)
            .field("src_queue_family_index", &self.src_queue_family_index)
            .field("dst_queue_family_index", &self.dst_queue_family_index)
            .field("offset", &self.offset)
            .field("size", &self.size)
            .finish()
    }
}
impl<'a, A: DeviceMemoryAllocator> PartialEq for ImageMemoryBarrier<'a, A> {
    fn eq(&self, other: &Self) -> bool {
        self.src_access_mask                   == other.src_access_mask                   &&
        self.dst_access_mask                   == other.dst_access_mask                   &&
        self.old_layout                        == other.old_layout                        &&
        self.new_layout                        == other.new_layout                        &&
        self.src_queue_family_index            == other.src_queue_family_index            &&
        self.dst_queue_family_index            == other.dst_queue_family_index            &&
        self.image as *const Image<A> as usize == other.image as *const Image<A> as usize &&
        self.subresource_range                 == other.subresource_range
    }
}
impl<'a, A: DeviceMemoryAllocator> PartialEq for BufferMemoryBarrier<'a, A> {
    fn eq(&self, other: &Self) -> bool {
        self.src_access_mask                     == other.src_access_mask                     &&
        self.dst_access_mask                     == other.dst_access_mask                     &&
        self.src_queue_family_index              == other.src_queue_family_index              &&
        self.dst_queue_family_index              == other.dst_queue_family_index              &&
        self.buffer as *const Buffer<A> as usize == other.buffer as *const Buffer<A> as usize &&
        self.offset                              == other.offset                              &&
        self.size                                == other.size
    }
}
impl<'a, A: DeviceMemoryAllocator> Eq for ImageMemoryBarrier<'a, A> {}
impl<'a, A: DeviceMemoryAllocator> Eq for BufferMemoryBarrier<'a, A> {}



impl<'a, L: CommandBufferLevel> super::CommandBufferBuilder<'a, L> {
    pub unsafe fn cmd_pipeline_barrier_unchecked<'b, BA: DeviceMemoryAllocator, IA: DeviceMemoryAllocator>(
        self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: PipelineBarrierDependencyFlags,
        memory_barriers: &'b [MemoryBarrier],
        image_memory_barriers: &'b [ImageMemoryBarrier<'b, BA>],
        buffer_memory_barriers: &'b [BufferMemoryBarrier<'b, IA>]
    ) -> Self {
        let memory_barriers: SmallVec<[VkMemoryBarrier; 1]> = memory_barriers.iter()
            .copied()
            .map(Into::into)
            .collect();
        let image_memory_barriers: SmallVec<[VkImageMemoryBarrier; 1]> = image_memory_barriers.iter()
            .cloned()
            .map(Into::into)
            .collect();
        let buffer_memory_barriers: SmallVec<[VkBufferMemoryBarrier; 1]> = buffer_memory_barriers.iter()
            .copied()
            .map(Into::into)
            .collect();

        self.command_pool.as_ref().unwrap_unchecked().device
            .cmd_pipeline_barrier(
                self.command_buffer,
                src_stage_mask.into(),
                dst_stage_mask.into(),
                dependency_flags.into(),
                memory_barriers.as_slice(),
                buffer_memory_barriers.as_slice(),
                image_memory_barriers.as_slice()
            );

        self
    }
}