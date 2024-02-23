use bitflags::bitflags;
use arrayvec::ArrayVec;
use ash::{
    vk::MemoryHeapFlags as VkMemoryHeapFlags,
    vk::MemoryPropertyFlags as VkMemoryPropertyFlags
};

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MemoryTypeProperties: u32 {
        const DEVICE_LOCAL = 0b1;
        const HOST_VISIBLE = 0b10;
        const HOST_COHERENT = 0b100;
        const HOST_HASHED = 0b1000;
        const LAZILY_ALLOCATED = 0b10000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MemoryHeapProperties: u32 {
        const DEVICE_LOCAL = 0b1;
    }
}

impl TryFrom<&str> for MemoryTypeProperties {
    type Error = bitflags::parser::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        bitflags::parser::from_str(value)
    }
}

impl Into<MemoryTypeProperties> for VkMemoryPropertyFlags {
    fn into(self) -> MemoryTypeProperties {
        MemoryTypeProperties(self.as_raw().into())
    }
}

impl TryFrom<&str> for MemoryHeapProperties {
    type Error = bitflags::parser::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        bitflags::parser::from_str(value)
    }
}

impl Into<MemoryHeapProperties> for VkMemoryHeapFlags {
    fn into(self) -> MemoryHeapProperties {
        MemoryHeapProperties(self.as_raw().into())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryType {
    pub properties: MemoryTypeProperties,
    pub heap_index: u32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryHeap {
    pub properties: MemoryHeapProperties,
    pub size: u64 // TODO: Add DeviceSize type
}

impl Into<MemoryType> for ash::vk::MemoryType {
    fn into(self) -> MemoryType {
        MemoryType { 
            properties: self.property_flags.into(),
            heap_index: self.heap_index
        }
    }
}

impl Into<MemoryHeap> for ash::vk::MemoryHeap {
    fn into(self) -> MemoryHeap {
        MemoryHeap {
            properties: self.flags.into(),
            size: self.size
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceMemoryProperties {
    pub memory_types: ArrayVec<MemoryType, 32/*ash::vk::MAX_MEMORY_TYPES*/>, // TODO: Create own type for heap and mem type
    pub memory_heaps: ArrayVec<MemoryHeap, 32/*ash::vk::MAX_MEMORY_HEAPS*/>
}

impl Into<DeviceMemoryProperties> for ash::vk::PhysicalDeviceMemoryProperties {
    fn into(self) -> DeviceMemoryProperties {
        DeviceMemoryProperties {
            memory_types: ArrayVec::from_iter(self.memory_types.into_iter().map(Into::into)),
            memory_heaps: ArrayVec::from_iter(self.memory_heaps.into_iter().map(Into::into))
        }
    }
}