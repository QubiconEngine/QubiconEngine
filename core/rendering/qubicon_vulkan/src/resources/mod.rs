//pub mod format;

//mod image;
mod buffer;
//mod image_view;
mod buffer_view;
//mod resource_factory;


use crate::instance::physical_device::DeviceSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,

    pub memory_types: bitvec::BitArr!(for 32, in u32)
}

impl From<ash::vk::MemoryRequirements> for MemoryRequirements {
    fn from(value: ash::vk::MemoryRequirements) -> Self {
        Self {
            size: value.size,
            alignment: value.alignment,

            memory_types: bitvec::array::BitArray::new([value.memory_type_bits])
        }
    }
}