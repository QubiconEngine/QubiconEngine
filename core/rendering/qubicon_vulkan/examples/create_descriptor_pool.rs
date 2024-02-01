// this example for some reason gives SIGSEGV on my laptop with nvidia
// if we allocate some queue from device, it some how starts working

use std::sync::Arc;
use qubicon_vulkan::{
    Instance,
    descriptors::{
        DescriptorBinding,
        DescriptorPoolCreateInfo,
        alloc::{DescriptorPoolSize, descriptor_set::{DescriptorWrite, BufferWriteInfo}},
        DescriptorSetLayoutCreateInfo
    },
    memory::{resources::buffer::{
        BufferCreateInfo,
        BufferUsageFlags
    }, alloc::standart_device_memory_allocator::StandartMemoryAllocator},
    shaders::ShaderStageFlags,
    device::create_info::QueueFamilyUsage,
    instance::physical_device::memory_properties::MemoryTypeProperties,
};

fn main() {
    let instance = Instance::create(&Default::default()).unwrap();
    let device = instance.enumerate_devices()
        .unwrap()
        .next()
        .unwrap();

    let device = device.create_logical_device::<[QueueFamilyUsage; 0]>(Default::default()).unwrap();
    let allocator = StandartMemoryAllocator::new(&device);

    let desc_layout = unsafe {
        device.create_descriptor_set_layout_unchecked(DescriptorSetLayoutCreateInfo {
            bindings: [
                DescriptorBinding {
                    shader_stage_flags: ShaderStageFlags::COMPUTE,
                    r#type: qubicon_vulkan::descriptors::DescriptorType::StorageBuffer,
                    count: 1
                },
                DescriptorBinding {
                    shader_stage_flags: ShaderStageFlags::COMPUTE,
                    r#type: qubicon_vulkan::descriptors::DescriptorType::StorageBuffer,
                    count: 1
                }
            ]
        })
    }.unwrap();

    let pool = device.create_descriptor_pool(
        DescriptorPoolCreateInfo {
            max_sets: 1,
            pool_sizes: [
                DescriptorPoolSize {
                    r#type: qubicon_vulkan::descriptors::DescriptorType::StorageBuffer,
                    count: 2
                }
            ]
        }
    ).unwrap();

    let set = unsafe {
        pool.allocate_descriptor_set_unchecked(Arc::clone(&desc_layout))
    }.unwrap();

    let buffer = device.create_buffer(
        Arc::clone(&allocator),
        MemoryTypeProperties::HOST_VISIBLE,
        &BufferCreateInfo {
            usage_flags: BufferUsageFlags::STORAGE_BUFFER,
            size: 1024,

            ..Default::default()
        }
    ).unwrap();

    unsafe {
        set.update_unchecked(&[
            DescriptorWrite {
                binding: 0,
                index: 0,
                write_info: BufferWriteInfo {
                    buffer: &buffer,
                    offset: 0,
                    len: 1024
                }
            }
        ])
    }
}