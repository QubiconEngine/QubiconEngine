use qubicon_vulkan::{
    Instance,
    commands::command_buffers::CommandBufferUsageFlags,
    device::create_info::{
        DeviceCreateInfo,
        QueueFamilyUsage
    },
};

fn main() {
    let instance = Instance::create(&Default::default()).unwrap();
    let device = instance.enumerate_devices()
        .unwrap()
        .next()
        .unwrap();

    println!("{}", device.get_properties().device_name);

    let queues = vec![
        QueueFamilyUsage {
            family_index: 0,
            queue_count: 1
        }
    ];
    
    let device = device.create_logical_device(
        DeviceCreateInfo {
            features: Default::default(),
            queues
        }
    ).unwrap();

    let queue = device.get_queue(0, 0).unwrap();
    let command_pool = queue.create_command_pool()/*.unwrap()*/;

    println!("{:?}", queue.get_capabilities());

    let _command_buffer = command_pool
        .create_primary_command_buffer(CommandBufferUsageFlags::SIMULTANEOUS_USE)
        .build()/*.unwrap()*/;
}