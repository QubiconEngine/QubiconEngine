use std::{ ffi::CStr, marker::PhantomData, pin::Pin, ops::Deref};
use libpulse_sys::*;

use crate::{Error, Result, raw::ChannelMap};

mod callbacks {
    use libpulse_sys::*;
    use super::*;

    // pub extern "C" fn stream_write_callback<F: Format>(stream: *mut pa_stream, writable_len: usize, data: *mut core::ffi::c_void) {
    //     unsafe {
    //         let data = &mut *data.cast::<PlaybackStream<F>>();
    //         let (first_chunk, second_chunk) = data.buf.as_slices();

    //         let first_len = core::mem::size_of_val(first_chunk);
    //         let second_len = core::mem::size_of_val(second_chunk);

    //         let mut total_written = 0;

    //         {
    //             let write_ammount = first_len.min(writable_len);

    //             total_written += write_ammount;

    //             let _ = pa_stream_write(
    //                 stream,
    //                 first_chunk.as_ptr().cast(),
    //                 write_ammount,
    //                 None,
    //                 0,
    //                 pa_seek_mode_t::Relative
    //             );
    //         }
            
    //         if writable_len > first_len {
    //             let writable_len = writable_len - first_len;
    //             let write_ammount = second_len.min(writable_len);

    //             total_written += write_ammount;
                
    //             let _ = pa_stream_write(
    //                 stream,
    //                 second_chunk.as_ptr().cast(),
    //                 write_ammount,
    //                 None,
    //                 0,
    //                 pa_seek_mode_t::Relative
    //             );
    //         }

    //         let _ = data.buf.drain(0..(total_written / core::mem::size_of::<F>()));
    //     }
    // }

    // pub extern "C" fn stream_underflow_callback(stream: *mut pa_stream, data: *mut core::ffi::c_void) {
    //     unsafe {
    //         let data = &mut *data.cast::<BaseStream>();
    //     }
    // }
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


pub trait StreamWrite<F> {
    fn available_len(self: Pin<&Self>) -> Result<usize>;
    fn write(self: Pin<&mut Self>, data: &[F]) -> Result<usize>;
}

pub trait StreamRead<F> {
    fn available_len(self: Pin<&Self>) -> Result<usize>;
    fn read(self: Pin<&mut Self>, data: &mut [F]) -> Result<usize>;
}


pub struct BaseStream {
    stream: *mut pa_stream,

    rate: u32,
    channel_map: ChannelMap,

    paused: bool,
    _ph: std::marker::PhantomPinned
}

impl BaseStream {
    /// # Safety
    /// Ideally, value should be pinned to be safely usable in callbacks
    pub unsafe fn new_unpinned(ctx: *mut pa_context, name: &CStr, format: pa_sample_format_t, rate: u32, channel_map: &ChannelMap) -> Self {
        let sample_spec = pa_sample_spec {
            format,

            rate,
            channels: channel_map.len() as u8
        };

        unsafe {
            let raw_channel_map = channel_map.into();
            let stream = pa_stream_new_with_proplist(
                ctx,
                name.as_ptr(),
                &sample_spec,
                &raw_channel_map,
                core::ptr::null_mut()
            );

            Self {
                stream,

                rate,
                channel_map: channel_map.clone(),

                paused: false,

                _ph: Default::default()
            }
        }
    }

    pub fn state(&self) -> pa_stream_state_t {
        unsafe { pa_stream_get_state(self.stream) }
    }
}

impl BaseStream {
    fn _is_ready(&self) -> Result<()> {
        let state = self.state();

        match state == pa_stream_state_t::Ready {
            true => Ok(()),
            false => Err(Error::StreamIsNotReady { stream_state: state })
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
    _ph: PhantomData<F>
}

impl<F: Format> PlaybackStream<F> {
    pub fn new(ctx: *mut pa_context, name: &CStr, rate: u32, channel_map: &ChannelMap) -> Result<Pin<Box<Self>>> {
        unsafe {
            let base = BaseStream::new_unpinned(ctx, name, F::FORMAT, rate, channel_map);
            let mut value = Box::pin( Self { base, _ph: Default::default() } );

            {
                let this = value.as_mut().get_unchecked_mut();

                handle_pa_error!(
                    pa_stream_connect_playback(
                        this.base.stream,
                        core::ptr::null(),
                        core::ptr::null(),
                        Default::default(),
                        core::ptr::null(),
                        core::ptr::null_mut()
                    )
                ).unwrap();

                //pa_stream_set_write_callback(this.base.stream, Some(callbacks::stream_write_callback::<F>), (this as *mut Self).cast());
                //pa_stream_set_underflow_callback(this.base.stream, Some(callbacks::stream_underflow_callback), (this as *mut Self).cast());
            }

            Ok ( value )
        }
    }
}

impl<F: Format> Deref for PlaybackStream<F> {
    type Target = BaseStream;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<F: Format> StreamWrite<F> for PlaybackStream<F> {
    fn available_len(self: Pin<&Self>) -> Result<usize> {
        // if not ready, return
        self._is_ready()?;
        
        unsafe {
            Ok ( pa_stream_writable_size(self.base.stream) / core::mem::size_of::<F>() )
        }
    }

    fn write(self: Pin<&mut Self>, data: &[F]) -> Result<usize> {
        // if not ready, return
        self._is_ready()?;

        unsafe {
            let mut len = core::mem::size_of_val(data);
            let mut dst = core::ptr::null_mut();

            handle_pa_error!(pa_stream_begin_write(self.base.stream, &mut dst, &mut len))
                .map_err(| e | Error::StreamWriteError { pa_error: e })?;

            core::ptr::copy_nonoverlapping(data.as_ptr(), dst.cast(), len / core::mem::size_of::<F>());

            handle_pa_error!(pa_stream_write(self.base.stream, dst, len, None, 0, pa_seek_mode_t::Relative))
                .inspect_err(| _ | { pa_stream_cancel_write(self.base.stream); })
                .map_err(| e | Error::StreamWriteError { pa_error: e })?;

            Ok( len / core::mem::size_of::<F>() )
        }
    }
}



pub struct RecordStream<F: Format> {
    base: BaseStream,
    _ph: PhantomData<F>
}

impl<F: Format> RecordStream<F> {
    pub fn new(ctx: *mut pa_context, name: &CStr, rate: u32, channel_map: &ChannelMap) -> Result<Pin<Box<Self>>> {
        unsafe {
            let base = BaseStream::new_unpinned(ctx, name, F::FORMAT, rate, channel_map);
            let mut value = Box::pin(Self { base, _ph: Default::default() });

            {
                let this = value.as_mut().get_unchecked_mut();

                handle_pa_error!(
                    pa_stream_connect_record(
                        this.stream,
                        core::ptr::null(),
                        core::ptr::null(),
                        Default::default()
                    )
                ).unwrap();

                // there should be code what registers callbacks
            }

            Ok ( value )
        }
    }
}

impl<F: Format> Deref for RecordStream<F> {
    type Target = BaseStream;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<F: Format> StreamRead<F> for RecordStream<F> {
    fn available_len(self: Pin<&Self>) -> Result<usize> {
        // if not ready, return
        self._is_ready()?;

        unsafe {
            Ok ( pa_stream_readable_size(self.base.stream) )
        }
    }
    fn read(self: Pin<&mut Self>, data: &mut [F]) -> Result<usize> {
        // if not ready, return
        self._is_ready()?;

        todo!()
    }
}