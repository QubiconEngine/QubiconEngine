use std::{ffi::CString, mem::MaybeUninit, pin::Pin};
use libpulse_sys::*;

mod raw;
pub mod error;

use raw::{Format, PlaybackStream, PulseContext};

pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;

pub struct AudioServer {
    data: Pin<Box<PulseContext>>
}

impl AudioServer {
    // TODO: Add app name
    pub fn init() -> Result<Self> {
        // assume_init on Box is currently night only
        #[allow(invalid_value, clippy::uninit_assumed_init)]
        let mut data = Box::pin(
            unsafe {
                MaybeUninit::<PulseContext>::uninit().assume_init()
            }
        );

        unsafe { data.as_mut().init(&CString::new("test").unwrap()) }?;

        Ok ( Self { data } )
    }

    pub fn create_stream<F: Format>(&self, name: &str, rate: u32, channels: u8, preallocated_buffer_len: usize) -> Pin<Box<PlaybackStream<F>>> {
        self.data.create_new_playback_stream(&CString::new(name.as_bytes()).unwrap(), rate, channels, preallocated_buffer_len)
    }

    pub fn update(&self) {
        self.data.update();
    }    
}

#[cfg(test)]
mod tests {
    use super::AudioServer;

    #[test]
    fn init_test() {
        AudioServer::init().unwrap();
    }

    #[test]
    fn stream_creation_test() {
        let server = AudioServer::init().unwrap();
        let mut stream = server.create_stream::<f32>("test", 44100, 1, 44100);

        unsafe {
            let buf = stream.as_mut().get_unchecked_mut().buf();

            for x in 0..44100 {
                buf.push_back((x as f32 / 1000.0).sin() * 10.0);
            }
        }

        loop {
            server.update();
        }
    }
}