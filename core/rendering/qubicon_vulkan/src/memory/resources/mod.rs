use thiserror::Error;
use std::{error::Error, sync::Arc};

use crate::Error as VulkanError;
use super::alloc::DeviceMemoryAllocator;

pub mod format;
pub mod image;
pub mod buffer;
pub mod image_view;
pub mod buffer_view;
pub mod mapped_resource;
pub mod resource_factory;

pub(crate) struct ResourceMemory<A: DeviceMemoryAllocator> {
    allocator: Arc<A>,
    // in Option because needs to be owned on destruction time
    fragment: Option<A::MemoryFragmentType>
}

impl<A: DeviceMemoryAllocator> ResourceMemory<A> {
    pub(crate) fn new(allocator: Arc<A>, fragment: A::MemoryFragmentType) -> Self {
        Self { allocator, fragment: Some(fragment) }
    }
}

impl<A: DeviceMemoryAllocator> core::ops::Deref for ResourceMemory<A> {
    type Target = A::MemoryFragmentType;

    fn deref(&self) -> &Self::Target {
        unsafe { self.fragment.as_ref().unwrap_unchecked() }
    }
}

impl<A: DeviceMemoryAllocator> Drop for ResourceMemory<A> {
    fn drop(&mut self) {
        unsafe { self.allocator.dealloc(self.fragment.take().unwrap_unchecked()) };
    }
}

pub enum SharingMode<'a> {
    Exclusive,
    Concurent { queue_families: &'a [u32] }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceCreationError<AllocError: Error> {
    #[error("error ocured during resource creation")]
    CreationError(VulkanError),
    #[error("error ocured during memory allocation")]
    AllocationError(AllocError)
}

impl<AllocError: Error> ResourceCreationError<AllocError> {
    pub fn from_creation_error(e: VulkanError) -> Self {
        Self::CreationError(e)
    }
    pub fn from_allocation_error(e: AllocError) -> Self {
        Self::AllocationError(e)
    }
}