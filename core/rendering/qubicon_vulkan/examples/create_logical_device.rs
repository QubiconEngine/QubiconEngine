use qubicon_vulkan::device::create_info::DeviceCreateInfo;
use qubicon_vulkan::{Instance, instance::physical_device::properties::DeviceType, device::create_info::QueueFamilyUsage};

fn main() {
    let instance = Instance::create(&Default::default())
        .expect("Failed to create instance");

    let queue_usage = [
        QueueFamilyUsage {
            family_index: 0,
            queue_count: 1
        }
    ];

    let device = instance
        .enumerate_devices()
        .expect("Failed to enumerate physical devices")
        .find(| d | d.get_properties().device_type == DeviceType::IntegratedGpu)
        .expect("No devices found")
        .create_logical_device(DeviceCreateInfo {
            features: Default::default(),
            queues: queue_usage.to_vec()
        })
        .expect("Failed to create logical device");
    
    let _queue = device.get_queue(0, 0)
        .expect("Failed to create queue");
}