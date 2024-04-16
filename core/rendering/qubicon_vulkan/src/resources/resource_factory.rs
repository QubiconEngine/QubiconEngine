use std::{marker::PhantomData, sync::Arc};

use crate::{
    commands::{
        command_buffers::{
            self,
            command_buffer_builder::{
                barrier::{
                    AccessFlags,
                    BufferMemoryBarrier,
                    ImageMemoryBarrier,
                    PipelineBarrierDependencyFlags
                },
                copy::{
                    BufferCopy,
                    BufferImageCopy
                }
            },
            CommandBufferBuilder,
            CommandBufferUsageFlags
        },
        CommandPool
    },
    device::{
        inner::DeviceInner,
        Device
    },
    instance::physical_device::{
        memory_properties::MemoryTypeProperties, 
        queue_info::QueueFamilyCapabilities
    }, 
    memory::resources::buffer::BufferCreateInfo, 
    queue::{Queue, Submission}, 
    shaders::PipelineStageFlags, 
    sync::semaphore_types
};
use self::{
    alloc::{
        hollow_device_memory_allocator::HollowDeviceMemoryAllocator,
        DeviceMemoryAllocator
    },
    resources::{
        buffer::{
            Buffer,
            BufferCreateFlags,
            BufferUsageFlags
        },
        format::Format,
        image::{
            Image,
            ImageCreateFlags,
            ImageCreateInfo,
            ImageLayout,
            ImageSampleCountFlags,
            ImageTiling,
            ImageType, 
            ImageUsageFlags
        }, 
        image_view::{
            ImageAspect, 
            ImageSubresourceLayers, 
            ImageSubresourceRange
        }
    }
};

pub struct ResourceFactory {
    device: Arc<DeviceInner>,

    // Used for generating mipmaps
    graphics_queue: Option<Queue>,
    // For copying from staging buffer
    transfer_queue: Queue,

    graphics_queue_family_index: u32,
    transfer_queue_family_index: u32,

    graphics_pool: Option<CommandPool>,
    transfer_pool: CommandPool
}

impl ResourceFactory {
    // TODO: Propper checks, graphics queue and a lot more
    pub fn init(device: &Device, transfer_queue: Queue) -> Result<Self, crate::Error> {
        if !transfer_queue.get_capabilities().contains(QueueFamilyCapabilities::TRANSFER) {
            panic!("What the fuck you are doing!")
        }

        let transfer_queue_family_index = transfer_queue.as_inner().family_index;
        
        Ok(
            Self {
                device: Arc::clone(&device.inner),

                transfer_pool: transfer_queue.create_command_pool()?,
                transfer_queue,
                transfer_queue_family_index,

                graphics_pool: None,
                graphics_queue: None,
                graphics_queue_family_index: 0,
            }
        )
    }

    pub fn create_order<Alloc: DeviceMemoryAllocator>(&self, allocator: Arc<Alloc>) -> Result<OrderBuilder<Alloc>, crate::Error> {
        let transfer_builder = self.transfer_pool.create_primary_command_buffer(
            CommandBufferUsageFlags::ONE_TIME_SUBMIT
        )?;

        Ok(
            OrderBuilder {
                factory: self,

                allocator,
                transfer_builder: Some(transfer_builder),
                
                image_list: Vec::new(),
                buffer_list: Vec::new()
            }
        )
    }
}

#[derive(Clone, Copy)]
pub struct BufferStagingBufferInfo<'a, A: DeviceMemoryAllocator> {
    pub buffer: &'a Buffer<A>,
    pub regions: &'a [BufferCopy]
}

pub struct ImageStagingBufferInfo<'a, A: DeviceMemoryAllocator> {
    pub buffer: &'a Buffer<A>,

    pub offset: u64,
    pub row_length: u32,
    pub image_heigth: u32,
    pub subresource: ImageSubresourceLayers
}
impl<'a, A: DeviceMemoryAllocator> Clone for ImageStagingBufferInfo<'a, A> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
            row_length: self.row_length,
            image_heigth: self.image_heigth,
            // this sucks 
            subresource: self.subresource.clone()
        }
    }
}

