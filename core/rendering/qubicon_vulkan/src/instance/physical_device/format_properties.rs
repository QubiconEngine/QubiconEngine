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
    linear_tiling: FormatFeatures,
    optimal_tiling: FormatFeatures,
    buffer_features: FormatFeatures
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