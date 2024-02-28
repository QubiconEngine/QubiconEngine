use keymaps::{Abs, Key};
use qubicon_input_server::{LinuxInputServer, ActionInputEntry, ActionEventType};

fn main() {
    let mut server = LinuxInputServer::new().unwrap();

    server.add_input_action(
        "left",
        [
            ActionInputEntry {
                device_id: None,
                r#type: ActionEventType::Abs { abs: Abs::LX, range: 0.0..1.1 }
            },
            ActionInputEntry {
                device_id: None,
                r#type: ActionEventType::Key { key: Key::D, pressed: true }
            }
        ]
    );

    loop {
        server.update(| ev | println!("{ev:?}"));

        println!("{} - {}", server.is_action_pressed("left"), server.get_action_force("left"));
    }
}