use std::sync::Arc;

use crate::{
    Error,
    device::{
        Device,
        inner::DeviceInner
    }
};
use super::{
    DeviceMemoryObject,
    DeviceMemoryAllocator,
    AllocatedDeviceMemoryFragment, MapGuard, MappableAllocatedDeviceMemoryFragment
};

pub struct StandartMemoryAllocator {
    device: Arc<DeviceInner>
}

impl StandartMemoryAllocator {
    pub fn new(device: &Device) -> Arc<StandartMemoryAllocator> {
        Arc::new(
            Self {
                device: Arc::clone(&device.inner)
            }
        )
    }
}

unsafe impl DeviceMemoryAllocator for StandartMemoryAllocator {
    type AllocError = Error;
    type MemoryFragmentType = StandartDeviceMemoryFragment;
    
    unsafe fn alloc(&self, memory_type_index: u8, size: u64, _align: u64) -> Result<Self::MemoryFragmentType, Self::AllocError> {
        let memory = DeviceMemoryObject::allocate(
            Arc::clone(&self.device),
            memory_type_index,
            size
        )?;
        let memory = Arc::into_inner(memory)
            .unwrap_unchecked();

        Ok(
            StandartDeviceMemoryFragment {
                memory
            }
        )
    }

    unsafe fn dealloc(&self, fragment: Self::MemoryFragmentType) {
        core::mem::drop(fragment);
    }
}

pub struct StandartDeviceMemoryFragment {
    memory: DeviceMemoryObject
}

unsafe impl AllocatedDeviceMemoryFragment for StandartDeviceMemoryFragment {
    unsafe fn as_memory_object_and_offset(&self) -> (&DeviceMemoryObject, u64) {
        (&self.memory, 0)
    }
}

unsafe impl<'a> MappableAllocatedDeviceMemoryFragment<'a> for StandartDeviceMemoryFragment {
    type MapError = Error;
    type MapGuard = StandartMapGuard<'a>;

    fn map(&'a self) -> Result<Self::MapGuard, Self::MapError> {
        Ok(
            StandartMapGuard {
                fragment: self,
                ptr: unsafe { self.memory.map() }?
            }
        )
    }
}


pub struct StandartMapGuard<'a> {
    fragment: &'a StandartDeviceMemoryFragment,
    ptr: *mut ()
}

unsafe impl<'a> MapGuard<'a> for StandartMapGuard<'a> {
    unsafe fn as_ptr(&self) -> *const () {
        self.ptr
    }

    unsafe fn as_mut_ptr(&mut self) -> *mut () {
        self.ptr
    }
}

impl<'a> Drop for StandartMapGuard<'a> {
    fn drop(&mut self) {
        unsafe { self.fragment.memory.unmap() }
    }
}