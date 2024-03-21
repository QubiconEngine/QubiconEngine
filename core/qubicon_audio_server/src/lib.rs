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

    pub fn create_stream<F: Format>(&self, name: &str, rate: u32, channels: u8) -> Result<Pin<Box<PlaybackStream<F>>>> {
        self.data.create_new_playback_stream(&CString::new(name.as_bytes()).unwrap(), rate, channels)
    }

    pub fn update(&self) -> Result<()> {
        self.data.update()
    }
}

#[cfg(test)]
mod tests {
    use super::AudioServer;
    use super::raw::StreamWrite;

    #[test]
    fn init_test() {
        AudioServer::init()
            .expect("failed to init audio server");
    }

    #[test]
    fn stream_creation_test() {
        let server = AudioServer::init()
            .expect("failed to init audio server");
        let mut stream = server.create_stream::<f32>("test", 44100, 1)
            .expect("failed to create stream");

        let mut total_x = 0usize;

        loop {
            server.update().unwrap();

            if let Ok(ammount) = stream.as_ref().available_len() {
                if ammount == 0 { continue; }

                let mut buf = Vec::with_capacity(ammount);

                for x in 0..ammount {
                    let time = (total_x + x) as f32 / 44100.0;

                    buf.push((time * 100.0).sin() * 10.0);
                }

                total_x += ammount;

                let _ = stream.as_mut().write(&buf);
            }
        }
    }
}