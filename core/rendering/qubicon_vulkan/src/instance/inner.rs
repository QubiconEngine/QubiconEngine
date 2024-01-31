use std::ops::Deref;
use arrayvec::ArrayVec;
use ash::{
    Entry,
    Instance,
    
    vk::InstanceCreateInfo
};
use crate::{
    Error,
    error::VkError
};

#[cfg(feature = "windowing")]
use ash::extensions::khr::Surface;
#[cfg(feature = "x11")]
use ash::extensions::khr::XlibSurface;

pub(crate) struct InstanceInner {
    pub(crate) _entry: Entry,
    pub(crate) instance: Instance,

    #[cfg(feature = "windowing")]
    pub(crate) surface: Option<Surface>,
    #[cfg(feature = "x11")]
    pub(crate) x_surface: Option<XlibSurface>
}

const _TMP_LAYER: &str = "VK_LAYER_KHRONOS_validation\0";

impl InstanceInner {
    // TODO: Use creation info
    pub(crate) fn create(info: &super::creation_info::InstanceCreateInfo) -> Result<Self, super::error::InstanceError> {
        let mut enabled_extensions: ArrayVec<*const u8, 5> = ArrayVec::new();
        
        #[cfg(feature = "windowing")]
        if info.enable_windowing {
            enabled_extensions.push("VK_KHR_surface\0".as_ptr());

            #[cfg(feature = "x11")]
            enabled_extensions.push("VK_KHR_xlib_surface\0".as_ptr());
        }

        unsafe {
            let entry = Entry::load()?;
            let instance = entry.create_instance(
                &InstanceCreateInfo {
                    //p_application_info: (),
                    enabled_layer_count: 1,
                    pp_enabled_layer_names: core::mem::transmute(&_TMP_LAYER),
                    enabled_extension_count: enabled_extensions.len() as u32,
                    pp_enabled_extension_names: enabled_extensions.as_ptr().cast(),

                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())
             .map_err(Error::from)?;

            // TODO: Normal surface ext init
            #[cfg(feature = "windowing")]
            let surface = info.enable_windowing
                .then(|| Surface::new(&entry, &instance));

            #[cfg(feature = "x11")]
            let x_surface = info.enable_windowing
                .then(|| XlibSurface::new(&entry, &instance));

            Ok(
                Self {
                    _entry: entry,
                    instance,

                    #[cfg(feature = "windowing")]
                    surface,
                    #[cfg(feature = "x11")]
                    x_surface
                }
            )
        }
    }
}

impl PartialEq for InstanceInner {
    fn eq(&self, other: &Self) -> bool {
        self.instance.handle() == other.instance.handle()
    }
}

impl Deref for InstanceInner {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl Drop for InstanceInner {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}