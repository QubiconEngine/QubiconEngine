use std::path::Path;
use arrayvec::ArrayString;

use bitvec::{bitarr, BitArr};
use keymaps::{State, Relative, Key, Abs, Ev};
use nix::{libc, unistd, fcntl, sys, Result};

#[allow(dead_code)]
mod ioctl {
    use nix::libc;
    use nix::{ioctl_read, ioctl_write_int, ioctl_read_buf, ioctl_write_ptr};

    ioctl_read!(eviocgversion, b'E', 0x01, i32);
    ioctl_read!(eviocgid, b'E', 0x02, libc::input_id);
    ioctl_read!(eviocgrep, b'E', 0x03, [u32; 2]);
    ioctl_write_ptr!(eviocsrep, b'E', 0x03, [u32; 2]);
    ioctl_read!(eviocgkeycode, b'E', 0x04, [u32; 2]);
    ioctl_read!(eviocgkeycode2, b'E', 0x04, libc::input_keymap_entry);
    ioctl_write_ptr!(eviocskeycode, b'E', 0x05, [u32; 2]);
    ioctl_write_ptr!(eviocskeycode2, b'E', 0x05, libc::input_keymap_entry);
    ioctl_read_buf!(eviocgname, b'E', 0x06, u8);
    ioctl_read_buf!(eviocgphys, b'E', 0x07, u8);
    ioctl_read_buf!(eviocguniq, b'E', 0x08, u8);
    ioctl_read_buf!(eviocgprop, b'E', 0x09, u8);
    ioctl_read_buf!(eviocgmtslots, b'E', 0x0a, u8);
    ioctl_read_buf!(eviocgled, b'E', 0x19, u8); // \ todo: full function
    ioctl_read_buf!(eviocgsnd, b'E', 0x1a, u8); // \
    ioctl_read_buf!(eviocgsw, b'E', 0x1b, u8);  // \
    ioctl_write_ptr!(eviocsff, b'E', 0x80, libc::ff_effect);
    ioctl_write_int!(eviocrmff, b'E', 0x81);
    ioctl_read!(eviocgeffects, b'E', 0x84, i32);

    ioctl_write_int!(eviocgrab, b'E', 0x90);
    ioctl_write_int!(eviocrevoke, b'E', 0x91);

    ioctl_read!(eviocgmask, b'E', 0x92, libc::input_mask);
    ioctl_write_ptr!(eviocsmask, b'E', 0x93, libc::input_mask);
    ioctl_write_int!(eviocsclockid, b'E', 0xa0);

    pub unsafe fn eviocgkey(fd: libc::c_int, len: usize /* len in bits */, data: *mut u8) -> nix::Result<libc::c_int> {
        let res = libc::ioctl(
            fd,
            nix::request_code_read!(
                b'E',
                0x18,
                len
            ),
            data
        );

        nix::errno::Errno::result(res)
    }

    pub unsafe fn eviocgbit(fd: libc::c_int, ev: u16, len: usize /* len in bits */, data: *mut u8) -> nix::Result<libc::c_int> {
        let res = libc::ioctl(
            fd,
            nix::request_code_read!(
                b'E',
                0x20 + ev,
                len
            ),
            data
        );

        nix::errno::Errno::result(res)
    }

    pub unsafe fn eviocgabs(fd: libc::c_int, abs: u16, data: *mut libc::input_absinfo) -> nix::Result<libc::c_int> {
        let res = libc::ioctl(
            fd,
            nix::request_code_read!(
                b'E',
                0x40 + abs,
                core::mem::size_of::<libc::input_absinfo>()
            ),
            data
        );

        nix::errno::Errno::result(res)
    }

    pub unsafe fn eviocsabs(fd: libc::c_int, abs: u16, data: *mut libc::input_absinfo) -> nix::Result<libc::c_int> {
        let res = libc::ioctl(
            fd,
            nix::request_code_write!(
                b'E',
                0xc0 + abs,
                core::mem::size_of::<libc::input_absinfo>()
            ),
            data
        );

        nix::errno::Errno::result(res)
    }
}



#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct DeviceState {
    //abs_state: Option<Box<()>>,
    key_state: Option<Box<BitArr!(for Key::MAX as usize)>>
}

impl DeviceState {
    pub fn key_state(&self) -> Option<&BitArr!(for Key::MAX as usize)> {
        self.key_state.as_deref()
    }
}

// TODO: Add device type from constants
/// Input device and also an endless iterator over input events!
pub struct InputDevice {
    fd: i32,

    name: Option<String>,
    unique_name: Option<String>,
    physical_path: Option<String>,

    device_id: libc::input_id,
    driver_version: i32,

    supported_events: BitArr!(for Ev::MAX as usize),
    supported_abs: Option<BitArr!(for Abs::MAX as usize)>,
    supported_keys: Option<BitArr!(for Key::MAX as usize)>,
    supported_rel: Option<BitArr!(for Relative::MAX as usize)>,

    current_state: Option<DeviceState>,

    grabed: bool
}

