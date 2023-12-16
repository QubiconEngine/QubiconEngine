use std::sync::Arc;
use qubicon_vulkan::{
    Instance,
    shaders::{
        PipelineShaderStageFlags,
        PipelineShaderStageCreateInfo,
        compute::ComputePipelineCreateInfo
    }, device::create_info::{DeviceCreateInfo, QueueFamilyUsage}, instance::physical_device::queue_info::QueueFamilyCapabilities, commands::command_buffers::{command_buffer, CommandBufferUsageFlags}
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
    
    let pipeline_layout = device.create_pipeline_layout([]).unwrap();
    let shader_module = device.create_shader_module(&shader_sources).unwrap();

    let shader = device.create_compute_pipeline(
        ComputePipelineCreateInfo {
            create_flags: Default::default(),
            stage: PipelineShaderStageCreateInfo {
                stage: PipelineShaderStageFlags::COMPUTE,
                module: &shader_module,
                entry_name: "main"
            },
            layout: Arc::clone(&pipeline_layout),
            base_pipeline: None
        }
    ).unwrap();

    let queue = device.get_queue(compute_family_idx, 0)
        .unwrap();
    let command_pool = queue.create_command_pool();
    let command_buffer = unsafe {
        command_pool.create_primary_command_buffer(CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .cmd_bind_compute_pipeline_unchecked(&shader)
            .cmd_dispatch_unchecked(100, 100, 100)
            .build()
    };
}