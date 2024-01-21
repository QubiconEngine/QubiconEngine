use evdev::{Device, DeviceState};
use std::{io::Result, path::Path};

pub struct InputDevice {
    device: Device
}

impl InputDevice {
    pub fn open_from(path: impl AsRef<Path>) -> Result<Self> {
        let device = Device::open(path)?;

        Ok( Self { device } )
    }

    // TODO: Error handling
    pub fn update_state(&mut self) {
        self.device.fetch_events().unwrap();
    }

    pub fn state(&self) -> &DeviceState {
        self.device.cached_state()
    }

    pub fn name(&self) -> Option<&str> {
        self.device.name()
    }

    pub fn unique_name(&self) -> Option<&str> {
        self.device.unique_name()
    }

    pub fn physical_path(&self) -> Option<&str> {
        self.device.physical_path()
    }
}