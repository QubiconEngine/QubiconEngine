use libpulse_sys::*;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("context connection failed. {ctx_state:?}")]
    ContextConnectionFailed { ctx_state: pa_context_state_t }
}