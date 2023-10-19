use qubicon_vulkan::Instance;

fn main() {
    let instance = Instance::create(&Default::default())
        .expect("Failed to create instance");
    let devices = instance.enumerate_devices()
        .expect("Failed to enumerate devices");

    for device in devices {
        let props = device.get_properties();
        println!("{}:", props.device_name);

        for queue in device.get_queue_family_infos() {
            println!(
                "Queue family have {} queues with {:?}",
                queue.queue_count,
                queue.capabilities
            )
        }

        println!();
    }
}