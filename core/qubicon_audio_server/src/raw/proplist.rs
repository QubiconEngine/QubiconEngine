use std::ffi::CStr;
use libpulse_sys::*;

use crate::{Result, Error};


pub mod properties {
    #[allow(unused_imports)]
    pub use libpulse_sys::{
        PA_PROP_MEDIA_NAME as MEDIA_NAME,
        PA_PROP_MEDIA_ARTIST as MEDIA_ARTIST,
        PA_PROP_MEDIA_COPYRIGHT as MEDIA_COPYRIGHT,
        PA_PROP_MEDIA_SOFTWARE as MEDIA_SOFTWARE,
        PA_PROP_MEDIA_LANGUAGE  as MEDIA_LANGUAGE,
        PA_PROP_MEDIA_FILENAME as MEDIA_FILENAME,
        PA_PROP_MEDIA_ICON as MEDIA_ICON,
        PA_PROP_MEDIA_ICON_NAME as MEDIA_ICON_NAME,
        PA_PROP_MEDIA_ROLE as MEDIA_ROLE,
        PA_PROP_FILTER_WANT as FILTER_WANT,
        PA_PROP_FILTER_APPLY as FILTER_APPLY,
        PA_PROP_FILTER_SUPPRESS as FILTER_SUPPRESS,
        PA_PROP_EVENT_ID as EVENT_ID,
        PA_PROP_EVENT_DESCRIPTION as EVENT_DESCRIPTION,
        PA_PROP_EVENT_MOUSE_X as EVENT_MOUSE_X,
        PA_PROP_EVENT_MOUSE_Y as EVENT_MOUSE_Y,
        PA_PROP_EVENT_MOUSE_HPOS as EVENT_MOUSE_HPOS,
        PA_PROP_EVENT_MOUSE_VPOS as EVENT_MOUSE_VPOS,
        PA_PROP_EVENT_MOUSE_BUTTON as EVENT_MOUSE_BUTTON,
        PA_PROP_WINDOW_NAME as WINDOW_NAME,
        PA_PROP_WINDOW_ID as WINDOW_ID,
        PA_PROP_WINDOW_ICON as WINDOW_ICON,
        PA_PROP_WINDOW_ICON_NAME as WINDOW_ICON_NAME,
        PA_PROP_WINDOW_X as WINDOW_X,
        PA_PROP_WINDOW_Y as WINDOW_Y,
        PA_PROP_WINDOW_WIDTH as WINDOW_WIDTH,
        PA_PROP_WINDOW_HEIGHT as WINDOW_HEIGHT,
        PA_PROP_WINDOW_HPOS as WINDOW_HPOS,
        PA_PROP_WINDOW_VPOS as WINDOW_VPOS,
        PA_PROP_WINDOW_DESKTOP as WINDOW_DESKTOP,
        PA_PROP_WINDOW_X11_DISPLAY as WINDOW_X11_DISPLAY,
        PA_PROP_WINDOW_X11_SCREEN as WINDOW_X11_SCREEN,
        PA_PROP_WINDOW_X11_MONITOR as WINDOW_X11_MONITOR,
        PA_PROP_WINDOW_X11_XID as WINDOW_X11_XID,
        PA_PROP_APPLICATION_NAME as APPLICATION_NAME,
        PA_PROP_APPLICATION_ID as APPLICATION_ID,
        PA_PROP_APPLICATION_VERSION as APPLICATION_VERSION,
        PA_PROP_APPLICATION_ICON as APPLICATION_ICON,
        PA_PROP_APPLICATION_ICON_NAME as APPLICATION_ICON_NAME,
        PA_PROP_APPLICATION_LANGUAGE as APPLICATION_LANGUAGE,
        PA_PROP_APPLICATION_PROCESS_ID as APPLICATION_PROCESS_ID,
        PA_PROP_APPLICATION_PROCESS_BINARY as APPLICATION_PROCESS_BINARY,
        PA_PROP_APPLICATION_PROCESS_USER as APPLICATION_PROCESS_USER,
        PA_PROP_APPLICATION_PROCESS_HOST as APPLCIATION_PROCESS_HOST,
        PA_PROP_APPLICATION_PROCESS_MACHINE_ID as APPLICATION_MACHINE_ID,
        PA_PROP_APPLICATION_PROCESS_SESSION_ID as APPLICATION_PROCESS_SESSION_ID,
        PA_PROP_DEVICE_STRING as DEVICE_STRING,
        PA_PROP_DEVICE_API as DEVICE_API,
        PA_PROP_DEVICE_DESCRIPTION as DEVICE_DESCRIPTION,
        PA_PROP_DEVICE_BUS_PATH as DEVICE_BUS_PATH,
        PA_PROP_DEVICE_SERIAL as DEVICE_SERIAL,
        PA_PROP_DEVICE_VENDOR_ID as DEVICE_VENDOR_ID,
        PA_PROP_DEVICE_VENDOR_NAME as DEVICE_VENDOR_NAME,
        PA_PROP_DEVICE_PRODUCT_ID as DEVICE_PRODUCT_ID,
        PA_PROP_DEVICE_PRODUCT_NAME as DEVICE_PRODUCT_NAME,
        PA_PROP_DEVICE_CLASS as DEVICE_CLASS,
        PA_PROP_DEVICE_FORM_FACTOR as DEVICE_FORM_FACTOR,
        PA_PROP_DEVICE_BUS as DEVICE_BUS,
        PA_PROP_DEVICE_ICON as DEVICE_ICON,
        PA_PROP_DEVICE_ICON_NAME as DEVICE_ICON_NAME,
        PA_PROP_DEVICE_ACCESS_MODE as DEVICE_ACCESS_MODE,
        PA_PROP_DEVICE_MASTER_DEVICE as DEVICE_MASTER_DEVICE,
        PA_PROP_DEVICE_BUFFERING_BUFFER_SIZE as DEVICE_BUFFERING_BUFFER_SIZE,
        PA_PROP_DEVICE_BUFFERING_FRAGMENT_SIZE as DEVICE_BUFFERING_FRAGMENT_SIZE,
        PA_PROP_DEVICE_PROFILE_NAME as DEVICE_PROFILE_NAME,
        PA_PROP_DEVICE_INTENDED_ROLES as DEVICE_INTENDED_ROLES,
        PA_PROP_DEVICE_PROFILE_DESCRIPTION as DEVICE_PROFILE_DESCRIPTION,
        PA_PROP_MODULE_AUTHOR as MODULE_AUTHOR,
        PA_PROP_MODULE_USAGE as MODULE_USAGE,
        PA_PROP_MODULE_VERSION as MODULE_VERSION,
        PA_PROP_FORMAT_SAMPLE_FORMAT as FORMAT_SAMPLE_FORMAT,
        PA_PROP_FORMAT_RATE as FORMAT_RATE,
        PA_PROP_FORMAT_CHANNELS as FORMAT_CHANNELS,
        PA_PROP_FORMAT_CHANNEL_MAP as FORMAT_CHANNEL_MAP
        // there should be two more from newer versions of pulseaudio
    };
}


