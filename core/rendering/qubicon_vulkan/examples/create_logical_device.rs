use std::collections::HashMap;

use qubicon_vulkan::{ Instance, device::DeviceCreateInfo, instance::{ AppId, InstanceCreateInfo, Version } };

const APP_ID: AppId = AppId {
    app_version: Version::new(0, 1, 0, 0),
    engine_version: Version::new(0, 1, 0, 0),

    vulkan_version: Version::new(0, 1, 0, 0),

    app_name: "Device enumerator",
    engine_name: "Shit"
};

fn main() {
    let instance = Instance::new(&InstanceCreateInfo { app_id: APP_ID })
        .expect("failed to create instance");


    let device = instance.enumerate_devices()
        .expect("failed to enumerate devices")
        .next()
        .expect("no devices found");


    let _logical_device = device.create_logical_device(
        DeviceCreateInfo {
            // no queues used
            queue_families: HashMap::new(),
            features: Default::default()
        }
    ).expect("failed to create logical device");
}