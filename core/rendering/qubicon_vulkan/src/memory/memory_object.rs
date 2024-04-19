use std::sync::{ Arc, Mutex, atomic::Ordering };

use super::{ DeviceSize, MemoryTypeProperties };
use crate::{ error::VkError, device::Device, instance::physical_device::PhysicalDevice };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllocationInfo {
    pub size: DeviceSize,
    pub memory_type: u32
}

impl AllocationInfo {
    pub fn validate(&self, device: &PhysicalDevice) {
        if self.size == 0 {
            panic!("allocation size shouldnt be zero");
        }

        let memory_types = &device.memory_properties().memory_types;
        let memory_heaps = &device.memory_properties().memory_heaps;

        if memory_types.len() < self.memory_type as usize {
            panic!("no memory type with index {} exist for this device", self.memory_type);
        }

        let heap_idx = memory_types[self.memory_type as usize].heap_index;
    
        if self.size > memory_heaps[heap_idx as usize].size {
            panic!("allocation size should be less than heap size");
        }

        // TODO: more checks
    }
}

impl From<AllocationInfo> for ash::vk::MemoryAllocateInfo {
    fn from(value: AllocationInfo) -> Self {
        Self::builder()
            .allocation_size(value.size)
            .memory_type_index(value.memory_type)
            .build()
    }
}



struct MapData {
    map_count: u32,
    map_addr: *mut ()
}

pub struct MemoryObject {
    device: Arc<Device>,
    
    size: DeviceSize,
    memory_type: u32,
    memory_properties: MemoryTypeProperties,

    map_data: Mutex<MapData>,
    
    memory: ash::vk::DeviceMemory
}

impl MemoryObject {
    pub(crate) unsafe fn as_raw(&self) -> ash::vk::DeviceMemory {
        self.memory
    }

    pub fn allocate_from(device: Arc<Device>, allocation_info: AllocationInfo) -> Result<Self, VkError> {
        allocation_info.validate(device.physical_device());

        let memory_properties = device.physical_device()
            .memory_properties()
            .memory_types[allocation_info.memory_type as usize]
            .properties;

        
        { // check memory objects count, and, if too much, return VkError::TooManyObjects
            let max_memory_objects_count = device.physical_device().properties().limits.max_memory_allocation_count;
            let memory_objects_count = device.memory_objects_count();

            if memory_objects_count >= max_memory_objects_count {
                return Err( VkError::TooManyObjects );
            }
        }

        let memory = unsafe {
            let allocate_info = allocation_info.into();

            device.as_raw().allocate_memory(&allocate_info, None)
        }?;


        unsafe { device.edit_memory_objects_count().fetch_add(1, Ordering::SeqCst) };


        let result = Self {
            device,

            size: allocation_info.size,
            memory_type: allocation_info.memory_type,
            memory_properties,

            map_data: Mutex::new(
                MapData {
                    map_count: 0,
                    map_addr: core::ptr::null_mut()
                }
            ),

            memory
        };

        Ok ( result )
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    
    pub fn size(&self) -> DeviceSize {
        self.size
    }

    pub fn memory_type(&self) -> u32 {
        self.memory_type
    }


    /// # Safety
    /// Mapped memory is always mutable. Some synchronization needs to be done
    pub unsafe fn map(&self, offset: DeviceSize) -> Result<MapGuard, VkError> {
        if !self.memory_properties.contains( MemoryTypeProperties::HOST_VISIBLE ) {
            return Err( VkError::MemoryMapFailed );
        }
        
        assert!(offset > self.size, "offset is greater than size. Offset is {}, size is {}", offset, self.size);

        // Maybe unwrap_unchecked ?
        let mut map_data = self.map_data.lock().unwrap();


        if map_data.map_count == 0 {
            map_data.map_addr = self.device.as_raw()
                .map_memory(self.memory, 0, self.size, Default::default())
                .map(| ptr | ptr.cast())?;
        }

        map_data.map_count += 1;


        let result = MapGuard {
            memory_object: self,
            ptr: map_data.map_addr.byte_add(offset as usize),
            
            offset
        };

        Ok( result )
    }

    /// # Safety
    /// Memory should be previously mapped
    unsafe fn unmap(&self) {
        let mut map_data = self.map_data.lock().unwrap();


        map_data.map_count -= 1;

        if map_data.map_count == 0 {
            self.device.as_raw().unmap_memory(self.memory)
        }
    }
}

impl Drop for MemoryObject {
    fn drop(&mut self) {
        unsafe {
            self.device.edit_memory_objects_count().fetch_sub(1, Ordering::SeqCst);

            self.device.as_raw().free_memory( self.memory, None )
        }
    }
}



pub struct MapGuard<'a> {
    memory_object: &'a MemoryObject,
    ptr: *mut (),
    offset: DeviceSize
}

impl<'a> Drop for MapGuard<'a> {
    fn drop(&mut self) {
        unsafe { self.memory_object.unmap() }
    }
}

impl<'a> MapGuard<'a> {
    pub fn ptr(&self) -> *mut () {
        self.ptr
    }

    pub fn offset(&self) -> DeviceSize {
        self.offset
    }
}