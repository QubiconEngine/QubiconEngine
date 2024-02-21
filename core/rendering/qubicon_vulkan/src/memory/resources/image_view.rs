use std::{
    sync::Arc,
    ops::Range
};
use bitflags::bitflags;
use ash::vk::{
    ImageView as VkImageView,
    ImageViewType as VkImageViewType,
    ImageAspectFlags as VkImageAspectFlags,
    ComponentMapping as VkComponentMapping,
    ComponentSwizzle as VkComponentSwizzle,
    ImageViewCreateInfo as VkImageViewCreateInfo,
    ImageSubresourceRange as VkImageSubresourceRange,
    ImageSubresourceLayers as VkImageSubresourceLayers
};

use crate::{
    memory::alloc::DeviceMemoryAllocator,
    Error,
    error::VkError,
};
use super::{image::{Image, ImageInner}, format::Format};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ImageAspect: u8 {
        const COLOR = 0b1;
        const DEPTH = 0b10;
        const STENCIL = 0b100;
        const METADATA = 0b1000;
    }
}
impl Default for ImageAspect {
    fn default() -> Self {
        Self::COLOR
    }
}
impl From<VkImageAspectFlags> for ImageAspect {
    fn from(value: VkImageAspectFlags) -> Self {
        Self::from_bits_truncate(value.as_raw() as u8)
    }
}
impl Into<VkImageAspectFlags> for ImageAspect {
    fn into(self) -> VkImageAspectFlags {
        VkImageAspectFlags::from_raw(self.bits() as u32)
    }
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentSwizzle {
    #[default]
    Identity,
    Zero,
    One,
    R,
    G,
    B
}
impl From<VkComponentSwizzle> for ComponentSwizzle {
    fn from(value: VkComponentSwizzle) -> Self {
        match value {
            VkComponentSwizzle::IDENTITY => Self::Identity,
            VkComponentSwizzle::ZERO => Self::Zero,
            VkComponentSwizzle::ONE => Self::One,
            VkComponentSwizzle::R => Self::R,
            VkComponentSwizzle::G => Self::G,
            VkComponentSwizzle::B => Self::B,

            _ => unreachable!()
        }
    }
}
impl Into<VkComponentSwizzle> for ComponentSwizzle {
    fn into(self) -> VkComponentSwizzle {
        match self {
            Self::Identity => VkComponentSwizzle::IDENTITY,
            Self::Zero => VkComponentSwizzle::ZERO,
            Self::One => VkComponentSwizzle::ONE,
            Self::R => VkComponentSwizzle::R,
            Self::G => VkComponentSwizzle::G,
            Self::B => VkComponentSwizzle::B
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageViewType {
    Type1D,
    #[default]
    Type2D,
    Type3D,
    Cube,
    Type1DArray,
    Type2DArray,
    CubeArray
}
impl Into<VkImageViewType> for ImageViewType {
    fn into(self) -> VkImageViewType {
        let desc: u8 = unsafe { core::mem::transmute(self) };

        VkImageViewType::from_raw(desc as i32)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle
}
impl From<VkComponentMapping> for ComponentMapping {
    fn from(value: VkComponentMapping) -> Self {
        Self {
            r: value.r.into(),
            g: value.g.into(),
            b: value.b.into(),
            a: value.a.into()
        }
    }
}
impl Into<VkComponentMapping> for ComponentMapping {
    fn into(self) -> VkComponentMapping {
        VkComponentMapping {
            r: self.r.into(),
            g: self.g.into(),
            b: self.b.into(),
            a: self.a.into()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspect,
    pub mip_levels: Range<u32>,
    pub array_layers: Range<u32>
}
impl Default for ImageSubresourceRange {
    #[inline]
    fn default() -> Self {
        Self {
            aspect_mask: Default::default(),
            mip_levels: 0..1,
            array_layers: 0..1
        }
    }
}
impl Into<VkImageSubresourceRange> for ImageSubresourceRange {
    fn into(self) -> VkImageSubresourceRange {
        VkImageSubresourceRange {
            aspect_mask: self.aspect_mask.into(),
            base_mip_level: self.mip_levels.start,
            level_count: self.mip_levels.count() as u32,
            base_array_layer: self.array_layers.start,
            layer_count: self.array_layers.count() as u32
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageSubresourceLayers {
    aspect_mask: ImageAspect,
    mip_level: u32,
    array_layers: Range<u32>
}
impl Default for ImageSubresourceLayers {
    #[inline]
    fn default() -> Self {
        Self {
            aspect_mask: Default::default(),
            mip_level: 0,
            array_layers: 0..1
        }
    }
}
impl Into<VkImageSubresourceLayers> for ImageSubresourceLayers {
    fn into(self) -> VkImageSubresourceLayers {
        VkImageSubresourceLayers {
            aspect_mask: self.aspect_mask.into(),
            mip_level: self.mip_level,
            base_array_layer: self.array_layers.start,
            layer_count: self.array_layers.count() as u32
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ImageViewCreateInfo {
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange
}

pub struct ImageView<A: DeviceMemoryAllocator> {
    image: Arc<ImageInner<A>>,
    image_view: VkImageView,

    create_info: ImageViewCreateInfo
}

impl<A: DeviceMemoryAllocator> ImageView<A> {
    pub(crate) unsafe fn as_raw(&self) -> VkImageView {
        self.image_view
    }

    /// # Safety
    /// * Format size of image view should match format size of original image
    /// * If view type is Cube, then original image should be created with cube compatiple flag
    pub(crate) unsafe fn create_unchecked(
        image: &Image<A>,
        create_info: &ImageViewCreateInfo
    ) -> Result<Arc<Self>, Error> {
        let image = Arc::clone(image.as_inner());
        let create_info = create_info.clone();

        let image_view = image.device.create_image_view(
            &VkImageViewCreateInfo {
                image: image.image,
                view_type: create_info.view_type.into(),
                format: create_info.format.into(),
                components: create_info.components.into(),
                subresource_range: create_info.subresource_range.clone().into(),

                ..Default::default()
            },
            None
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        Ok(
            Arc::new(
                Self {
                    image,
                    image_view,
                    create_info
                }
            )
        )
    }

    pub fn view_type(&self) -> ImageViewType {
        self.create_info.view_type
    }

    pub fn format(&self) -> Format {
        self.create_info.format
    }

    pub fn components(&self) -> ComponentMapping {
        self.create_info.components
    }

    pub fn subresource_range(&self) -> ImageSubresourceRange {
        self.create_info.subresource_range.clone()
    }
}

impl<A: DeviceMemoryAllocator> Drop for ImageView<A> {
    fn drop(&mut self) {
        unsafe {
            self.image.device.destroy_image_view(
                self.image_view,
                None
            )
        }
    }
}