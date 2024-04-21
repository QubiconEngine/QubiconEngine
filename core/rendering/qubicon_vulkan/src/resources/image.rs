use bitflags::bitflags;

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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32
}

impl From<Extent2D> for ash::vk::Extent2D {
    fn from(value: Extent2D) -> Self {
        Self::builder()
            .width(value.width)
            .height(value.height)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

impl From<Extent3D> for ash::vk::Extent3D {
    fn from(value: Extent3D) -> Self {
        Self::builder()
            .width(value.width)
            .height(value.height)
            .depth(value.depth)
            .build()
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
    pub create_flags: ImageCreateFlags,
    pub sample_count_flags: ImageSampleCountFlags,

    pub initial_layout: ImageLayout,
    pub image_tiling: ImageTiling,
    pub image_type: ImageType,
    
    pub array_layers: u32,
    pub format: Format,

    /// For automatic synchronization
    pub main_layout: ImageLayout,
    /// For automatic synchronization
    pub main_owner_queue_family: u32
}

pub(crate) struct ImageInner<A: DeviceMemoryAllocator> {
    pub(crate) device: Arc<DeviceInner>,
    
    pub(crate) image: VkImage,
    pub(crate) info: ImageCreateInfo,
    pub(crate) mip_levels: u32,

    // we shouldnt drop image if it is from swapchain
    pub(crate) drop_required: bool,
    pub(crate) memory: Option<ResourceMemory<A>>
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
            let inner = ManuallyDrop::new(inner);

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
                // hard way of taking field of struct what implements drop
                device: Arc::clone(&inner.device),
                image: inner.image,
                info: inner.info,
                mip_levels: inner.mip_levels,
                drop_required: inner.drop_required,
                memory: Some( ResourceMemory::new(allocator, memory) )
            };

            return Ok( Self { inner: Arc::new(inner) } );
        }
    }

    // no guarantees what it is a valid inner!
    pub(crate) unsafe fn from_inner(inner: ImageInner<A>) -> Self {
        Self { inner: Arc::new(inner) }
    }

    // maybe this should be allowed in public API ?
    pub(crate) fn from_inner_arc(inner: Arc<ImageInner<A>>) -> Self {
        Self { inner }
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
                    self.inner.memory.as_ref().unwrap_unchecked().map()?,
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