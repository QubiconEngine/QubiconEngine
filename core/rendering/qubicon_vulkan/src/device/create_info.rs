use std::collections::HashMap;
use crate::instance::physical_device::{ DeviceFeatures, PhysicalDevice };

pub type QueueFamilyIndex = u32;

pub struct DeviceCreateInfo {
    pub queue_families: HashMap<QueueFamilyIndex, QueueFamilyUsage>,
    pub features: DeviceFeatures,
}

impl DeviceCreateInfo {
    pub fn validate(&self, device: &PhysicalDevice) {
        let queue_infos = device.get_queue_family_infos();

        for ( &family_index, family_usage ) in self.queue_families.iter() {
            let queue_info = queue_infos.get(family_index)
                .unwrap_or_else(|| panic!("device dont have queue family with index {family_index}"));

            if family_usage.queues.len() > queue_info.queue_count {
                panic!(
                    "too much queues with family index {} requested. max is {}, requested {}",
                    family_index,
                    queue_info.queue_count,
                    family_usage.queues.len()
                );
            }
        }
    }
}


pub struct QueueFamilyUsage {
    pub queues: Vec<f32>,

    // TODO: flags
}