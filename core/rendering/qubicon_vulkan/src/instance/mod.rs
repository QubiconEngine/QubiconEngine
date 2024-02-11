use std::{sync::Arc, fmt::Debug};
use physical_device::PhysicalDevice;
use crate::{
    Error,
    error::{
        VkError,
        ValidationError
    }
};

#[cfg(feature = "windowing")]
use crate::surface::Surface;

pub mod error;
pub mod creation_info;
pub mod physical_device;
pub(crate) mod inner;

pub struct Instance {
    inner: Arc<inner::InstanceInner>
}

impl Instance {
    pub fn create(create_info: &creation_info::InstanceCreateInfo) -> Result<Self, error::InstanceError> {
        Ok(
            Self {
                inner: Arc::new(inner::InstanceInner::create(create_info)?)
            }
        )
    }

    pub fn enumerate_devices(&self) -> ash::prelude::VkResult<impl Iterator<Item = PhysicalDevice>> {
        let inner = Arc::clone(&self.inner);
        let iter = unsafe { self.inner.enumerate_physical_devices()? }
            .into_iter()
            .map(move | dev | unsafe {
                PhysicalDevice::from_instance_and_raw_physical_device(
                    Arc::clone(&inner),
                    dev
                )
            });

        Ok(iter)
    }
}

impl Instance {
    pub(crate) fn from_inner(inner: Arc<inner::InstanceInner>) -> Self {
        Self { inner }
    }
}

impl Instance {
    #[cfg(feature = "x11")]
    /// # Safety
    /// *display* and *window* must be valid X objects
    pub unsafe fn create_surface_x11(&self, display: *mut x11::xlib::Display, window: x11::xlib::Window) -> Result<Surface, Error> {
        use ash::vk::XlibSurfaceCreateInfoKHR;

        if let Some(x_surface_ext_calls) = self.inner.x_surface.as_ref() {
            unsafe {
                let raw_surface = x_surface_ext_calls.create_xlib_surface(
                    &XlibSurfaceCreateInfoKHR {
                        dpy: display.cast(),
                        window,

                        ..Default::default()
                    },
                    None
                ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

                return Ok(Surface::from_raw(Arc::clone(&self.inner), raw_surface));
            }
        }

        return Err(ValidationError::NoWindowingEnabled.into());
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instance")
    }
}