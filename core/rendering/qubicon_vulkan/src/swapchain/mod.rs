use std::{marker::PhantomData, sync::Arc};
use ash::vk::{
    Extent2D as VkExtent2D,
    SwapchainCreateInfoKHR as VkSwapchainCreateInfo
};

use crate::{surface::{Surface, ColorSpace, SurfaceTransformFlags, CompositeAlphaFlags, PresentMode}, device::inner::DeviceInner, memory::{alloc::{hollow_device_memory_allocator::{HollowDeviceMemoryAllocator, HollowMemoryFragment}, DeviceMemoryAllocator}, resources::{format::Format, image::{Image, ImageCreateInfo, ImageInner, ImageLayout, ImageSampleCountFlags, ImageTiling, ImageType, ImageUsageFlags, RawImage}, image_view::{ImageView, ImageViewCreateInfo, ImageViewType}, ResourceMemory}}, error::{ValidationError, VkError}, Error, queue::Submission, sync::{Semaphore, semaphore_types::SemaphoreType, Fence}};

pub(crate) mod inner;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapchainCreateInfo {
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

impl Default for SwapchainCreateInfo {
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

impl Into<VkSwapchainCreateInfo> for &SwapchainCreateInfo {
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

pub struct Swapchain {
    inner: Arc<inner::SwapchainInner>,
    images: Box<[Image<inner::SwapchainInner>]>,

    // To disable Sync
    _ph: PhantomData<core::cell::Cell<()>>
}

impl Swapchain {
    pub(crate) unsafe fn create_unchecked(device: Arc<DeviceInner>, surface: Surface, create_info: &SwapchainCreateInfo) -> Result<Self, Error> {
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
                device: Arc::clone(&device),

                info: *create_info
            }
        );

        let image_create_info = _build_image_create_info(create_info);

        let images: Box<[_]> = device.swapchain.as_ref().unwrap_unchecked().get_swapchain_images(swapchain)
            .map_err(| e | VkError::try_from(e).unwrap_unchecked())?
            .into_iter()
            .map(| raw_image | {
                let inner = ImageInner {
                    device: Arc::clone(&device),
                    image: raw_image,
                    info: image_create_info,
                    mip_levels: 1,
                    drop_required: false,
                    memory: Some(ResourceMemory::new(Arc::clone(&inner), inner::SwapchainMemoryFragment))
                };

                Image::from_inner(inner)
            }).collect();

        return Ok( Self { inner, images, _ph: Default::default() } );
    }


    pub fn create_info(&self) -> &SwapchainCreateInfo {
        &self.inner.info
    }

    pub fn image_extent(&self) -> (u32, u32) {
        self.inner.info.image_extent
    }

    pub fn image_format(&self) -> Format {
        self.inner.info.image_format
    }

    pub fn color_space(&self) -> ColorSpace {
        self.inner.info.image_color_space
    }
    // TODO: add more methods to query info


    // I dont know if destroying swapchain after falied recreation is safe. For now let it be like this
    pub fn recreate(mut self, create_info: &SwapchainCreateInfo) -> Result<Self, Error> {
        let images_in_use = self.images.iter()
            .map(| img | img.as_inner())
            .all(| inner | Arc::strong_count(inner) + Arc::weak_count(inner) <= 1);

        if images_in_use {
            return Err(ValidationError::ObjectInUse.into());
        }
        
        unsafe {
            let swapchain = self.inner.device.swapchain.as_ref().unwrap_unchecked()
                .create_swapchain(
                    &VkSwapchainCreateInfo {
                        old_swapchain: self.inner.swapchain,

                        ..create_info.into()
                    },
                    None
                ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            {
                // what the fuck ?
                let inner_ptr = self.inner.as_ref() as *const inner::SwapchainInner as *mut inner::SwapchainInner;

                (*inner_ptr).info = *create_info;
                (*inner_ptr).swapchain = swapchain;
            }

            let image_info = _build_image_create_info(create_info);
            let images: Box<[_]> = self.inner.device.swapchain.as_ref().unwrap_unchecked()
                .get_swapchain_images(swapchain)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked())?
                .into_iter()
                .map(| raw_image | {
                    let inner = ImageInner {
                        device: Arc::clone(&self.inner.device),
                        image: raw_image,
                        info: image_info,
                        mip_levels: 1,
                        drop_required: false,
                        memory: None
                    };

                    Image::from_inner(inner)
                }).collect();

            self.images = images;
        }

        return Ok( self );
    }

    /// # Safety
    /// Should be provided fence or semaphore. 
    pub unsafe fn acquare_next_image_unchecked<T: SemaphoreType>(
        &mut self,
        semaphore: Option<&Semaphore<T>>,
        fence: Option<&Fence>,
        timeout: u64
    ) -> Result<&Image<impl DeviceMemoryAllocator>, Error> {
        let semaphore = semaphore.map(| s | s.as_raw()).unwrap_or_default();
        let fence = fence.map(| f | f.as_raw()).unwrap_or_default();

        let (image_index, _suboptimal) = self.inner.device.swapchain.as_ref().unwrap_unchecked().acquire_next_image(
            self.inner.swapchain,
            timeout,
            semaphore,
            fence
        ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

        return Ok( &self.images[image_index as usize] )
    }

    pub fn drop(self) -> Result<Surface, ValidationError> {
        let mut inner = Arc::into_inner(self.inner)
            .ok_or(ValidationError::ObjectInUse)?;

        unsafe { Ok(inner.surface.take().unwrap_unchecked()) }
    }
}

fn _build_image_create_info(create_info: &SwapchainCreateInfo) -> ImageCreateInfo {
    ImageCreateInfo {
        usage_flags: create_info.image_usage,
        create_flags: Default::default(),
        sample_count_flags: ImageSampleCountFlags::TYPE_1,
        initial_layout: ImageLayout::General,
        image_tiling: ImageTiling::Optimal,
        image_type: ImageType::Type2D {
            width: create_info.image_extent.0,
            height: create_info.image_extent.1,
            miplevels_enabled: false
        },
        array_layers: create_info.image_array_layers,
        format: create_info.image_format
    }
}