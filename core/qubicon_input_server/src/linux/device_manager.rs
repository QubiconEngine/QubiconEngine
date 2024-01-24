use nix::unistd;
use arrayvec::ArrayString;
use std::{
    collections::HashMap,
    os::unix::ffi::OsStrExt,
    ops::{
        Deref,
        DerefMut
    },
    fmt::Write
};

use crate::input_device::InputDevice;

pub struct DeviceManager {
    devices: HashMap<u16, InputDevice>,
    
    udev_monitor: udev::MonitorSocket
}


// TODO: Error handling
impl DeviceManager {
    #[allow(all)]
    pub fn new() -> Self {
        let devices: HashMap<_, _> = std::fs::read_dir("/dev/input")
            .unwrap()
            .filter_map(| d | d.ok())
            
            .filter(| d | junk::is_event_file(d.file_name().as_bytes()))
            .filter_map(| d | Some(
                (
                    junk::extract_id_from_file_name(d.file_name().to_str().unwrap()),
                    InputDevice::open_from(d.path()).ok()?
                )
            ))
            
            .collect();

        let udev_monitor = udev::MonitorBuilder::new()
            .unwrap()
            .match_subsystem("input")
            .unwrap()
            .listen()
            .unwrap();

        Self {
            devices,
            udev_monitor
        }
    }

    pub fn update_devices_state(&mut self) {
        for device in self.devices.values_mut() {
            device.update_state().unwrap();
        }
    }

    pub fn update_device_list(&mut self) {
        // let events = match self.notify.read_events() {
        //     Ok(e) => e,
        //     Err(_) => return,
        // };

        // let events = events.into_iter()
        //     .filter(| e | junk::is_event_file(e.name.as_ref().unwrap().as_bytes()))
        //     .map(| e | (
        //         junk::extract_id_from_file_name(e.name.unwrap().to_str().unwrap()),
        //         e.mask
        //     ));

        // for (file_id, ev_type) in events {
        //     match ev_type {
        //         // New device is connected, we put it onto a waiting list until it initialized
        //         inotify::AddWatchFlags::IN_CREATE => {
        //             self.event_files_in_init_process.push(file_id)
        //         },
        //         // Some device is removed. If it contained in our hash map, delete
        //         inotify::AddWatchFlags::IN_DELETE => {
        //             let _ = self.devices.remove(&file_id);
        //         },

        //         // Linux cant give us events with type what we not provided in descriptor mask
        //         _ => unreachable!()
        //     }
        // }
    }
}

impl Drop for DeviceManager {
    fn drop(&mut self) {
        // Idk if this required. Let it be there
        let _ = self.notify.rm_watch(self.input_dir_desc);
    }
}

impl Deref for DeviceManager {
    type Target = HashMap<u16, InputDevice>;

    fn deref(&self) -> &Self::Target {
        &self.devices
    }
}

impl DerefMut for DeviceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.devices
    }
}


// Some helper functions with shit code inside
// Idk how to write this in another way
mod junk {
    // Takes linux file name as slice of bytes
    // This code cant normaly operate on OsStr
    pub(super) fn is_event_file(name: &[u8]) -> bool {
        name.starts_with("event".as_bytes())
    }

    // event13
    //      ^^
    //      -- Exactly what we need
    pub(super) fn extract_id_from_file_name(name: &str) -> u16 {
        name
            .split("event")
            .nth(1)
            .expect("no number element in file name. Maybe invalid file?")
            .parse()
            .expect("failed to parse event file id")
    }
}