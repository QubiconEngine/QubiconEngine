use std::sync::Arc;

pub(crate) mod inner;

pub struct Queue {
    inner: Arc<inner::QueueInner>
}

impl From<Arc<inner::QueueInner>> for Queue {
    fn from(value: Arc<inner::QueueInner>) -> Self {
        Self { inner: value }
    }
}
