pub mod format;

pub mod image;
pub mod buffer;
//mod image_view;
//mod resource_factory;


use crate::memory::{ DeviceSize, alloc::{ Allocator, Allocation } };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,

    pub memory_types: bitvec::BitArr!(for 32, in u32)
}

impl MemoryRequirements {
    pub fn validate_allocation(&self, allocation: &impl Allocation) {
        if allocation.size() < self.size {
            panic!("allocation is too small");
        }

        if allocation.offset() % self.alignment != 0 {
            panic!("allocation has invalid alignment");
        }

        if allocation.offset() + allocation.size() > unsafe { allocation.memory_object() }.size().get() {
            panic!("memory object cant fit allocation inside")
        }



        let memory_type = unsafe { allocation.memory_object() }.memory_type();
        let memory_type_is_valid = self.memory_types.iter().enumerate()
            .filter(| (_, allowed) | **allowed)
            .any(| (idx, _) | idx as u32 == memory_type);

        if !memory_type_is_valid {
            panic!("allocation is located in memory object, what has incorrect memory type")
        }
    }
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



use core::mem::ManuallyDrop;

struct AllocHandle<A: Allocator> {
    allocator: A,
    allocation: ManuallyDrop<A::Allocation>
}

impl<A: Allocator> AllocHandle<A> {
    fn new(allocator: A, allocation: A::Allocation) -> Self {
        Self { allocator, allocation: ManuallyDrop::new(allocation) }
    }
}

impl<A: Allocator> core::ops::Deref for AllocHandle<A> {
    type Target = A::Allocation;

    fn deref(&self) -> &Self::Target {
        &self.allocation
    }
}

impl<A: Allocator> Drop for AllocHandle<A> {
    fn drop(&mut self) {
        self.allocator.dealloc(
            unsafe { ManuallyDrop::take(&mut self.allocation) }
        )
    }
}