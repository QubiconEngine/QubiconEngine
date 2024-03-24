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
                    .map_err(| _e | todo!())
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
                        .map_err(| _e | todo!())
                })
            })
        }
    }

    pub fn unset(&mut self, key: &str) -> Result<()> {
        unsafe {
            super::with_c_string(key, | key | {
                handle_pa_error!(pa_proplist_unset(self.0, key.as_ptr()))
                    .map(| _ | ())
                    .map_err(| _e | todo!())
            })
        }
    }

    /// UpdateMode::Set is equal to clone in some way
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



#[cfg(test)]
mod tests {
    use super::Proplist;

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