pub struct ImageRequest<'a, StagingAlloc: DeviceMemoryAllocator = HollowDeviceMemoryAllocator> {
    pub format: Format,
    pub usage_flags: ImageUsageFlags,
    pub create_flags: ImageCreateFlags,
    pub sample_count_flags: ImageSampleCountFlags,
    pub type_: ImageType,
    pub tiling: ImageTiling,
    pub array_layers: u16,

    pub main_layout: ImageLayout,
    pub main_owner_queue_family: u32,

    pub staging_buffer: Option<ImageStagingBufferInfo<'a, StagingAlloc>>
}
impl<'a, StagingAlloc: DeviceMemoryAllocator> Clone for ImageRequest<'a, StagingAlloc> {
    fn clone(&self) -> Self {
        Self {
            format: self.format,
            usage_flags: self.usage_flags,
            create_flags: self.create_flags,
            sample_count_flags: self.sample_count_flags,
            type_: self.type_,
            tiling: self.tiling,
            array_layers: self.array_layers,
            main_layout: self.main_layout,
            main_owner_queue_family: self.main_owner_queue_family,
            staging_buffer: self.staging_buffer.clone()
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct BufferRequest<'a, StagingAlloc: DeviceMemoryAllocator = HollowDeviceMemoryAllocator> {
    pub usage_flags: BufferUsageFlags,
    pub create_flags: BufferCreateFlags,

    pub size: u64,
    pub main_owner_queue_family: u32,

    pub staging_buffer: Option<BufferStagingBufferInfo<'a, StagingAlloc>>
}

pub struct OrderBuilder<'a, Alloc: DeviceMemoryAllocator> {
    factory: &'a ResourceFactory,
    allocator: Arc<Alloc>,

    // I am piece of shit. Builder takes ownership of itself
    transfer_builder: Option<CommandBufferBuilder<'a, command_buffers::levels::Primary>>,
    
    image_list: Vec<Image<Alloc>>,
    buffer_list: Vec<Buffer<Alloc>>
}

impl<'a, Alloc: DeviceMemoryAllocator> OrderBuilder<'a, Alloc> {
    // TODO: Add better handling of image aspects. Also staging buffer bounds checks required
    pub fn request_image<StagingAlloc: DeviceMemoryAllocator>(
        &mut self,
        memory_properties: MemoryTypeProperties,
        request: ImageRequest<'a, StagingAlloc>
    ) -> Result<(), resources::ResourceCreationError<Alloc::AllocError>> {
        let (width, height, depth) = match request.type_ {
            ImageType::Type1D { width } => (width, 1, 1),
            ImageType::Type2D { width, height, .. } => (width, height, 1),
            ImageType::Type3D { width, height, depth } => (width, height, depth)
        };
        
        let usage_flags = match request.staging_buffer.is_some() {
            true => request.usage_flags | ImageUsageFlags::TRANSFER_DST,
            false => request.usage_flags
        };
        
        let create_info = ImageCreateInfo {
            usage_flags,

            initial_layout: ImageLayout::Undefined,
            create_flags: request.create_flags,
            sample_count_flags: request.sample_count_flags,
            image_tiling: request.tiling,
            image_type: request.type_,
            array_layers: request.array_layers as u32,
            format: request.format,

            main_layout: request.main_layout,
            main_owner_queue_family: request.main_owner_queue_family
        };

        let image = Image::create_and_allocate(
            Arc::clone(&self.factory.device),
            Arc::clone(&self.allocator),
            memory_properties,
            create_info
        )?;

        unsafe {
            let mut transfer_builder = self.transfer_builder.take().unwrap_unchecked();

            // if staging buffer is on different queue, we need barriers to transfer ownership
            transfer_builder = if let Some(staging_buffer) = request.staging_buffer {
                let staging_buffer_owner_family = staging_buffer.buffer.create_info().main_owner_queue_family;
                
                let buffer_barriers = if self.factory.transfer_queue_family_index != staging_buffer_owner_family {
                    let start = BufferMemoryBarrier {
                        src_access_mask: AccessFlags::empty(),
                        dst_access_mask: AccessFlags::TRANSFER_READ,

                        src_queue_family_index: staging_buffer_owner_family,
                        dst_queue_family_index: self.factory.transfer_queue_family_index,

                        buffer: staging_buffer.buffer,
                        
                        // Whole size
                        offset: 0,
                        size: u64::MAX
                    };
                    let end = BufferMemoryBarrier {
                        src_access_mask: AccessFlags::TRANSFER_READ,
                        dst_access_mask: AccessFlags::empty(),

                        src_queue_family_index: staging_buffer_owner_family,
                        dst_queue_family_index: self.factory.transfer_queue_family_index,
                        
                        buffer: staging_buffer.buffer,
                        
                        // Whole size
                        offset: 0,
                        size: u64::MAX
                    };

                    Some( (start, end) )
                } else {
                    None
                };


                // commands themselves
                transfer_builder
                    .cmd_pipeline_barrier_unchecked::<_, StagingAlloc>(
                        PipelineStageFlags::TOP_OF_PIPE,
                        PipelineStageFlags::TRANSFER,
                        PipelineBarrierDependencyFlags::empty(),
                        &[],
                        &[
                            ImageMemoryBarrier {
                                src_access_mask: AccessFlags::empty(),
                                dst_access_mask: AccessFlags::TRANSFER_WRITE,
                                old_layout: ImageLayout::Undefined,
                                new_layout: ImageLayout::TransferDstOptimal,
                                src_queue_family_index: u32::MAX,
                                dst_queue_family_index: self.factory.transfer_queue_family_index,
                                image: &image,
                                subresource_range: ImageSubresourceRange {
                                    aspect_mask: ImageAspect::all(),
                                    mip_levels: 0..image.mip_levels_count(),
                                    array_layers: 0..image.array_layers_count()
                                }
                            }
                        ],
                        buffer_barriers.as_ref().map(| (s, _) | core::slice::from_ref(s)).unwrap_or(&[])
                    )
                    .cmd_copy_buffer_to_image(
                        staging_buffer.buffer,
                        &image,
                        ImageLayout::TransferDstOptimal,
                        &[
                            BufferImageCopy {
                                buffer_offset: staging_buffer.offset,
                                buffer_row_length: staging_buffer.row_length,
                                buffer_image_height: staging_buffer.image_heigth,
                                image_subresource: staging_buffer.subresource.clone(),
                                image_offset: (0, 0, 0),
                                image_extent: (width, height, depth)
                            }
                        ]
                    )
                    .cmd_pipeline_barrier_unchecked::<_, StagingAlloc>(
                        PipelineStageFlags::TRANSFER,
                        PipelineStageFlags::BOTTOM_OF_PIPE,
                        PipelineBarrierDependencyFlags::empty(),
                        &[],
                        &[
                            ImageMemoryBarrier {
                                src_access_mask: AccessFlags::TRANSFER_WRITE,
                                dst_access_mask: AccessFlags::empty(),
                                old_layout: ImageLayout::TransferDstOptimal,
                                new_layout: request.main_layout,
                                src_queue_family_index: self.factory.transfer_queue_family_index,
                                dst_queue_family_index: request.main_owner_queue_family,
                                image: &image,
                                subresource_range: ImageSubresourceRange {
                                    aspect_mask: ImageAspect::all(),
                                    mip_levels: 0..image.mip_levels_count(),
                                    array_layers: 0..image.array_layers_count()
                                }
                            }
                        ],
                        buffer_barriers.as_ref().map(| (_, e) | core::slice::from_ref(e)).unwrap_or(&[])
                    )
            } else {
                transfer_builder
                    .cmd_pipeline_barrier_unchecked::<_, HollowDeviceMemoryAllocator>(
                        PipelineStageFlags::TOP_OF_PIPE,
                        PipelineStageFlags::TOP_OF_PIPE,
                        PipelineBarrierDependencyFlags::empty(),
                        &[],
                        &[
                            ImageMemoryBarrier {
                                src_access_mask: AccessFlags::empty(),
                                dst_access_mask: AccessFlags::empty(),
                                old_layout: ImageLayout::Undefined,
                                new_layout: request.main_layout,
                                src_queue_family_index: u32::MAX,
                                dst_queue_family_index: u32::MAX,
                                image: &image,
                                subresource_range: ImageSubresourceRange {
                                    aspect_mask: ImageAspect::all(),
                                    mip_levels: 0..image.mip_levels_count(),
                                    array_layers: 0..image.array_layers_count()
                                }
                            }
                        ],
                        &[]
                    )
            };

            self.transfer_builder = Some(transfer_builder)
        }

        self.image_list.push(image);

        Ok(())
    }

