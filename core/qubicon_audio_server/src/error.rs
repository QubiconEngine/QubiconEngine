use libpulse_sys::*;
use thiserror::Error;

// TODO: Change this shit. These errors dont tell any information at all
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("context connection failed. {pa_error:?}")]
    ContextConnectionFailed { pa_error: pa_error_code_t },

    #[error("context is in bad state. {ctx_state:?}")]
    ContextBadState { ctx_state: pa_context_state_t },

    #[error("context update failed. {pa_error:?}")]
    ContextUpdateFailed { pa_error: pa_error_code_t },
    
    #[error("stream state is not ready, but {stream_state:?}")]
    StreamIsNotReady { stream_state: pa_stream_state_t },

    #[error("stream write error. {pa_error:?}")]
    StreamWriteError { pa_error: pa_error_code_t },

    #[error("proplist edit error. {pa_error:?}")]
    ProplistEditError { pa_error: pa_error_code_t }
}