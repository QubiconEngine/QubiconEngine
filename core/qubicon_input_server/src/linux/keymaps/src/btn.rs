use super::consts::*;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Btn0 = BTN_0,
    Btn1 = BTN_1,
    Btn2 = BTN_2,
    Btn3 = BTN_3,
    Btn4 = BTN_4,
    Btn5 = BTN_5,
    Btn6 = BTN_6,
    Btn7 = BTN_7,
    Btn8 = BTN_8,
    Btn9 = BTN_9,

    Left = BTN_LEFT,
    Right = BTN_RIGHT,
    Middle = BTN_MIDDLE,
    Side = BTN_SIDE,
    Extra = BTN_EXTRA,
    Forward = BTN_FORWARD,
    Back = BTN_BACK,
    Task = BTN_TASK,
    Trigger = BTN_TRIGGER,
    Thumb = BTN_THUMB,
    Thumb2 = BTN_THUMB2,
    ThumbL = BTN_THUMBL,
    ThumbR = BTN_THUMBR,
    Top = BTN_TOP,
    Top2 = BTN_TOP2,
    Pinkie = BTN_PINKIE,
    Base = BTN_BASE,
    Base2 = BTN_BASE2,
    Base3 = BTN_BASE3,
    Base4 = BTN_BASE4,
    Base5 = BTN_BASE5,
    Base6 = BTN_BASE6,
    Dead = BTN_DEAD,

    A = BTN_A,
    B = BTN_B,
    C = BTN_C,
    X = BTN_X,
    Y = BTN_Y,

    DPadLeft = BTN_DPAD_LEFT,
    DPadRight = BTN_DPAD_RIGHT,
    DPadUp = BTN_DPAD_UP,
    DPadDown = BTN_DPAD_DOWN,

    TL = BTN_TL,
    TR = BTN_TR,
    TL2 = BTN_TL2,
    TR2 = BTN_TR2,
    Select = BTN_SELECT,
    Start = BTN_START,

    Mode = BTN_MODE,
    Digi = BTN_DIGI,
    //Wheel = BTN_WHEEL,
    GearDown = BTN_GEAR_DOWN,
    GearUp = BTN_GEAR_UP,

    TriggerHappy1 = BTN_TRIGGER_HAPPY1,
    TriggerHappy2 = BTN_TRIGGER_HAPPY2,
    TriggerHappy3 = BTN_TRIGGER_HAPPY3,
    TriggerHappy4 = BTN_TRIGGER_HAPPY4,
    TriggerHappy5 = BTN_TRIGGER_HAPPY5,
    TriggerHappy6 = BTN_TRIGGER_HAPPY6,
    TriggerHappy7 = BTN_TRIGGER_HAPPY7,
    TriggerHappy8 = BTN_TRIGGER_HAPPY8,
    TriggerHappy9 = BTN_TRIGGER_HAPPY9,
    TriggerHappy10 = BTN_TRIGGER_HAPPY10,
    TriggerHappy11 = BTN_TRIGGER_HAPPY11,
    TriggerHappy12 = BTN_TRIGGER_HAPPY12,
    TriggerHappy13 = BTN_TRIGGER_HAPPY13,
    TriggerHappy14 = BTN_TRIGGER_HAPPY14,
    TriggerHappy15 = BTN_TRIGGER_HAPPY15,
    TriggerHappy16 = BTN_TRIGGER_HAPPY16,
    TriggerHappy17 = BTN_TRIGGER_HAPPY17,
    TriggerHappy18 = BTN_TRIGGER_HAPPY18,
    TriggerHappy19 = BTN_TRIGGER_HAPPY19,
    TriggerHappy20 = BTN_TRIGGER_HAPPY20,
    TriggerHappy21 = BTN_TRIGGER_HAPPY21,
    TriggerHappy22 = BTN_TRIGGER_HAPPY22,
    TriggerHappy23 = BTN_TRIGGER_HAPPY23,
    TriggerHappy24 = BTN_TRIGGER_HAPPY24,
    TriggerHappy25 = BTN_TRIGGER_HAPPY25,
    TriggerHappy26 = BTN_TRIGGER_HAPPY26,
    TriggerHappy27 = BTN_TRIGGER_HAPPY27,
    TriggerHappy28 = BTN_TRIGGER_HAPPY28,
    TriggerHappy29 = BTN_TRIGGER_HAPPY29,
    TriggerHappy30 = BTN_TRIGGER_HAPPY30,
    TriggerHappy31 = BTN_TRIGGER_HAPPY31,
    TriggerHappy32 = BTN_TRIGGER_HAPPY32,
    TriggerHappy33 = BTN_TRIGGER_HAPPY33,
    TriggerHappy34 = BTN_TRIGGER_HAPPY34,
    TriggerHappy35 = BTN_TRIGGER_HAPPY35,
    TriggerHappy36 = BTN_TRIGGER_HAPPY36,
    TriggerHappy37 = BTN_TRIGGER_HAPPY37,
    TriggerHappy38 = BTN_TRIGGER_HAPPY38,
    TriggerHappy39 = BTN_TRIGGER_HAPPY39,
    TriggerHappy40 = BTN_TRIGGER_HAPPY40
}

impl Button {
    /// # Safety
    /// *value* muse be valid abs value presentable via this enum
    pub unsafe fn from_raw(value: u16) -> Self {
        core::mem::transmute(value)
    }
}

impl Into<u16> for Button {
    fn into(self) -> u16 {
        unsafe { core::mem::transmute(self) }
    }
}