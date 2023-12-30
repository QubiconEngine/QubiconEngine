use std::{
    sync::Arc,
    cell::Cell,
    marker::PhantomData
};
use ash::vk::{
    Semaphore as VkSemaphore,
    SemaphoreWaitInfo as VkSemaphoreWaitInfo,
    SemaphoreSignalInfo as VkSemaphoreSignalInfo,
    SemaphoreCreateInfo as VkSemaphoreCreateInfo,
    SemaphoreTypeCreateInfo as VkSemaphoreTypeCreateInfo
};

use crate::{
    Error,
    error::VkError,
    device::inner::DeviceInner
};

pub mod types {
    pub struct Binary;
    pub struct Timeline;

    pub trait SemaphoreType: sealed::SemaphoreTypeSealed {}
    impl<T: sealed::SemaphoreTypeSealed> SemaphoreType for T {} 

    mod sealed {
        use ash::vk::SemaphoreType;

        pub trait SemaphoreTypeSealed {
            const TYPE: SemaphoreType;
        }


        impl SemaphoreTypeSealed for super::Binary {
            const TYPE: SemaphoreType = SemaphoreType::BINARY;
        }
        impl SemaphoreTypeSealed for super::Timeline {
            const TYPE: SemaphoreType = SemaphoreType::TIMELINE;
        }
    }
}


pub struct Semaphore<Type: types::SemaphoreType> {
    device: Arc<DeviceInner>,
    semaphore: VkSemaphore,

    _ty: PhantomData<Type>,
    // For disabling Sync
    _ph: PhantomData<Cell<()>>
}

impl<Type: types::SemaphoreType> Semaphore<Type> {
    pub(crate) fn create(device: Arc<DeviceInner>) -> Result<Self, Error> {
        let r#type = VkSemaphoreTypeCreateInfo {
            semaphore_type: Type::TYPE,
            initial_value: 0,

            ..Default::default()
        };
        
        let semaphore = unsafe {
            device.create_semaphore(
                &VkSemaphoreCreateInfo {
                    p_next: (&r#type as *const VkSemaphoreTypeCreateInfo).cast(),

                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?
        };
        
        Ok(
            Self {
                device,
                semaphore,

                _ty: Default::default(),
                _ph: Default::default()
            }
        )
    }

    pub(crate) unsafe fn as_raw(&self) -> VkSemaphore {
        self.semaphore
    }
}


impl Semaphore<types::Timeline> {
    pub fn wait(&self, timeline: u64, timeout: u64) -> Result<(), VkError> {
        unsafe {
            self.device.wait_semaphores(
                &VkSemaphoreWaitInfo {
                    semaphore_count: 1,
                    p_semaphores: &self.semaphore,
                    p_values: &timeline,

                    ..Default::default()
                },
                timeout
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())
        }
    }

    pub fn get_counter_value(&self) -> Result<u64, VkError> {
        unsafe {
            self.device.get_semaphore_counter_value(self.semaphore)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked())
        }
    }


    pub fn signal(&mut self, timeline_value: u64) -> Result<(), VkError> {
        unsafe {
            self.device.signal_semaphore(
                &VkSemaphoreSignalInfo {
                    semaphore: self.semaphore,
                    value: timeline_value,

                    ..Default::default()
                }
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())
        }
    }
}

impl<Type: types::SemaphoreType> Drop for Semaphore<Type> {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_semaphore(
                self.semaphore,
                None
            )
        }
    }
}