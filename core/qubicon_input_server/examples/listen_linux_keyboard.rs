use keymaps::Key;
use q_input_server::{LinuxInputServer, LinuxInputEvent, EventType};

fn main() {
    let mut server = LinuxInputServer::new();

    server.add_input_action(
        "space_press",
        [
            LinuxInputEvent {
                device_id: None,
                r#type: EventType::Key(Key::Space),
                activation_value: 0.0
            }
        ]
    );

    loop {
        server.update();

        println!("{}", server.is_action_pressed("space_press"));
        println!("{}", server.get_action_force("space_press"));
    }
}