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

pub mod create_info;
pub mod physical_device;

pub struct Instance {
    _entry: ash::Entry,
    instance: ash::Instance
}

impl Instance {
    pub(crate) unsafe fn as_raw(&self) -> &ash::Instance {
        &self.instance
    }

    pub fn new(create_info: &create_info::InstanceCreateInfo) -> Arc<Self> {
        let (_entry, instance) = unsafe {
            let entry = ash::Entry::load().unwrap(); // TODO: Error handling

            let app_info = ash::vk::ApplicationInfo::builder()
                .api_version(create_info.app_id.vulkan_version.into())
                .engine_version(create_info.app_id.engine_version.into())
                .application_version(create_info.app_id.app_version.into());

            let create_info = ash::vk::InstanceCreateInfo::builder()
                //.application_info(application_info)
                //.enabled_layer_names(enabled_layer_names)
                //.enabled_extension_names(enabled_extension_names)
                //.flags(flags)
                .build();

            let instance = entry.create_instance(&create_info, None);

            (entry, instance)
        };

        Arc::new(
            Self {
                _entry,
                instance: instance.unwrap() // TODO: Error handling
            }
        )
    }

    // TODO: Change error type
    pub fn enumerate_devices(self: &Arc<Self>) -> ash::prelude::VkResult<impl Iterator<Item = PhysicalDevice>> {
        let self_ = Arc::clone(self);
        let iter = unsafe { self.instance.enumerate_physical_devices()? }
            .into_iter()
            .map(move | dev | unsafe {
                PhysicalDevice::from_instance_and_raw_physical_device(
                    Arc::clone(&self_),
                    dev
                )
            });

        Ok(iter)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) }
    }
}

// impl Instance {
//     #[cfg(feature = "x11")]
//     /// # Safety
//     /// *display* and *window* must be valid X objects
//     pub unsafe fn create_surface_x11(&self, display: *mut x11::xlib::Display, window: x11::xlib::Window) -> Result<Surface, Error> {
//         use ash::vk::XlibSurfaceCreateInfoKHR;

//         if let Some(x_surface_ext_calls) = self.inner.x_surface.as_ref() {
//             unsafe {
//                 let raw_surface = x_surface_ext_calls.create_xlib_surface(
//                     &XlibSurfaceCreateInfoKHR {
//                         dpy: display.cast(),
//                         window,

//                         ..Default::default()
//                     },
//                     None
//                 ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

//                 return Ok(Surface::from_raw(Arc::clone(&self.inner), raw_surface));
//             }
//         }

//         return Err(ValidationError::NoWindowingEnabled.into());
//     }
// }

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instance")
    }
}