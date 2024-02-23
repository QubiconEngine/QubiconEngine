use ash::vk::{
    Offset3D as VkOffset3D,
    Extent3D as VkExtent3D,
    BufferCopy as VkBufferCopy,
    BufferImageCopy as VkBufferImageCopy
};

use crate::{commands::command_buffers::levels, memory::{alloc::DeviceMemoryAllocator, resources::{buffer::Buffer, image::{Image, ImageLayout}, image_view::ImageSubresourceLayers}}};
use super::CommandBufferBuilder;

// // TODO: DeviceSize
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct BufferCopy {
//     src_offset: u64,
//     dst_offset: u64,

//     size: u64
// }
// No difference in layout. Later maybe changed
pub type BufferCopy = VkBufferCopy;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferImageCopy {
    // TODO: DeviceSize
    pub buffer_offset: u64,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: (i32, i32, i32),
    pub image_extent: (u32, u32, u32)
}
impl Into<VkBufferImageCopy> for BufferImageCopy {
    fn into(self) -> VkBufferImageCopy {
        VkBufferImageCopy {
            buffer_offset: self.buffer_offset,
            buffer_row_length: self.buffer_row_length,
            buffer_image_height: self.buffer_image_height,
            image_subresource: self.image_subresource.into(),
            image_offset: VkOffset3D {
                x: self.image_offset.0,
                y: self.image_offset.1,
                z: self.image_offset.2
            },
            image_extent: VkExtent3D {
                width: self.image_extent.0,
                height: self.image_extent.1,
                depth: self.image_extent.2
            }
        }
    }
}

impl<'a, L: levels::CommandBufferLevel> CommandBufferBuilder<'a, L> {
    pub unsafe fn cmd_copy_buffer_to_image(
        self,
        buffer: &Buffer<impl DeviceMemoryAllocator>,
        dst_image: &Image<impl DeviceMemoryAllocator>,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy]
    ) -> Self {
        let regions: Vec<_> = regions.iter()
            .cloned()
            .map(Into::into)
            .collect();

        self.command_pool.as_ref().unwrap_unchecked().device.cmd_copy_buffer_to_image(
            self.command_buffer,
            buffer.as_inner().buffer,
            dst_image.as_inner().image,
            dst_image_layout.into(),
            &regions
        );

        self
    }

    pub unsafe fn cmd_copy_buffer(
        self,
        src_buffer: &Buffer<impl DeviceMemoryAllocator>,
        dst_buffer: &Buffer<impl DeviceMemoryAllocator>,
        regions: &[BufferCopy]
    ) -> Self {
        self.command_pool.as_ref().unwrap_unchecked().device.cmd_copy_buffer(
            self.command_buffer,
            src_buffer.as_inner().buffer,
            dst_buffer.as_inner().buffer,
            regions
        );

        self
    }
}