use keymaps::{Abs, Key};
use q_input_server::{LinuxInputServer, ActionInputEntry, ActionEventType};

fn main() {
    let mut server = LinuxInputServer::new().unwrap();

    server.add_input_action(
        "left",
        [
            ActionInputEntry {
                device_id: None,
                r#type: ActionEventType::Abs{ abs: Abs::LX, range: 0.0..1.1},
                activation_value: 0.55
            },
            ActionInputEntry {
                device_id: None,
                r#type: ActionEventType::Key { key: Key::D, pressed: true },
                activation_value: 1.0
            }
        ]
    );

    loop {
        // server.read_device_events(| event | println!("{event:?}"));
        // server.update_actions();

        server.update(| ev | println!("{ev:?}"));

        //println!("{}", server.is_action_pressed("left"));
        //println!("{}", server.get_action_force("left"));
    }
}