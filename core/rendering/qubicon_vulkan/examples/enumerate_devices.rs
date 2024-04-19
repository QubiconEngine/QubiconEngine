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

        println!(
            "Device {}\n\tDriver version: {}\n\tMemory types:",

            properties.device_name,
            properties.driver_version,
        );

        for (idx, ty) in memory_properties.memory_types.iter().enumerate() {
            println!(
                "\t\t№ {idx}:\n\t\t\tProperties: {:?}\n\t\t\tHeap: {}",

                 ty.properties,
                 ty.heap_index
            );
        }

        println!("\tMemory heaps:");

        for (idx, heap) in memory_properties.memory_heaps.iter().enumerate() {
            println!(
                "\t\t№ {idx}\n\t\t\tSize: {}\n\t\t\tProperties: {:?}",

                 heap.size,
                 heap.properties
            )
        }
    }
}