use nix::sys;
use bitvec::bitarr;
use std::ops::Range;
use std::collections::HashMap;
use keymaps::{Relative, Abs, Key, Ev};

use super::{device_manager::DeviceManager, device_state::DeviceState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEventData {
    Abs{ abs: Abs, value: f32 },
    Key{ key: Key, state: bool },
    Rel{ rel: Relative, delta: i32 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InputEvent {
    device_id: u16,
    time: sys::time::TimeVal,

    data: InputEventData
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionEventType {
    /// *key* is a key what being checked, and *pressed* tells if key should be pressed or released to activate action
    Key { key: Key, pressed: bool },
    /// *abs* - axis, *range* is an activation range. If axis value is in range, action will be active
    Abs { abs: Abs, range: Range<f32>}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionInputEntry {
    /// If None, then event from any device will count
    pub device_id: Option<u16>,
    pub r#type: ActionEventType
}

#[derive(Debug, Default, Clone, PartialEq)]
struct ActionState {
    /// f32 for analog events
    pub action_force: f32,
    pub input_events: Vec<ActionInputEntry>
}

pub struct LinuxInputServer {
    device_manager: DeviceManager,
    devices_state: HashMap<u16, DeviceState>,
    input_actions: HashMap<String, ActionState>
}

impl LinuxInputServer {
    pub fn new() -> nix::Result<Self> {
        Ok(
            Self {
                device_manager: DeviceManager::new()?,
                devices_state: HashMap::new(),
                input_actions: HashMap::new()
            }
        )
    }
 
    pub fn update(&mut self, event_handler: impl Fn(&InputEvent)) {
        self.device_manager.update_device_list();

        self.read_device_events(event_handler);
        self.update_actions();
    }

    pub fn add_input_action(&mut self, action: impl Into<String>, input_events: impl Into<Vec<ActionInputEntry>>) {
        self.input_actions.insert(
            action.into(),
            ActionState {
                action_force: 0.0,
                input_events: input_events.into()
            }
        );
    }

    pub fn is_action_pressed(&self, action: impl AsRef<str>) -> bool {
        self.input_actions[action.as_ref()].action_force != 0.0
    }

    pub fn get_action_force(&self, action: impl AsRef<str>) -> f32 {
        self.input_actions[action.as_ref()].action_force
    }
}

impl LinuxInputServer {
    fn read_device_events(&mut self, event_handler: impl Fn(&InputEvent)) {
        for (&device_id, device) in self.device_manager.iter_mut() {  
            let device_state = self.devices_state
                .entry(device_id)
                .or_default();

            while let Ok(event) = device.next_event() {
                let ev = unsafe { Ev::from_raw(event.type_) };
                let time = sys::time::TimeVal::new(
                    event.time.tv_sec,
                    event.time.tv_usec
                );

                let data = match ev {
                    Ev::Key => InputEventData::Key { key: unsafe { Key::from_raw(event.code) }, state: event.value > 0 },
                    Ev::Rel => InputEventData::Rel { rel: unsafe { Relative::from_raw(event.code) }, delta: event.value },
                    Ev::Abs => {
                        let abs = unsafe { Abs::from_raw(event.code) };
                        let abs_info = unsafe {
                            device
                                .supported_abs()
                                .unwrap_unchecked()
                                .get(&abs)
                                .unwrap_unchecked()
                        };

                        let value = junk::normalize_abs_value(abs_info.min, abs_info.max, event.value);

                        InputEventData::Abs { abs, value }
                    },

                    _ => continue
                };

                match data {
                    InputEventData::Key { key, state } => {
                        device_state.key_state
                            .get_or_insert_with(|| Box::new(bitarr!(0; Key::MAX as usize)))
                            .set(Into::<u16>::into(key) as usize, state)
                    },
                    InputEventData::Abs { abs, value } => {
                        let _ = device_state.abs_state
                            .get_or_insert(HashMap::new()).insert(abs, value);
                    },
                    
                    _ => {}
                }

                let event = InputEvent {
                    device_id,
                    time,
                    data
                };

                event_handler(&event)
            }
        }
    }

    pub fn update_actions(&mut self) {
        for (_action_name, action_state) in self.input_actions.iter_mut() {
            // Action force may be not zero from previous update
            // so we need to reset it. Otherwise action will be always enabled
            action_state.action_force = 0.0;

            for event in action_state.input_events.iter() {
                let devices: &mut dyn Iterator<Item = u16>;


                let mut _all_devices;
                let mut _single_device;

                match event.device_id {
                    Some(id) => {
                        _single_device = core::iter::once(id);

                        devices = &mut _single_device;
                    },
                    None => {
                        _all_devices = self.device_manager.keys().copied();

                        devices = &mut _all_devices
                    }
                }


                for device_id in devices {
                    let state = &self.devices_state[&device_id];

                    match event.r#type.clone() {
                        ActionEventType::Abs{ abs, range } => {
                            if let Some(state) = state.abs_state.as_ref() {
                                if !state.contains_key(&abs) {
                                    continue;
                                }

                                let value = state[&abs];

                                if range.contains(&value) {
                                    action_state.action_force = value.abs();
                                }
                            }
                        },
                        ActionEventType::Key{ key, pressed } => {
                            if let Some(key_state) = state.key_state.as_ref() {
                                if pressed == key_state[Into::<u16>::into(key) as usize] {
                                    action_state.action_force = 1.0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

mod junk {
    // Just a shity value normalization
    pub fn normalize_abs_value(min: i32, max: i32, val: i32) -> f32 {
        if min.signum() != max.signum() {
            if val.signum() == min.signum() {
                -(val as f32 / min as f32)
            } else {
                val as f32 / max as f32
            }
        } else {
            (val - min) as f32 / (max - min) as f32
        }
    }
}