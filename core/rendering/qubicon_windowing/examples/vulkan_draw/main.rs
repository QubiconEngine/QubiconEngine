use std::sync::Arc;

use qubicon_vulkan::{commands::command_buffers::{command_buffer_builder::{barrier::{AccessFlags, ImageMemoryBarrier}, PipelineBindPoint}, CommandBufferUsageFlags}, descriptors::{alloc::{descriptor_set::{DescriptorWrite, ImageWriteInfo}, DescriptorPoolSize}, DescriptorBinding, DescriptorPoolCreateInfo, DescriptorSetLayoutCreateInfo, DescriptorType}, device::create_info::{DeviceCreateInfo, QueueFamilyUsage}, instance::{creation_info::InstanceCreateInfo, physical_device::queue_info::QueueFamilyCapabilities}, memory::{alloc::hollow_device_memory_allocator::HollowDeviceMemoryAllocator, resources::{image::{ImageLayout, ImageUsageFlags}, image_view::{ImageAspect, ImageSubresourceRange, ImageViewCreateInfo, ImageViewType}}}, queue::{PresentInfo, PresentInfoSwapchainEntry}, shaders::{compute::ComputePipelineCreateInfo, PipelineShaderStageCreateInfo, PipelineStageFlags, ShaderStageFlags}, sync::{semaphore_types::SemaphoreType, FenceCreateFlags, FenceCreateInfo}, Instance};
use qubicon_windowing::{x11::{WindowEvent, WindowingServer}, AssociatedSwapchainCreateInfo};

const SHADER_BIN: &[u8] = include_bytes!("shader_bin.spv");