    pub fn request_buffer<StagingAlloc: DeviceMemoryAllocator>(
        &mut self,
        memory_properties: MemoryTypeProperties,
        request: BufferRequest<StagingAlloc>
    ) -> Result<(), resources::ResourceCreationError<Alloc::AllocError>> {
        if let Some(staging_buffer) = request.staging_buffer.as_ref() {
            let src_end_offset = staging_buffer.regions
                .iter()
                .map(| r | r.src_offset + r.size)
                .max();
            let dst_end_offset = staging_buffer.regions
                .iter()
                .map(| r | r.dst_offset + r.size)
                .max();

            if let (Some(src_end_offset), Some(dst_end_offset)) = (src_end_offset, dst_end_offset) {
                // offsets should be in bounds!
                
                if src_end_offset > staging_buffer.buffer.size() {
                    panic!("source offset is out of bounds")
                }

                if dst_end_offset > request.size {
                    panic!("destination offset is out of bounds")
                }
            }
        }

        let usage_flags = match request.staging_buffer.is_some() {
            true => request.usage_flags | BufferUsageFlags::TRANSFER_DST,
            false => request.usage_flags
        };

        let buffer = Buffer::create_and_allocate(
            Arc::clone(&self.factory.device),
            Arc::clone(&self.allocator),
            memory_properties,
            &BufferCreateInfo {
                usage_flags,
                create_flags: request.create_flags,
                size: request.size,
                main_owner_queue_family: request.main_owner_queue_family
            }
        )?;

        if let Some(staging_buffer) = request.staging_buffer {
            let staging_buffer_queue_family = staging_buffer.buffer.create_info().main_owner_queue_family;

            unsafe {
                let mut transfer_builder = self.transfer_builder.take().unwrap_unchecked();

                if self.factory.transfer_queue_family_index != staging_buffer_queue_family {
                    transfer_builder = transfer_builder.cmd_pipeline_barrier_unchecked::<HollowDeviceMemoryAllocator, _>(
                        PipelineStageFlags::empty(),
                        PipelineStageFlags::TRANSFER,
                        PipelineBarrierDependencyFlags::empty(),
                        &[],
                        &[],
                        &[
                            BufferMemoryBarrier {
                                src_access_mask: AccessFlags::empty(),
                                dst_access_mask: AccessFlags::TRANSFER_READ,
                                src_queue_family_index: staging_buffer_queue_family,
                                dst_queue_family_index: self.factory.transfer_queue_family_index,
                                buffer: staging_buffer.buffer,
                                
                                // whole size
                                offset: 0,
                                size: u64::MAX
                            }
                        ]
                    );
                }

                transfer_builder = transfer_builder
                    .cmd_copy_buffer(
                        staging_buffer.buffer,
                        &buffer,
                        staging_buffer.regions
                    );

                // If with same statement as before. SHIT
                if self.factory.transfer_queue_family_index != staging_buffer_queue_family {
                    transfer_builder = transfer_builder.cmd_pipeline_barrier_unchecked::<HollowDeviceMemoryAllocator, _>(
                        PipelineStageFlags::TRANSFER,
                        PipelineStageFlags::BOTTOM_OF_PIPE,
                        PipelineBarrierDependencyFlags::empty(),
                        &[],
                        &[],
                        &[
                            BufferMemoryBarrier {
                                src_access_mask: AccessFlags::TRANSFER_READ,
                                dst_access_mask: AccessFlags::empty(),
                                src_queue_family_index: self.factory.transfer_queue_family_index,
                                dst_queue_family_index: staging_buffer_queue_family,
                                buffer: staging_buffer.buffer,
        
                                // also whole size
                                offset: 0,
                                size: u64::MAX
                            }
                        ]
                    );
                }

                self.transfer_builder = Some(transfer_builder)
            }
        }

        self.buffer_list.push(buffer);
        
        Ok(())
    }