impl InputDevice {
    // Message for future me: add final fn to handle fd closing
    pub fn open_from(path: impl AsRef<Path>) -> Result<Self> {
        fn with_string_buffer<const CAP: usize>(op: impl Fn(&mut [u8]) -> bool) -> Option<String> {
            // If result string greater than CAP, we are in a big trouble
            let mut buf = ArrayString::<CAP>::new();

            unsafe {
                let slice = core::slice::from_raw_parts_mut(
                    buf.as_bytes_mut().as_mut_ptr(),
                    CAP
                );

                if op(slice) {
                    buf.set_len(libc::strlen(slice.as_ptr().cast()));

                    Some(buf.to_string())
                } else {
                    None
                }
            }
        }
        fn with_type_buffer<T: Sized, R>(op: impl Fn(*mut T) -> Result<R>) -> Result<T> {
            unsafe {
                let mut t = core::mem::zeroed();

                op(&mut t as *mut T).map(move | _ | t)
            }
        }

        // Closes file when error occured
        struct Final(/* fd */ i32);

        impl Drop for Final {
            fn drop(&mut self) {
                let _ = unistd::close(self.0);
            }
        }



        let fd = fcntl::open(
            path.as_ref(),
            fcntl::OFlag::O_NONBLOCK,
            sys::stat::Mode::S_IWGRP | sys::stat::Mode::S_IRGRP
        )?;

        let _final = Final(fd);


        let name = with_string_buffer::<256>(| buf | unsafe { ioctl::eviocgname(fd, buf).is_ok() });
        let unique_name = with_string_buffer::<256>(| buf | unsafe { ioctl::eviocguniq(fd, buf).is_ok() });
        let physical_path = with_string_buffer::<256>(| buf | unsafe { ioctl::eviocgphys(fd, buf).is_ok() });

        let driver_version = with_type_buffer::<i32, _>(| data | unsafe { ioctl::eviocgversion(fd, data) })?;
        let device_id = with_type_buffer::<libc::input_id, _>(| data | unsafe { ioctl::eviocgid(fd, data) })?;



        let supported_events = unsafe {
            let mut buf = bitarr![0; Ev::MAX as usize];

            ioctl::eviocgbit(
                fd,
                0,
                Ev::MAX as usize,
                buf.as_mut_bitptr().pointer().cast()
            )?;

            buf
        };

        let supported_abs = if supported_events[Into::<u16>::into(Ev::Abs) as usize] {
            let mut buf = bitarr![0; Abs::MAX as usize];
            
            unsafe {
                ioctl::eviocgbit(
                    fd,
                    Ev::Abs.into(),
                    Abs::MAX as usize,
                    buf.as_mut_bitptr().pointer().cast()
                )?;
            }

            Some(buf)
        } else {
            None
        };
        let supported_keys = if supported_events[Into::<u16>::into(Ev::Key) as usize] {
            let mut buf = bitarr![0; Key::MAX as usize];
            
            unsafe {
                ioctl::eviocgbit(
                    fd,
                    Ev::Key.into(),
                    Key::MAX as usize,
                    buf.as_mut_bitptr().pointer().cast()
                )?;
            }

            Some(buf)
        } else {
            None
        };
        let supported_rel = if supported_events[Into::<u16>::into(Ev::Rel) as usize] {
            let mut buf = bitarr![0; Relative::MAX as usize];
            
            unsafe {
                ioctl::eviocgbit(
                    fd,
                    Ev::Rel.into(),
                    Relative::MAX as usize,
                    buf.as_mut_bitptr().pointer().cast()
                )?;
            }

            Some(buf)
        } else {
            None
        };


        // All operations are success ! No need for closing file
        core::mem::forget(_final);

        Ok(
            Self {
                fd,
                name,
                unique_name,
                physical_path,

                device_id,
                driver_version,

                supported_events,
                supported_abs,
                supported_keys,
                supported_rel,

                current_state: None,

                grabed: false
            }
        )
    }

    pub fn update_state(&mut self) -> Result<()> {
        self.current_state = Some(self.construct_state()?);

        Ok(())
    }

    pub fn current_state(&self) -> Option<&DeviceState> {
        self.current_state.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn unique_name(&self) -> Option<&str> {
        self.unique_name.as_deref()
    }

    pub fn physical_path(&self) -> Option<&str> {
        self.physical_path.as_deref()
    }
}

impl InputDevice {
    fn construct_state(&self) -> Result<DeviceState> {
        let mut state = DeviceState::default();

        if self.supported_events[Into::<u16>::into(Ev::Key) as usize] {
            let mut keys = Box::new(bitarr!(0; Key::MAX as usize));

            unsafe {
                ioctl::eviocgkey(
                    self.fd,
                    Key::MAX as usize,
                    keys.as_mut_bitptr().pointer().cast()
                )?;
            }

            state.key_state = Some(keys);
        }

        Ok(state)
    }
}

impl Iterator for InputDevice {
    type Item = libc::input_event;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut buf: libc::input_event = core::mem::zeroed();

            let _ = unistd::read(
                self.fd,
                core::slice::from_raw_parts_mut(
                    (&mut buf as *mut libc::input_event).cast(),
                    core::mem::size_of::<libc::input_event>()
                )
            ).ok()?;

            Some(buf)
        }
    }
}

impl Drop for InputDevice {
    fn drop(&mut self) {
        let _ = unistd::close(self.fd);
    }
}