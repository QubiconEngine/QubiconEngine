pub use error::Error;
pub use instance::Instance;

pub mod queue;
pub mod device;
pub mod memory;
pub mod descriptors;
pub mod instance;
pub mod commands;
pub mod shaders;
pub mod error;
pub mod sync;

#[cfg(feature = "windowing")]
pub mod swapchain;
#[cfg(feature = "windowing")]
pub mod surface;


trait Validate {
    fn validate(&self);
}