    pub fn do_order(self) -> Result<Order<'a, Alloc>, crate::Error> {
        let transfer_cmd = unsafe { self.transfer_builder.unwrap_unchecked() }
            .build()?;

        let submission = self.factory.transfer_queue.submit::<semaphore_types::Binary, semaphore_types::Binary>(
            core::iter::empty(),
            core::iter::empty(),
            core::iter::once(transfer_cmd)
        )?;
        
        Ok(
            Order {
                submission,
                
                images: self.image_list,
                buffers: self.buffer_list,

                _ph: Default::default()
            }
        )
    }
}

pub struct Order<'a, Alloc: DeviceMemoryAllocator> {
    submission: Submission<
        command_buffers::CommandBuffer<command_buffers::levels::Primary>,
        semaphore_types::Binary,
        semaphore_types::Binary>,
    
    images: Vec<Image<Alloc>>,
    buffers: Vec<Buffer<Alloc>>,

    _ph: PhantomData<&'a ResourceFactory>
}

impl<'a, Alloc: DeviceMemoryAllocator> Order<'a, Alloc> {
    pub fn wait(self) -> (Vec<Image<Alloc>>, Vec<Buffer<Alloc>>) {
        // TODO: Currently this shit will block if error occured
        self.submission.wait_owned(u64::MAX)
            .map(move | _ | (self.images, self.buffers))
            .map_err(| (_, e) | e)
            .expect("Failed to finish resource creation operations")
    } 
}



