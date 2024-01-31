use bitflags::bitflags;
use thiserror::Error as ErrorDerive;
use super::{format::{Format, formats_representation::Format as FormatTrait}, image_view::{ImageView, ImageViewCreateInfo}, mapped_resource::{MappableType, MappedResource}, ResourceCreationError, ResourceMemory};
use std::{
    sync::Arc,
    error::Error as ErrorTrait
};
use crate::{
    Error,
    device::inner::DeviceInner,
    error::{VkError, ValidationError},
    instance::physical_device::memory_properties::MemoryTypeProperties,
    memory::alloc::{hollow_device_memory_allocator::HollowDeviceMemoryAllocator, AllocatedDeviceMemoryFragment, DeviceMemoryAllocator, MappableAllocatedDeviceMemoryFragment}
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

impl From<VkImageUsageFlags> for ImageUsageFlags {
    fn from(value: VkImageUsageFlags) -> Self {
        Self::from_bits_truncate(value.as_raw())
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
    pub mipmaps_enabled: bool,
    pub format: Format
}

pub(crate) struct ImageInner<A: DeviceMemoryAllocator> {
    pub(crate) device: Arc<DeviceInner>,
    
    pub(crate) image: VkImage,
    pub(crate) info: ImageCreateInfo,
    pub(crate) mip_levels: u32,

    // we shouldnt drop image if it is from swapchain
    drop_required: bool,
    memory: Option<ResourceMemory<A>>
}

impl<A: DeviceMemoryAllocator> Drop for ImageInner<A> {
    fn drop(&mut self) {
        core::mem::drop(self.memory.take());

        if self.drop_required {
            unsafe {
                self.device.destroy_image(
                    self.image,
                    None
                )
            }
        }
    }
}

pub struct RawImage {
    inner: Arc<ImageInner<HollowDeviceMemoryAllocator>>
}

impl RawImage {
    pub(crate) fn create(
        device: Arc<DeviceInner>,
        create_info: &ImageCreateInfo
    ) -> Result<Self, Error> {
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

                    format: create_info.format.into(),
                    extent,

                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            let inner = ImageInner {
                device,
                image,
                
                info: *create_info,
                mip_levels,

                drop_required: true,
                memory: None
            };

            Ok( Self { inner: Arc::new(inner) } )
        }
    }

    pub(crate) fn as_inner(&self) -> &Arc<ImageInner<HollowDeviceMemoryAllocator>> {
        &self.inner
    }

    // bruh
    pub fn create_info(&self) -> &ImageCreateInfo {
        &self.inner.info
    }

    pub fn usage_flags(&self) -> ImageUsageFlags {
        self.inner.info.usage_flags
    }

    pub fn create_flags(&self) -> ImageCreateFlags {
        self.inner.info.create_flags
    }

    pub fn sample_count_flags(&self) -> ImageSampleCountFlags {
        self.inner.info.sample_count_flags
    }

    pub fn tiling(&self) -> ImageTiling {
        self.inner.info.image_tiling
    }

    pub fn r#type(&self) -> ImageType {
        self.inner.info.image_type
    }

    pub fn array_layers_count(&self) -> u32 {
        self.inner.info.array_layers
    }

    pub fn format(&self) -> Format {
        self.inner.info.format
    }

    pub fn mip_levels_count(&self) -> u32 {
        self.inner.mip_levels
    }
}

pub struct Image<A: DeviceMemoryAllocator> {
    inner: Arc<ImageInner<A>>
}

impl<A: DeviceMemoryAllocator> Image<A> {
    pub(crate) fn create_and_allocate(
        device: Arc<DeviceInner>,
        allocator: Arc<A>,
        memory_properties: MemoryTypeProperties,
        create_info: ImageCreateInfo
    ) -> Result<Self, ResourceCreationError<A::AllocError>> {
        Self::from_raw(
            RawImage::create(device, &create_info).map_err(ResourceCreationError::from_creation_error)?,
            allocator,
            memory_properties
        )
    }
    
