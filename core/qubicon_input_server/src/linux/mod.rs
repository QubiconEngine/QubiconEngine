use nix::sys;
use std::collections::HashMap;
use keymaps::{Relative, Abs, Key, Ev};

use self::{device_manager::DeviceManager, input_device::InputDevice};

pub(crate) mod device_manager;
pub(crate) mod input_device;
pub(crate) mod input_event;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Key(Key, bool),
    Abs(Abs),
    Rel(Relative, i32)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinuxInputEvent {
    /// If None, then event from any device will count
    pub device_id: Option<u16>,
    pub r#type: EventType,
    pub activation_value: f32
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ActionState {
    /// f32 for analog events
    pub action_force: f32,
    pub input_events: Vec<LinuxInputEvent>
}

pub struct LinuxInputServer {
    device_manager: DeviceManager,
    input_actions: HashMap<String, ActionState>
}

impl LinuxInputServer {
    pub fn new() -> Self {
        Self {
            device_manager: DeviceManager::new(),
            input_actions: HashMap::new()
        }
    }

    pub fn update(&mut self) {
        self.device_manager.update_device_list();
        self.device_manager.update_devices_state();

        self.read_device_events();
        self.update_actions();
    }

    pub fn add_input_action(&mut self, action: impl Into<String>, input_events: impl Into<Vec<LinuxInputEvent>>) {
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
    fn read_device_events(&mut self) {
        for (&_device_id, device) in self.device_manager.iter_mut() {            
            while let Ok(event) = device.next_event() {
                let ev = unsafe { Ev::from_raw(event.type_) };
                let timeval = sys::time::TimeVal::new(
                    event.time.tv_sec,
                    event.time.tv_usec
                );

                match ev {
                    Ev::Rel => {},
                    Ev::Key => {},
                    Ev::Abs => {},

                    _ => {}
                }
            }
        }
    }

    fn update_actions(&mut self) {
        for (_action_name, action_state) in self.input_actions.iter_mut() {
            // Action force may be not zero from previous update
            // so we need to reset it. Otherwise action will be always enabled
            action_state.action_force = 0.0;

            for event in action_state.input_events.iter() {
                let devices: &mut dyn Iterator<Item = &InputDevice>;


                let mut _all_devices;
                let mut _single_device;

                match event.device_id {
                    Some(id) => {
                        _single_device = core::iter::once(&self.device_manager[&id]);

                        devices = &mut _single_device;
                    },
                    None => {
                        _all_devices = self.device_manager.values();

                        devices = &mut _all_devices
                    }
                }


                for device in devices {
                    let state = device.current_state().unwrap();

                    match event.r#type {
                        EventType::Abs(ty) => {
                            if let Some(abs) = state.abs_state() {
                                if !abs.contains_key(&ty) {
                                    continue;
                                }

                                let value = abs[&ty];

                                let max = value.maximum - value.minimum;
                                let val = value.value - value.minimum;

                                let value = val as f32 / max as f32;

                                if value > event.activation_value {
                                    action_state.action_force = value;
                                }
                            }
                        },
                        EventType::Key(ke, st) => {
                            if let Some(key_state) = state.key_state() {
                                if st == key_state[Into::<u16>::into(ke) as usize] {
                                    action_state.action_force = 1.0;
                                }
                            }
                        },
                        EventType::Rel(re, de) => {}
                    }
                }
            }
        }
    }
}