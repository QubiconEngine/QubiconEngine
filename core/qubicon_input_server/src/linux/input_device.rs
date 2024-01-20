use evdev::Device;
use std::{io::Result, path::Path};

pub struct InputDevice {
    device: Device
}

impl InputDevice {
    pub fn open_from(path: impl AsRef<Path>) -> Result<Self> {
        let device = Device::open(path)?;

        Ok( Self { device } )
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