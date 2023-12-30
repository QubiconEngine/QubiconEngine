use std::{
    sync::Arc,
    cell::Cell,
    marker::PhantomData
};
use ash::vk::Event as VkEvent;

use crate::{
    Error,
    error::VkError,
    device::inner::DeviceInner
};

pub struct Event {
    device: Arc<DeviceInner>,
    event: VkEvent,

    _ph: PhantomData<Cell<()>>
}

impl Event {
    pub(crate) fn create(device: Arc<DeviceInner>) -> Result<Self, Error> {
        let event = unsafe {
            device.create_event(
                &Default::default(),
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?
        };

        Ok(
            Self {
                device,
                event,

                _ph: Default::default()
            }
        )
    }

    /// Renamed *VkGetEventStatus*
    pub fn signaled(&self) -> Result<bool, VkError> {
        unsafe {
            self.device.get_event_status(self.event)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked())
        }
    }


    pub fn set(&mut self) -> Result<(), VkError> {
        unsafe {
            self.device.set_event(self.event)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked())
        }
    }

    pub fn reset(&mut self) -> Result<(), VkError> {
        unsafe {
            self.device.reset_event(self.event)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked())
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_event(
                self.event,
                None
            )
        }
    }
}