use bitvec::BitArr;
use keymaps::{Abs, Key};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceState {
    pub abs_state: Option<HashMap<Abs, f32>>,
    pub key_state: Option<Box<BitArr!(for Key::MAX as usize)>>
}