    pub fn from_raw(
        raw: RawImage,
        allocator: Arc<A>,
        memory_properties: MemoryTypeProperties
    ) -> Result<Self, ResourceCreationError<A::AllocError>> {
        unsafe {
            let inner = Arc::into_inner(raw.inner)
                .expect("image is in use");

            let requirements = inner.device.get_image_memory_requirements(inner.image);
            let memory_type_index = bitvec::array::BitArray::<u32, bitvec::order::Lsb0>::from(requirements.memory_type_bits)
                .into_iter()
                .enumerate()
                .filter(| (_, t) | *t)
                .map(| (i, _) | i)
                .filter(| i | inner.device.memory_properties.memory_types[*i].properties.contains(memory_properties))
                .map(| i | i as u8)
                .next()
                .ok_or(ValidationError::NoValidMemoryTypeFound.into())
                .map_err(ResourceCreationError::from_creation_error)?;

            let memory = allocator.alloc(
                memory_type_index,
                requirements.size,
                requirements.alignment
            ).map_err(ResourceCreationError::from_allocation_error)?;

            let (raw_memory, offset) = memory.as_memory_object_and_offset();

            if raw_memory.device != inner.device {
                return Err(ResourceCreationError::from_creation_error(ValidationError::InvalidDevice.into()));
            }

            inner.device.bind_image_memory(
                inner.image,
                raw_memory.device_memory,
                offset
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked().into())
             .map_err(ResourceCreationError::from_creation_error)?;

            // there should be a better way
            let inner = ImageInner {
                device: inner.device,
                image: inner.image,
                info: inner.info,
                mip_levels: inner.mip_levels,
                drop_required: inner.drop_required,
                memory: Some( ResourceMemory::new(allocator, memory) )
            };

            return Ok( Self { inner: Arc::new(inner) } );
        }
    }

    pub(crate) fn as_inner(&self) -> &Arc<ImageInner<A>> {
        &self.inner
    }

    /// # Safety
    /// * Format size of image view should match format size of original image
    /// * If view type is Cube, then original image should be created with cube compatiple flag
    pub unsafe fn create_image_view_unchecked(
        &self,
        create_info: &ImageViewCreateInfo
    ) -> Result<Arc<ImageView<A>>, Error> {
        ImageView::create_unchecked(&self, create_info)
    }

    // bruh times two
    pub fn create_info(&self) -> &ImageCreateInfo {
        &self.inner.info
    }

    pub fn usage_flags(&self) -> ImageUsageFlags {
        self.inner.info.usage_flags
    }

    pub fn create_flags(&self) -> ImageCreateFlags {
        self.inner.info.create_flags
    }

    pub fn sample_count_flags(&self) -> ImageSampleCountFlags {
        self.inner.info.sample_count_flags
    }

    pub fn tiling(&self) -> ImageTiling {
        self.inner.info.image_tiling
    }

    pub fn r#type(&self) -> ImageType {
        self.inner.info.image_type
    }

    pub fn array_layers_count(&self) -> u32 {
        self.inner.info.array_layers
    }

    pub fn format(&self) -> Format {
        self.inner.info.format
    }

    pub fn mip_levels_count(&self) -> u32 {
        self.inner.mip_levels
    }
}

impl<'a, A: DeviceMemoryAllocator> Image<A>
    where A::MemoryFragmentType: MappableAllocatedDeviceMemoryFragment<'a>
{
    pub fn map<T: MappableType + FormatTrait>(&'a self) ->
        Result<MappedResource<'a, T, A>, ImageMapError<<A::MemoryFragmentType as MappableAllocatedDeviceMemoryFragment<'a>>::MapError>>
    {
        if T::FORMAT_ENUM != self.inner.info.format {
            Err(ImageMapError::FormatMismatch)?
        }
        if self.tiling() != ImageTiling::Linear {
            Err(ImageMapError::NotLinearLayout)?
        }

        let len = match self.inner.info.image_type {
            ImageType::Type1D { width } => width as usize,
            ImageType::Type2D { width, height, .. } => width as usize * height as usize,
            ImageType::Type3D { width, height, depth } => width as usize * height as usize * depth as usize
        };

        unsafe {
            Ok(
                MappedResource::new(
                    self.inner.memory.unwrap_unchecked().map()?,
                    len
                )
            )
        }
    }
}

#[derive(ErrorDerive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageMapError<E: ErrorTrait> {
    #[error("image format dont match required map format")]
    FormatMismatch,
    #[error("image is not in linear layout")]
    NotLinearLayout,
    #[error("error ocurred during memory mapping")]
    MappingError(#[from] E)
}


// Helper function
#[inline]
pub(crate) fn calc_mip_levels_for_resolution(width: u32, height: u32) -> u32 {
    (width.max(height) as f32).log2().floor() as u32 + 1
}