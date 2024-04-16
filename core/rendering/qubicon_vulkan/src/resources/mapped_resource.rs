use std::{
    ops::{
        Deref,
        DerefMut
    },
    mem::MaybeUninit,
    marker::PhantomData
};
use crate::memory::alloc::{DeviceMemoryAllocator, MappableAllocatedDeviceMemoryFragment, MapGuard};

/// # Safety
/// Layout of types what implement this trait should match their layout in shaders
pub unsafe trait MappableType: Sized + Copy + 'static {}

unsafe impl<T: Send + Sync + Sized + Copy + 'static> MappableType for T {}


pub struct MappedResource<'a, T: MappableType, A: DeviceMemoryAllocator>
    where A::MemoryFragmentType: MappableAllocatedDeviceMemoryFragment<'a>
{
    map_guard: <A::MemoryFragmentType as MappableAllocatedDeviceMemoryFragment<'a>>::MapGuard,
    len: usize,

    _ph: PhantomData<T>
}

impl<'a, T: MappableType, A: DeviceMemoryAllocator> MappedResource<'a, T, A>
    where A::MemoryFragmentType: MappableAllocatedDeviceMemoryFragment<'a>
{
    pub(crate) unsafe fn new(
        map_guard: <A::MemoryFragmentType as MappableAllocatedDeviceMemoryFragment<'a>>::MapGuard,
        len: usize
    ) -> Self {
        Self {
            map_guard,
            len,

            _ph: Default::default()
        }
    }
}

impl<'a, T: MappableType, A: DeviceMemoryAllocator> Deref for MappedResource<'a, T, A>
    where A::MemoryFragmentType: MappableAllocatedDeviceMemoryFragment<'a>
{
    type Target = [MaybeUninit<T>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(
                self.map_guard.as_ptr().cast(),
                self.len
            )
        }
    }
}

impl<'a, T: MappableType, A: DeviceMemoryAllocator> DerefMut for MappedResource<'a, T, A>
    where A::MemoryFragmentType: MappableAllocatedDeviceMemoryFragment<'a> 
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            core::slice::from_raw_parts_mut(
                self.map_guard.as_mut_ptr().cast(),
                self.len
            )
        }
    }
}