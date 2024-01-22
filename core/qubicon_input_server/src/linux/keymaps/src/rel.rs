use crate::consts::*;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Relative {
    Reserved = REL_RESERVED,

    X = REL_X,
    Y = REL_Y,
    Z = REL_Z,
    RX = REL_RX,
    RY = REL_RY,
    RZ = REL_RZ,

    HWheel = REL_HWHEEL,
    HWheelHiRes = REL_HWHEEL_HI_RES,

    Wheel = REL_WHEEL,
    WheelHiRes = REL_WHEEL_HI_RES,

    Dial = REL_DIAL,
    Misc = REL_MISC,
}

impl Relative {
    pub const MAX: u16 = REL_MAX;

    /// # Safety
    /// *value* muse be valid abs value presentable via this enum
    pub unsafe fn from_raw(value: u16) -> Self {
        core::mem::transmute(value)
    }
}

impl Into<u16> for Relative {
    fn into(self) -> u16 {
        unsafe { core::mem::transmute(self) }
    }
}