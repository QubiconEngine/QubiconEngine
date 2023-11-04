use thiserror::Error;
use bitflags::bitflags;
use std::{
    sync::Arc,
    ops::Deref
};
use crate::{
    device::inner::DeviceInner,
    memory::alloc::{
        Allocator,
        device_memory::AllocatedMemory, error::AllocationError
    },
    instance::physical_device::memory_properties::MemoryTypeProperties
};
use ash::vk::{
    Image as VkImage,
    Extent3D as VkExtent3D,

    ImageType as VkImageType,
    ImageTiling as VkImageTiling,
    ImageLayout as VkImageLayout,

    ImageCreateInfo as VkImageCreateInfo,
    ImageUsageFlags as VkImageUsageFlags,
    ImageCreateFlags as VkImageCreateFlags,
    SampleCountFlags as VkSampleCountFlags
};

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ImageCreateFlags: u32 {
        const SPARSE_BINDING = 0b1;
        const SPARSE_RESIDENCY = 0b10;
        const SPARSE_ALIASED = 0b100;
        const MUTABLE_FORMAT = 0b1000;
        const CUBE_COMPATIBLE = 0b1_0000;
    }
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ImageUsageFlags: u32 {
        const TRANSFER_SRC = 0b1;
        const TRANSFER_DST = 0b10;
        const SAMPLED = 0b100;
        const STORAGE = 0b1000;
        const COLOR_ATTACHMENT = 0b1_0000;
        const DEPTH_STENCIL_ATTACHMENT = 0b10_0000;
        const TRANSIENT_ATTACHMENT = 0b100_0000;
        const INPUT_ATTACHMENT = 0b1000_0000;
    }
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ImageSampleCountFlags: u32 {
        const TYPE_1 = 0b1;
        const TYPE_2 = 0b10;
        const TYPE_4 = 0b100;
        const TYPE_8 = 0b1000;
        const TYPE_16 = 0b1_0000;
        const TYPE_32 = 0b10_0000;
        const TYPE_64 = 0b100_0000;
    }
}

impl Into<VkImageCreateFlags> for ImageCreateFlags {
    fn into(self) -> VkImageCreateFlags {
        VkImageCreateFlags::from_raw(self.bits())
    }
}

impl Into<VkImageUsageFlags> for ImageUsageFlags {
    fn into(self) -> VkImageUsageFlags {
        VkImageUsageFlags::from_raw(self.bits())
    }
}

