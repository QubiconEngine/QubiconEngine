use libpulse_sys::*;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("context connection failed. {ctx_state:?}")]
    ContextConnectionFailed { ctx_state: pa_context_state_t },
    
    #[error("stream state is not ready, but {stream_state:?}")]
    StreamIsNotReady { stream_state: pa_stream_state_t },

    #[error("stream write error. {pa_error:?}")]
    StreamWriteError { pa_error: pa_error_code_t },

}