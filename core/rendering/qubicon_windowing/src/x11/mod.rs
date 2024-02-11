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
        },
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
    Resize { width: u32, height: u32 },
    Move { x: i32, y: i32 },
    Close
}


pub struct WindowingServer {
    display: *mut xlib::Display,

    screen: i32,
    visual_id: u64,
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
            let visual_id = xlib::XVisualIDFromVisual(xlib::XDefaultVisual(display, screen));


            Self {
                display,

                screen,
                visual_id,
                root_window,

                windows: Default::default()
            }
        }
    }

    /// Creates simple window without any graphics things
    pub fn create_window(&mut self, width: u32, height: u32) -> WindowId {
        let (window, wm_destroy_event) = unsafe { self.create_window_impl(width, height)};

        // no other window should be there, so this will return None everytime
        let _ = self.windows.insert(
            window,
            WindowData {
                width,
                height,
                wm_destroy_event,
                
                ..Default::default()
            }
        );

        return window;
    }

    #[cfg(feature = "vulkan")]
    pub fn create_window_vulkan(
        &mut self,
        device: &qubicon_vulkan::device::Device,
        width: u32,
        height: u32,
        create_info: &super::AssociatedSwapchainCreationInfo,
        present_mode_selector: impl Fn(qubicon_vulkan::surface::PresentMode) -> bool,
        format_and_colorspace_selector: impl Fn(qubicon_vulkan::surface::SurfaceFormat) -> bool,
    ) -> Result<WindowId, qubicon_vulkan::Error> {
        struct RetHandler {
            display: *mut xlib::Display,
            window: u64
        }

        impl Drop for RetHandler {
            fn drop(&mut self) {
                unsafe { xlib::XDestroyWindow(self.display, self.window) };
            }
        }


        let phys_dev = device.get_physical_device();
        let instance = device.associated_instance();
        
        unsafe {
            let (window, wm_destroy_event) = self.create_window_impl(width, height);
            let _handler = RetHandler { display: self.display, window };


            let surface = instance.create_surface_x11(self.display, window)?;

            let format = surface.get_physical_device_surface_formats(phys_dev)?
                .into_iter()
                .find(| &f | format_and_colorspace_selector(f))
                .ok_or(qubicon_vulkan::error::VkError::Unknown)?;
            let present_mode = surface.get_physical_device_surface_present_modes(phys_dev)?
                .into_iter()
                .find(| &m | present_mode_selector(m))
                .ok_or(qubicon_vulkan::error::VkError::Unknown)?;


            let create_info = qubicon_vulkan::swapchain::SwapchainCreationInfo {
                min_image_count: create_info.min_image_count,
                
                image_extent: (width, height),
                image_array_layers: create_info.image_array_layers,
                
                image_usage: create_info.image_usage,
                clipped: true,


                image_format: format.format,
                image_color_space: format.color_space,
                pre_transform: create_info.pre_transform,
                composite_alpha: create_info.composite_alpha,
                present_mode
            };

            let swapchain = device.create_swapchain_unchecked(surface, &create_info)?;

            // same as in simple create_window
            let _ = self.windows.insert(
                window,
                WindowData {
                    width,
                    height,
                    swapchain: Some(swapchain),
                    wm_destroy_event,

                    ..Default::default()
                }
            );

            core::mem::forget(_handler);

            return Ok(window);
        }
    }

    pub fn destroy_window(&mut self, window_id: WindowId) {
        if self.windows.remove(&window_id).is_none() {
            return
        }

        unsafe {
            xlib::XDestroyWindow(self.display, window_id);
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

                        let xlib::XConfigureEvent { width, height, x, y, .. } = event.configure;
                        let width = width as u32;
                        let height = height as u32;

                        if x != window.data.x || y != window.data.y {
                            window.data.x = x;
                            window.data.y = y;

                            window.data.event_queue.push_front(
                                WindowEvent::Move { x, y }
                            )
                        }
                        
                        if width != window.data.width || height != window.data.height {
                            window.data.width = width;
                            window.data.height = height;

                            window.data.event_queue.push_front(
                                WindowEvent::Resize { width, height }
                            )
                        }
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


    pub fn windows(&self) -> impl Iterator<Item = Window> {
        self.windows.iter()
            .map(| (&window, data) | Window { display: self.display, window, data })
    }

    pub fn windows_mut(&mut self) -> impl Iterator<Item = WindowMut> {
        self.windows.iter_mut()
            .map(| (&window, data) | WindowMut { display: self.display, window, data })
    }

    #[cfg(feature = "vulkan")]
    pub fn is_device_supports_presentation(
        &self,
        queue_family_index: u32,
        dev: &qubicon_vulkan::instance::physical_device::PhysicalDevice
    ) -> Result<bool, qubicon_vulkan::error::ValidationError> {
        unsafe {
            dev.get_x_presentation_support(
                queue_family_index, 
                self.display,
                self.visual_id
            )
        }
    }
}

impl WindowingServer {
    // raw window creation code
    unsafe fn create_window_impl(&mut self, width: u32, height: u32) -> (WindowId, /* WM_DELETE_WINDOW */ u64) {
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

        (window, wm_destroy_event)
    }
}

impl Drop for WindowingServer {
    fn drop(&mut self) {
        unsafe {
            for (&window, data) in self.windows.iter_mut() {
                #[cfg(feature = "vulkan")]
                // swapchain should be dropped before window
                core::mem::drop(data.swapchain.take());
                
                xlib::XDestroyWindow(self.display, window);
            }
            
            xlib::XCloseDisplay(self.display);
        }
    }
}


#[derive(Default)]
struct WindowData {
    x: i32,
    y: i32,
    width: u32,
    height: u32,

    wm_destroy_event: u64,
    event_queue: VecDeque<WindowEvent>,

    #[cfg(feature = "vulkan")]
    swapchain: Option<qubicon_vulkan::swapchain::Swapchain>
}


#[derive(Clone, Copy)]
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

    pub fn size(&self) -> (u32, u32) {
        (self.data.width, self.data.height)
    }

    pub fn position(&self) -> (i32, i32) {
        (self.data.x, self.data.y)
    }

    #[cfg(feature = "vulkan")]
    pub fn swapchain(&self) -> Option<&qubicon_vulkan::swapchain::Swapchain> {
        self.data.swapchain.as_ref()
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

    pub fn resize(&self, width: u32, height: u32) {
        unsafe { xlib::XResizeWindow(self.display, self.window, width, height) };
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

    #[cfg(feature = "vulkan")]
    /// # Safety
    /// Swapchain should not be replaced with other swapchain!
    pub unsafe fn swapchain_mut(&mut self) -> Option<&mut qubicon_vulkan::swapchain::Swapchain> {
        self.data.swapchain.as_mut()
    }
}

impl<'a> core::ops::Deref for WindowMut<'a> {
    type Target = Window<'a>;

    // Absolute shit. I dont know if it is fully safe.
    // Both types have practicaly equal fields, so this should work
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

impl<'a> From<WindowMut<'a>> for Window<'a> {
    fn from(value: WindowMut<'a>) -> Self {
        Self {
            display: value.display,
            window: value.window,

            data: value.data
        }
    }
}