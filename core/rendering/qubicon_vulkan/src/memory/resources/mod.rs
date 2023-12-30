use thiserror::Error;
use std::error::Error;

use crate::Error as VulkanError;

pub enum SharingMode<'a> {
    Exclusive,
    Concurent { queue_families: &'a [u32] }
}

pub mod image;
pub mod buffer;

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