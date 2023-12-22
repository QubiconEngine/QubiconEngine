use thiserror::Error;
use bitflags::bitflags;
use std::{
    sync::Arc,
    ops::Deref
};
use crate::{
    Error,
    error::VkError,
    device::inner::DeviceInner,
    memory::alloc::{device_memory::AllocatedMemory, Allocator, error::AllocationError}, instance::physical_device::memory_properties::MemoryTypeProperties
};
use ash::vk::{
    Buffer as VkBuffer,
    BufferCreateInfo as VkBufferCreateInfo,

    BufferUsageFlags as VkBufferUsageFlags,
    BufferCreateFlags as VkBufferCreateFlags
};

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BufferUsageFlags: u32 {
        const TRANSFER_SRC = 0b1;
        const TRANSFER_DST = 0b10;
        const UNIFORM_TEXEL_BUFFER = 0b100;
        const STORAGE_TEXEL_BUFFER = 0b1000;
        const UNIFORM_BUFFER = 0b1_0000;
        const STORAGE_BUFFER = 0b10_0000;
        const INDEX_BUFFER = 0b100_0000;
        const VERTEX_BUFFER = 0b1000_0000;
        const INDIRECT_BUFFER = 0b1_0000_0000;
    }
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BufferCreateFlags: u32 {
        const SPARSE_BINDING = 0b1;
        const SPARSE_RESIDENCY = 0b10;
        const SPARSE_ALIASED = 0b100;
    }
}

impl Into<BufferUsageFlags> for VkBufferUsageFlags {
    fn into(self) -> BufferUsageFlags {
        BufferUsageFlags(self.as_raw().into())
    }
}

impl From<BufferUsageFlags> for VkBufferUsageFlags {
    fn from(value: BufferUsageFlags) -> Self {
        Self::from_raw(value.bits())
    }
}

impl Into<BufferCreateFlags> for VkBufferCreateFlags {
    fn into(self) -> BufferCreateFlags {
        BufferCreateFlags(self.as_raw().into())
    }
}

impl From<BufferCreateFlags> for VkBufferCreateFlags {
    fn from(value: BufferCreateFlags) -> Self {
        Self::from_raw(value.bits())
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferCreateInfo {
    pub usage_flags: BufferUsageFlags,
    pub create_flags: BufferCreateFlags,

    pub size: u64
}

/// Buffer without specified memory
pub struct RawBuffer {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) buffer: VkBuffer,

    pub(crate) size: u64,
    pub(crate) usage_flags: BufferUsageFlags,
    pub(crate) create_flags: BufferCreateFlags
}

impl RawBuffer {
    pub(crate) fn create(
        device: Arc<DeviceInner>,
        create_info: &BufferCreateInfo
    ) -> Result<Self, Error> {
        if !create_info.create_flags.is_empty() {
            unimplemented!()
        }

        unsafe {
            let buffer = device.create_buffer(
                &VkBufferCreateInfo {
                    flags: create_info.create_flags.into(),
                    usage: create_info.usage_flags.into(),
                    size: create_info.size,

                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            Ok(
                Self {
                    device,
                    buffer,

                    size: create_info.size,
                    usage_flags: create_info.usage_flags,
                    create_flags: create_info.create_flags
                }
            )
        }
    }

    #[inline]
    pub fn get_usage_flags(&self) -> BufferUsageFlags {
        self.usage_flags
    }

    #[inline]
    pub fn get_create_flags(&self) -> BufferCreateFlags {
        self.create_flags
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.size
    }
}

impl Drop for RawBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_buffer(
                self.buffer,
                None
            );
        }
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferCreationError {
    #[error("error during raw buffer creation")]
    RawBufferError(#[from] RawBufferCreationError),
    #[error("error during memory allocation")]
    AllocationError(#[from] AllocationError),
    #[error("device dont have memory type with given flags what support given buffer")]
    NoValidMemoryTypeFound,
    #[error("binding memory to buffer failed")]
    MemoryBindError,
    #[error("buffer device and allocator device dont match")]
    InvalidDevice
}

/// Wrapper for raw buffer what contains allocated memory
pub struct Buffer {
    pub(crate) raw: RawBuffer,

    pub(crate) allocator: Arc<Allocator>,
    pub(crate) memory: AllocatedMemory,
}

impl Buffer {
    pub(crate) fn create_and_allocate(
        device: Arc<DeviceInner>,
        allocator: Arc<Allocator>,
        memory_properties: MemoryTypeProperties,
        create_info: &BufferCreateInfo
    ) -> Result<Self, BufferCreationError> {
        Ok(
            Self::from_raw(
                RawBuffer::create(device, create_info)?,
                allocator,
                memory_properties
            )?
        )
    }

    pub fn from_raw(
        raw: RawBuffer,
        allocator: Arc<Allocator>,
        memory_properties: MemoryTypeProperties
    ) -> Result<Self, BufferCreationError> {
        if allocator.get_device().ne(&raw.device) {
            return Err(BufferCreationError::InvalidDevice)
        }

        unsafe {
            let requirement = raw.device.get_buffer_memory_requirements(raw.buffer);
            let memory_type_index = bitvec::array::BitArray::<u32, bitvec::order::Lsb0>::from(requirement.memory_type_bits)
                .into_iter()
                .enumerate()
                .filter(| (_, t) | *t)
                .map(| (i, _) | i)
                .filter(| i | raw.device.memory_properties.memory_types[*i].properties.contains(memory_properties))
                .map(| i | i as u32)
                .next()
                .ok_or(BufferCreationError::NoValidMemoryTypeFound)?;

            let memory = allocator.allocate(
                memory_type_index,
                requirement.size,
                requirement.alignment
            )?;

            raw.device.bind_buffer_memory(
                raw.buffer,
                memory.memory.device_memory,
                memory.offset
            ).map_err(|_| BufferCreationError::MemoryBindError)?;

            Ok(
                Self {
                    raw,

                    allocator,
                    memory
                }
            )
        }
    }
}

impl Deref for Buffer {
    type Target = RawBuffer;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}