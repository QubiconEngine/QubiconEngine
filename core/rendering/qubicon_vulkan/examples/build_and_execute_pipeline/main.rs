use std::sync::Arc;
use qubicon_vulkan::{
    shaders::{
        ShaderStageFlags,
        PipelineShaderStageCreateInfo,
        compute::ComputePipelineCreateInfo
    },
    device::create_info::{
        DeviceCreateInfo,
        QueueFamilyUsage
    },
    Instance,
    commands::command_buffers::{CommandBufferUsageFlags, command_buffer_builder::PipelineBindPoint},
    instance::physical_device::{queue_info::QueueFamilyCapabilities, memory_properties::MemoryTypeProperties}, sync::semaphore_types::Binary, descriptors::{DescriptorSetLayoutCreateInfo, DescriptorBinding, DescriptorType, DescriptorPoolCreateInfo, alloc::{DescriptorPoolSize, descriptor_set::{DescriptorWrite, BufferWriteInfo}}}, memory::{alloc::standart_device_memory_allocator::StandartMemoryAllocator, resources::buffer::{BufferCreateInfo, BufferUsageFlags}}
};

const SHADER: &[u8] = include_bytes!("shader.spv");

fn main() {
    let instance = Instance::create(&Default::default()).unwrap();
    let device = instance.enumerate_devices()
        .unwrap()
        .next()
        .unwrap();

    let compute_family_idx = device.get_queue_family_infos().iter()
        .enumerate()
        .find(| (_, family) | family.capabilities.contains(QueueFamilyCapabilities::COMPUTE))
        .map(| (i, _) | i as u32)
        .unwrap();

    let device = device.create_logical_device(DeviceCreateInfo {
        features: Default::default(),
        queues: [
            QueueFamilyUsage {
                family_index: compute_family_idx,
                queue_count: 1
            }
        ]
    }).unwrap();

    let mut shader_sources = Vec::<u32>::with_capacity(SHADER.len() / 4);

    unsafe {
        core::ptr::copy_nonoverlapping(SHADER.as_ptr().cast(), shader_sources.as_mut_ptr(), SHADER.len() / 4);
        shader_sources.set_len(SHADER.len() / 4);
    }

    let descriptor_set_layout = unsafe {
        device.create_descriptor_set_layout_unchecked(
            DescriptorSetLayoutCreateInfo {
                bindings: [
                    DescriptorBinding {
                        shader_stage_flags: ShaderStageFlags::COMPUTE,
                        r#type: DescriptorType::StorageBuffer,
                        count: 1
                    },
                    DescriptorBinding {
                        shader_stage_flags: ShaderStageFlags::COMPUTE,
                        r#type: DescriptorType::StorageBuffer,
                        count: 1
                    }
                ]
            }
        )
    }.unwrap();
    
    let pipeline_layout = device.create_pipeline_layout([Arc::clone(&descriptor_set_layout)]).unwrap();
    let shader_module = unsafe { device.create_shader_module_from_binary(&shader_sources) }.unwrap();

    let shader = unsafe {
        device.create_compute_pipeline_unchecked(
            ComputePipelineCreateInfo {
                create_flags: Default::default(),
                stage: PipelineShaderStageCreateInfo {
                    stage: ShaderStageFlags::COMPUTE,
                    module: &shader_module,
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
                    r#type: DescriptorType::StorageBuffer,
                    count: 2
                }
            ]
        }
    ).unwrap();

    let descriptor_set = unsafe {
        descriptor_pool.allocate_descriptor_set_unchecked(
            Arc::clone(&descriptor_set_layout)
        )
    }.unwrap();

    let allocator = StandartMemoryAllocator::new(&device);
    let src_buffer = device.create_buffer(
        Arc::clone(&allocator),
        MemoryTypeProperties::HOST_VISIBLE,
        &BufferCreateInfo {
            usage_flags: BufferUsageFlags::STORAGE_BUFFER,
            size: 1024 * 4,

            ..Default::default()
        }
    ).unwrap();
    let dst_buffer = device.create_buffer(
        Arc::clone(&allocator),
        MemoryTypeProperties::HOST_VISIBLE,
        &BufferCreateInfo {
            usage_flags: BufferUsageFlags::STORAGE_BUFFER,
            size: 1024 * 4,
            ..Default::default()
        }
    ).unwrap();

    unsafe {
        descriptor_set.update_unchecked(&[
            DescriptorWrite {
                binding: 0,
                index: 0,
                write_info: BufferWriteInfo {
                    buffer: Arc::clone(&src_buffer),
                    offset: 0,
                    len: 1024 * 4
                }
            },
            DescriptorWrite {
                binding: 1,
                index: 0,
                write_info: BufferWriteInfo {
                    buffer: Arc::clone(&dst_buffer),
                    offset: 0,
                    len: 1024 * 4
                }
            }
        ])
    }


    let queue = device.get_queue(compute_family_idx, 0)
        .unwrap();
    let command_pool = queue.create_command_pool().unwrap();
    let command_buffer = unsafe {
        command_pool.create_primary_command_buffer(CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .unwrap()
            .cmd_bind_descriptor_set_unchecked(PipelineBindPoint::Compute, 0, &pipeline_layout, &descriptor_set)
            .cmd_bind_compute_pipeline_unchecked(&shader)
            .cmd_dispatch_unchecked(1024, 1, 1)
            .build()
    }.unwrap();

    let _submit = queue.submit::<Binary, Binary>(
        [],
        [],
        [command_buffer]
    ).unwrap();
}