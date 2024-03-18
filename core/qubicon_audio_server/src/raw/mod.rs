pub use stream::{Format, StreamWrite, PlaybackStream};
pub use context::PulseContext;

// unsafe macro. should not be used on non-pulse functions.
macro_rules! handle_pa_error {
    ( $call:expr ) => {
        {
            let r: i32 = $call;

            if r == 0 || r.is_positive() {
                Ok ( r )
            } else {
                use libpulse_sys::*;
                use num_traits::cast::FromPrimitive;

                let r: pa_error_code_t = pa_error_code_t::from_i32(r.abs()).unwrap_unchecked();

                Err ( r )
            }
        }
    };
}

pub mod stream;
pub mod context;