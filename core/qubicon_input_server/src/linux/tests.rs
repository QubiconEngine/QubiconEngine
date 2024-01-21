use evdev::Key;

use crate::{device_manager::DeviceManager, LinuxInputServer, LinuxInputEvent, EventType};

#[test]
fn test_device_manager() {
    let mut manager = DeviceManager::new();

    loop {
        manager.update_device_list();
    }
}

#[test]
fn test_input_server() {
    let mut server = LinuxInputServer::new();

    server.add_input_action(
        "mouse_press",
        [
            LinuxInputEvent {
                device_id: None,
                r#type: EventType::Key(Key::BTN_LEFT),
                activation_value: 0.0
            }
        ]
    );

    loop {
        server.update();

        println!("{}", server.is_action_pressed("mouse_press"));
        println!("{}", server.get_action_force("mouse_press"));
    }
}