fn main() {
    let mut win_server = WindowingServer::init();

    let vk_instance = Instance::create(&InstanceCreateInfo { enable_windowing: true })
        .expect("Failed to create Vulkan instance");

    let device = vk_instance.enumerate_devices()
        .expect("Failed to enumerate devices")
        .filter(| dev | dev.get_properties().device_name.starts_with("AMD"))
        .filter(| dev | dev.get_queue_family_infos()[0].capabilities.contains(QueueFamilyCapabilities::COMPUTE))
        .find(| dev | win_server.is_device_supports_presentation(0, dev).unwrap())
        .expect("Failed to find correct device")
        .create_logical_device(DeviceCreateInfo {
            features: Default::default(),
            enable_swapchain: true,

            queues: [
                QueueFamilyUsage {
                    family_index: 0,
                    queue_count: 1
                }
            ]
        }).expect("Failed to create logical device");

    let descriptor_set_layout = unsafe {
        device.create_descriptor_set_layout_unchecked(
            DescriptorSetLayoutCreateInfo {
                bindings: [
                    DescriptorBinding {
                        shader_stage_flags: ShaderStageFlags::COMPUTE,
                        r#type: DescriptorType::StorageImage,
                        count: 1
                    }
                ]
            }
        )
    }.unwrap();

    let pipeline_layout = device.create_pipeline_layout(
        [Arc::clone(&descriptor_set_layout)]
    ).unwrap();

    let shader = unsafe {
        let mut shader_sources = Vec::<u32>::with_capacity(SHADER_BIN.len() / 4);

        core::ptr::copy_nonoverlapping(SHADER_BIN.as_ptr().cast(), shader_sources.as_mut_ptr(), SHADER_BIN.len() / 4);
        shader_sources.set_len(SHADER_BIN.len() / 4);

        device.create_shader_module_from_binary(&shader_sources)
    }.unwrap();

    let pipeline = unsafe {
        device.create_compute_pipeline_unchecked(
            ComputePipelineCreateInfo {
                create_flags: Default::default(),
                stage: PipelineShaderStageCreateInfo {
                    stage: ShaderStageFlags::COMPUTE,
                    module: &shader,
                    entry_name: "main"
                },
                layout: Arc::clone(&pipeline_layout),
                base_pipeline: None
            }
        )
    }.unwrap();


    let descriptor_pool = device.create_descriptor_pool(
        DescriptorPoolCreateInfo {
            max_sets: 1,
            pool_sizes: [
                DescriptorPoolSize {
                    r#type: DescriptorType::StorageImage,
                    count: 1
                }
            ]
        }
    ).unwrap();

    let descriptor_set = unsafe {
        descriptor_pool.allocate_descriptor_set_unchecked(Arc::clone(&descriptor_set_layout))
    }.unwrap();

    let queue = device.get_queue(0, 0).unwrap();
    let command_pool = queue.create_command_pool().unwrap();


    
    let window_id = win_server.create_window_vulkan(
        &device,
        100,
        100,
        &AssociatedSwapchainCreateInfo {
            min_image_count: 1,
            image_array_layers: 1,
            image_usage: ImageUsageFlags::STORAGE,
            pre_transform: Default::default(),
            composite_alpha: Default::default(),
            clipped: false
        },

        // dont care what format and what mode
        | _ | true,
        | _ | true
    ).unwrap();

    win_server.window_mut(window_id).unwrap().show();

    'event_loop: loop {
        win_server.update();

        let mut window = win_server.window_mut(window_id).unwrap();

        { // Event handling
            let mut swapchain_resize_required = false;

            for event in window.events() {
                match event {
                    WindowEvent::Resize { .. } => swapchain_resize_required = true,
                    WindowEvent::Close => break 'event_loop,

                    _ => {},
                }

                if swapchain_resize_required {

                }
            }

            if swapchain_resize_required {
                window.force_swapchain_resize().unwrap()
            }
        }

        { // Rendering
            let swapchain = unsafe { window.swapchain_mut().unwrap_unchecked() };
            let image = unsafe {
                swapchain.acquare_next_image_unchecked::<qubicon_vulkan::sync::semaphore_types::Binary>(None, None, u64::MAX)
            }.unwrap();

            let image_view = unsafe {
                image.create_image_view_unchecked(
                    &ImageViewCreateInfo {
                        view_type: ImageViewType::Type2D,
                        format: image.format(),
                        components: Default::default(),
                        subresource_range: ImageSubresourceRange {
                            aspect_mask: ImageAspect::COLOR,
                            mip_levels: 0..1,
                            array_layers: 0..1
                        }
                    }
                )
            }.unwrap();

            unsafe {
                    descriptor_set.update_unchecked(
                    &[
                        DescriptorWrite {
                            binding: 0,
                            index: 0,
                            write_info: ImageWriteInfo {
                                sampler: None,
                                image_view: &image_view,
                                image_layout: ImageLayout::General
                            }
                        }
                    ]
                )
            }

            let cmd_buffer = command_pool.create_primary_command_buffer(
                CommandBufferUsageFlags::ONE_TIME_SUBMIT
            ).unwrap();

            let cmd_buffer = unsafe {
                cmd_buffer
                    .cmd_pipeline_barrier_unchecked::<_, HollowDeviceMemoryAllocator>(
                        PipelineStageFlags::TOP_OF_PIPE,
                        PipelineStageFlags::TOP_OF_PIPE,
                        Default::default(),
                        &[],
                        &[
                            ImageMemoryBarrier {
                                src_access_mask: Default::default(),
                                dst_access_mask: AccessFlags::SHADER_WRITE,
                                old_layout: ImageLayout::Undefined,
                                new_layout: ImageLayout::General,
                                src_queue_family_index: 0,
                                dst_queue_family_index: 0,
                                image: &image,
                                subresource_range: ImageSubresourceRange {
                                    aspect_mask: ImageAspect::COLOR,
                                    mip_levels: 0..1,
                                    array_layers: 0..1
                                }
                            }
                        ],
                        &[]
                    )
                    .cmd_bind_descriptor_set_unchecked(PipelineBindPoint::Compute, 0, &pipeline_layout, &descriptor_set)
                    .cmd_bind_compute_pipeline_unchecked(&pipeline)
                    .cmd_dispatch_unchecked(swapchain.image_extent().0, swapchain.image_extent().1, 1)
                    .cmd_pipeline_barrier_unchecked::<_, HollowDeviceMemoryAllocator>(
                        PipelineStageFlags::BOTTOM_OF_PIPE,
                        PipelineStageFlags::BOTTOM_OF_PIPE,
                        Default::default(),
                        &[],
                        &[
                            ImageMemoryBarrier {
                                src_access_mask: AccessFlags::SHADER_WRITE,
                                dst_access_mask: Default::default(),
                                old_layout: ImageLayout::General,
                                new_layout: ImageLayout::PresentSrc,
                                src_queue_family_index: 0,
                                dst_queue_family_index: 0,
                                image: &image,
                                subresource_range: ImageSubresourceRange {
                                    aspect_mask: ImageAspect::COLOR,
                                    mip_levels: 0..1,
                                    array_layers: 0..1
                                }
                            }
                        ],
                        &[]
                    )
                    .build().unwrap()
            };

            let semaphore = Arc::new(
                device.create_semaphore::<qubicon_vulkan::sync::semaphore_types::Binary>().unwrap()
            );
            
            let submission = queue.submit::<_, qubicon_vulkan::sync::semaphore_types::Binary>(
                core::iter::once(Arc::clone(&semaphore)),
                core::iter::empty(),
                core::iter::once(cmd_buffer)
            ).unwrap();

            queue.present(
                PresentInfo {
                    wait_semaphores: &[&semaphore],
                    entries: &mut [
                        PresentInfoSwapchainEntry {
                            swapchain: &swapchain,
                            swapchain_image: &image,
                            result: Ok(())
                        }
                    ]
                }
            );

            submission.wait(u64::MAX);
        };
    }
}