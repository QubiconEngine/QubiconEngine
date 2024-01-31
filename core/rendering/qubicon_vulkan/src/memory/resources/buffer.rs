use std::sync::Arc;
use bitflags::bitflags;
use super::{buffer_view::{BufferView, BufferViewCreateInfo}, mapped_resource::{MappedResource, MappableType}, ResourceCreationError, ResourceMemory};
use crate::{
    Error,
    device::inner::DeviceInner,
    error::{VkError, ValidationError},
    memory::alloc::{hollow_device_memory_allocator::HollowDeviceMemoryAllocator, AllocatedDeviceMemoryFragment, DeviceMemoryAllocator, MappableAllocatedDeviceMemoryFragment},
    instance::physical_device::memory_properties::MemoryTypeProperties,
};
use ash::vk::{
    Buffer as VkBuffer, BufferCreateFlags as VkBufferCreateFlags, BufferCreateInfo as VkBufferCreateInfo, BufferUsageFlags as VkBufferUsageFlags
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

pub(crate) struct BufferInner<A: DeviceMemoryAllocator> {
    pub(crate) device: Arc<DeviceInner>,

    pub(crate) buffer: VkBuffer,
    pub(crate) info: BufferCreateInfo,

    // in some cases we dont want to destroy our resources
    drop_required: bool,
    memory: Option<ResourceMemory<A>>
}

impl<A: DeviceMemoryAllocator> Drop for BufferInner<A> {
    fn drop(&mut self) {
        core::mem::drop(self.memory.take());
        
        if self.drop_required {
            unsafe {
                self.device.destroy_buffer(
                    self.buffer,
                    None
                );
            }
        }
    }
}

/// Buffer without specified memory
pub struct RawBuffer {
    inner: Arc<BufferInner<HollowDeviceMemoryAllocator>>
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

            let inner = BufferInner {
                device,
                buffer,
                info: *create_info,
                drop_required: true,
                memory: None
            };

            Ok( Self { inner: Arc::new(inner) } )
        }
    }

    pub(crate) fn as_inner(&self) -> &Arc<BufferInner<HollowDeviceMemoryAllocator>> {
        &self.inner
    }

    #[inline]
    pub fn create_info(&self) -> &BufferCreateInfo {
        &self.inner.info
    }

    #[inline]
    pub fn usage_flags(&self) -> BufferUsageFlags {
        self.inner.info.usage_flags
    }

    #[inline]
    pub fn create_flags(&self) -> BufferCreateFlags {
        self.inner.info.create_flags
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.inner.info.size
    }
}

/// Wrapper for raw buffer what contains allocated memory
pub struct Buffer<A: DeviceMemoryAllocator> {
    inner: Arc<BufferInner<A>>
}

impl<A: DeviceMemoryAllocator> Buffer<A> {
    pub(crate) fn create_and_allocate(
        device: Arc<DeviceInner>,
        allocator: Arc<A>,
        memory_properties: MemoryTypeProperties,
        create_info: &BufferCreateInfo
    ) -> Result<Self, ResourceCreationError<A::AllocError>> {
        Self::from_raw(
            RawBuffer::create(device, create_info).map_err(ResourceCreationError::from_creation_error)?,
            allocator,
            memory_properties
        )
    }

    pub fn from_raw(
        raw: RawBuffer,
        allocator: Arc<A>,
        memory_properties: MemoryTypeProperties
    ) -> Result<Self, ResourceCreationError<A::AllocError>> {
        unsafe {
            let inner = Arc::into_inner(raw.inner)
                .expect("buffer is in use");

            let requirement = inner.device.get_buffer_memory_requirements(inner.buffer);
            let memory_type_index = bitvec::array::BitArray::<u32, bitvec::order::Lsb0>::from(requirement.memory_type_bits)
                .into_iter()
                .enumerate()
                .filter(| (_, t) | *t)
                .map(| (i, _) | i)
                .filter(| i | inner.device.memory_properties.memory_types[*i].properties.contains(memory_properties))
                .map(| i | i as u8)
                .next()
                .ok_or(ValidationError::NoValidMemoryTypeFound.into())
                .map_err(ResourceCreationError::from_creation_error)?;

            let memory = allocator.alloc(
                memory_type_index,
                requirement.size,
                requirement.alignment
            ).map_err(ResourceCreationError::from_allocation_error)?;

            let (raw_memory, offset) = memory.as_memory_object_and_offset();

            if raw_memory.device != inner.device {
                return Err(ResourceCreationError::from_creation_error(ValidationError::InvalidDevice.into()));
            }

            inner.device.bind_buffer_memory(
                inner.buffer,
                raw_memory.device_memory,
                offset
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked().into())
             .map_err(ResourceCreationError::from_creation_error)?;

            // just copying into new inner
            let inner = BufferInner {
                device: inner.device,
                buffer: inner.buffer,
                info: inner.info,
                drop_required: inner.drop_required,
                memory: Some( ResourceMemory::new(allocator, memory) )
            };

            Ok( Self { inner: Arc::new(inner) } )
        }
    }

    /// # Safety
    /// * Range should be in bounds of buffer, be multiple of format size and countain at least one element
    /// * Buffer should have one of Texel usage flags
    pub unsafe fn create_buffer_view_unchecked(
        &self,
        create_info: &BufferViewCreateInfo
    ) -> Result<Arc<BufferView<A>>, Error> {
        BufferView::create_unchecked(&self, create_info)
    }

    pub(crate) fn as_inner(&self) -> &Arc<BufferInner<A>> {
        &self.inner
    }

    // Literaly the same code as in RawBuffer
    #[inline]
    pub fn create_info(&self) -> &BufferCreateInfo {
        &self.inner.info
    }

    #[inline]
    pub fn usage_flags(&self) -> BufferUsageFlags {
        self.inner.info.usage_flags
    }

    #[inline]
    pub fn create_flags(&self) -> BufferCreateFlags {
        self.inner.info.create_flags
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.inner.info.size
    }
}

impl<'a, A: DeviceMemoryAllocator> Buffer<A>
    where A::MemoryFragmentType: MappableAllocatedDeviceMemoryFragment<'a>
{
    /// # Safety
    /// Buffer content is unknown. Cast types on your own risk!
    pub unsafe fn map<T: MappableType>(&'a self) -> Result<MappedResource<'a, T, A>, <A::MemoryFragmentType as MappableAllocatedDeviceMemoryFragment<'a>>::MapError> {
        if self.size() as usize % core::mem::size_of::<T>() != 0 {
            panic!("Buffer size is not multiple of type size!");
        }

        Ok(
            MappedResource::new(
                self.inner.memory.unwrap_unchecked().map()?,
                self.inner.info.size as usize / core::mem::size_of::<T>()
            )
        )
    }
}