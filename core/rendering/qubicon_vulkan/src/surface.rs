use std::sync::Arc;
use bitflags::bitflags;
use ash::vk::{
    SurfaceKHR as VkSurface,
    ColorSpaceKHR as VkColorSpace,
    PresentModeKHR as VkPresentMode,
    SurfaceFormatKHR as VkSurfaceFormat,
    SurfaceCapabilitiesKHR as VkSurfaceCapabilities,
    CompositeAlphaFlagsKHR as VkCompositeAlphaFlags,
    SurfaceTransformFlagsKHR as VkSurfaceTransformFlags
};

use crate::{instance::{inner::InstanceInner, physical_device::PhysicalDevice}, memory::resources::{image::ImageUsageFlags, format::Format}, Error, error::{VkError, ValidationError}};

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SurfaceTransformFlags: u32 {
        const IDENTITY = 0b1;
        const ROTATE_90 = 0b10;
        const ROTATE_180 = 0b100;
        const ROTATE_270 = 0b1000;
        const HORIZONTAL_MIRROR = 0b1_0000;
        const HORIZONTAL_MIRROR_ROTATE_90 = 0b10_0000;
        const HORIZONTAL_MIRROR_ROTATE_180 = 0b100_0000;
        const HORIZONTAL_MIRROR_ROTATE_270 = 0b1000_0000;
        const INHERIT = 0b1_0000_0000;
    }

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CompositeAlphaFlags: u8 {
        const OPAQUE = 0b1;
        const PRE_MULTIPLIED = 0b10;
        const POST_MULTIPLIED = 0b100;
        const INHERIT = 0b1000;
    }
}

impl From<VkSurfaceTransformFlags> for SurfaceTransformFlags {
    fn from(value: VkSurfaceTransformFlags) -> Self {
        Self::from_bits_truncate(value.as_raw())
    }
}
impl Into<VkSurfaceTransformFlags> for SurfaceTransformFlags {
    fn into(self) -> VkSurfaceTransformFlags {
        VkSurfaceTransformFlags::from_raw(self.bits())
    }
}
impl From<VkCompositeAlphaFlags> for CompositeAlphaFlags {
    fn from(value: VkCompositeAlphaFlags) -> Self {
        Self::from_bits_truncate(value.as_raw() as u8)
    }
} 
impl Into<VkCompositeAlphaFlags> for CompositeAlphaFlags {
    fn into(self) -> VkCompositeAlphaFlags {
        VkCompositeAlphaFlags::from_raw(self.bits() as u32)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSpace {
    #[default]
    SRGB_Nonlinear,

    // TODO: Add more formats with VK_EXT_swapchain_colorspace
}

impl From<VkColorSpace> for ColorSpace {
    fn from(value: VkColorSpace) -> Self {
        match value {
            VkColorSpace::SRGB_NONLINEAR => Self::SRGB_Nonlinear,

            _ => unreachable!()
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceFormat {
    pub format: Format,
    pub color_space: ColorSpace
}

impl From<VkSurfaceFormat> for SurfaceFormat {
    fn from(value: VkSurfaceFormat) -> Self {
        Self {
            format: value.format.into(),
            color_space: value.color_space.into()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentMode {
    Immediate,
    Mailbox,
    FIFO,
    FIFORelaxed,

    // There actualy more modes in extensions
}

impl From<VkPresentMode> for PresentMode {
    fn from(value: VkPresentMode) -> Self {
        match value {
            VkPresentMode::IMMEDIATE => Self::Immediate,
            VkPresentMode::MAILBOX => Self::Mailbox,
            VkPresentMode::FIFO => Self::FIFO,
            VkPresentMode::FIFO_RELAXED => Self::FIFORelaxed,

            _ => unreachable!()
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceSurfaceCapabilities {
    pub min_image_count: u32,
    pub max_image_count: u32,

    /// 0 is width, 1 is height
    pub current_extent: (u32, u32),
    /// 0 is width, 1 is height
    pub min_image_extent: (u32, u32),
    /// 0 is width, 1 is height
    pub max_image_extent: (u32, u32),

    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlags,
    pub current_transform: SurfaceTransformFlags,
    pub supported_composite_alpha: CompositeAlphaFlags,
    pub supported_usage_flags: ImageUsageFlags
}

impl From<VkSurfaceCapabilities> for PhysicalDeviceSurfaceCapabilities {
    fn from(value: VkSurfaceCapabilities) -> Self {
        Self {
            min_image_count: value.min_image_count,
            max_image_count: value.max_image_count,

            current_extent: (value.current_extent.width, value.current_extent.height),
            min_image_extent: (value.min_image_extent.width, value.min_image_extent.height),
            max_image_extent: (value.max_image_extent.width, value.max_image_extent.height),

            max_image_array_layers: value.max_image_array_layers,
            supported_transforms: value.supported_transforms.into(),
            current_transform: value.current_transform.into(),
            supported_composite_alpha: value.supported_composite_alpha.into(),
            supported_usage_flags: value.supported_usage_flags.into()
        }
    }
}

pub struct Surface {
    instance: Arc<InstanceInner>,
    surface: VkSurface
}

impl Surface {
    pub fn get_physical_device_surface_capabilities(&self, device: &PhysicalDevice) -> Result<PhysicalDeviceSurfaceCapabilities, Error> {
        unsafe {
            self.instance.surface.as_ref().unwrap_unchecked().get_physical_device_surface_capabilities(device.dev, self.surface)
            .map_err(| e | VkError::try_from(e).unwrap_unchecked().into())
            .map(PhysicalDeviceSurfaceCapabilities::from)
        }
    }

    pub fn get_physical_device_surface_formats(&self, device: &PhysicalDevice) -> Result<Vec<SurfaceFormat>, Error> {
        unsafe {
            self.instance.surface.as_ref().unwrap_unchecked().get_physical_device_surface_formats(device.dev, self.surface)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked().into())
                .map(| v | 
                    v.into_iter()
                        .map(Into::into)
                        .collect()
                )
        }
    }

    pub fn get_physical_device_surface_present_modes(&self, device: &PhysicalDevice) -> Result<Vec<PresentMode>, Error> {
        unsafe {
            self.instance.surface.as_ref().unwrap_unchecked().get_physical_device_surface_present_modes(device.dev, self.surface)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked().into())
                .map(| v |
                    v.into_iter()
                        .map(Into::into)
                        .collect()
                )
        }
    }

    pub fn get_physical_device_surface_support(&self, device: &PhysicalDevice, queue_family_index: u32) -> Result<bool, Error> {
        if queue_family_index >= device.get_queue_family_infos().len() as u32 {
            return Err(ValidationError::InvalidQueueFamilyIndex.into());
        }

        unsafe {
            self.instance.surface.as_ref().unwrap_unchecked().get_physical_device_surface_support(
                device.dev,
                queue_family_index,
                self.surface
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked().into())
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.instance.surface.as_ref().unwrap_unchecked().destroy_surface(
                self.surface,
                None
            )
        }
    }
}