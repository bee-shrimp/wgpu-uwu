#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

mod renderer;
use renderer::Renderer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    Still,
}

#[derive(Default)]
struct App {
    renderer: Option<Renderer>,
    key_table: Box<[bool]>,
    time: f32,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        self.key_table = vec![false; 255].into_boxed_slice(); //Before event loop

        let renderer = pollster::block_on(Renderer::new(
            event_loop.owned_display_handle(),
            window.clone(),
        ));
        self.renderer = Some(renderer);

        self.time = 0.0;

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let renderer = self.renderer.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                renderer.render();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                renderer.resize(size);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key_code = match event.physical_key {
                    PhysicalKey::Code(keycode) => Some(keycode),
                    _ => None,
                };

                if event.state.is_pressed()
                    && let Some(code) = key_code
                {
                    self.key_table[code as usize] = true
                } else if let Some(code) = key_code {
                    self.key_table[code as usize] = false
                }
            }
            _ => (),
        }
    }

    #[allow(unused_variables)]
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let renderer = self.renderer.as_mut().unwrap();

        if self.key_table[KeyCode::ArrowLeft as usize] {
            todo!()
        }

        self.time += 0.01;
        renderer.update(self.time);
        renderer.get_window().request_redraw();
    }
}

fn main() {
    // wgpu uses `log` for all of our logging, so we initialize a logger with the `env_logger` crate.
    //
    // To change the log level, set the `RUST_LOG` environment variable. See the `env_logger`
    // documentation for more information.
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    // When the current loop iteration finishes, immediately begin a new
    // iteration regardless of whether or not new events are available to
    // process. Preferred for applications that want to render as fast as
    // possible, like games.
    // event_loop.set_control_flow(ControlFlow::Poll);

    // When the current loop iteration finishes, suspend the thread until
    // another event arrives. Helps keeping CPU utilization low if nothing
    // is happening, which is preferred if the application might be idling in
    // the background.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
