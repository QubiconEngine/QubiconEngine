use crate::consts::*;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ev {
    Syn = EV_SYN,
    Key = EV_KEY,
    Rel = EV_REL,
    Abs = EV_ABS,
    Msc = EV_MSC,
    Sw = EV_SW,
    Led = EV_LED,
    Snd = EV_SND,
    Rep = EV_REP,
    FF = EV_FF,
    PWR = EV_PWR,
    FFStatus = EV_FF_STATUS
}

impl Ev {
    pub const MAX: u16 = EV_MAX;

    /// # Safety
    /// *value* muse be valid abs value presentable via this enum
    pub unsafe fn from_raw(value: u16) -> Self {
        core::mem::transmute(value)
    }
}

impl Into<u16> for Ev {
    fn into(self) -> u16 {
        unsafe { core::mem::transmute(self) }
    }
}