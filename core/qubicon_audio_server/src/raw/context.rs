use core::{ pin::Pin, ffi::CStr };
use libpulse_sys::*;

use crate::{ Result, Error, raw::PlaybackStream };

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

        pa_context_connect(this.ctx, core::ptr::null(), Default::default(), core::ptr::null());
        pa_context_set_state_callback(this.ctx, Some(ctx_state_callback), (this as *mut Self).cast());

        
        // yes, its busy loop
        // ctx_state is changed in callback
        #[allow(clippy::while_immutable_condition)]
        while this.ctx_state != pa_context_state_t::Ready {
            pa_mainloop_iterate(this.mainloop, 1, core::ptr::null_mut());
            
            if !pa_context_is_good(this.ctx_state) {
                this.destroy_resources();

                return Err(Error::ContextConnectionFailed { ctx_state: this.ctx_state })
            }
        }


        Ok( () )
    }

    pub fn update(&self) {
        unsafe { pa_mainloop_iterate(self.mainloop, 0, core::ptr::null_mut()) };
    }

    pub fn create_new_playback_stream(&self, name: &CStr, rate: u32, channels: u8, preallocated_buffer_len: usize) -> Pin<Box<PlaybackStream>> {
        PlaybackStream::new(self.ctx, name, rate, channels, preallocated_buffer_len)
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