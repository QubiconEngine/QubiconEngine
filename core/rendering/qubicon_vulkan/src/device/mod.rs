pub use create_info::*;

use std::sync::Arc;
use crate::{error::VkError, instance::physical_device::{ DeviceFeatures, PhysicalDevice }};

mod create_info;

pub struct Device {
    enabled_features: DeviceFeatures,
    queue_families: Box<[QueueFamilyUsage]>,

    physical_device: PhysicalDevice,
    device: ash::Device
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.device.destroy_device( None ) }
    }
}

impl Device {
    pub fn from_physical_device(
        physical_device: PhysicalDevice,
        create_info: DeviceCreateInfo
    ) -> Result<Arc<Self>, VkError> {
        let queue_families = create_info.queue_families.into_boxed_slice();



        let device = { 
            let features: ash::vk::PhysicalDeviceFeatures = create_info.features.into();
            let queue_create_infos: Vec<_> = queue_families.iter().map(Into::into).collect();


            let create_info = ash::vk::DeviceCreateInfo::builder()
                .enabled_features(&features)
                //.enabled_extension_names(enabled_extension_names)
                //.flags(flags)
                .queue_create_infos(&queue_create_infos)
                .build();

            unsafe {
                physical_device.instance.as_raw().create_device(
                    physical_device.dev,
                    &create_info,
                    None
                )
            }
        };



        let result = Arc::new(
            Self {
                enabled_features: create_info.features,
                queue_families,

                physical_device,
                device: device?
            }
        );

        Ok( result )
    }

    pub fn enabled_features(&self) -> &DeviceFeatures {
        &self.enabled_features
    }

    pub fn queue_families(&self) -> &[QueueFamilyUsage] {
        &self.queue_families
    }

    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }
}

// pub mod error;
// pub(crate) mod inner;

// pub struct Device {
//     pub(crate) inner: Arc<inner::DeviceInner>
// }

// impl Device {
//     pub fn create_from_physical_device<T: Into<Box<[QueueFamilyUsage]>>>(
//         create_info: create_info::DeviceCreateInfo<T>,
//         physical_device: crate::instance::physical_device::PhysicalDevice
//     ) -> Result<Self, Error> {
//         let inner = Arc::new(
//             inner::DeviceInner::create_from_physical_device(
//                 create_info,
//                 physical_device
//             )?
//         );

//         Ok( Self { inner } )
//     }

//     #[inline]
//     pub fn get_physical_device(&self) -> &crate::instance::physical_device::PhysicalDevice {
//         &self.inner.physical_device
//     }

//     #[inline]
//     pub fn get_enabled_features(&self) -> &crate::instance::physical_device::features::DeviceFeatures {
//         &self.inner.features
//     }

//     #[inline]
//     pub fn get_device_properties(&self) -> &crate::instance::physical_device::properties::DeviceProperties {
//         &self.inner.properties
//     }

//     #[inline]
//     pub fn get_device_memory_properties(&self) -> &crate::instance::physical_device::memory_properties::DeviceMemoryProperties {
//         &self.inner.memory_properties
//     }

//     pub fn get_queue(&self, family_index: u32, queue_index: u32) -> Result<crate::queue::Queue, error::QueueError> {
//         unsafe {
//             let queue_count = self.inner.get_queue_usage()
//                 .iter()
//                 .copied()
//                 .find(| q | q.family_index == family_index)
//                 .ok_or(error::QueueError::NoQueueFamily { family_index })?
//                 .queue_count;

//             if queue_index >= queue_count {
//                 return Err(error::QueueError::NoQueueWithIndex { queue_index });
//             }

//             let queue = self.inner.get_device_queue(family_index, queue_index);
//             let queue_inner = Arc::new(
//                 crate::queue::inner::QueueInner {
//                     queue_index,
//                     family_index,
//                     capabilities: self.get_physical_device()
//                         .get_queue_family_infos()[family_index as usize].capabilities,
//                     device: Arc::clone(&self.inner),

//                     queue
//                 }
//             );

//             Ok(queue_inner.into())
//         }
//     }

//     /// Returns instance from wich device was created
//     pub fn associated_instance(&self) -> crate::Instance {
//         crate::Instance::from_inner(Arc::clone(&self.inner.physical_device.instance))
//     }

//     pub fn allocate_memory(&self, memory_type_index: u8, size: u64) -> Result<Arc<DeviceMemoryObject>, Error> {
//         DeviceMemoryObject::allocate(Arc::clone(&self.inner), memory_type_index, size)
//     }

//     pub fn create_descriptor_pool<T: Into<Box<[DescriptorPoolSize]>>>(&self, create_info: DescriptorPoolCreateInfo<T>) -> Result<DescriptorPool, Error> {
//         DescriptorPool::new(
//             Arc::clone(&self.inner),
//             create_info
//         )
//     }

