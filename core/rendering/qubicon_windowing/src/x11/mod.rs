use smallstr::SmallString;
use x11::xlib;
use std::{
    mem::MaybeUninit,
    collections::{
        HashMap,
        VecDeque,

        vec_deque::{
            Iter as QueueIter,
            Drain as QueueDrain
        }
    },
};

pub type WindowId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisibilityState {
    Unobscured,
    PartialyObscured,
    FullyObscured
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowEvent {
    Visibility { state: VisibilityState },
    Configure { width: u32, height: u32 },
    Close
}


pub struct WindowingServer {
    display: *mut xlib::Display,

    screen: i32,
    root_window: u64,

    windows: HashMap<WindowId, WindowData>
}

impl WindowingServer {
    pub fn init() -> Self {
        unsafe {
            let display = xlib::XOpenDisplay(core::ptr::null());

            if display.is_null() {
                panic!("failed to open X display");
            }

            let screen = xlib::XDefaultScreen(display);
            let root_window = xlib::XRootWindow(display, screen);


            Self {
                display,

                screen,
                root_window,

                windows: Default::default()
            }
        }
    }

    pub fn create_window(&mut self, width: u32, height: u32) -> WindowId {
        unsafe {
            let window = xlib::XCreateSimpleWindow(
                self.display,
                self.root_window,
                0, // x | These two should be window position. Dont care
                0, // y |
                width,
                height,
                2, // border width
                0, // border     | Colors for border and background.
                0  // background | Our rendering will be done with swapchain, so these values not important
            );

            // Idk why, but SetWMProtocols requires mutable reference
            // By the way, WM_DELETE_WINDOW is send when X button on decorator is pressed
            let mut wm_destroy_event = xlib::XInternAtom(self.display, "WM_DELETE_WINDOW\0".as_ptr().cast(), xlib::False);

            xlib::XSetWMProtocols(self.display, window, &mut wm_destroy_event, 1);
            xlib::XSelectInput(self.display, window, xlib::StructureNotifyMask | xlib::VisibilityChangeMask);

            // no other window should be there, so this will return None everytime
            let _ = self.windows.insert(
                window,
                WindowData {
                    wm_destroy_event,
                    event_queue: Default::default()
                }
            );

            window
        }
    }

    pub fn update(&mut self) {
        let mut event = MaybeUninit::uninit();

        while unsafe { xlib::XEventsQueued(self.display, 2) } > 0 {
            unsafe {
                xlib::XNextEvent(self.display, event.assume_init_mut());

                let event = event.assume_init_ref();

                match event.type_ {
                    xlib::ConfigureNotify => {
                        let window = event.configure.window;
                        let window = match self.window_mut(window) {
                            Some(w) => w,
                            None => continue
                        };

                        let xlib::XConfigureEvent { width, height, .. } = event.configure;

                        window.data.event_queue.push_front(WindowEvent::Configure { width: width as u32, height: height as u32 });
                    },
                    xlib::VisibilityNotify => {
                        let window = event.visibility.window;
                        let window = match self.window_mut(window) {
                            Some(w) => w,
                            None => continue
                        };

                        let state = match event.visibility.state {
                            xlib::VisibilityUnobscured => VisibilityState::Unobscured,
                            xlib::VisibilityPartiallyObscured => VisibilityState::PartialyObscured,
                            xlib::VisibilityFullyObscured => VisibilityState::FullyObscured,

                            _ => unreachable!("invalid visibility state")
                        };

                        window.data.event_queue.push_front(WindowEvent::Visibility { state });
                    },
                    xlib::ClientMessage => {
                        let window = event.client_message.window;
                        let window = match self.window_mut(window) {
                            Some(w) => w,
                            None => continue
                        };

                        if event.client_message.data.get_long(0) as u64 == window.data.wm_destroy_event {
                            window.data.event_queue.push_front(WindowEvent::Close);
                        }
                    },

                    _ => {}
                }
            }
        }
    }

    pub fn window(&self, window_id: WindowId) -> Option<Window> {
        self.windows.get(&window_id)
            .map(| data | Window { display: self.display, window: window_id, data })
    }

    pub fn window_mut(&mut self, window_id: WindowId) -> Option<WindowMut> {
        self.windows.get_mut(&window_id)
            .map(| data | WindowMut { display: self.display, window: window_id, data })
    }

    pub fn destroy_window(&mut self, window_id: WindowId) {
        if self.windows.remove(&window_id).is_none() {
            return
        }

        unsafe {
            xlib::XDestroyWindow(self.display, window_id);
        }
    }
}

impl Drop for WindowingServer {
    fn drop(&mut self) {
        unsafe {
            for (&window, _) in self.windows.iter() {
                xlib::XDestroyWindow(self.display, window);
            }
            
            xlib::XCloseDisplay(self.display);
        }
    }
}


#[derive(Default)]
struct WindowData {
    wm_destroy_event: u64,
    event_queue: VecDeque<WindowEvent>
}



pub struct Window<'a> {
    display: *mut xlib::Display,
    window: WindowId,

    data: &'a WindowData
}

impl<'a> Window<'a> {
    /// Returns iterator over new events.
    pub fn events_ref(&self) -> QueueIter<WindowEvent> {
        self.data.event_queue.iter()
    }

    pub fn window_id(&self) -> WindowId {
        self.window
    }
}

pub struct WindowMut<'a> {
    display: *mut xlib::Display,
    window: WindowId,

    data: &'a mut WindowData
}

impl<'a> WindowMut<'a> {
    pub fn show(&self) {
        unsafe { xlib::XMapWindow(self.display, self.window) };
    }

    pub fn hide(&self) {
        unsafe { xlib::XUnmapWindow(self.display, self.window) };
    }

    pub fn set_name(&self, name: impl AsRef<str>) {
        // just a buffer to add \0
        let mut name = SmallString::<[u8; 64]>::from_str(name.as_ref());

        name.push('\0');

        unsafe { xlib::XStoreName(self.display, self.window, name.as_ptr().cast()) };
    }

    /// Returns iterator over new events. Each steep removes event from event queue
    pub fn events(&mut self) -> QueueDrain<WindowEvent> {
        self.data.event_queue.drain(..)
    }
}

impl<'a> Into<Window<'a>> for WindowMut<'a> {
    /// Idk how to make WindowMut a subtype of Window, so use into
    fn into(self) -> Window<'a> {
        Window {
            display: self.display,
            window: self.window,

            data: self.data
        }
    }
}