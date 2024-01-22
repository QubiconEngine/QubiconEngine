use super::consts::*;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Abs {
    Reserved = ABS_RESERVED,

    LX = ABS_X,
    LY = ABS_Y,
    LZ = ABS_Z,

    RX = ABS_RX,
    RY = ABS_RY,
    RZ = ABS_RZ,

    Throttle = ABS_THROTTLE,
    Rudder = ABS_RUDDER,
    Wheel = ABS_WHEEL,
    Gas = ABS_GAS,
    Brake = ABS_BRAKE,

    Hat0X = ABS_HAT0X,
    Hat0Y = ABS_HAT0Y,
    Hat1X = ABS_HAT1X,
    Hat1Y = ABS_HAT1Y,
    Hat2X = ABS_HAT2X,
    Hat2Y = ABS_HAT2Y,
    Hat3X = ABS_HAT3X,
    Hat3Y = ABS_HAT3Y,

    Pressure = ABS_PRESSURE,
    Distance = ABS_DISTANCE,

    TiltX = ABS_TILT_X,
    TiltY = ABS_TILT_Y,

    Volume = ABS_VOLUME,
    Profile = ABS_PROFILE,
    Misc = ABS_MISC,
    MtTouchMajor = ABS_MT_TOUCH_MAJOR,
    MtTouchMinor = ABS_MT_TOUCH_MINOR,
    MtWidthMajor = ABS_MT_WIDTH_MAJOR,
    MtWidthMinor = ABS_MT_WIDTH_MINOR,
    MtOrientation = ABS_MT_ORIENTATION,
    MtPositionX = ABS_MT_POSITION_X,
    MtPositionY = ABS_MT_POSITION_Y,
    MtToolType = ABS_MT_TOOL_TYPE,
    MtBlobId = ABS_MT_BLOB_ID,
    MtTrackingId = ABS_MT_TRACKING_ID,
    MtPressure = ABS_MT_PRESSURE,
    MtDistance = ABS_MT_DISTANCE,
    MtToolX = ABS_MT_TOOL_X,
    MtToolY = ABS_MT_TOOL_Y
}

impl Abs {
    pub const MAX: u16 = ABS_MAX;

    /// # Safety
    /// *value* muse be valid abs value presentable via this enum
    pub unsafe fn from_raw(value: u16) -> Self {
        core::mem::transmute(value)
    }
}

impl Into<u16> for Abs {
    fn into(self) -> u16 {
        unsafe { core::mem::transmute(self) }
    }
}