use std::time::Instant;

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
        let start_time = Instant::now();

        server.update();

        let mut window = server.window_mut(window_id).unwrap();

        for event in window.events() {
            match event {
                WindowEvent::Visibility { state } => println!("visibility state changed! {state:?}"),
                WindowEvent::Resize { width, height } => println!("resized! {width} {height}"),
                WindowEvent::Move { x, y } => println!("moved to {x} {y}"),


                WindowEvent::Close => {
                    println!("closing");

                    break 'event_loop;
                }
            }
        }

        let time_passed = start_time.elapsed().as_secs_f64();

        println!("{}", 1.0 / time_passed);
    }

    server.destroy_window(window_id);
}