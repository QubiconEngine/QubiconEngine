use std::{collections::HashMap, path::Path};
use arrayvec::ArrayString;

use bitvec::{bitarr, BitArr};
use keymaps::{Relative, Key, Abs, Ev};
use nix::{libc, unistd, fcntl, sys, Result};

mod check_sets {
    use super::AbsInfo;
    use std::collections::HashMap;

    use bitvec::BitArr;
    use keymaps::{Relative, Key, Abs, Ev};

    pub fn gamepad(
        supported_abs: Option<&HashMap<Abs, AbsInfo>>,
        supported_keys: Option<&BitArr!(for Key::MAX as usize)>
    ) -> bool {
        const KEY_LIST: &[Key] = &[
            Key::BtnA,
            Key::BtnB,
            Key::BtnY,
            Key::BtnX
        ];
        const ABS_LIST: &[Abs] = &[
            // Left stick
            Abs::LX,
            Abs::LY,

            // Right stick
            Abs::RX,
            Abs::RY
        ];


        let supported_keys = match supported_keys {
            Some(s) => s,
            None => return false
        };
        let supported_abs = match supported_abs {
            Some(s) => s,
            None => return false
        };

        let keys = KEY_LIST.iter()
            .all(| &key | supported_keys[Into::<u16>::into(key) as usize]);

        let abs = ABS_LIST.iter()
            .all(| abs | supported_abs.contains_key(abs));

        keys & abs
    }

    pub fn keyboard(
        supported_ev: &BitArr!(for Ev::MAX as usize),
        supported_keys: Option<&BitArr!(for Key::MAX as usize)>
    ) -> bool {
        supported_ev[Into::<u16>::into(Ev::Rep) as usize] && supported_keys.is_some()
    }

    pub fn mouse(
        supported_keys: Option<&BitArr!(for Key::MAX as usize)>,
        supported_rel: Option<&BitArr!(for Relative::MAX as usize)>
    ) -> bool {
        const KEY_LIST: &[Key] = &[
            Key::BtnLeft,
            Key::BtnRight
        ];
        const REL_LIST: &[Relative] = &[
            Relative::X,
            Relative::Y
        ];

        
        let supported_keys = match supported_keys {
            Some(s) => s,
            None => return false
        };
        let supported_rel = match supported_rel {
            Some(s) => s,
            None => return false
        };

        let keys = KEY_LIST.iter()
            .all(| &key | supported_keys[Into::<u16>::into(key) as usize]);

        let rel = REL_LIST.iter()
            .all(| &rel | supported_rel[Into::<u16>::into(rel) as usize]);

        keys & rel
    }
}

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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum DeviceProperty {
//     Pointer = 0x00,
//     Direct = 0x01,
//     ButtonPad = 0x02,
//     SemiMT = 0x03,
//     TopButtonPad = 0x04,
//     PointingStick = 0x05,
//     Accelerometer = 0x06
// }

// libc::input_absinfo without value field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AbsInfo {
    pub min: i32,
    pub max: i32,
    pub res: i32,
    pub fuzz: i32,
    pub flat: i32
}

impl From<libc::input_absinfo> for AbsInfo {
    fn from(value: libc::input_absinfo) -> Self {
        Self {
            min: value.minimum,
            max: value.maximum,
            res: value.resolution,
            fuzz: value.fuzz,
            flat: value.flat
        }
    }
}

const EVENT_BUF_CAPACITY: u8 = 16;

// TODO: Add device type from constants
/// Input device and also an endless iterator over input events!
pub struct InputDevice {
    fd: i32,

    // maybe use Arc ?
    name: Box<str>,
    physical_path: Box<str>,
    unique_name: Option<Box<str>>,

    device_id: libc::input_id,
    driver_version: i32,

    supported_events: BitArr!(for Ev::MAX as usize),
    supported_abs: Option<HashMap<Abs, AbsInfo>>,
    supported_keys: Option<BitArr!(for Key::MAX as usize)>,
    supported_rel: Option<BitArr!(for Relative::MAX as usize)>,

    event_buf: Vec<libc::input_event>,
    current_event_idx: u8,

    grabed: bool
}

