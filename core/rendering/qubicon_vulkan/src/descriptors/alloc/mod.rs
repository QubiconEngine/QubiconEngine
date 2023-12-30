use smallvec::SmallVec;
use std::sync::{
    Arc,
    Mutex
};
use ash::vk::{
    DescriptorPoolSize as VkDescriptorPoolSize,
    DescriptorPoolCreateInfo as VkDescriptorPoolCreateInfo,
    DescriptorPoolCreateFlags as VkDescriptorPoolCreateFlags,
    DescriptorSetAllocateInfo as VkDescriptorSetAllocateInfo
};

pub mod descriptor_set;
pub(crate) mod descriptor_pool_inner;

use crate::{
    Error,
    error::VkError,
    device::inner::DeviceInner
};
use super::{DescriptorType, DescriptorSetLayout};
use descriptor_set::DescriptorSet;
use descriptor_pool_inner::DescriptorPoolInner;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DescriptorPoolSize {
    pub r#type: DescriptorType,
    pub count: u32
}

impl From<VkDescriptorPoolSize> for DescriptorPoolSize {
    fn from(value: VkDescriptorPoolSize) -> Self {
        Self {
            r#type: value.ty.into(),
            count: value.descriptor_count
        }
    }
}
impl Into<VkDescriptorPoolSize> for DescriptorPoolSize {
    fn into(self) -> VkDescriptorPoolSize {
        VkDescriptorPoolSize {
            ty: self.r#type.into(),
            descriptor_count: self.count
        }
    }
}


#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorPoolCreateInfo<T: Into<Box<[DescriptorPoolSize]>> = Vec<DescriptorPoolSize>> {
    pub max_sets: u32,
    pub pool_sizes: T
}

/// Internaly syncronized via Mutex
pub struct DescriptorPool {
    inner: Arc<DescriptorPoolInner>
}

impl DescriptorPool {
    // TODO: Result
    pub(crate) fn new<T: Into<Box<[DescriptorPoolSize]>>>(
        device: Arc<DeviceInner>,
        create_info: DescriptorPoolCreateInfo<T>
    ) -> Result<Self, Error> {
        Self::new_with_vec_sizes(device, create_info.max_sets, create_info.pool_sizes.into())
    }

    pub unsafe fn allocate_descriptor_set_unchecked(&self, layout: Arc<DescriptorSetLayout>) -> Result<Arc<DescriptorSet>, Error> {
        let _lock = self.inner.tracker.lock().unwrap();
        
        unsafe {
            let descriptor_set = self.inner.device.allocate_descriptor_sets(
                &VkDescriptorSetAllocateInfo {
                    descriptor_pool: self.inner.descriptor_pool,
                    descriptor_set_count: 1,
                    p_set_layouts: &layout.descriptor_set_layout,

                    ..Default::default()
                }
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?[0];

            Ok(
                DescriptorSet::new(
                    Arc::clone(&self.inner),
                    descriptor_set,
                    layout
                )
            )
        }
    }

    // pub fn allocate_descriptor_set(&self, layout: Arc<DescriptorSetLayout>) -> Arc<DescriptorSet> {
    //     let tracker = self.inner.tracker.lock().unwrap();

    //     tracker.sets_tracker += 1;
        
    //     for binding in 
    // }
}

impl DescriptorPool {
    fn new_with_vec_sizes(
        device: Arc<DeviceInner>,
        max_sets: u32,
        pool_sizes: Box<[DescriptorPoolSize]>
    ) -> Result<Self, Error> {
        unsafe {
            let raw_sizes: SmallVec<[VkDescriptorPoolSize; 11]> = pool_sizes.iter()
                .copied()
                .map(Into::into)
                .collect();

            let descriptor_pool = device.create_descriptor_pool(
                &VkDescriptorPoolCreateInfo {
                    max_sets,
                    pool_size_count: raw_sizes.len() as u32,
                    p_pool_sizes: raw_sizes.as_ptr(),

                    flags: VkDescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,

                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            let tracker = descriptor_pool_inner::Tracker {
                sets_tracker: 0,
                pool_sizes_tracker: pool_sizes.iter().map(| s | s.count).collect()
            };

            Ok(
                Self {
                    inner: Arc::new(
                        DescriptorPoolInner {
                            device,
                            descriptor_pool,

                            max_sets,
                            pool_sizes,

                            tracker: Mutex::new(tracker)
                        }
                    )
                }
            )
        }
    }
}