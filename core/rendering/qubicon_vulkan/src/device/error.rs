use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError {
    #[error("no queue with index {queue_index} found")]
    NoQueueWithIndex { queue_index: u32 },
    #[error("no allocated queues with family index {family_index}")]
    NoQueueFamily{ family_index: u32 },
}