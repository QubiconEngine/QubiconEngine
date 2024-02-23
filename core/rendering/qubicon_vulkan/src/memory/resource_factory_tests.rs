use std::sync::Arc;

use crate::{device::create_info::{DeviceCreateInfo, QueueFamilyUsage}, instance::physical_device::{memory_properties::MemoryTypeProperties, queue_info::QueueFamilyCapabilities, PhysicalDevice}, Instance};

use super::{alloc::{hollow_device_memory_allocator::HollowDeviceMemoryAllocator, standart_device_memory_allocator::StandartMemoryAllocator}, resources::{buffer::{BufferCreateInfo, BufferUsageFlags}, format::Format, image::{ImageLayout, ImageSampleCountFlags, ImageTiling, ImageType, ImageUsageFlags}, image_view::{ImageAspect, ImageSubresourceLayers}}, ImageRequest, ResourceFactory};

fn queue_family_with_capability(dev: &PhysicalDevice, required_capabilities: QueueFamilyCapabilities) -> Option<u32> {
    dev.get_queue_family_infos()
        .iter()
        .enumerate()
        .find(| (_, family) | family.capabilities.contains(required_capabilities))
        .map(| (idx, _) | idx as u32)
}

#[test]
fn image_creation() {
    let instance = Instance::create(&Default::default())
        .unwrap();

    let (family_index, device) = instance.enumerate_devices()
        .unwrap()
        .filter_map(| dev | Some(
            (queue_family_with_capability(&dev, QueueFamilyCapabilities::TRANSFER)?, dev)
        ))
        .next()
        .unwrap();

    let device = device.create_logical_device(
        DeviceCreateInfo {
            queues: [
                QueueFamilyUsage {
                    family_index,
                    queue_count: 1
                }
            ].as_slice(),

            ..Default::default()
        }
    ).unwrap();

    let allocator = StandartMemoryAllocator::new(&device);
    let resource_factory = ResourceFactory::init(
        &device,
        device.get_queue(family_index, 0).unwrap(),
        family_index
    ).unwrap();

    let mut order = resource_factory.create_order(Arc::clone(&allocator)).unwrap();

    order.request_image::<HollowDeviceMemoryAllocator>(
        Default::default(),
        super::ImageRequest {
            format: Format::R8G8B8A8_SRGB,
            usage_flags: ImageUsageFlags::SAMPLED, 
            create_flags: Default::default(), 
            sample_count_flags: ImageSampleCountFlags::TYPE_1, 
            type_: ImageType::Type2D { width: 150, height: 150, miplevels_enabled: false }, 
            tiling: ImageTiling::Optimal, 
            array_layers: 1, 
            
            main_layout: ImageLayout::General, 
            main_owner_queue_family: family_index, 
            
            staging_buffer: None
        }).unwrap();

    let image = order.do_order().unwrap().wait().swap_remove(0);
}

#[test]
fn image_creation_with_staging_buffer() {
    let instance = Instance::create(&Default::default()).unwrap();

    let (family_index, device) = instance.enumerate_devices()
        .unwrap()
        .filter_map(| dev | Some(
            (queue_family_with_capability(&dev, QueueFamilyCapabilities::TRANSFER)?, dev)
        ))
        .next()
        .unwrap();

    let device = device.create_logical_device(
        DeviceCreateInfo {
            queues: [
                QueueFamilyUsage {
                    family_index,
                    queue_count: 1
                }
            ].as_slice(),

            ..Default::default()
        }
    ).unwrap();

    let allocator = StandartMemoryAllocator::new(&device);
    let resource_factory = ResourceFactory::init(
        &device,
        device.get_queue(family_index, 0).unwrap(),
        family_index
    ).unwrap();


    let staging_buffer = device.create_buffer(
        Arc::clone(&allocator),
        MemoryTypeProperties::HOST_VISIBLE,
        &BufferCreateInfo {
            usage_flags: BufferUsageFlags::TRANSFER_SRC,
            size: 1024,
            main_owner_queue_family: family_index,

            ..Default::default()
        }
    ).unwrap();


    unsafe { staging_buffer.map::<u32>() }
        .unwrap()
        .iter_mut()
        .for_each(| m | { m.write(0); });

    
    let mut order = resource_factory.create_order(Arc::clone(&allocator))
        .unwrap();

    order.request_image(
        MemoryTypeProperties::DEVICE_LOCAL,
        ImageRequest {
            format: Format::R8G8B8A8_UINT,
            usage_flags: ImageUsageFlags::STORAGE,
            create_flags: Default::default(),
            sample_count_flags: ImageSampleCountFlags::TYPE_1,
            tiling: ImageTiling::Optimal,
            type_: ImageType::Type2D { width: 16, height: 16, miplevels_enabled: false },
            array_layers: 1,
            main_layout: ImageLayout::General,
            main_owner_queue_family: family_index,
            staging_buffer: Some(
                super::StagingBufferInfo {
                    buffer: &staging_buffer,
                    offset: 0,
                    row_length: 16,
                    image_heigth: 16,
                    subresource: ImageSubresourceLayers {
                        aspect_mask: ImageAspect::COLOR,
                        mip_level: 0,
                        array_layers: 0..1
                    }
                }
            )
        }
    ).unwrap();

    let image = order.do_order().unwrap().wait().swap_remove(0);
}