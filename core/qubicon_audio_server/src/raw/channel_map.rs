use std::{fmt::Debug, mem::MaybeUninit, ops::{Deref, DerefMut}};

use arrayvec::ArrayVec;
use libpulse_sys::*;

pub type ChannelPosition = pa_channel_position_t;

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelMap (ArrayVec<ChannelPosition, {PA_CHANNELS_MAX as usize}>);

impl ChannelMap {
    pub fn mono() -> Self {
        unsafe {
            #[allow(invalid_value, clippy::uninit_assumed_init)]
            let mut raw = MaybeUninit::uninit().assume_init();

            pa_channel_map_init_mono(&mut raw);

            raw.into()
        }
    } 

    pub fn stereo() -> Self {
        unsafe {
            #[allow(invalid_value, clippy::uninit_assumed_init)]
            let mut raw = MaybeUninit::uninit().assume_init();

            pa_channel_map_init_stereo(&mut raw);

            raw.into()
        }
    }

    pub fn auto(channels: u8) -> Self {
        if channels > PA_CHANNELS_MAX {
            panic!("too much channels. max is {PA_CHANNELS_MAX}, but required {channels}");
        }

        unsafe {
            #[allow(invalid_value, clippy::uninit_assumed_init)]
            let mut raw = MaybeUninit::uninit().assume_init();

            pa_channel_map_init_extend(
                &mut raw,
                channels as u32,
                // TODO: remove this hardcoded value
                pa_channel_map_def_t::ALSA
            );

            raw.into()
        }
    }

    pub fn from_positions(positions: &[ChannelPosition]) -> Self {
        if positions.len() > PA_CHANNELS_MAX as usize {
            panic!("too much positions. max is {PA_CHANNELS_MAX}, but provided {}", positions.len());
        }

        unsafe {
            let mut vec = ArrayVec::new_const();
            vec.try_extend_from_slice(positions).unwrap_unchecked();

            Self ( vec )
        }
    }
}

impl Default for ChannelMap {
    fn default() -> Self {
        Self::mono()
    }
}

impl Debug for ChannelMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelMap")
            .field("channel_count", &self.0.len())
            .field("channels", &self.0.as_slice())
            .finish()
    }
}

impl Deref for ChannelMap {
    type Target = [ChannelPosition];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChannelMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<pa_channel_map> for &ChannelMap {
    fn into(self) -> pa_channel_map {
        unsafe {
            #[allow(invalid_value, clippy::uninit_assumed_init)]
            let mut map: [pa_channel_position_t; PA_CHANNELS_MAX as usize] = MaybeUninit::uninit().assume_init();

            core::ptr::copy_nonoverlapping(self.0.as_ptr(), map.as_mut_ptr(), self.0.len());
            
            pa_channel_map {
                channels: self.0.len() as u8,
                map
            }
        }
    }
}

impl From<pa_channel_map> for ChannelMap {
    fn from(value: pa_channel_map) -> Self {
        unsafe {
            let mut vec = ArrayVec::new_const();
            vec.try_extend_from_slice(&value.map[0..value.channels as usize]).unwrap_unchecked();

            Self ( vec )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelMap;

    #[test]
    fn channel_map_creation() {
        let mono = ChannelMap::mono();
        let stereo = ChannelMap::stereo();
        let auto = ChannelMap::auto(32);

        println!("mono: {mono:?}");
        println!("stereo: {stereo:?}");
        println!("auto: {auto:?}");
    }
}