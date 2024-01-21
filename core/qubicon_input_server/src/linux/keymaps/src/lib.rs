#![allow(unused)]
mod consts;

pub use keyboard_key::KeyboardKey;
pub use rel::Relative;
pub use btn::Button;
pub use abs::Abs;

use consts::*;

pub mod keyboard_key;
pub mod btn;
pub mod abs;
pub mod rel;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum State {
    Release = 0,
    Pressed = 1
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum MouseKey {
    Left = BTN_LEFT,
    Right = BTN_RIGHT,
    Middle = BTN_MIDDLE,
    Side = BTN_SIDE,
    Extra = BTN_EXTRA
}