impl InputDevice {
    pub fn open_from(path: impl AsRef<Path>) -> Result<Self> {
        fn with_string_buffer<const CAP: usize>(op: impl Fn(&mut [u8]) -> Result<libc::c_int>) -> Result<Box<str>> {
            // If result string greater than CAP, we are in a big trouble
            let mut buf = ArrayString::<CAP>::new();

            unsafe {
                let slice = core::slice::from_raw_parts_mut(
                    buf.as_bytes_mut().as_mut_ptr(),
                    CAP
                );

                op(slice)?;

                buf.set_len(libc::strlen(slice.as_ptr().cast()));
            }

            Ok(buf.as_str().into())
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


        let name = with_string_buffer::<256>(| buf | unsafe { ioctl::eviocgname(fd, buf) })?;
        let physical_path = with_string_buffer::<256>(| buf | unsafe { ioctl::eviocgphys(fd, buf) })?;
        let unique_name = with_string_buffer::<256>(| buf | unsafe { ioctl::eviocguniq(fd, buf) }).ok();

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

            let abs: HashMap<_, _> = buf
                .into_iter()
                .enumerate()
                .map(| (abs, s) | (unsafe { Abs::from_raw(abs as u16) }, s))
                .filter(| &(_, s) | s)
                .map(| (abs, _) | abs)
                .filter_map(| abs | {
                    let mut abs_into_raw: core::mem::MaybeUninit<libc::input_absinfo> = core::mem::MaybeUninit::uninit();

                    unsafe { ioctl::eviocgabs(fd, abs.into(), abs_into_raw.as_mut_ptr()) }
                        .ok()?; // If call failed, return

                    Some((abs, unsafe { abs_into_raw.assume_init() }.into()))
                })
                .collect();

            Some(abs)
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

        // TODO: Delete this shit. This piece of code is here only for one reason: filter out
        // devices what not a keyboard, gamepad or mouse
        if !check_sets::keyboard(&supported_events, supported_keys.as_ref()) &&
           !check_sets::gamepad(supported_abs.as_ref(), supported_keys.as_ref()) &&
           !check_sets::mouse(supported_keys.as_ref(), supported_rel.as_ref())
        {
            // I dont realy care what to return, just filter this shit out
            return Err(nix::Error::EINVAL)
        }

        // in theory, we can get out of memory there. By default, rust panics on out of memory.
        // but idk how this would work. Linux can actualy overcommit memory and all that stuff.
        // just let it be there
        let event_buf = Vec::with_capacity(EVENT_BUF_CAPACITY as usize);

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

                event_buf,
                current_event_idx: 0,

                grabed: false
            }
        )
    }

    /// May result in EAGAIN. this signals about what no more events left
    pub fn next_event(&mut self) -> Result<libc::input_event> {
        if self.current_event_idx as usize == self.event_buf.len() {
            self.update_event_buf()?;
            self.current_event_idx = 0;
        }

        let event = self.event_buf.get(self.current_event_idx as usize)
            .copied()
            .unwrap();

        self.current_event_idx += 1;

        Ok(event)
    }

    pub fn supported_events(&self) -> &BitArr!(for Ev::MAX as usize) {
        &self.supported_events
    }

    pub fn supported_abs(&self) -> Option<&HashMap<Abs, AbsInfo>> {
        self.supported_abs.as_ref()
    }

    pub fn supported_keys(&self) -> Option<&BitArr!(for Key::MAX as usize)> {
        self.supported_keys.as_ref()
    }

    pub fn supported_rel(&self) -> Option<&BitArr!(for Relative::MAX as usize)> {
        self.supported_rel.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn physical_path(&self) -> &str {
        &self.physical_path
    }

    pub fn unique_name(&self) -> Option<&str> {
        self.unique_name.as_deref()
    }
}

impl InputDevice {
    fn update_event_buf(&mut self) -> Result<()> {
        unsafe {
            let buf = core::slice::from_raw_parts_mut(
                self.event_buf.as_mut_ptr().cast(),
                EVENT_BUF_CAPACITY as usize * core::mem::size_of::<libc::input_event>()
            );

            let len = unistd::read(self.fd, buf)?;

            self.event_buf.set_len(len / core::mem::size_of::<libc::input_event>());
        }

        Ok(())
    }
}

impl Drop for InputDevice {
    fn drop(&mut self) {
        let _ = unistd::close(self.fd);
    }
}