use std::{ pin::Pin, ffi::CStr, collections::VecDeque};
use libpulse_sys::*;

//use crate::{Error, Result};

mod callbacks {
    use libpulse_sys::*;
    use super::*;

    pub extern "C" fn stream_write_callback<F: Format>(stream: *mut pa_stream, writable_len: usize, data: *mut core::ffi::c_void) {
        unsafe {
            let data = &mut *data.cast::<PlaybackStream<F>>();
            let (first_chunk, second_chunk) = data.buf.as_slices();

            let first_len = core::mem::size_of_val(first_chunk);
            let second_len = core::mem::size_of_val(second_chunk);

            let mut total_written = 0;

            {
                let write_ammount = first_len.min(writable_len);

                total_written += write_ammount;

                let _ = pa_stream_write(
                    stream,
                    first_chunk.as_ptr().cast(),
                    write_ammount,
                    None,
                    0,
                    pa_seek_mode_t::Relative
                );
            }
            
            if writable_len > first_len {
                let writable_len = writable_len - first_len;
                let write_ammount = second_len.min(writable_len);

                total_written += write_ammount;
                
                let _ = pa_stream_write(
                    stream,
                    second_chunk.as_ptr().cast(),
                    write_ammount,
                    None,
                    0,
                    pa_seek_mode_t::Relative
                );
            }

            let _ = data.buf.drain(0..(total_written / core::mem::size_of::<F>()));
        }
    }

    pub extern "C" fn stream_underflow_callback(stream: *mut pa_stream, data: *mut core::ffi::c_void) {
        unsafe {
            let data = &mut *data.cast::<BaseStream>();
        }
    }
}

pub trait Format: format_trait_sealed::FormatSealed {}

impl<T: format_trait_sealed::FormatSealed> Format for T {}

mod format_trait_sealed {
    use libpulse_sys::*;
    
    pub trait FormatSealed {
        const FORMAT: pa_sample_format_t;
    }

    impl FormatSealed for u8 {
        const FORMAT: pa_sample_format_t = pa_sample_format_t::U8;
    }

    impl FormatSealed for i16 {
        #[cfg(target_endian = "little")]
        const FORMAT: pa_sample_format_t = pa_sample_format_t::S16le;
    
        #[cfg(target_endian = "big")]
        const FORMAT: pa_sample_format_t = pa_sample_format_t::S16be;
    }

    impl FormatSealed for i32 {
        #[cfg(target_endian = "little")]
        const FORMAT: pa_sample_format_t = pa_sample_format_t::S32le;
    
        #[cfg(target_endian = "big")]
        const FORMAT: pa_sample_format_t = pa_sample_format_t::S32be;
    }

    impl FormatSealed for f32 {
        #[cfg(target_endian = "little")]
        const FORMAT: pa_sample_format_t = pa_sample_format_t::F32le;
    
        #[cfg(target_endian = "big")]
        const FORMAT: pa_sample_format_t = pa_sample_format_t::F32be;
    }
    // TODO: more formats
}

pub struct BaseStream {
    stream: *mut pa_stream,

    rate: u32,
    channels: u8,

    paused: bool,
    
    _ph: std::marker::PhantomPinned
}

impl BaseStream {
    /// # Safety
    /// Ideally, value should be pinned to be safely usable in callbacks
    pub unsafe fn new_unpinned(ctx: *mut pa_context, name: &CStr, format: pa_sample_format_t, rate: u32, channels: u8) -> Self {
        if channels > PA_CHANNELS_MAX {
            panic!("channel count is {}, but max is {}", channels, PA_CHANNELS_MAX);
        }

        let sample_spec = pa_sample_spec {
            format,

            rate,
            channels
        };

        unsafe {
            let stream = pa_stream_new_with_proplist(
                ctx,
                name.as_ptr(),
                &sample_spec,
                core::ptr::null(),
                core::ptr::null_mut()
            );

            Self {
                stream,

                rate,
                channels,

                paused: false,

                _ph: Default::default()
            }
        }
    }
}

impl Drop for BaseStream {
    fn drop(&mut self) {
        unsafe {
            pa_stream_disconnect(self.stream);
            pa_stream_unref(self.stream);
        }
    }
}

pub struct PlaybackStream<F: Format> {
    base: BaseStream,
    buf: VecDeque<F>
}

impl<F: Format> PlaybackStream<F> {
    pub fn new(ctx: *mut pa_context, name: &CStr, rate: u32, channels: u8, preallocated_buffer_len: usize) -> Pin<Box<Self>> {
        unsafe {
            let base = BaseStream::new_unpinned(ctx, name, F::FORMAT, rate, channels);
            let buf = VecDeque::with_capacity(preallocated_buffer_len);

            let mut value = Box::pin( Self { base, buf } );

            {
                let this = value.as_mut().get_unchecked_mut();

                let _ = pa_stream_connect_playback(
                    this.base.stream,
                    core::ptr::null(),
                    core::ptr::null(),
                    Default::default(),
                    core::ptr::null(),
                    core::ptr::null_mut()
                );

                pa_stream_set_write_callback(this.base.stream, Some(callbacks::stream_write_callback::<F>), (this as *mut Self).cast());
                pa_stream_set_underflow_callback(this.base.stream, Some(callbacks::stream_underflow_callback), (this as *mut Self).cast());
            }

            value
        }
    }

    pub fn buf(&mut self) -> &mut VecDeque<F> {
        &mut self.buf
    }
}

pub struct RecordStream {
    base: BaseStream,
    buf: VecDeque<u16>
}