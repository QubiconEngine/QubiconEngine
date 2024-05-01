use std::sync::Arc;

use crate::{ error::VkError, device::{ Device, QueueFamilyIndex } };

pub struct Queue {
    device: Arc<Device>,

    family_index: QueueFamilyIndex,
    queue_index: u32,

    queue: ash::vk::Queue
}

impl Queue {
    pub(crate) unsafe fn as_raw(&self) -> ash::vk::Queue {
        self.queue
    }

    /// # Safety
    /// There should be at least ***queue_index*** *+ 1* queues specified in [QueueFamilyUsage]
    /// with **family_index** in [DeviceCreateInfo] during device creation
    /// 
    /// [QueueFamilyUsage]: crate::device::QueueFamilyUsage
    /// [DeviceCreateInfo]: crate::device::DeviceCreateInfo
    pub unsafe fn new_unchecked(device: Arc<Device>, family_index: QueueFamilyIndex, queue_index: u32) -> Self {
        let queue = device.as_raw().get_device_queue(
            family_index,
            queue_index
        );

        Self {
            device,
            
            family_index,
            queue_index,

            queue
        }
    }

    /// If **family_index** or **queue_index** is invalid, will return [VkError]::[InititalizationFailed]
    /// 
    /// [VkError]: crate::error::VkError
    /// [InititalizationFailed]: crate::error::VkError::InitializationFailed
    pub fn new(device: Arc<Device>, family_index: QueueFamilyIndex, queue_index: u32) -> Result<Self, VkError> {
        let usage = device.queue_families().get(&family_index)
            .ok_or(VkError::InitializationFailed)?;

        if queue_index as usize >= usage.queues.len() {
            return Err( VkError::InitializationFailed );
        }
        
        Ok( unsafe { Self::new_unchecked(device, family_index, queue_index) } )
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn family_index(&self) -> QueueFamilyIndex {
        self.family_index
    }

    pub fn queue_index(&self) -> u32 {
        self.queue_index
    }
}