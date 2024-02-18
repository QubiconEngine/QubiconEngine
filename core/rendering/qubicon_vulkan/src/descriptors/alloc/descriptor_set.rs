use std::{sync::Arc, fmt::Debug};
use ash::vk::{
    DescriptorSet as VkDescriptorSet,
    WriteDescriptorSet as VkWriteDescriptorSet,
    DescriptorImageInfo as VkDescriptorImageInfo,
    DescriptorBufferInfo as VkDescriptorBufferInfo
};

use crate::memory::{alloc::DeviceMemoryAllocator, resources::{buffer::Buffer, image::ImageLayout, image_view::ImageView}};

use super::{
    super::layout::DescriptorSetLayout,
    descriptor_pool_inner::DescriptorPoolInner,
};

// #[derive(Clone)]
// pub enum WriteInfo {
//     //TODO: Image {  },
//     Buffer { buffer: Arc<Buffer>, offset: u64, len: u64 },
//     //TODO: TexelBuffer
// }

// impl Debug for WriteInfo {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Buffer { buffer: _, offset, len } => f
//                 .debug_struct("DescriptorBufferWrite")
//                 .field("offset", offset)
//                 .field("len", len)
//                 .finish()
//         }
//     }
// }

pub struct BufferWriteInfo<'a, A: DeviceMemoryAllocator> {
    pub buffer: &'a Buffer<A>,
    pub offset: u64,
    pub len: u64
}

impl<'a, A: DeviceMemoryAllocator> Copy for BufferWriteInfo<'a, A> {}
impl<'a, A: DeviceMemoryAllocator> Clone for BufferWriteInfo<'a, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, A: DeviceMemoryAllocator> Debug for BufferWriteInfo<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferWriteInfo")
            .field("offset", &self.offset)
            .field("len", &self.len)
            .finish()
    }
}

pub struct ImageWriteInfo<'a, A: DeviceMemoryAllocator> {
    pub sampler: Option<()>,
    pub image_view: &'a ImageView<A>,
    pub image_layout: ImageLayout
}

impl<'a, A: DeviceMemoryAllocator> Copy for ImageWriteInfo<'a, A> {}
impl<'a, A: DeviceMemoryAllocator> Clone for ImageWriteInfo<'a, A> {
    fn clone(&self) -> Self {
        *self
    }
} 

pub trait WriteInfo: write_info_sealed::WriteInfoSealed {}
impl<T: write_info_sealed::WriteInfoSealed> WriteInfo for T {} 

mod write_info_sealed {
    use super::*;

    pub trait WriteInfoSealed: Clone {
        type VkInfo;

        unsafe fn construct_info(&self) -> Self::VkInfo;
        unsafe fn update_raw_descriptor_write_unchecked(dst: &mut VkWriteDescriptorSet, src: &Self::VkInfo);
    }

    impl<'a, A: DeviceMemoryAllocator> WriteInfoSealed for BufferWriteInfo<'a, A> {
        type VkInfo = VkDescriptorBufferInfo;

        unsafe fn construct_info(&self) -> Self::VkInfo {
            VkDescriptorBufferInfo {
                buffer: self.buffer.as_inner().buffer,
                offset: self.offset,
                range: self.len
            }
        }
        unsafe fn update_raw_descriptor_write_unchecked(dst: &mut VkWriteDescriptorSet, src: &Self::VkInfo) {
            dst.p_buffer_info = src
        }
    }
    impl<'a, A: DeviceMemoryAllocator> WriteInfoSealed for ImageWriteInfo<'a, A> {
        type VkInfo = VkDescriptorImageInfo;

        unsafe fn construct_info(&self) -> Self::VkInfo {
            VkDescriptorImageInfo {
                image_view: self.image_view.as_raw(),
                image_layout: self.image_layout.into(),

                ..Default::default()
            }
        }

        unsafe fn update_raw_descriptor_write_unchecked(dst: &mut VkWriteDescriptorSet, src: &Self::VkInfo) {
            dst.p_image_info = src
        }
    }
}

#[derive(Debug, Clone)]
pub struct DescriptorWrite<I: WriteInfo> {
    pub binding: u32,
    pub index: u32,
    pub write_info: I
}


pub struct DescriptorSet {
    descriptor_pool: Arc<DescriptorPoolInner>,
    descriptor_set: VkDescriptorSet,

    layout: Arc<DescriptorSetLayout>
}

impl DescriptorSet {
    pub(crate) unsafe fn new(
        descriptor_pool: Arc<DescriptorPoolInner>,
        descriptor_set: VkDescriptorSet,
        layout: Arc<DescriptorSetLayout>
    ) -> Arc<Self> {
        Arc::new(
            Self {
                descriptor_pool,
                descriptor_set,

                layout
            }
        )
    }

    pub(crate) unsafe fn as_raw(&self) -> VkDescriptorSet {
        self.descriptor_set
    }

    /// # Safety
    /// * All usage parameters should match with descriptor types. Binding and index must be a valid indexes
    /// * Resources provided in writes must stay allive until revriten in descriptor set, or descriptor set is dropped.
    // TODO: Rework
    pub unsafe fn update_unchecked<I: WriteInfo>(&self, writes: &[DescriptorWrite<I>]) {
        let raw_type_write_infos: Vec<_> = writes.iter()
            .map(| write | write.write_info.construct_info())
            .collect();
        let mut raw_writes: Vec<_> = writes.iter()
            .map(| write | VkWriteDescriptorSet {
                dst_set: self.descriptor_set,
                dst_binding: write.binding,
                dst_array_element: write.index,
                descriptor_count: 1,
                descriptor_type: self.layout.bindings.get_unchecked(write.binding as usize).r#type.into(),

                ..Default::default()
            }).collect();

        raw_writes
            .iter_mut()
            .zip(raw_type_write_infos.iter())
            .for_each(| (dst, src) | I::update_raw_descriptor_write_unchecked(dst, src));

        
        let _lock = self.descriptor_pool.tracker.lock().unwrap();
            
        self.descriptor_pool.device.update_descriptor_sets(
            &raw_writes,
            &[]
        )
    }
}



impl Drop for DescriptorSet {
    fn drop(&mut self) {
        let _lock = self.descriptor_pool.tracker.lock().unwrap();

        unsafe {
            self.descriptor_pool.device.free_descriptor_sets(
                self.descriptor_pool.descriptor_pool,
                &[self.descriptor_set]
            ).unwrap();
        }
    }
}