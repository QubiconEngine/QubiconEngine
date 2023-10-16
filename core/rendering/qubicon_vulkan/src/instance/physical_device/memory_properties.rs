use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct DeviceMemoryProperties {
    pub memory_types: SmallVec<[ash::vk::MemoryType; ash::vk::MAX_MEMORY_TYPES]>, // TODO: Create own type for heap and mem type
    pub memory_heaps: SmallVec<[ash::vk::MemoryHeap; ash::vk::MAX_MEMORY_HEAPS]>
}

impl Into<DeviceMemoryProperties> for ash::vk::PhysicalDeviceMemoryProperties {
    fn into(self) -> DeviceMemoryProperties {
        DeviceMemoryProperties {
            memory_types: SmallVec::from_buf_and_len(
                self.memory_types,
                self.memory_type_count as usize
            ),
            memory_heaps: SmallVec::from_buf_and_len(
                self.memory_heaps,
                self.memory_heap_count as usize
            )
        }
    }
}