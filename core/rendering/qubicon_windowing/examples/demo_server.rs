use qubicon_windowing::x11::{WindowEvent, WindowingServer};

fn main() {
    let mut server = WindowingServer::init();

    let window_id = server.create_window(16 * 20, 9 * 20);
    
    {
        let window = server.window_mut(window_id).unwrap();

        window.show();
        window.set_name("Test window");
    }

    'event_loop: loop {
        server.update();

        let mut window = server.window_mut(window_id).unwrap();

        for event in window.events() {
            match event {
                WindowEvent::Visibility { state } => println!("visibility state changed! {state:?}"),
                WindowEvent::Configure { width, height } => println!("configure event! size: {width} {height}"),
                WindowEvent::Close => {
                    println!("closing");

                    break 'event_loop;
                }
            }
        }
    }

    server.destroy_window(window_id);
}