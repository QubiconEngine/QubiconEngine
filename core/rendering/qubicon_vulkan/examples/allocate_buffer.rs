// same case as **create_descriptor_pool**. This dont work on my setup because of nvidia
// if we allocate at least one queue, it starts working.

use std::sync::Arc;
use qubicon_vulkan::{
    Instance,
    device::create_info::QueueFamilyUsage,
    instance::physical_device::memory_properties::MemoryTypeProperties,
    memory::{resources::buffer::{
        BufferCreateInfo,
        BufferCreateFlags,
        BufferUsageFlags
    }, alloc::standart_device_memory_allocator::StandartMemoryAllocator}
};

fn main() {
    let instance = Instance::create(&Default::default())
        .expect("Failed to create instance");
    let device = instance.enumerate_devices()
        .expect("Failed to enumerate devices")
        .next()
        .expect("No devices found")
        .create_logical_device::<[QueueFamilyUsage; 0]>(Default::default())
        .expect("Failed to create logical device");

    let allocator = StandartMemoryAllocator::new(&device);

    let _buffer = device.create_buffer(
        Arc::clone(&allocator),
        MemoryTypeProperties::HOST_VISIBLE,
        &BufferCreateInfo {
            create_flags:   BufferCreateFlags::empty(),
            usage_flags:    BufferUsageFlags::STORAGE_BUFFER,
            size: 1024
        }
    ).expect("Buffer creation failed");
}