use crate::consts::*;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
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

    Max = REL_MAX,
    Cnt = REL_CNT
}