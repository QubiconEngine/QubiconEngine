use std::ffi::CStr;
use libpulse_sys::*;

use crate::{Result, Error};

pub type UpdateMode = pa_update_mode_t;

pub struct Proplist (*mut pa_proplist);

impl Proplist {
    pub fn new() -> Self {
        unsafe { Self ( pa_proplist_new() ) }
    }

    pub fn size(&self) -> u32 {
        unsafe { pa_proplist_size(self.0) }
    }

    pub fn empty(&self) -> bool {
        unsafe { pa_proplist_isempty(self.0) > 0 }
    }

    pub fn contains(&self, key: &CStr) -> bool {
        unsafe { pa_proplist_contains(self.0, key.as_ptr()) == 1}
    }

    pub fn get<'a>(&'a self, key: &CStr) -> Option<&'a [u8]> {
        let (ptr, size) = unsafe {
            let mut size = 0;
            let mut ptr = core::ptr::null();

            let res = pa_proplist_get(self.0, key.as_ptr(), &mut ptr, &mut size);

            if res != 0 {
                return None;
            }

            (ptr, size)
        };

        Some( unsafe { core::slice::from_raw_parts(ptr.cast(), size) } )
    }

    pub fn get_string<'a>(&'a self, key: &CStr) -> Option<&'a str> {
        let str = unsafe { pa_proplist_gets(self.0, key.as_ptr()) };

        if !str.is_null() {
            let str = unsafe { CStr::from_ptr(str) };

            // libpulse returns null if data is not in UTF-8
            return Some( unsafe { core::str::from_utf8_unchecked(str.to_bytes()) } )
        }

        None
    }




    pub fn clear(&mut self) {
        unsafe { pa_proplist_clear(self.0) }
    }

    pub fn set(&mut self, key: &CStr, value: &[u8]) -> Result<()> {
        unsafe {
            handle_pa_error!(pa_proplist_set(self.0, key.as_ptr(), value.as_ptr().cast(), value.len()))
                .map(| _ | ())
                .map_err(| _e | todo!())
        }
    }

    pub fn set_string(&mut self, key: &CStr, value: &CStr) -> Result<()> {
        unsafe {
            handle_pa_error!(pa_proplist_sets(self.0, key.as_ptr(), value.as_ptr()))
                .map(| _ | ())
                .map_err(| _e | todo!())
        }
    }

    pub fn unset(&mut self, key: &CStr) -> Result<()> {
        unsafe {
            handle_pa_error!(pa_proplist_unset(self.0, key.as_ptr()))
                .map(| _ | ())
                .map_err(| _e | todo!())
        }
    }

    /// Set is equal to clone in some way
    pub fn update(&mut self, other: &Proplist, mode: UpdateMode) {
        unsafe { pa_proplist_update(self.0, mode, other.0); }
    }
}

impl PartialEq for Proplist {
    fn eq(&self, other: &Self) -> bool {
        unsafe { pa_proplist_equal(self.0, other.0) > 0 }
    }
}

impl Eq for Proplist {}

impl Clone for Proplist {
    fn clone(&self) -> Self {
        unsafe { Self ( pa_proplist_copy(self.0) ) }
    }

    fn clone_from(&mut self, source: &Self) {
        self.update(source, UpdateMode::Set)
    }
}

impl Drop for Proplist {
    fn drop(&mut self) {
        unsafe { pa_proplist_free(self.0); }
    }
}