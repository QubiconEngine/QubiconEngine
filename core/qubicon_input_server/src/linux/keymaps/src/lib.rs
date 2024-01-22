#![allow(unused, clippy::from_over_into)]
mod consts;

pub use key::Key;
pub use rel::Relative;
pub use btn::Button;
pub use abs::Abs;
pub use ev::Ev;

use consts::*;

mod key;
mod btn;
mod abs;
mod rel;
mod ev;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum State {
    Release = 0,
    Pressed = 1
}