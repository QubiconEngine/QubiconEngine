#[cfg(feature = "x11")]
pub mod x11;




#[cfg(feature = "vulkan")]
use qubicon_vulkan::{memory::resources::image::ImageUsageFlags, surface::{CompositeAlphaFlags, SurfaceTransformFlags}};

#[cfg(feature = "vulkan")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssociatedSwapchainCreateInfo {
    pub min_image_count: u32,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub pre_transform: SurfaceTransformFlags,
    pub composite_alpha: CompositeAlphaFlags,
    pub clipped: bool
}