use std::sync::Arc;
use bitflags::bitflags;

use super::{ MemoryRequirements, AllocHandle };
use crate::{ error::VkError, device::Device, memory::{ DeviceSize, alloc::{ Allocator, Allocation, MapGuard } } };

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

impl From<ash::vk::BufferUsageFlags> for BufferUsageFlags {
    fn from(value: ash::vk::BufferUsageFlags) -> Self {
        Self ( value.as_raw().into() )
    }
}

impl From<BufferUsageFlags> for ash::vk::BufferUsageFlags {
    fn from(value: BufferUsageFlags) -> Self {
        Self::from_raw(value.bits())
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

impl From<ash::vk::BufferCreateFlags> for BufferCreateFlags {
    fn from(value: ash::vk::BufferCreateFlags) -> Self {
        Self ( value.as_raw().into() )
    }
}

impl From<BufferCreateFlags> for ash::vk::BufferCreateFlags {
    fn from(value: BufferCreateFlags) -> Self {
        Self::from_raw(value.bits())
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferCreateInfo {
    pub usage_flags: BufferUsageFlags,
    // TODO: create_flags. They may require some additional structures

    pub size: DeviceSize,

    // TODO: Sharing mode
}

impl BufferCreateInfo {
    pub fn validate(&self) {
        if self.size == 0 {
            panic!("size is zero");
        }

        // TODO: Sharing mode check
        // TODO: create_flags check
    }
}

impl From<BufferCreateInfo> for ash::vk::BufferCreateInfo {
    fn from(value: BufferCreateInfo) -> Self {
        Self::builder()
            .usage(value.usage_flags.into())
            .size(value.size)
            //.sharing_mode(sharing_mode)
            .build()
    }
}



pub struct UnbindedBuffer {
    device: Arc<Device>,

    size: DeviceSize,
    usage_flags: BufferUsageFlags,

    buffer: ash::vk::Buffer
}

impl UnbindedBuffer {
    pub(crate) unsafe fn as_raw(&self) -> ash::vk::Buffer {
        self.buffer
    }

    pub fn new(device: Arc<Device>, create_info: &BufferCreateInfo) -> Result<Self, VkError> {
        create_info.validate();


        let buffer = unsafe {
            device.as_raw().create_buffer(&(*create_info).into(), None)
        };
        

        let result = Self {
            device,

            size: create_info.size,
            usage_flags: create_info.usage_flags,

            buffer: buffer?
        };

        Ok ( result )
    }

    pub fn size(&self) -> DeviceSize {
        self.size
    }

    pub fn usage_flags(&self) -> BufferUsageFlags {
        self.usage_flags
    }

    pub fn memory_requirements(&self) -> MemoryRequirements {
        unsafe { self.device.as_raw().get_buffer_memory_requirements(self.as_raw()) }
            .into()
    }
}

impl Drop for UnbindedBuffer {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().destroy_buffer( self.buffer, None ) }
    }
}


pub struct Buffer<A: Allocator> {
    // this field is dropped first due to RFC 1857
    buffer: UnbindedBuffer,
    alloc: AllocHandle<A>
}

impl<A: Allocator> Buffer<A> {
    /// # Safety
    /// Allocation should be valid and perfectly match buffer [MemoryRequirements]
    /// 
    /// This means what:
    /// * Allocation should contain enough space to fit buffer inside
    /// * Allocation should be properly aligned
    /// * Allocation should be located in memory, what type is allowed by [MemoryRequirements]
    /// * Allocation must not be outside of memory object ( allocation.offset() + allocation.size() <= memory_object.size() )
    /// 
    /// ['MemoryRequirements']: crate::resources::MemoryRequirements
    pub unsafe fn from_buffer_and_allocation_unchecked(buffer: UnbindedBuffer, allocator: A, allocation: A::Allocation) -> Result<Self, VkError> {
        let memory_object = allocation.memory_object();
        
        buffer.device.as_raw().bind_buffer_memory( 
            buffer.as_raw(),
            memory_object.as_raw(),
            allocation.offset()
        )?;
        
        let result = Self {
            buffer,
            alloc: AllocHandle::new(allocator, allocation)
        };

        Ok( result )
    }

    pub fn from_buffer_and_allocation(buffer: UnbindedBuffer, allocator: A, allocation: A::Allocation) -> Result<Self, VkError> {
        buffer.memory_requirements()
            .validate_allocation(&allocation);

        unsafe { Self::from_buffer_and_allocation_unchecked(buffer, allocator, allocation) }
    }
}

impl<A: Allocator> core::ops::Deref for Buffer<A> {
    type Target = UnbindedBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
} 

impl<A: Allocator> Drop for Buffer<A> {
    fn drop(&mut self) { /* Yes, its empty. Just so we dont accidentally take some field out */ }
}


pub struct TypedBuffer<T: BufferType, A: Allocator> {
    buffer: Buffer<A>,

    _ph: core::marker::PhantomData<T>
}

impl<T: BufferType, A: Allocator> TypedBuffer<T, A> {
    pub fn from_buffer(buffer: Buffer<A>) -> Self {
        if buffer.size() as usize % T::size() != 0 {
            panic!("buffer size is not multiple of contained type");
        }

        Self { buffer, _ph: Default::default() }
    }

    // Throws compile time error if 'a is removed
    #[allow(clippy::needless_lifetimes)]
    pub fn map<'a>(&'a self) -> Result<BufferMapGuard<'a, T, <A::Allocation as Allocation>::MapGuard<'a>>, VkError> {
        let result = BufferMapGuard {
            map_guard: self.alloc.map()?,
            size: self.size() as usize,

            _ph: Default::default()
        };

        Ok( result )
    }
}

impl<T: BufferType, A: Allocator> core::ops::Deref for TypedBuffer<T, A> {
    type Target = Buffer<A>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}



/// This structure is an absolute piece of shit. Should be rewritten in future
pub struct BufferMapGuard<'a, T: BufferType + ?Sized, M: MapGuard> {
    map_guard: M,
    size: usize,

    _ph: core::marker::PhantomData<&'a T>
}

impl<'a, T: BufferType + Sized, M: MapGuard> core::ops::Deref for BufferMapGuard<'a, T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.map_guard.as_ptr()
                .cast::<T>()
                .as_ref()
                .unwrap()
        }
    }
}

impl<'a, T: BufferType + Sized, M: MapGuard> core::ops::Deref for BufferMapGuard<'a, [T], M>
    where [T]: BufferType
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(
                self.map_guard.as_ptr().cast(),
                self.size / core::mem::size_of::<T>()
            )
        }
    }
}

/// # Safety
/// Type shouldnt implement drop, it internal fields must also implement this trait and etc
pub unsafe trait BufferType {
    /// Size of type or size of slice element
    fn size() -> usize;
}

mod impl_buffer_type {
    use super::BufferType;

    macro_rules! impl_buffer_type {
        ($ty:tt) => {
            unsafe impl BufferType for $ty {
                fn size() -> usize {
                    core::mem::size_of::<$ty>()
                }
            }
        };
    }

    impl_buffer_type!(u32);
    impl_buffer_type!(i32);
    impl_buffer_type!(f32);

    unsafe impl<T: BufferType> BufferType for [T] {
        fn size() -> usize {
            T::size()
        }
    }
}