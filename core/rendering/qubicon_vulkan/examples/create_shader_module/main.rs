use qubicon_vulkan::{Instance, device::create_info::QueueFamilyUsage};

const SHADER: &[u8] = include_bytes!("shader.spv");

fn main() {
    let instance = Instance::create(&Default::default())
        .expect("Failed to create instance");
    let device = instance.enumerate_devices()
        .expect("Failed to enumerate physical devices")
        .next()
        .expect("No vulkan devices found!");

    let device = device.create_logical_device::<[QueueFamilyUsage; 0]>(Default::default())
        .expect("Failed to create logical device");

    let _shader_module = device.create_shader_module(
        unsafe { core::mem::transmute(SHADER) }
    ).expect("Failed to create shader module");
}