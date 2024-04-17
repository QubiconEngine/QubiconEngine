use bitflags::bitflags;
use arrayvec::ArrayVec;


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

impl From<ash::vk::MemoryPropertyFlags> for MemoryTypeProperties {
    fn from(value: ash::vk::MemoryPropertyFlags) -> Self {
        Self ( value.as_raw().into() )
    }
}

impl TryFrom<&str> for MemoryHeapProperties {
    type Error = bitflags::parser::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        bitflags::parser::from_str(value)
    }
}

impl From<ash::vk::MemoryHeapFlags> for MemoryHeapProperties {
    fn from(value: ash::vk::MemoryHeapFlags) -> Self {
        Self ( value.as_raw().into() )
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

impl From<ash::vk::MemoryType> for MemoryType {
    fn from(value: ash::vk::MemoryType) -> Self {
        Self {
            properties: value.property_flags.into(),
            heap_index: value.heap_index
        }
    }
}

impl From<ash::vk::MemoryHeap> for MemoryHeap {
    fn from(value: ash::vk::MemoryHeap) -> Self {
        Self {
            properties: value.flags.into(),
            size: value.size
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceMemoryProperties {
    pub memory_types: ArrayVec<MemoryType, 32/*ash::vk::MAX_MEMORY_TYPES*/>,
    pub memory_heaps: ArrayVec<MemoryHeap, 32/*ash::vk::MAX_MEMORY_HEAPS*/>
}


impl From<ash::vk::PhysicalDeviceMemoryProperties> for DeviceMemoryProperties {
    fn from(value: ash::vk::PhysicalDeviceMemoryProperties) -> Self {
        Self {
            memory_types: ArrayVec::from_iter(value.memory_types.into_iter().take(value.memory_type_count).map(Into::into)),
            memory_heaps: ArrayVec::from_iter(value.memory_heaps.into_iter().take(value.memory_heap_count).map(Into::into))
        }
    }
}