pub type UpdateMode = pa_update_mode_t;

pub struct Proplist (*mut pa_proplist);

impl Proplist {
    pub(crate) unsafe fn as_raw(&self) -> *mut pa_proplist {
        self.0
    }


    pub fn new() -> Self {
        unsafe { Self ( pa_proplist_new() ) }
    }

    pub fn size(&self) -> u32 {
        unsafe { pa_proplist_size(self.0) }
    }

    pub fn empty(&self) -> bool {
        unsafe { pa_proplist_isempty(self.0) > 0 }
    }

    pub fn contains(&self, key: &str) -> bool {
        unsafe {
            super::with_c_string(key, | key | pa_proplist_contains(self.0, key.as_ptr()) == 1)
        }
    }

    pub fn get<'a>(&'a self, key: &str) -> Option<&'a [u8]> {
        super::with_c_string(key, | key | {
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
        })
    }

    pub fn get_string<'a>(&'a self, key: &str) -> Option<&'a str> {
        super::with_c_string(key, | key | {
            let str = unsafe { pa_proplist_gets(self.0, key.as_ptr()) };

            if !str.is_null() {
                let str = unsafe { CStr::from_ptr(str) };

                // libpulse returns null if data is not in UTF-8
                return Some( unsafe { core::str::from_utf8_unchecked(str.to_bytes()) } )
            }

            None
        })
    }




    pub fn clear(&mut self) {
        unsafe { pa_proplist_clear(self.0) }
    }

    pub fn set(&mut self, key: &str, value: &[u8]) -> Result<()> {
        unsafe {
            super::with_c_string(key, | key | {
                handle_pa_error!(pa_proplist_set(self.0, key.as_ptr(), value.as_ptr().cast(), value.len()))
                    .map(| _ | ())
                    .map_err(| e | Error::ProplistEditError { pa_error: e })
            })
        }
    }

    pub fn set_string(&mut self, key: &str, value: &str) -> Result<()> {
        unsafe {
            // heavy af
            super::with_c_string(key, | key | {
                super::with_c_string(value, | value | {
                    handle_pa_error!(pa_proplist_sets(self.0, key.as_ptr(), value.as_ptr()))
                        .map(| _ | ())
                        .map_err(| e | Error::ProplistEditError { pa_error: e })
                })
            })
        }
    }

    pub fn unset(&mut self, key: &str) -> Result<()> {
        unsafe {
            super::with_c_string(key, | key | {
                handle_pa_error!(pa_proplist_unset(self.0, key.as_ptr()))
                    .map(| _ | ())
                    .map_err(| e | Error::ProplistEditError { pa_error: e })
            })
        }
    }

    /// UpdateMode::Set is equal to clone in some way
    pub fn update(&mut self, other: &Proplist, mode: UpdateMode) {
        unsafe { pa_proplist_update(self.0, mode, other.0); }
    }
}

impl Default for Proplist {
    fn default() -> Self {
        Self::new()
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



#[cfg(test)]
mod tests {
    use super::Proplist;

    #[test]
    fn is_empty() {
        let proplist = Proplist::new();

        assert!(proplist.empty());
    }

    #[test]
    #[should_panic]
    fn string_and_raw_values_incompatibility() {
        let mut proplist = Proplist::new();

        proplist.set("some_entry", &[ 1, 2, 3, 4, 5, 6, 7, 8, 9 ]).unwrap();

        // should panic there
        println!("{}", proplist.get_string("some_entry").unwrap())
    }

    #[test]
    fn string_entry() {
        let mut proplist = Proplist::new();

        proplist.set_string("some_string_entry", "random string").unwrap();

        assert_eq!(
            proplist.get_string("some_string_entry").unwrap(),
            "random string"
        )
    }

    #[test]
    fn raw_entry() {
        let mut proplist = Proplist::new();

        proplist.set("some_raw_value", &[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();

        assert_eq!(
            proplist.get("some_raw_value").unwrap(),
            &[1, 2, 3, 4, 5, 6, 7, 8]
        )
    }
}