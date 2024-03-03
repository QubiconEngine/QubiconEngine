use bitflags::bitflags;
use std::{
    sync::Arc,
    cell::Cell,
    marker::PhantomData,
};
use ash::vk::{
    Fence as VkFence,
    FenceCreateInfo as VkFenceCreateInfo,
    FenceCreateFlags as VkFenceCreateFlags
};

use crate::{
    Error,
    error::VkError,
    device::inner::DeviceInner
};



bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FenceCreateFlags: u32 {
        const CREATE_SIGNALED = 0x00000001;
    }
}
impl Into<VkFenceCreateFlags> for FenceCreateFlags {
    fn into(self) -> VkFenceCreateFlags {
        VkFenceCreateFlags::from_raw(self.bits())
    }
}
impl From<VkFenceCreateFlags> for FenceCreateFlags {
    fn from(value: VkFenceCreateFlags) -> Self {
        Self::from_bits_truncate(value.as_raw())
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FenceCreateInfo {
    pub flags: FenceCreateFlags
}
impl Into<VkFenceCreateInfo> for FenceCreateInfo {
    fn into(self) -> VkFenceCreateInfo {
        VkFenceCreateInfo {
            flags: self.flags.into(),

            ..Default::default()
        }
    }
}
impl From<VkFenceCreateInfo> for FenceCreateInfo {
    fn from(value: VkFenceCreateInfo) -> Self {
        Self { flags: value.flags.into() }
    }
}


/// One of **Vulkan** synchronization primitives
/// Used for waiting until operation on *GPU* is finished.
/// 
/// Waits unlimited time on drop
pub struct Fence {
    device: Arc<DeviceInner>,
    fence: VkFence,

    // To disable Sync implementation
    _ph: PhantomData<Cell<()>>
}

impl Fence {
    pub(crate) fn create(device: Arc<DeviceInner>, create_info: FenceCreateInfo) -> Result<Self, Error> {
        let fence = unsafe {
            device.create_fence(
                &create_info.into(),
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?
        };

        Ok(
            Self {
                device,
                fence,

                _ph: Default::default()
            }
        )
    }

    pub(crate) unsafe fn as_raw(&self) -> VkFence {
        self.fence
    }


    pub fn signaled(&self) -> Result<bool, VkError> {
        unsafe {
            self.device.get_fence_status(self.fence)
                .map_err(| e | VkError::try_from(e).unwrap_unchecked())
        }
    }

    pub fn wait(&self, timeout: u64) -> Result<(), VkError> {
        unsafe {
            self.device.wait_for_fences(
                &[self.fence],
                true,
                timeout
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())
        }
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_fence(
                self.fence,
                None
            )
        }
    }
}

// /// Structure what holds fence and additional data
// /// After fence is signaled, data can be obtained
// pub struct FenceGuard<D> {
//     fence: Fence,
//     data: D
// }

// impl<D> FenceGuard<D> {
//     pub fn new(fence: Fence, data: D) -> Self {
//         Self { fence, data }
//     }

//     pub fn signaled(&self) -> Result<bool, VkError> {
//         self.fence.signaled()
//     }

//     pub fn wait(self, timeout: u64) -> Result<D, (Self, VkError)> {
//         let data = self.data;
//         let fence = self.fence;
        
//         match fence.wait(timeout) {
//             Ok(_) => Ok(data),
//             Err(e) => Err((Self { fence, data }, e))
//         }
//     }
// }

// impl<D> From<(Fence, D)> for FenceGuard<D> {
//     fn from((fence, data): (Fence, D)) -> Self {
//         Self { fence, data }
//     }
// }