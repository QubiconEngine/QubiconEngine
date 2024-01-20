use std::{sync::Arc, mem::MaybeUninit};
use ash::vk::{
    Extent2D as VkExtent2D,
    SwapchainKHR as VkSwapchain,
    SwapchainCreateInfoKHR as VkSwapchainCreateInfo
};

use crate::{surface::{Surface, ColorSpace, SurfaceTransformFlags, CompositeAlphaFlags, PresentMode}, device::inner::DeviceInner, memory::resources::{format::Format, image::{ImageUsageFlags, Image, RawImage, ImageSampleCountFlags, ImageTiling, ImageType, ImageLayout}, image_view::{ImageView, ImageViewCreateInfo, ImageViewType}}, error::{ValidationError, VkError}, Error, queue::Submission, sync::{Semaphore, semaphore_types::SemaphoreType, Fence}};

pub(crate) mod inner;
mod memory_allocator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapchainCreationInfo {
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpace,
    /// 0 is width, 1 is height
    pub image_extent: (u32, u32),
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    // TODO: image_sharing_mode
    // pub image_sharing_mode,
    // pub queue_family_indices: &'a [u32],
    pub pre_transform: SurfaceTransformFlags,
    pub composite_alpha: CompositeAlphaFlags,
    pub present_mode: PresentMode,

    pub clipped: bool
}

impl Default for SwapchainCreationInfo {
    #[inline]
    fn default() -> Self {
        Self {
            min_image_count: 1,
            image_format: Format::UNDEFINED,
            image_color_space: ColorSpace::SRGB_Nonlinear,
            image_extent: (1, 1),
            image_array_layers: 1,
            image_usage: ImageUsageFlags::default(),
            pre_transform: SurfaceTransformFlags::INHERIT,
            composite_alpha: CompositeAlphaFlags::INHERIT,
            present_mode: PresentMode::Immediate,

            clipped: false
        }
    }
}

impl Into<VkSwapchainCreateInfo> for &SwapchainCreationInfo {
    fn into(self) -> VkSwapchainCreateInfo {
        VkSwapchainCreateInfo {
            min_image_count: self.min_image_count,
            image_format: self.image_format.into(),
            image_color_space: self.image_color_space.into(),
            image_extent: VkExtent2D {
                width: self.image_extent.0,
                height: self.image_extent.1
            },
            image_array_layers: self.image_array_layers,
            image_usage: self.image_usage.into(),
            pre_transform: self.pre_transform.into(),
            composite_alpha: self.composite_alpha.into(),
            present_mode: self.present_mode.into(),
            clipped: self.clipped as u32,

            ..Default::default()
        }
    }
}

// TODO: Disable image drop
pub struct Swapchain {
    inner: Arc<inner::SwapchainInner>,
    allocator: Arc<memory_allocator::SwapchainImageMemoryAllocator>,
    images: Box<[Arc<Image<memory_allocator::SwapchainImageMemoryAllocator>>]>
}

impl Swapchain {
    pub(crate) unsafe fn create_unchecked(device: Arc<DeviceInner>, surface: Surface, create_info: &SwapchainCreationInfo) -> Result<Self, Error> {
        let raw_create_info = VkSwapchainCreateInfo {
            surface: surface.as_raw(),

            ..create_info.into()
        };

        let swapchain = device.swapchain.as_ref().unwrap_unchecked().create_swapchain(
            &raw_create_info,
            None
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        let inner = Arc::new(
            inner::SwapchainInner {
                swapchain,
                surface: Some(surface),
                device: Arc::clone(&device)
            }
        );
        let allocator = Arc::new(
            memory_allocator::SwapchainImageMemoryAllocator { _swapchain: Arc::clone(&inner) }
        );

        let images: Box<[_]> = device.swapchain.as_ref().unwrap_unchecked().get_swapchain_images(swapchain)
            .map_err(| e | VkError::try_from(e).unwrap_unchecked())?
            .into_iter()
            .map(| i | {
                Arc::new(
                    Image {
                        raw: RawImage {
                            device: Arc::clone(&device),
                            image: i,
                            usage_flags: create_info.image_usage,
                            create_flags: Default::default(),
                            sample_count_flags: ImageSampleCountFlags::TYPE_1,
                            initital_layout: ImageLayout::General,
                            image_tiling: ImageTiling::Optimal,
                            image_type: ImageType::Type2D {
                                width: create_info.image_extent.0,
                                height: create_info.image_extent.1,
                                miplevels_enabled: false
                            },
                            array_layers: create_info.image_array_layers,
                            mip_levels: 1,
                            format: create_info.image_format
                        },
                        allocator: Arc::clone(&allocator),
                        memory: MaybeUninit::new(memory_allocator::SwapchainImageMemoryFragment)
                    }
                )
            }).collect();

        Ok(
            Self {
                inner,
                allocator,
                images
            }
        )
    }

    /// # Safety
    /// Should be provided fence or semaphore. 
    pub unsafe fn acquare_next_image_unchecked<T: SemaphoreType>(
        &mut self,
        semaphore: Option<&Semaphore<T>>,
        fence: Option<&Fence>,
        timeout: u64,
        create_info: &ImageViewCreateInfo
    ) -> Result<Arc<ImageView<memory_allocator::SwapchainImageMemoryAllocator>>, Error> {
        let semaphore = semaphore.map(| s | s.as_raw()).unwrap_or_default();
        let fence = fence.map(| f | f.as_raw()).unwrap_or_default();

        let (image_index, _suboptimal) = self.inner.device.swapchain.as_ref().unwrap_unchecked().acquire_next_image(
            self.inner.swapchain,
            timeout,
            semaphore,
            fence
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        self.images[image_index as usize].create_image_view_unchecked(create_info)
    }

    pub fn drop(self) -> Result<Surface, ValidationError> {
        let mut inner = Arc::into_inner(self.inner)
            .ok_or(ValidationError::ObjectInUse)?;

        unsafe { Ok(inner.surface.take().unwrap_unchecked()) }
    }
}