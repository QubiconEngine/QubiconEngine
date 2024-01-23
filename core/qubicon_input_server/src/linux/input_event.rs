use keymaps::{Abs, Key, Relative};

pub enum InputEvent {
    Key { key: Key, state: bool },
    Abs { abs: Abs, val: f32 },
    Rel { rel: Relative, delta: i32 }
}