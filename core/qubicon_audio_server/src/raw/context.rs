use core::{ pin::Pin, ffi::CStr };
use libpulse_sys::*;

use crate::{ Result, Error, raw::{ Format, ChannelMap, PlaybackStream } };

extern "C" fn ctx_state_callback(ctx: *mut pa_context, data: *mut core::ffi::c_void) {
    // its ffi, all code is unsafe
    unsafe {
        let data = &mut *data.cast::<PulseContext>();
        
        data.ctx_state = pa_context_get_state(ctx);
    }
}

pub struct PulseContext {
    mainloop: *mut pa_mainloop,
    mainloop_api: *const pa_mainloop_api,

    ctx: *mut pa_context,
    ctx_state: pa_context_state_t,

    _ph: std::marker::PhantomPinned
}

impl PulseContext {
    // rewrites all data whats inside without destructor
    pub unsafe fn init(self: Pin<&mut Self>, name: &CStr) -> Result<()> {
        let this = self.get_unchecked_mut();

        // set to some random value
        // this field should not contain some random data, because will be used in initialization process
        this.ctx_state = pa_context_state_t::Unconnected;

        this.mainloop = pa_mainloop_new();
        this.mainloop_api = pa_mainloop_get_api(this.mainloop);

        this.ctx = pa_context_new(this.mainloop_api, name.as_ptr());

        handle_pa_error!(pa_context_connect(this.ctx, core::ptr::null(), Default::default(), core::ptr::null()))
            .inspect_err(| _ | { pa_context_unref(this.ctx); pa_mainloop_free(this.mainloop); })
            .map_err(| e | Error::ContextConnectionFailed { pa_error: e })?;
        
        pa_context_set_state_callback(this.ctx, Some(ctx_state_callback), (this as *mut Self).cast());

        
        // yes, its busy loop
        // ctx_state is changed in callback
        #[allow(clippy::while_immutable_condition)]
        while this.ctx_state != pa_context_state_t::Ready {
            handle_pa_error!(pa_mainloop_iterate(this.mainloop, 1, core::ptr::null_mut()))
                .inspect_err(| _ | this.destroy_resources())
                .map_err(| e | Error::ContextConnectionFailed { pa_error: e })?;
            
            if !pa_context_is_good(this.ctx_state) {
                this.destroy_resources();

                return Err(Error::ContextBadState { ctx_state: this.ctx_state })
            }
        }


        Ok( () )
    }

    pub fn update(&self) -> Result<()> {
        unsafe {
            handle_pa_error!(pa_mainloop_iterate(self.mainloop, 0, core::ptr::null_mut()))
                .map(| _ | ())
                .map_err(| e | Error::ContextUpdateFailed { pa_error: e })
        }
    }

    pub fn create_new_playback_stream<F: Format>(&self, name: &CStr, rate: u32, channel_map: ChannelMap) -> Result<Pin<Box<PlaybackStream<F>>>> {
        PlaybackStream::new(self.ctx, name, rate, channel_map)
    }
}

impl PulseContext {
    // after this structure will be unusable
    unsafe fn destroy_resources(&mut self) {
        unsafe {
            pa_context_disconnect(self.ctx);
            pa_context_unref(self.ctx);

            pa_mainloop_free(self.mainloop);
        }
    }
}

impl Drop for PulseContext {
    fn drop(&mut self) {
        unsafe { self.destroy_resources() }
    }
}