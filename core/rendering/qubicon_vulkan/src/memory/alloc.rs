use crate::{ error::VkError, resources::MemoryRequirements };
use super::{
    DeviceSize,
    MemoryTypeProperties,
    
    MemoryObject
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryKind {
    /// Memory is local to GPU and cant be accessed from CPU.
    /// GPU can access this memory way faster than any other.
    /// Preffered format to textures, models and all that stuff
    Local,
    /// Memory can be accessed both from CPU and GPU.
    /// Effective for transfering data to GPU
    Upload,
    /// Memory also can be accessed both from CPU and GPU.
    /// Effective for reading data from GPU
    Download,
    
    /// Memory for some special needs. Will be rarely used.
    Custom {
        /// Properties what memory type should have
        allowed_properties: MemoryTypeProperties,
        /// Properties what memory type shouldnt have
        denied_properties: MemoryTypeProperties
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationLifetime {
    /// Usable for staging buffers, temporal images and etc.
    /// Basicaly for resources what needed for a short period.
    Short,
    /// Usable for textures and models, because they are
    /// required for a pretty long period.
    Long
}


pub trait Allocator {
    type Allocation: Allocation;
    
    fn alloc(&self, requirements: MemoryRequirements, kind: MemoryKind, lifetime: AllocationLifetime) -> Result<Self::Allocation, VkError>;
    fn dealloc(&self, allocation: Self::Allocation);
}

pub trait Allocation {
    type MapGuard<'a>: MapGuard where Self: 'a;
    type MutMapGuard<'a>: MutMapGuard where Self: 'a;

    // If 'a will be removed, these methods will throw a compile time error
    #[allow(clippy::needless_lifetimes)]
    fn map<'a>(&'a self) -> Result<Self::MapGuard<'a>, VkError>;
    #[allow(clippy::needless_lifetimes)]
    fn map_mut<'a>(&'a mut self) -> Result<Self::MutMapGuard<'a>, VkError>;

    /// Size of allocation
    fn size(&self) -> DeviceSize;
    /// Offset in MemoryObject
    fn offset(&self) -> DeviceSize;

    /// # Safety
    /// Caller shouldnt use memory outside of range, defined by offset and size
    unsafe fn memory_object(&self) -> &MemoryObject;
}

pub trait MapGuard {
    fn as_ptr(&self) -> *const ();
}
pub trait MutMapGuard {
    fn as_mut_ptr(&mut self) -> *mut ();
}