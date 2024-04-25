use super::DeviceSize;
use bitflags::bitflags;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FormatFeatures: u32 {
        const SAMPLED_IMAGE = 0b1;
        const STORAGE_IMAGE = 0b10;
        const STORAGE_IMAGE_ATOMIC = 0b100;
        const UNIFORM_TEXEL_BUFFER = 0b1000;
        const STORAGE_TEXEL_BUFFER = 0b1_0000;
        const STORAGE_TEXEL_BUFFER_ATOMIC = 0b10_0000;
        const VERTEX_BUFFER = 0b100_0000;
        const COLOR_ATTACHMENT = 0b1000_0000;
        const COLOR_ATTACHMENT_BLEND = 0b1_0000_0000;
        const DEPTH_STENCIL_ATTACHMENT = 0b10_0000_0000;
        const BLIT_SRC = 0b100_0000_0000;
        const BLIT_DST = 0b1000_0000_0000;
        const SAMPLED_IMAGE_FILTER_LINEAR = 0b1_0000_0000_0000;

        // from Vulkan 1.1
        const TRANSFER_SRC = 0x00004000;
        const TRANSFER_DST = 0x00008000;

        // There are more of them
    }
}

impl From<ash::vk::FormatFeatureFlags> for FormatFeatures {
    fn from(value: ash::vk::FormatFeatureFlags) -> Self {
        Self ( value.as_raw().into() )
    }
}


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormatProperties {
    pub linear_tiling: FormatFeatures,
    pub optimal_tiling: FormatFeatures,
    pub buffer_features: FormatFeatures
}

impl From<ash::vk::FormatProperties> for FormatProperties {
    fn from(value: ash::vk::FormatProperties) -> Self {
        Self {
            linear_tiling: value.linear_tiling_features.into(),
            optimal_tiling: value.optimal_tiling_features.into(),
            buffer_features: value.buffer_features.into()
        }
    }
}


use crate::resources::image::{ Extent3D, ImageSampleCountFlags };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: ImageSampleCountFlags,
    pub max_resource_size: DeviceSize
}

impl From<ash::vk::ImageFormatProperties> for ImageFormatProperties {
    fn from(value: ash::vk::ImageFormatProperties) -> Self {
        Self {
            max_extent: value.max_extent.into(),
            max_mip_levels: value.max_mip_levels,
            max_array_layers: value.max_array_layers,
            sample_counts: value.sample_counts.into(),
            max_resource_size: value.max_resource_size
        }
    }
}

// TODO: SparseImageFormatProperties