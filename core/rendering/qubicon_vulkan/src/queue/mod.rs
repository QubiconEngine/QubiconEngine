use std::sync::Arc;
use smallvec::SmallVec;
use ash::vk::{
    SubmitInfo as VkSubmitInfo,
    CommandPoolCreateInfo as VkCommandPoolCreateInfo
};
use crate::{
    commands::{
        CommandPool,
        command_pool_inner::CommandPoolInner,
        command_buffers::{
            levels,
            CommandBuffer
        }
    },
    sync::{
        Fence,
        Semaphore,
        semaphore_types,
        semaphore_types::SemaphoreType
    },
    Error,
    error::VkError,
    instance::physical_device::queue_info::QueueFamilyCapabilities, shaders::PipelineStageFlags
};


pub(crate) mod inner;

pub struct Queue {
    inner: Arc<inner::QueueInner>
}

impl From<Arc<inner::QueueInner>> for Queue {
    fn from(value: Arc<inner::QueueInner>) -> Self {
        Self { inner: value }
    }
}

impl Queue {
    pub fn get_family_index(&self) -> u32 {
        self.inner.family_index
    }
    pub fn get_queue_index(&self) -> u32 {
        self.inner.queue_index
    }
    pub fn get_capabilities(&self) -> QueueFamilyCapabilities {
        self.inner.capabilities
    }

    pub fn create_command_pool(&self) -> Result<CommandPool, Error> {
        unsafe {
            let command_pool = self.inner.device.create_command_pool(
                &VkCommandPoolCreateInfo {
                    queue_family_index: self.inner.family_index,
                    ..Default::default()
                },
                None
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

            Ok(
                Arc::new(
                    CommandPoolInner::new(
                        Arc::clone(&self.inner.device),
                        Arc::clone(&self.inner),
                        command_pool
                    )
                ).into()
            )
        }
    }

    /// TODO: Add ability to create multiple submissions at single time
    pub fn submit<SS: SemaphoreType, SW: SemaphoreType>(
        &self,
        signals: impl IntoIterator<Item = Arc<Semaphore<SS>>>,
        waits: impl IntoIterator<Item = (Arc<Semaphore<SW>>, PipelineStageFlags)>,
        command_buffers: impl IntoIterator<Item = CommandBuffer<levels::Primary>>
    ) -> Result<Submission<CommandBuffer<levels::Primary>, SS, SW>, Error> {
        let fence = Fence::create(
            Arc::clone(&self.inner.device),
            Default::default()
        )?;

        let signals: SmallVec<[Arc<Semaphore<SS>>; 1]> = signals.into_iter().collect();
        let waits: SmallVec<[(Arc<Semaphore<SW>>, PipelineStageFlags); 1]> = waits.into_iter().collect();
        let command_buffers: SmallVec<[CommandBuffer<levels::Primary>; 1]> = command_buffers.into_iter().collect();

        let raw_signal_semaphores: SmallVec<[_; 1]> = signals.iter()
            .map(| s | unsafe { s.as_raw() })
            .collect();
        let raw_wait_semaphores: SmallVec<[_; 1]> = waits.iter()
            .map(| (s, _) | unsafe { s.as_raw() })
            .collect();
        let raw_wait_masks: SmallVec<[_; 1]> = waits.iter()
            .map(| (_, m) | (*m).into())
            .collect();
        let raw_command_buffers: SmallVec<[_; 1]> = command_buffers.iter()
            .map(| c | c.command_buffer)
            .collect();

        let submit_info = VkSubmitInfo {
            wait_semaphore_count: raw_wait_semaphores.len() as u32,
            p_wait_semaphores: raw_wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: raw_wait_masks.as_ptr(),
            command_buffer_count: raw_command_buffers.len() as u32,
            p_command_buffers: raw_command_buffers.as_ptr(),
            signal_semaphore_count: raw_signal_semaphores.len() as u32,
            p_signal_semaphores: raw_signal_semaphores.as_ptr(),

            ..Default::default()
        };

        unsafe {
            self.inner.device.queue_submit(
                self.inner.queue,
                &[submit_info],
                fence.as_raw()
            ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?
        }

        let waits = waits.into_iter().map(| (s, _) | s).collect();

        Ok (
            Submission {
                fence,
                semaphore_to_wait: waits,
                semaphore_to_signal: signals,
                command_buffers: Some(command_buffers)
            }
        )
    }
}


/// Represents multiple command buffers, what being run on a queue
pub struct Submission<C, SS: SemaphoreType = semaphore_types::Binary, SW: SemaphoreType = semaphore_types::Binary> {
    fence: Fence,
    semaphore_to_wait: SmallVec<[Arc<Semaphore<SW>>; 1]>,
    semaphore_to_signal: SmallVec<[Arc<Semaphore<SS>>; 1]>,

    command_buffers: Option<SmallVec<[C; 1]>>
}

impl<C, SS: SemaphoreType, SW: SemaphoreType> Submission<C, SS, SW> {
    pub fn wait(&self, timeout: u64) -> Result<(), VkError> {
        self.fence.wait(timeout)
    }
    
    pub fn wait_owned(mut self, timeout: u64) -> Result<SmallVec<[C; 1]>, (Self, VkError)> {
        self.wait(timeout)
            .map(| _ | unsafe { self.command_buffers.take().unwrap_unchecked() })
            .map_err(| e | (self, e))
    }
}

impl<C, SS: SemaphoreType, SW: SemaphoreType> Drop for Submission<C, SS, SW> {
    fn drop(&mut self) {
        self.fence.wait(u64::MAX)
            .expect("Failed to finish queue submission on destruction")
    }
}