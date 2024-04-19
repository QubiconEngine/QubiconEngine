use qubicon_vulkan::{instance::{AppId, InstanceCreateInfo, Version}, Instance};

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
    let devices = instance.enumerate_devices()
        .expect("failed to enumerate devices");

    for device in devices {
        let properties = device.properties();
        let memory_properties = device.memory_properties();

        println!("Device {}\n\t{}", properties.device_name, properties.driver_version)
    }
}