#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{commands::command_buffers::command_buffer_builder::copy::BufferCopy, device::create_info::{DeviceCreateInfo, QueueFamilyUsage}, instance::physical_device::{memory_properties::MemoryTypeProperties, queue_info::QueueFamilyCapabilities, PhysicalDevice}, Instance};
    use super::{alloc::{hollow_device_memory_allocator::HollowDeviceMemoryAllocator, standart_device_memory_allocator::StandartMemoryAllocator}, resources::{buffer::{BufferCreateInfo, BufferUsageFlags}, format::Format, image::{ImageLayout, ImageSampleCountFlags, ImageTiling, ImageType, ImageUsageFlags}, image_view::{ImageAspect, ImageSubresourceLayers}}, ImageRequest, ResourceFactory};
    
    fn queue_family_with_capability(dev: &PhysicalDevice, required_capabilities: QueueFamilyCapabilities) -> Option<u32> {
        dev.get_queue_family_infos()
            .iter()
            .enumerate()
            .find(| (_, family) | family.capabilities.contains(required_capabilities))
            .map(| (idx, _) | idx as u32)
    }

    #[test]
    fn image_creation() {
        let instance = Instance::create(&Default::default())
            .unwrap();

        let (family_index, device) = instance.enumerate_devices()
            .unwrap()
            .filter_map(| dev | Some(
                (queue_family_with_capability(&dev, QueueFamilyCapabilities::TRANSFER)?, dev)
            ))
            .next()
            .unwrap();

        let device = device.create_logical_device(
            DeviceCreateInfo {
                queues: [
                    QueueFamilyUsage {
                        family_index,
                        queue_count: 1
                    }
                ].as_slice(),

                ..Default::default()
            }
        ).unwrap();

        let allocator = StandartMemoryAllocator::new(&device);
        let resource_factory = ResourceFactory::init(
            &device,
            device.get_queue(family_index, 0).unwrap()
        ).unwrap();

        let mut order = resource_factory.create_order(Arc::clone(&allocator)).unwrap();

        order.request_image::<HollowDeviceMemoryAllocator>(
            Default::default(),
            super::ImageRequest {
                format: Format::R8G8B8A8_SRGB,
                usage_flags: ImageUsageFlags::SAMPLED, 
                create_flags: Default::default(), 
                sample_count_flags: ImageSampleCountFlags::TYPE_1, 
                type_: ImageType::Type2D { width: 150, height: 150, miplevels_enabled: false }, 
                tiling: ImageTiling::Optimal, 
                array_layers: 1, 

                main_layout: ImageLayout::General, 
                main_owner_queue_family: family_index, 

                staging_buffer: None
            }).unwrap();

        let image = order.do_order().unwrap().wait().0.swap_remove(0);
    }

    #[test]
    fn image_creation_with_staging_buffer() {
        let instance = Instance::create(&Default::default()).unwrap();

        let (family_index, device) = instance.enumerate_devices()
            .unwrap()
            .filter_map(| dev | Some(
                (queue_family_with_capability(&dev, QueueFamilyCapabilities::TRANSFER)?, dev)
            ))
            .next()
            .unwrap();

        let device = device.create_logical_device(
            DeviceCreateInfo {
                queues: [
                    QueueFamilyUsage {
                        family_index,
                        queue_count: 1
                    }
                ].as_slice(),

                ..Default::default()
            }
        ).unwrap();

        let allocator = StandartMemoryAllocator::new(&device);
        let resource_factory = ResourceFactory::init(
            &device,
            device.get_queue(family_index, 0).unwrap()
        ).unwrap();


        let staging_buffer = device.create_buffer(
            Arc::clone(&allocator),
            MemoryTypeProperties::HOST_VISIBLE,
            &BufferCreateInfo {
                usage_flags: BufferUsageFlags::TRANSFER_SRC,
                size: 1024,
                main_owner_queue_family: family_index,

                ..Default::default()
            }
        ).unwrap();


        unsafe { staging_buffer.map::<u32>() }
            .unwrap()
            .iter_mut()
            .for_each(| m | { m.write(0); });


        let mut order = resource_factory.create_order(Arc::clone(&allocator))
            .unwrap();

        order.request_image(
            MemoryTypeProperties::DEVICE_LOCAL,
            ImageRequest {
                format: Format::R8G8B8A8_UINT,
                usage_flags: ImageUsageFlags::STORAGE,
                create_flags: Default::default(),
                sample_count_flags: ImageSampleCountFlags::TYPE_1,
                tiling: ImageTiling::Optimal,
                type_: ImageType::Type2D { width: 16, height: 16, miplevels_enabled: false },
                array_layers: 1,
                main_layout: ImageLayout::General,
                main_owner_queue_family: family_index,
                staging_buffer: Some(
                    super::ImageStagingBufferInfo {
                        buffer: &staging_buffer,
                        offset: 0,
                        row_length: 16,
                        image_heigth: 16,
                        subresource: ImageSubresourceLayers {
                            aspect_mask: ImageAspect::COLOR,
                            mip_level: 0,
                            array_layers: 0..1
                        }
                    }
                )
            }
        ).unwrap();

        let image = order.do_order().unwrap().wait().0.swap_remove(0);
    }

    #[test]
    fn buffer_creation_with_staging_buffer() {
        let instance = Instance::create(&Default::default()).unwrap();

        let (family_index, device) = instance.enumerate_devices()
            .unwrap()
            .filter_map(| dev | Some(
                (queue_family_with_capability(&dev, QueueFamilyCapabilities::TRANSFER)?, dev)
            ))
            .next()
            .unwrap();

        let device = device.create_logical_device(
            DeviceCreateInfo {
                queues: [
                    QueueFamilyUsage {
                        family_index,
                        queue_count: 1
                    }
                ].as_slice(),

                ..Default::default()
            }
        ).unwrap();

        let allocator = StandartMemoryAllocator::new(&device);
        let resource_factory = ResourceFactory::init(
            &device,
            device.get_queue(family_index, 0).unwrap()
        ).unwrap();


        let staging_buffer = device.create_buffer(
            Arc::clone(&allocator),
            MemoryTypeProperties::HOST_VISIBLE,
            &BufferCreateInfo {
                usage_flags: BufferUsageFlags::TRANSFER_SRC,
                size: 1024,
                main_owner_queue_family: family_index,

                ..Default::default()
            }
        ).unwrap();


        let mut order = resource_factory.create_order(Arc::clone(&allocator))
            .unwrap();

        order.request_buffer(
            MemoryTypeProperties::DEVICE_LOCAL,
            super::BufferRequest {
                usage_flags: BufferUsageFlags::STORAGE_BUFFER,
                create_flags: Default::default(),
                size: 1024,
                main_owner_queue_family: family_index,
                staging_buffer: Some(
                    super::BufferStagingBufferInfo {
                        buffer: &staging_buffer,
                        regions: &[
                            BufferCopy {
                                src_offset: 0,
                                dst_offset: 0,
                                size: 1024
                            }
                        ]
                    }
                )
            }
        ).unwrap();

        let buffer = order.do_order().unwrap().wait().1.swap_remove(0);
    }
}