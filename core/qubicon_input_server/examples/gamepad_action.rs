use keymaps::Abs;
use q_input_server::{LinuxInputServer, LinuxInputEvent, EventType};

fn main() {
    let mut server = LinuxInputServer::new();

    server.add_input_action(
        "left",
        [
            LinuxInputEvent {
                device_id: None,
                r#type: EventType::Abs(Abs::LX),
                activation_value: 0.55
            }
        ]
    );

    loop {
        server.update();

        println!("{}", server.is_action_pressed("left"));
        println!("{}", server.get_action_force("left"));
    }
}