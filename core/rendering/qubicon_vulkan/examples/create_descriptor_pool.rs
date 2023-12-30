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
    shaders::PipelineShaderStageFlags,
    device::create_info::QueueFamilyUsage,
    instance::physical_device::memory_properties::MemoryTypeProperties,
};

fn main() {
    let instance = Instance::create(&Default::default()).unwrap();
    let device = instance.enumerate_devices()
        .unwrap()
        .next()
        .unwrap();

    println!("{}", device.get_properties().device_name);

    let device = device.create_logical_device::<[QueueFamilyUsage; 0]>(Default::default()).unwrap();
    let allocator = StandartMemoryAllocator::new(&device);

    let desc_layout = device.create_descriptor_set_layout(DescriptorSetLayoutCreateInfo {
        bindings: [
            DescriptorBinding {
                shader_stage_flags: PipelineShaderStageFlags::COMPUTE,
                r#type: qubicon_vulkan::descriptors::DescriptorType::StorageBuffer,
                count: 1
            },
            DescriptorBinding {
                shader_stage_flags: PipelineShaderStageFlags::COMPUTE,
                r#type: qubicon_vulkan::descriptors::DescriptorType::StorageBuffer,
                count: 1
            }
        ]
    })/*.unwrap()*/;

    let pool = device.create_descriptor_pool(
        DescriptorPoolCreateInfo {
            max_sets: 1,
            pool_sizes: [
                DescriptorPoolSize {
                    r#type: qubicon_vulkan::descriptors::DescriptorType::StorageBuffer,
                    count: 4
                }
            ]
        }
    )/*.unwrap()*/;

    let (set1, set2) = unsafe {
        let set1 = pool.allocate_descriptor_set_unchecked(Arc::clone(&desc_layout))/*.unwrap()*/;
        let set2 = pool.allocate_descriptor_set_unchecked(Arc::clone(&desc_layout))/*.unwrap()*/;

        (set1, set2)
    };

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
        set1.update_unchecked(&[
            DescriptorWrite {
                binding: 0,
                index: 0,
                write_info: BufferWriteInfo {
                    buffer: Arc::clone(&buffer),
                    offset: 0,
                    len: 1024
                }
            }
        ])
    }
}