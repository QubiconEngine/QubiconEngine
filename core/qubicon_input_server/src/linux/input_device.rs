use std::{io::Result, path::Path};

pub struct InputDevice {

}

impl InputDevice {
    pub fn open_from(path: impl AsRef<Path>) -> Result<Self> {
        // let device = RawDevice::open(path)?;

        // device.get_abs_state()

        // Ok( Self { device } )
    }

    // TODO: Error handling
    pub fn update_state(&mut self) {
        
    }

    pub fn state(&self) -> () {
        todo!()
    }

    pub fn name(&self) -> Option<&str> {
        //self.device.name()
    }

    pub fn unique_name(&self) -> Option<&str> {
        //self.device.unique_name()
    }

    pub fn physical_path(&self) -> Option<&str> {
        //self.device.physical_path()
    }
}