//     pub fn create_pipeline_layout(&self, descriptor_sets: impl Into<Box<[Arc<DescriptorSetLayout>]>>) -> Result<Arc<PipelineLayout>, Error> {
//         PipelineLayout::create(
//             Arc::clone(&self.inner),
//             descriptor_sets
//         )
//     }

//     pub fn create_raw_buffer(&self, create_info: &BufferCreateInfo) -> Result<RawBuffer, Error> {
//         RawBuffer::create(
//             Arc::clone(&self.inner),
//             create_info
//         )
//     }

//     pub fn create_buffer<A: DeviceMemoryAllocator>(&self, allocator: Arc<A>, memory_properties: MemoryTypeProperties, create_info: &BufferCreateInfo) -> Result<Buffer<A>, ResourceCreationError<A::AllocError>> {
//         Buffer::create_and_allocate(
//             Arc::clone(&self.inner),
//             allocator,
//             memory_properties,
//             create_info
//         )
//     }

//     pub fn create_raw_image(&self, create_info: &ImageCreateInfo) -> Result<RawImage, Error> {
//         RawImage::create(
//             Arc::clone(&self.inner),
//             create_info
//         )
//     }

//     pub fn create_image<A: DeviceMemoryAllocator>(&self, allocator: Arc<A>, memory_properties: MemoryTypeProperties, create_info: ImageCreateInfo) -> Result<Image<A>, ResourceCreationError<A::AllocError>> {
//         Image::create_and_allocate(
//             Arc::clone(&self.inner),
//             allocator,
//             memory_properties,
//             create_info
//         )
//     }

    
//     pub fn create_fence(&self, create_info: FenceCreateInfo) -> Result<Fence, Error> {
//         Fence::create(
//             Arc::clone(&self.inner),
//             create_info
//         )
//     }

//     pub fn create_event(&self) -> Result<Event, Error> {
//         Event::create(Arc::clone(&self.inner))
//     }

//     pub fn create_semaphore<Type: semaphore_types::SemaphoreType>(&self) -> Result<Semaphore<Type>, Error> {
//         Semaphore::create(Arc::clone(&self.inner))
//     }

//     #[cfg(feature = "windowing")]
//     /// # Safety
//     /// * *surface* must be a valid Vulkan surface
//     pub unsafe fn create_swapchain_unchecked(&self, surface: Surface, create_info: &SwapchainCreateInfo) -> Result<Swapchain, Error> {
//         Swapchain::create_unchecked(Arc::clone(&self.inner), surface, create_info)
//     }

//     /// # Safety
//     /// * size must be not equal to 0 and be less than heap size and total device memory size
//     /// * type index must be less than device memory type count
//     pub unsafe fn allocate_memory_unchecked(&self, memory_type_index: u8, size: u64) -> Result<Arc<DeviceMemoryObject>, Error> {
//         DeviceMemoryObject::allocate_unchecked(Arc::clone(&self.inner), memory_type_index, size)
//     }

//     /// # Safety
//     /// Binary slice should contain valid **SPIR-V** binary
//     pub unsafe fn create_shader_module_from_binary(&self, binary: &[u32]) -> Result<ShaderModule, Error> {
//         ShaderModule::create_from_binary(
//             Arc::clone(&self.inner),
//             binary
//         )
//     }

//     /// # Safety
//     /// All bindings should match device limits
//     pub unsafe fn create_descriptor_set_layout_unchecked<T: Into<Box<[DescriptorBinding]>>>(&self, create_info: DescriptorSetLayoutCreateInfo<T>) -> Result<Arc<DescriptorSetLayout>, Error> {
//         DescriptorSetLayout::create_unchecked(
//             Arc::clone(&self.inner),
//             create_info
//         )
//     }

//     /// # Safety
//     /// Descriptor sets should be owned by this device
//     pub unsafe fn create_pipeline_layout_unchecked(&self, descriptor_sets: impl Into<Box<[Arc<DescriptorSetLayout>]>>) -> Result<Arc<PipelineLayout>, Error> {
//         PipelineLayout::create_unchecked(
//             Arc::clone(&self.inner),
//             descriptor_sets
//         )
//     }

//     /// # Safety
//     /// * Layout and shader module should be owned by this device
//     /// * Shader module should contain entry with *entry_name*
//     pub unsafe fn create_compute_pipeline_unchecked(&self, create_info: ComputePipelineCreateInfo) -> Result<Arc<ComputePipeline>, Error> {
//         ComputePipeline::create_unchecked(
//             Arc::clone(&self.inner),
//             create_info
//         )
//     }
// }

// impl Debug for Device {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Device")
//     }
// }