use ash::prelude::VkResult;
use std::{
    sync::Arc,
    fmt::Debug
};
use crate::{
    memory::{
        alloc::Allocator,
        resources::{buffer::{
            Buffer,
            RawBuffer,
            BufferCreateInfo,
            BufferCreationError,
            RawBufferCreationError
        }, image::{ImageCreateInfo, RawImage, RawImageCreationError, ImageCreationError, Image}}
    },
    instance::physical_device::memory_properties::MemoryTypeProperties
};

pub mod error;
pub mod create_info;
pub(crate) mod inner;

pub struct Device {
    inner: Arc<inner::DeviceInner>,
    allocator: Arc<Allocator>
}

impl Device {
    pub fn create_from_physical_device(
        create_info: create_info::DeviceCreateInfo,
        physical_device: crate::instance::physical_device::PhysicalDevice
    ) -> VkResult<Self> {
        let inner = Arc::new(
            inner::DeviceInner::create_from_physical_device(
                create_info,
                physical_device
            )?
        );
        let allocator = Arc::new(
            Allocator::new(Arc::clone(&inner))
        );

        Ok(
            Self {
                inner,
                allocator
            }
        )
    }

    #[inline]
    pub fn get_physical_device(&self) -> &crate::instance::physical_device::PhysicalDevice {
        &self.inner.physical_device
    }

    #[inline]
    pub fn get_enabled_features(&self) -> &crate::instance::physical_device::features::DeviceFeatures {
        &self.inner.features
    }

    #[inline]
    pub fn get_device_properties(&self) -> &crate::instance::physical_device::properties::DeviceProperties {
        &self.inner.properties
    }

    #[inline]
    pub fn get_device_memory_properties(&self) -> &crate::instance::physical_device::memory_properties::DeviceMemoryProperties {
        &self.inner.memory_properties
    }

    pub fn get_queue(&self, family_index: u32, queue_index: u32) -> Result<crate::queue::Queue, error::QueueError> {
        unsafe {
            let queue_count = self.inner.get_queue_usage()
                .iter()
                .copied()
                .find(| q | q.family_index == family_index)
                .ok_or(error::QueueError::NoQueueFamily { family_index })?
                .queue_count;

            if queue_index >= queue_count {
                return Err(error::QueueError::NoQueueWithIndex { queue_index });
            }

            let queue = self.inner.get_device_queue(family_index, queue_index);
            let queue_inner = Arc::new(
                crate::queue::inner::QueueInner {
                    queue_index,
                    family_index,
                    capabilities: self.get_physical_device()
                        .get_queue_family_infos()[family_index as usize].capabilities,
                    device: Arc::clone(&self.inner),

                    queue
                }
            );

            Ok(queue_inner.into())
        }
    }

    pub fn create_raw_buffer(&self, create_info: &BufferCreateInfo) -> Result<Arc<RawBuffer>, RawBufferCreationError> {
        RawBuffer::create(
            Arc::clone(&self.inner),
            create_info
        ).map(Arc::new)
    }
    pub fn create_buffer(&self, memory_properties: MemoryTypeProperties, create_info: &BufferCreateInfo) -> Result<Arc<Buffer>, BufferCreationError> {
        Buffer::create_and_allocate(
            Arc::clone(&self.inner),
            Arc::clone(&self.allocator),
            memory_properties,
            create_info
        ).map(Arc::new)
    }

    pub fn create_raw_image(&self, create_info: &ImageCreateInfo) -> Result<Arc<RawImage>, RawImageCreationError> {
        RawImage::create(
            Arc::clone(&self.inner),
            create_info
        ).map(Arc::new)
    }

    pub fn create_image(&self, memory_properties: MemoryTypeProperties, create_info: ImageCreateInfo) -> Result<Arc<Image>, ImageCreationError> {
        Image::create_and_allocate(
            Arc::clone(&self.inner),
            Arc::clone(&self.allocator),
            memory_properties,
            create_info
        ).map(Arc::new)
    }
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device")
    }
}