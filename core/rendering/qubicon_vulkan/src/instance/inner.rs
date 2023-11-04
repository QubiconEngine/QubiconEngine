use std::ops::Deref;
use ash::{
    Entry,
    Instance,
    
    vk::InstanceCreateInfo
};

#[cfg(feature = "windowing")]
use ash::extensions::khr::Surface;

pub(crate) struct InstanceInner {
    pub(crate) _entry: Entry,
    pub(crate) instance: Instance,

    #[cfg(feature = "windowing")]
    pub(crate) surface: Option<Surface>
}

impl InstanceInner {
    // TODO: Use creation info
    pub(crate) fn load(_info: &super::creation_info::InstanceCreateInfo) -> Result<Self, super::error::InstanceError> {
        unsafe {
            let entry = Entry::load()?;
            let instance = entry.create_instance(
                &InstanceCreateInfo {
                    //p_application_info: (),
                    //enabled_layer_count: (),
                    //pp_enabled_layer_names: (),
                    //enabled_extension_count: (),
                    //pp_enabled_extension_names: (),

                    ..Default::default()
                },
                None
            )?;

            // TODO: Normal surface ext init
            #[cfg(feature = "windowing")]
            let surface = if info.enable_windowing {
                Some(Surface::new(&entry, &instance))
            } else {
                None
            };

            Ok(
                Self {
                    _entry: entry,
                    instance,

                    #[cfg(feature = "windowing")]
                    surface
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