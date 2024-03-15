use std::{ffi::{CStr, CString}, mem::MaybeUninit, pin::Pin};
use libpulse_sys::*;

pub mod error;

pub type Result<T> = core::result::Result<T, error::Error>;

extern "C" fn ctx_state_callback(ctx: *mut pa_context, data: *mut core::ffi::c_void) {
    // its ffi, all code is unsafe
    unsafe {
        let data = &mut *data.cast::<PulseData>();
        
        data.ctx_state = pa_context_get_state(ctx);
    }
}

struct PulseData {
    mainloop: *mut pa_mainloop,
    mainloop_api: *const pa_mainloop_api,

    ctx: *mut pa_context,
    ctx_state: pa_context_state_t
}

impl PulseData {
    // Type should be fixed in place. I.e pinned
    pub unsafe fn init(&mut self, name: &CStr) -> Result<()> {
        // set to some random value
        // this field should not contain some random data, because will be used in initialization process
        self.ctx_state = pa_context_state_t::Unconnected;

        self.mainloop = pa_mainloop_new();
        self.mainloop_api = pa_mainloop_get_api(self.mainloop);

        self.ctx = pa_context_new(self.mainloop_api, name.as_ptr());

        pa_context_connect(self.ctx, core::ptr::null(), Default::default(), core::ptr::null());
        pa_context_set_state_callback(self.ctx, Some(ctx_state_callback), (self as *mut Self).cast());

        
        // yes, its busy loop
        // ctx_state is changed in callback
        #[allow(clippy::while_immutable_condition)]
        while self.ctx_state != pa_context_state_t::Ready {
            pa_mainloop_iterate(self.mainloop, 1, core::ptr::null_mut());
            
            if !pa_context_is_good(self.ctx_state) {
                self.destroy_resources();

                return Err(error::Error::ContextConnectionFailed { ctx_state: self.ctx_state })
            }
        }


        Ok( () )
    }

    // after this structure will be unusable
    unsafe fn destroy_resources(&mut self) {
        unsafe {
            pa_context_disconnect(self.ctx);
            pa_context_unref(self.ctx);

            pa_mainloop_free(self.mainloop);
        }
    }
}

impl Drop for PulseData {
    fn drop(&mut self) {
        unsafe { self.destroy_resources() }
    }
}

pub struct AudioServer {
    data: Pin<Box<PulseData>>
}

impl AudioServer {
    // TODO: Add app name
    pub fn init() -> Result<Self> {
        // assume_init on Box is currently night only
        #[allow(invalid_value, clippy::uninit_assumed_init)]
        let mut data = Box::pin(
            unsafe {
                MaybeUninit::<PulseData>::uninit().assume_init()
            }
        );

        unsafe { data.init(&CString::new("test").unwrap()) }?;

        Ok ( Self { data } )
    }
}

#[cfg(test)]
mod tests {
    use super::AudioServer;

    #[test]
    fn little_test() {
        AudioServer::init().unwrap();
    }
}