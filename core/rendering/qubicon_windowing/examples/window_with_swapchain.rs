// Currently causes lots of errors from validation layers

use qubicon_vulkan::{device::create_info::{DeviceCreateInfo, QueueFamilyUsage}, instance::{creation_info::InstanceCreateInfo, physical_device::{queue_info::QueueFamilyCapabilities, PhysicalDevice}}, memory::resources::image::ImageUsageFlags, surface::{ColorSpace, CompositeAlphaFlags, SurfaceTransformFlags}, swapchain::SwapchainCreateInfo, Instance};
use qubicon_windowing::{x11::{WindowEvent, WindowingServer}, AssociatedSwapchainCreateInfo};

fn find_family_index(dev: &PhysicalDevice) -> Option<u32> {
    dev.get_queue_family_infos().iter()
        .enumerate()
        .find(| (_, family) | family.capabilities.contains(QueueFamilyCapabilities::COMPUTE))
        .map(| (i, _) | i as u32)
} 

fn main() {
    let mut win_server = WindowingServer::init();

    let instance = Instance::create(&InstanceCreateInfo { enable_windowing: true })
        .expect("Failed to create vulkan instance");

    let (family_index, device) = instance.enumerate_devices()
        .expect("Failed to enumerate devices")
        .filter_map(| dev | Some(( find_family_index(&dev)?, dev )))
        .find(| (family_idx, dev) | 
            win_server
                .is_device_supports_presentation(*family_idx, dev)
                .expect("What the fuck?"))
        .expect("No device found");

    let device = device.create_logical_device(
        DeviceCreateInfo {
            enable_swapchain: true,
            queues: [
                QueueFamilyUsage {
                    family_index,
                    queue_count: 1
                }
            ],

            ..Default::default()
        }
    ).expect("Failed to create logical device");


    let window_id = win_server.create_window_vulkan(
        &device,
        100,
        100,
        &AssociatedSwapchainCreateInfo {
            min_image_count: 4,
            image_array_layers: 1,
            image_usage: ImageUsageFlags::STORAGE,
            pre_transform: SurfaceTransformFlags::IDENTITY,
            composite_alpha: CompositeAlphaFlags::OPAQUE,
            clipped: false
        },
        | _ | true,
        | _ | true 
    ).expect("Failed to create window");

    win_server.window_mut(window_id).unwrap().show();

    'event_loop: loop {
        win_server.update();

        let mut window = win_server.window_mut(window_id).unwrap();
        let mut swapchain_resize_required = false;

        for event in window.events() {
            match event {
                WindowEvent::Resize { .. } => swapchain_resize_required = true,
                WindowEvent::Close => break 'event_loop,

                _ => {}
            }
        }

        if swapchain_resize_required {
            window.force_swapchain_resize().unwrap();
        }
    }
}