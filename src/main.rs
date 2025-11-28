use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

#[derive(Default)]
struct App {
    window: Option<Arc<winit::window::Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attributes = Window::default_attributes()
                .with_title("Test Window")
                .with_inner_size(PhysicalSize::new(1920, 1080));

            let window = event_loop
                .create_window(attributes)
                .expect("failed to create window");
            self.window = Some(Arc::new(window));
            println!("Window created");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(window) = &self.window else {
            return;
        };

        // used to ignore events unrelated to the window we just created
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_new_size) => {
                window.request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    println!("Hello, world!");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    let _ = event_loop.run_app(&mut app);
}
