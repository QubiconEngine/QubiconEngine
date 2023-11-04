use qubicon_vulkan::{
    Instance,
    instance::physical_device::memory_properties::MemoryTypeProperties,
    memory::resources::buffer::{
        BufferCreateInfo,
        BufferCreateFlags,
        BufferUsageFlags
    }
};

fn main() {
    let instance = Instance::create(&Default::default())
        .expect("Failed to create instance");
    let device = instance.enumerate_devices()
        .expect("Failed to enumerate devices")
        .next()
        .expect("No devices found")
        .create_logical_device(Default::default())
        .expect("Failed to create logical device");

    let buffer = unsafe {
        device.allocate_buffer(
            MemoryTypeProperties::HOST_VISIBLE,
            BufferCreateInfo {
            create_flags:   BufferCreateFlags::empty(),
            usage_flags:    BufferUsageFlags::STORAGE_BUFFER,

                size: 1024
            }
        )
    };
}