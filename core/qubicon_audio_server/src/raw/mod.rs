pub use stream::{Format, StreamRead, StreamWrite, PlaybackStream};
pub use proplist::{properties, Proplist, UpdateMode};
pub use channel_map::{ChannelMap, ChannelPosition};
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

fn with_c_string<R>(str: &str, op: impl FnOnce(&core::ffi::CStr) -> R) -> R {
    use smallstr::SmallString;

    let mut buf = SmallString::<[u8; 128]>::from_str(str);
    buf.push('\0');

    op(unsafe { core::ffi::CStr::from_ptr(buf.as_ptr().cast()) })
} 

pub mod stream;
pub mod context;
pub mod proplist;
pub mod channel_map;