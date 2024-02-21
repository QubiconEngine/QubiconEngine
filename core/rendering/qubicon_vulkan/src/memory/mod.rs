pub mod alloc;
pub mod resources;

use std::{marker::PhantomData, sync::Arc};

use crate::{commands::{command_buffers::{self, command_buffer_builder::{barrier::{AccessFlags, BufferMemoryBarrier, ImageMemoryBarrier}, copy::BufferImageCopy}, CommandBufferBuilder, CommandBufferUsageFlags}, CommandPool}, device::inner::DeviceInner, instance::physical_device::memory_properties::MemoryTypeProperties, queue::{Queue, Submission}, shaders::PipelineStageFlags, sync::semaphore_types};
use self::{alloc::{hollow_device_memory_allocator::HollowDeviceMemoryAllocator, DeviceMemoryAllocator}, resources::{buffer::Buffer, format::Format, image::{Image, ImageCreateFlags, ImageCreateInfo, ImageLayout, ImageSampleCountFlags, ImageTiling, ImageType, ImageUsageFlags}, image_view::{ImageAspect, ImageSubresourceLayers, ImageSubresourceRange}}};

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
    pub fn create_order<Alloc: DeviceMemoryAllocator>(&self, allocator: Arc<Alloc>) -> Result<OrderBuilder<Alloc>, crate::Error> {
        let transfer_builder = self.transfer_pool.create_primary_command_buffer(
            CommandBufferUsageFlags::ONE_TIME_SUBMIT
        )?;

        Ok(
            OrderBuilder {
                factory: self,

                allocator,
                transfer_builder: Some(transfer_builder),
                image_list: Default::default()
            }
        )
    }
}

pub struct StagingBufferInfo<'a, A: DeviceMemoryAllocator> {
    buffer: &'a Buffer<A>,

    offset: u64,
    row_length: u32,
    image_heigth: u32,
    subresource: ImageSubresourceLayers
}
impl<'a, A: DeviceMemoryAllocator> Clone for StagingBufferInfo<'a, A> {
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

    pub staging_buffer: Option<StagingBufferInfo<'a, StagingAlloc>>
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

pub struct OrderBuilder<'a, Alloc: DeviceMemoryAllocator> {
    factory: &'a ResourceFactory,
    allocator: Arc<Alloc>,

    // I am piece of shit. Builder takes ownership of itself
    transfer_builder: Option<CommandBufferBuilder<'a, command_buffers::levels::Primary>>,
    image_list: Vec<Image<Alloc>>
}

impl<'a, Alloc: DeviceMemoryAllocator> OrderBuilder<'a, Alloc> {
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
                let staging_buffer_owner_family = staging_buffer.buffer.as_inner().info.main_owner_queue_family;
                
                let buffer_barriers = if self.factory.transfer_queue_family_index != staging_buffer_owner_family {
                    let start = BufferMemoryBarrier {
                        src_access_mask: Default::default(),
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
                        dst_access_mask: Default::default(),

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
                        Default::default(),
                        &[],
                        &[
                            ImageMemoryBarrier {
                                src_access_mask: AccessFlags::default(),
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
                        Default::default(),
                        &[],
                        &[
                            ImageMemoryBarrier {
                                src_access_mask: AccessFlags::TRANSFER_WRITE,
                                dst_access_mask: AccessFlags::default(),
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
                        Default::default(),
                        &[],
                        &[
                            ImageMemoryBarrier {
                                src_access_mask: Default::default(),
                                dst_access_mask: Default::default(),
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

    _ph: PhantomData<&'a ResourceFactory>
}

impl<'a, Alloc: DeviceMemoryAllocator> Order<'a, Alloc> {
    pub fn wait(self) -> Vec<Image<Alloc>> {
        // TODO: Currently this shit will block if error occured
        self.submission.wait_owned(u64::MAX)
            .map(move | _ | self.images)
            .map_err(| (_, e) | e)
            .expect("Failed to finish resource creation operations")
    } 
}