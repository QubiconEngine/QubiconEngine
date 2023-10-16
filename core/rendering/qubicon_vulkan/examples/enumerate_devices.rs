use qubicon_vulkan::Instance;

fn main() {
    let instance = Instance::create(&Default::default())
        .expect("Failed to create instance");
    let devices = instance.enumerate_devices()
        .expect("Failed to enumerate devices");

    for device in devices {
        let features = device.get_features();
        let props = device.get_properties();

        println!("{}:{:?}\n", props.device_name, features);
    }
}