impl Into<VkSampleCountFlags> for ImageSampleCountFlags {
    fn into(self) -> VkSampleCountFlags {
        VkSampleCountFlags::from_raw(self.bits())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageTiling {
    Linear,
    Optimal
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageType {
    Type1D { width: u32 },
    Type2D { width: u32, height: u32, miplevels_enabled: bool },
    Type3D { width: u32, height: u32, depth: u32 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageLayout {
    Undefined,
    General,
    ColorAttachmentOptimal,
    DepthStencilAttachmentOptimal,
    DepthStencilReadOnlyOptimal,
    ShaderReadOnlyOptimal,
    TransferSrcOptimal,
    TransferDstOptimal,
    Preinitialized,

    DepthAttachmentOptimal,
    StencilAttachmentOptimal,
    DepthReadOnlyOptimal,
    StencilReadOnlyOptimal
}

impl Into<VkImageTiling> for ImageTiling {
    fn into(self) -> VkImageTiling {
        match self {
            Self::Linear => VkImageTiling::LINEAR,
            Self::Optimal => VkImageTiling::OPTIMAL
        }
    }
}

impl Into<VkImageType> for ImageType {
    fn into(self) -> VkImageType {
        match self {
            Self::Type1D { .. } => VkImageType::TYPE_1D,
            Self::Type2D { .. } => VkImageType::TYPE_2D,
            Self::Type3D { .. } => VkImageType::TYPE_3D
        }
    }
}

impl Into<VkImageLayout> for ImageLayout {
    fn into(self) -> VkImageLayout {
        match self {
            Self::Undefined => VkImageLayout::UNDEFINED,
            Self::General => VkImageLayout::GENERAL,
            Self::ColorAttachmentOptimal => VkImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            Self::DepthStencilAttachmentOptimal => VkImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            Self::DepthStencilReadOnlyOptimal => VkImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            Self::ShaderReadOnlyOptimal => VkImageLayout::SHADER_READ_ONLY_OPTIMAL,
            Self::TransferSrcOptimal => VkImageLayout::TRANSFER_SRC_OPTIMAL,
            Self::TransferDstOptimal => VkImageLayout::TRANSFER_DST_OPTIMAL,
            Self::Preinitialized => VkImageLayout::PREINITIALIZED,

            Self::DepthAttachmentOptimal => VkImageLayout::DEPTH_ATTACHMENT_OPTIMAL,
            Self::StencilAttachmentOptimal => VkImageLayout::STENCIL_ATTACHMENT_OPTIMAL,
            Self::DepthReadOnlyOptimal => VkImageLayout::DEPTH_READ_ONLY_OPTIMAL,
            Self::StencilReadOnlyOptimal => VkImageLayout::STENCIL_READ_ONLY_OPTIMAL
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageCreateInfo {
    pub usage_flags: ImageUsageFlags,
    pub create_flags: ImageCreateFlags,
    pub sample_count_flags: ImageSampleCountFlags,

    pub initial_layout: ImageLayout,
    pub image_tiling: ImageTiling,
    pub image_type: ImageType,
    
    pub array_layers: u32,
    pub mipmaps_enabled: bool
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawImageCreationError {
    #[error("no more memory on host")]
    OutOfHostMemory,
    #[error("no more memory on device")]
    OutOfDeviceMemory,
    #[error("invalid opaque capture address")]
    InvalidOpaqueCaptureAddress
}

impl From<ash::vk::Result> for RawImageCreationError {
    fn from(value: ash::vk::Result) -> Self {
        match value {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => Self::OutOfHostMemory,
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => Self::OutOfDeviceMemory,
            ash::vk::Result::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => Self::InvalidOpaqueCaptureAddress,

            _ => unreachable!()
        }
    }
}

pub struct RawImage {
    pub(crate) device: Arc<DeviceInner>,
    pub(crate) image: VkImage,

    pub(crate) usage_flags: ImageUsageFlags,
    pub(crate) create_flags: ImageCreateFlags,
    pub(crate) sample_count_flags: ImageSampleCountFlags,

    pub(crate) initital_layout: ImageLayout,
    pub(crate) image_tiling: ImageTiling,
    pub(crate) image_type: ImageType,

    pub(crate) array_layers: u32,
    pub(crate) mip_levels: u32
}

impl RawImage {
    pub(crate) fn create(
        device: Arc<DeviceInner>,
        create_info: &ImageCreateInfo
    ) -> Result<Self, RawImageCreationError> {
        if !create_info.create_flags.difference(ImageCreateFlags::CUBE_COMPATIBLE).is_empty() {
            unimplemented!()
        }

        let extent = match create_info.image_type {
            ImageType::Type1D { width } => VkExtent3D { width, height: 1, depth: 1 },
            ImageType::Type2D { width, height, .. } => VkExtent3D { width, height, depth: 1 },
            ImageType::Type3D { width, height, depth } => VkExtent3D { width, height, depth }
        };
        let mip_levels = match create_info.image_type {
            ImageType::Type2D { width, height, miplevels_enabled } => if miplevels_enabled {
                calc_mip_levels_for_resolution(width, height)
            } else {
                1
            }
            _ => 1,
        };

        unsafe {
            let image = device.create_image(
                &VkImageCreateInfo {
                    flags: create_info.create_flags.into(),
                    usage: create_info.usage_flags.into(),
                    samples: create_info.sample_count_flags.into(),

                    initial_layout: create_info.initial_layout.into(),
                    tiling: create_info.image_tiling.into(),
                    image_type: create_info.image_type.into(),

                    array_layers: create_info.array_layers,
                    mip_levels,

                    //format: _,
                    extent,

                    ..Default::default()
                },
                None
            ).map_err(RawImageCreationError::from)?;

            Ok(
                Self {
                    device,
                    image,

                    usage_flags: create_info.usage_flags,
                    create_flags: create_info.create_flags,
                    sample_count_flags: create_info.sample_count_flags,

                    initital_layout: create_info.initial_layout,
                    image_tiling: create_info.image_tiling,
                    image_type: create_info.image_type,

                    array_layers: create_info.array_layers,
                    mip_levels
                }
            )
        }
    }
}

impl Drop for RawImage {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_image(
                self.image,
                None
            );
        }
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageCreationError {
    #[error("error during raw image creation")]
    RawImageError(#[from] RawImageCreationError),
    #[error("error during memory allocation")]
    AllocationError(#[from] AllocationError),
    #[error("device dont have memory type with given flags what support given image")]
    NoValidMemoryTypeFound,
    #[error("memory bind error")]
    MemoryBindError,
    #[error("device in raw image and allocator device dont match")]
    InvalidDevice
}

pub struct Image {
    pub(crate) raw: RawImage,
    pub(crate) allocator: Arc<Allocator>,
    pub(crate) memory: AllocatedMemory
}

impl Image {
    pub(crate) fn create_and_allocate(
        device: Arc<DeviceInner>,
        allocator: Arc<Allocator>,
        memory_properties: MemoryTypeProperties,
        create_info: ImageCreateInfo
    ) -> Result<Self, ImageCreationError> {
        Self::from_raw(
            RawImage::create(device, &create_info)?,
            allocator,
            memory_properties
        )
    }
    
    pub fn from_raw(
        raw: RawImage,
        allocator: Arc<Allocator>,
        memory_properties: MemoryTypeProperties
    ) -> Result<Self, ImageCreationError> {
        if allocator.get_device().ne(&raw.device) {
            return Result::Err(ImageCreationError::InvalidDevice)
        }

        unsafe {
            let requirements = raw.device.get_image_memory_requirements(raw.image);
            let memory_type_index = bitvec::array::BitArray::<u32, bitvec::order::Lsb0>::from(requirements.memory_type_bits)
                .into_iter()
                .enumerate()
                .filter(| (_, t) | *t)
                .map(| (i, _) | i)
                .filter(| i | raw.device.memory_properties.memory_types[*i].properties.contains(memory_properties))
                .map(| i | i as u32)
                .next()
                .ok_or(ImageCreationError::NoValidMemoryTypeFound)?;

            let memory = allocator.allocate(
                memory_type_index,
                requirements.size,
                requirements.alignment
            )?;

            raw.device.bind_image_memory(
                raw.image,
                memory.memory.device_memory,
                memory.offset
            ).map_err(|_| ImageCreationError::MemoryBindError)?;

            Ok(
                Self {
                    raw,
                    allocator,
                    memory
                }
            )
        }
    }
}

impl Deref for Image {
    type Target = RawImage;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}


// Helper function
pub(crate) fn calc_mip_levels_for_resolution(width: u32, height: u32) -> u32 {
    (width.max(height) as f32).log2().floor() as u32 + 1
}