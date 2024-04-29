use std::sync::Arc;
use core::num::NonZeroU32;
use bitflags::bitflags;

use super::{ MemoryRequirements, AllocHandle, format::Format };
use crate::{ error::VkError, device::Device, instance::physical_device::PhysicalDevice, memory::alloc::{ Allocator, Allocation } };

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

impl From<ImageCreateFlags> for ash::vk::ImageCreateFlags {
    fn from(value: ImageCreateFlags) -> Self {
        Self::from_raw(value.bits())
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

impl From<ash::vk::ImageUsageFlags> for ImageUsageFlags {
    fn from(value: ash::vk::ImageUsageFlags) -> Self {
        Self::from_bits_truncate(value.as_raw())
    }
}

impl From<ImageUsageFlags> for ash::vk::ImageUsageFlags {
    fn from(value: ImageUsageFlags) -> Self {
        Self::from_raw(value.bits())
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

impl From<ImageSampleCountFlags> for ash::vk::SampleCountFlags {
    fn from(value: ImageSampleCountFlags) -> Self {
        Self::from_raw(value.bits())
    }
}

impl From<ash::vk::SampleCountFlags> for ImageSampleCountFlags {
    fn from(value: ash::vk::SampleCountFlags) -> Self {
        Self ( value.as_raw().into() )
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extent2D {
    pub width: NonZeroU32,
    pub height: NonZeroU32
}

impl From<Extent2D> for ash::vk::Extent2D {
    fn from(value: Extent2D) -> Self {
        Self::builder()
            .width(value.width.get())
            .height(value.height.get())
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extent3D {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
    pub depth: NonZeroU32
}

impl From<Extent3D> for ash::vk::Extent3D {
    fn from(value: Extent3D) -> Self {
        Self::builder()
            .width(value.width.get())
            .height(value.height.get())
            .depth(value.depth.get())
            .build()
    }
}

impl TryFrom<ash::vk::Extent3D> for Extent3D {
    type Error = VkError;
    
    fn try_from(value: ash::vk::Extent3D) -> Result<Self, Self::Error> {
        let result = Self {
            width: NonZeroU32::new(value.width).ok_or(VkError::FormatNotSupported)?,
            height: NonZeroU32::new(value.height).ok_or(VkError::FormatNotSupported)?,
            depth: NonZeroU32::new(value.depth).ok_or(VkError::FormatNotSupported)?
        };

        Ok( result )
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageTiling {
    Optimal,
    Linear
}

impl From<ImageTiling> for ash::vk::ImageTiling {
    fn from(value: ImageTiling) -> Self {
        match value {
            ImageTiling::Optimal => Self::OPTIMAL,
            ImageTiling::Linear => Self::LINEAR
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageType {
    Type1D,
    Type2D,
    Type3D
}

impl From<ImageType> for ash::vk::ImageType {
    fn from(value: ImageType) -> Self {
        match value {
            ImageType::Type1D => Self::TYPE_1D,
            ImageType::Type2D => Self::TYPE_2D,
            ImageType::Type3D => Self::TYPE_3D
        }
    }
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
    StencilReadOnlyOptimal,

    #[cfg(feature = "windowing")]
    PresentSrc
}

impl From<ImageLayout> for ash::vk::ImageLayout {
    fn from(value: ImageLayout) -> Self {
        match value { // :p
            ImageLayout::Undefined                     => Self::UNDEFINED,
            ImageLayout::General                       => Self::GENERAL,
            ImageLayout::ColorAttachmentOptimal        => Self::COLOR_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthStencilAttachmentOptimal => Self::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthStencilReadOnlyOptimal   => Self::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            ImageLayout::ShaderReadOnlyOptimal         => Self::SHADER_READ_ONLY_OPTIMAL,
            ImageLayout::TransferSrcOptimal            => Self::TRANSFER_SRC_OPTIMAL,
            ImageLayout::TransferDstOptimal            => Self::TRANSFER_DST_OPTIMAL,
            ImageLayout::Preinitialized                => Self::PREINITIALIZED,
            ImageLayout::DepthAttachmentOptimal        => Self::DEPTH_ATTACHMENT_OPTIMAL,
            ImageLayout::StencilAttachmentOptimal      => Self::STENCIL_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthReadOnlyOptimal          => Self::DEPTH_READ_ONLY_OPTIMAL,
            ImageLayout::StencilReadOnlyOptimal        => Self::STENCIL_READ_ONLY_OPTIMAL,

            #[cfg(feature = "windowing")]
            ImageLayout::PresentSrc                    => Self::PRESENT_SRC_KHR
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageCreateInfo {
    pub usage_flags: ImageUsageFlags,
    // TODO: create_flags,
    pub sample_count: ImageSampleCountFlags,

    pub initial_layout: ImageLayout,
    pub tiling: ImageTiling,
    pub ty: ImageType,

    pub extent: Extent3D,
    
    pub array_layers: NonZeroU32,
    pub mip_levels: NonZeroU32,
    pub format: Format,

    // TODO: Sharing mode and synchronization
}

impl ImageCreateInfo {
    pub fn validate(&self, device: &PhysicalDevice) -> Result<(), VkError> {
        // TODO: Sharing mode checks
        let limits = &device.properties().limits;

        if self.array_layers.get() > limits.max_image_array_layers {
            panic!("too much array layers! Requested {}, but max is {}", self.array_layers, limits.max_image_array_layers);
        }


        if self.ty == ImageType::Type1D && self.extent.height.get() > 1 && self.extent.depth.get() > 1 {
            panic!("1D images cant have height or depth greater than 1");
        }

        if self.ty == ImageType::Type2D && self.extent.depth.get() > 1 {
            panic!("2D images cant have depth greater than 1");
        }



        let format_properties = device.image_format_properties(self.format, self.ty, self.tiling, self.usage_flags)?;

        if !format_properties.sample_counts.contains(self.sample_count) {
            panic!("unsupported samples count");
        }
        

        if self.mip_levels > format_properties.max_mip_levels {
            panic!("{} miplevels, but max is {}", self.mip_levels, format_properties.max_mip_levels);
        }

        if self.array_layers > format_properties.max_array_layers {
            panic!("{} array layers, but max is {}", self.array_layers, format_properties.max_array_layers);
        }


        if self.extent.width > format_properties.max_extent.width {
            panic!("width({}) is greater than max width({})", self.extent.width, format_properties.max_extent.width);
        }

        if self.extent.height > format_properties.max_extent.height {
            panic!("height({}) is greater than max height({})", self.extent.height, format_properties.max_extent.height);
        }

        if self.extent.depth > format_properties.max_extent.depth {
            panic!("depth({}) is greater tham max depth({})", self.extent.depth, format_properties.max_extent.depth);
        }


        if self.mip_levels.get() != 1 && self.mip_levels != mip_levels_for_extent(self.extent) {
            panic!("invalid mip leves count");
        }

        Ok(())
    }
}

impl From<ImageCreateInfo> for ash::vk::ImageCreateInfo {
    fn from(value: ImageCreateInfo) -> Self {
        Self::builder()
            .array_layers(value.array_layers.get())
            .extent(value.extent.into())
            //.flags()
            .format(value.format.into())
            .image_type(value.ty.into())
            .initial_layout(value.initial_layout.into())
            .mip_levels(value.mip_levels.get())
            //.sharing_mode(sharing_mode)
            //.queue_family_indices(queue_family_indices)
            .samples(value.sample_count.into())
            .tiling(value.tiling.into())
            .usage(value.usage_flags.into())
            .build()
    }
}

// Idk if mipmaps can actually be less than value, calculated by max(d1, d2, d3).log2().floor().
// I will find this out later
pub fn mip_levels_for_extent(extent: Extent3D) -> NonZeroU32 {
    let max_dimension = extent.width.max(extent.height).max(extent.depth).get() as f32;
    let mip_levels = max_dimension.log2().floor() as u32 + 1;

    unsafe { NonZeroU32::new_unchecked(mip_levels) }
}


pub struct UnbindedImage {
    device: Arc<Device>,


    usage: ImageUsageFlags,
    samples: ImageSampleCountFlags,
    
    layout: ImageLayout,
    tiling: ImageTiling,
    ty: ImageType,

    extent: Extent3D,
    array_layers: NonZeroU32,
    mip_levels: NonZeroU32,

    format: Format,


    image: ash::vk::Image
}

impl Drop for UnbindedImage {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().destroy_image(self.image, None) }
    }
}

impl UnbindedImage {
    pub(crate) unsafe fn as_raw(&self) -> ash::vk::Image {
        self.image
    }

    pub fn new(device: Arc<Device>, create_info: &ImageCreateInfo) -> Result<Self, VkError> {
        create_info.validate(device.physical_device())?;

        let image = unsafe {
            device.as_raw().create_image(&(*create_info).into(), None)
        }?;
        
        let result = Self {
            device,

            usage: create_info.usage_flags,
            samples: create_info.sample_count,
            
            layout: create_info.initial_layout,
            tiling: create_info.tiling,
            ty: create_info.ty,

            extent: create_info.extent,
            array_layers: create_info.array_layers,
            mip_levels: create_info.mip_levels,

            format: create_info.format,

            image
        };

        Ok ( result )
    }

    pub fn usage_flags(&self) -> ImageUsageFlags {
        self.usage
    }

    pub fn samples(&self) -> ImageSampleCountFlags {
        self.samples
    }

    
    pub fn layout(&self) -> ImageLayout {
        self.layout
    }

    pub fn tiling(&self) -> ImageTiling {
        self.tiling
    }

    pub fn ty(&self) -> ImageType {
        self.ty
    }


    pub fn extent(&self) -> &Extent3D {
        &self.extent
    }

    pub fn array_layers(&self) -> NonZeroU32 {
        self.array_layers
    }

    pub fn mip_levels(&self) -> NonZeroU32 {
        self.mip_levels
    }


    pub fn format(&self) -> Format {
        self.format
    }


    pub fn memory_requirements(&self) -> MemoryRequirements {
        unsafe { self.device.as_raw().get_image_memory_requirements(self.as_raw()) }
            .into()
    }
}


pub struct Image<A: Allocator> {
    // Dropped first due to RFC 1857
    image: UnbindedImage,
    _alloc: AllocHandle<A>
}

impl<A: Allocator> Image<A> {
    /// # Safety
    /// Allocation should be valid and match image [MemoryRequirements]
    /// 
    /// This means that:
    /// * Allocation should have enough space to fit image inside
    /// * Allocation should be properly aligned
    /// * Allocation should be located in memory, which type is allowed by [MemoryRequirements]
    /// * Allocation must not be outside of memory object ( allocation.offset() + allocation.size() <= memory_object.size() )
    /// 
    /// ['MemoryRequirements']: crate::resources::MemoryRequirements
    pub unsafe fn from_image_and_allocation_unchecked(image: UnbindedImage, allocator: A, allocation: A::Allocation) -> Result<Self, VkError> {
        let memory_object = allocation.memory_object();

        image.device.as_raw().bind_image_memory(
            image.as_raw(),
            memory_object.as_raw(),
            allocation.offset()
        )?;

        let result = Self {
            image,
            _alloc: AllocHandle::new(allocator, allocation)
        };

        Ok ( result )
    }

    pub fn from_image_and_allocation(image: UnbindedImage, allocator: A, allocation: A::Allocation) -> Result<Self, VkError> {
        image.memory_requirements()
            .validate_allocation(&allocation);
        
        unsafe { Self::from_image_and_allocation_unchecked(image, allocator, allocation) }
    }
}

impl<A: Allocator> Drop for Image<A> {
    // same as in Buffer
    // just so we dont accidentaly take some fields out
    fn drop(&mut self) {}
}

impl<A: Allocator> core::ops::Deref for Image<A> {
    type Target = UnbindedImage;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Filter {
    Nearest = 0,
    Linear = 1,

    // Cubic = 1000015000
}

impl From<Filter> for ash::vk::Filter {
    fn from(value: Filter) -> Self {
        match value {
            Filter::Nearest => Self::NEAREST,
            Filter::Linear => Self::LINEAR
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SamplerMipmapMode {
    Nearest = 0,
    Linear = 1
}

impl From<SamplerMipmapMode> for ash::vk::SamplerMipmapMode {
    fn from(value: SamplerMipmapMode) -> Self {
        match value {
            SamplerMipmapMode::Nearest => Self::NEAREST,
            SamplerMipmapMode::Linear => Self::LINEAR
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SamplerAddressMode {
    Repeat = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
    
    // MirrorClampToEdge = 4,
}

impl From<SamplerAddressMode> for ash::vk::SamplerAddressMode {
    fn from(value: SamplerAddressMode) -> Self {
        match value {
            SamplerAddressMode::Repeat => Self::REPEAT,
            SamplerAddressMode::MirroredRepeat => Self::MIRRORED_REPEAT,
            SamplerAddressMode::ClampToEdge => Self::CLAMP_TO_EDGE,
            SamplerAddressMode::ClampToBorder => Self::CLAMP_TO_BORDER
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderColor {
    #[default]
    FloatTransparentBlack = 0,
    IntTransparentBlack = 1,
    
    FloatOpaqueBlack = 2,
    IntOpaqueBlack = 3,

    FloatOpaqueWhite = 4,
    IntOpaqueWhite = 5,

    // FloatCustom = 1000287003
    // IntCustom = 1000287004
}

impl From<BorderColor> for ash::vk::BorderColor {
    fn from(value: BorderColor) -> Self {
        match value {
            BorderColor::FloatTransparentBlack => Self::FLOAT_TRANSPARENT_BLACK,
            BorderColor::IntTransparentBlack => Self::INT_TRANSPARENT_BLACK,

            BorderColor::FloatOpaqueBlack => Self::FLOAT_OPAQUE_BLACK,
            BorderColor::IntOpaqueBlack => Self::INT_OPAQUE_BLACK,

            BorderColor::FloatOpaqueWhite => Self::FLOAT_OPAQUE_WHITE,
            BorderColor::IntOpaqueWhite => Self::INT_OPAQUE_WHITE
        }
    }
}

// TODO: Move to another place
// Cant express this via cmp::Ordering :[
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompareOp {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessOrEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterOrEqual = 6,
    Always = 7
}

impl From<CompareOp> for ash::vk::CompareOp {
    fn from(value: CompareOp) -> Self {
        match value {
            CompareOp::Never => Self::NEVER,
            CompareOp::Less => Self::LESS,
            CompareOp::Equal => Self::EQUAL,
            CompareOp::LessOrEqual => Self::LESS_OR_EQUAL,
            CompareOp::Greater => Self::GREATER,
            CompareOp::NotEqual => Self::NOT_EQUAL,
            CompareOp::GreaterOrEqual => Self::GREATER_OR_EQUAL,
            CompareOp::Always => Self::ALWAYS
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SamplerCreateInfo {
    mag_filter: Filter,
    min_filter: Filter,
    mipmap_mode: SamplerMipmapMode,

    address_mode_u: SamplerAddressMode,
    address_mode_v: SamplerAddressMode,
    address_mode_w: SamplerAddressMode,

    mip_lod_bias: f32,

    max_anisotropy: Option<f32>,
    compare_op: Option<CompareOp>,

    min_lod: f32,
    max_lod: f32,

    border_color: BorderColor,
    unnormalized_cordinates: bool
}

impl SamplerCreateInfo {
    pub fn validate(&self, device: &Device) {
        let features = device.enabled_features();
        let limits = &device.physical_device().properties().limits;

        if self.mip_lod_bias > limits.max_sampler_lod_bias {
            panic!("mip_lod_bias({}) is greater than maximum({})", self.mip_lod_bias, limits.max_sampler_lod_bias);
        }

        if self.min_lod > self.max_lod {
            panic!("min_lod({}) is greater than max_lod({})", self.min_lod, self.max_lod);
        }

        if let Some(max_anisotropy) = self.max_anisotropy {
            let msg = "anisotropy is enabled, but";

            if !features.sampler_anisotropy {
                panic!("{msg} sampler_anisotropy feature id disabled");
            }

            if !(1.0..=limits.max_sampler_anisotropy).contains(&max_anisotropy) {
                panic!(
                    "{msg} max_anisotropy({}) isn`t in 1.0..=max_sampler_anisotropy({})",
                    max_anisotropy,
                    limits.max_sampler_anisotropy
                );
            }

            // cubic filter
        }

        // YCbCr conversion

        if self.unnormalized_cordinates {
            let msg = "unnormalized_cordinates is true, but";

            if self.min_filter != self.mag_filter {
                panic!("{msg} min_filter({:?}) and mag_filter({:?}) arent equal", self.min_filter, self.mag_filter);
            }

            if self.mipmap_mode != SamplerMipmapMode::Nearest {
                panic!("{msg} mipmap_mode({:?}) isn`t Nearest", self.mipmap_mode);
            }

            if self.min_lod != 0.0 && self.max_lod != 0.0 {
                panic!("{msg} min_lod({}) and max_lod({}) arent zero", self.min_lod, self.max_lod);
            }

            if self.address_mode_u != SamplerAddressMode::ClampToEdge && self.address_mode_u != SamplerAddressMode::ClampToBorder {
                panic!("{msg} address_mode_u({:?}) is not ClampToEdge or ClampToBorder", self.address_mode_u);
            }

            if self.address_mode_v != SamplerAddressMode::ClampToEdge && self.address_mode_v != SamplerAddressMode::ClampToBorder {
                panic!("{msg} address_mode_v({:?}) is not ClampToEdge or ClampToBorder", self.address_mode_v);
            }

            if self.max_anisotropy.is_some() {
                panic!("{msg} anisotropy is enabled");
            }

            if self.compare_op.is_some() {
                panic!("{msg} compare_op is enabled");
            }
        }
    }
}

impl From<SamplerCreateInfo> for ash::vk::SamplerCreateInfo {
    fn from(value: SamplerCreateInfo) -> Self {
        Self::builder()
            .mag_filter(value.mag_filter.into())
            .min_filter(value.min_filter.into())
            .mipmap_mode(value.mipmap_mode.into())
            .address_mode_u(value.address_mode_u.into())
            .address_mode_v(value.address_mode_v.into())
            .address_mode_w(value.address_mode_w.into())
            .mip_lod_bias(value.mip_lod_bias)
            .anisotropy_enable(value.max_anisotropy.is_some())
            .max_anisotropy(value.max_anisotropy.unwrap_or_default())
            .compare_enable(value.compare_op.is_some())
            .compare_op(value.compare_op.map(Into::into).unwrap_or_default())
            .min_lod(value.min_lod)
            .max_lod(value.max_lod)
            .border_color(value.border_color.into())
            .unnormalized_coordinates(value.unnormalized_cordinates)
            .build()
    }
}



use core::sync::atomic::Ordering;

pub struct Sampler {
    device: Arc<Device>,

    mag_filter: Filter,
    min_filter: Filter,
    mipmap_mode: SamplerMipmapMode,

    address_mode_u: SamplerAddressMode,
    address_mode_v: SamplerAddressMode,
    address_mode_w: SamplerAddressMode,

    mip_lod_bias: f32,

    max_anisotropy: Option<f32>,
    compare_op: Option<CompareOp>,

    min_lod: f32,
    max_lod: f32,

    border_color: BorderColor,
    unnormalized_cordinates: bool,

    sampler: ash::vk::Sampler
}

impl Sampler {
    pub(crate) unsafe fn as_raw(&self) -> ash::vk::Sampler {
        self.sampler
    }

    /// # Safety
    /// **create_info** should be valid
    pub unsafe fn new_unchecked(device: Arc<Device>, create_info: &SamplerCreateInfo) -> Result<Self, VkError> {
        {
            let max_samplers = device.physical_device().properties().limits.max_sampler_allocation_count;
            
            if device.samplers_count() >= max_samplers {
                return Err( VkError::TooManyObjects );
            }
        }

        let sampler = device.as_raw().create_sampler(
            &(*create_info).into(),
            None
        )?;

        device.edit_samplers_count().fetch_add(1, Ordering::SeqCst);

        let result = Self {
            device,

            mag_filter: create_info.mag_filter,
            min_filter: create_info.min_filter,
            mipmap_mode: create_info.mipmap_mode,

            address_mode_u: create_info.address_mode_u,
            address_mode_v: create_info.address_mode_v,
            address_mode_w: create_info.address_mode_w,

            mip_lod_bias: create_info.mip_lod_bias,

            max_anisotropy: create_info.max_anisotropy,
            compare_op: create_info.compare_op,

            min_lod: create_info.min_lod,
            max_lod: create_info.max_lod,

            border_color: create_info.border_color,
            unnormalized_cordinates: create_info.unnormalized_cordinates,

            sampler
        };

        Ok( result )
    }

    pub fn mag_filter(&self) -> Filter {
        self.mag_filter
    }

    pub fn min_filter(&self) -> Filter {
        self.min_filter
    }

    pub fn mipmap_mode(&self) -> SamplerMipmapMode {
        self.mipmap_mode
    }

    pub fn address_mode_u(&self) -> SamplerAddressMode {
        self.address_mode_u
    }

    pub fn address_mode_v(&self) -> SamplerAddressMode {
        self.address_mode_v
    }

    pub fn address_mode_w(&self) -> SamplerAddressMode {
        self.address_mode_w
    }

    pub fn mip_lod_bias(&self) -> f32 {
        self.mip_lod_bias
    }

    pub fn max_anisotropy(&self) -> Option<f32> {
        self.max_anisotropy
    }

    pub fn compare_op(&self) -> Option<CompareOp> {
        self.compare_op
    }

    pub fn min_lod(&self) -> f32 {
        self.min_lod
    }

    pub fn max_lod(&self) -> f32 {
        self.max_lod
    }

    pub fn border_color(&self) -> BorderColor {
        self.border_color
    }

    pub fn unnormalized_coordinates(&self) -> bool {
        self.unnormalized_cordinates
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            self.device.edit_samplers_count().fetch_sub(1, Ordering::SeqCst);

            self.device.as_raw().destroy_sampler(self.sampler, None);
        }
    }
}