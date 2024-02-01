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

    let mut shader_binary = Vec::<u32>::with_capacity(SHADER.len() / 4);

    unsafe {
        core::ptr::copy_nonoverlapping(SHADER.as_ptr().cast(), shader_binary.as_mut_ptr(), SHADER.len() / 4);
        shader_binary.set_len(SHADER.len() / 4);
    }

    let _shader_module = unsafe {
        device.create_shader_module_from_binary(
            &shader_binary
        )
    }.expect("Failed to create shader module");
}