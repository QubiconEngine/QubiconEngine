use std::sync::{
    Arc,
    Mutex
};
use crate::{device::inner::DeviceInner, error::{VkError, ValidationError}, Error, instance::physical_device::memory_properties::MemoryTypeProperties};
use ash::vk::{
    DeviceMemory as VkDeviceMemory,
    MemoryAllocateInfo as VkMemoryAllocateInfo
};

pub struct DeviceMemoryObject {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) device_memory: VkDeviceMemory,

    pub(crate) memory_type_index: u8,
    pub(crate) size: u64, // TODO: DeviceAddress

    mapped_memory: Mutex<(u32, Option<*mut ()>)>
}

impl DeviceMemoryObject {
    /// # Safety
    /// * size must be not equal to 0 and be less than heap size and total device memory size
    /// * type index must be less than device memory type count
    pub(crate) unsafe fn allocate_unchecked(
        device: Arc<DeviceInner>,
        memory_type_index: u8,
        size: u64
    ) -> Result<Arc<Self>, Error> {
        let device_memory = device.allocate_memory(
            &VkMemoryAllocateInfo {
                allocation_size: size,
                memory_type_index: memory_type_index as u32,

                ..Default::default()
            },
            None
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        Ok(
            Arc::new(
                Self {
                    device,
                    device_memory,

                    memory_type_index,
                    size,

                    mapped_memory: Mutex::new((0, None))
                }
            )
        )
    }

    // TODO: Add checks for max allocations and coherent memory
    pub(crate) fn allocate(
        device: Arc<DeviceInner>,
        memory_type_index: u8,
        size: u64
    ) -> Result<Arc<Self>, Error> {
        if memory_type_index > device.memory_properties.memory_types.len() as u8 {
            return Err(ValidationError::NoValidMemoryTypeFound.into())
        }
        if size == 0 {
            return Err(ValidationError::InvalidAllocationSize.into())
        }

        unsafe {
            let memory_type = device.memory_properties.memory_types
                .get_unchecked(memory_type_index as usize);
            let memory_heap = device.memory_properties.memory_heaps
                .get_unchecked(memory_type.heap_index as usize);

            if size > memory_heap.size {
                return Err(ValidationError::InvalidAllocationSize.into())
            }

            Self::allocate_unchecked(device, memory_type_index, size)
        }
    }

    /// Maps device memory into process memory
    /// Memory should be host visible
    /// 
    /// # Safety
    /// * For each map call after that should be called unmap
    /// * After destroying memory object no mapped pointers to what memory should left
    pub unsafe fn map(&self) -> Result<*mut (), Error> {
        let memory_type = self.device.memory_properties.memory_types
            .get_unchecked(self.memory_type_index as usize);

        if !memory_type.properties.contains(MemoryTypeProperties::HOST_VISIBLE) {
            return Err(ValidationError::MemoryMappingNotSupported.into())
        }

        let mut lock = self.mapped_memory.lock().unwrap();
        lock.0 += 1;

        if lock.0 == 1 {
            let ptr = self.device.map_memory(
                self.device_memory,
                0,
                self.size,
                Default::default()
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            lock.1 = Some(ptr.cast());

            Ok(ptr.cast())
        } else {
            Ok(lock.1.unwrap_unchecked())
        }
    }

    /// # Safety
    /// For each map call shoul be one unmap call
    pub unsafe fn unmap(&self) {
        let mut lock = self.mapped_memory.lock().unwrap();

        if lock.0 != 0 {
            lock.0 -= 1;
        }

        if lock.0 == 0 && lock.1.is_some() {
            lock.1 = None;

            self.device.unmap_memory(self.device_memory);
        }
    }

    pub fn memory_type_index(&self) -> u8 {
        self.memory_type_index
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

unsafe impl Send for DeviceMemoryObject {}
unsafe impl Sync for DeviceMemoryObject {}

impl Drop for DeviceMemoryObject {
    fn drop(&mut self) {
        unsafe {
            self.device.free_memory(
                self.device_memory,
                None
            )
        }
    }
}