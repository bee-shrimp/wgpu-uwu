#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

mod renderer;
use renderer::Renderer;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    Up,
    Down,
    #[default]
    Still,
}

#[derive(Default)]
struct App {
    renderer: Option<Renderer>,
    key_table: Box<[bool]>,
    brightness: f32,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // ----------------------------------------------------------------------------------------- create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        // --------------------------------------------------------------------------------------------- create key table
        self.key_table = vec![false; 255].into_boxed_slice();

        // ---------------------------------------------------------------------------------------------- create renderer
        let renderer = pollster::block_on(Renderer::new(
            event_loop.owned_display_handle(),
            window.clone(),
        ));
        self.renderer = Some(renderer);
        self.brightness = 0.0;

        // ------------------------------------------------------------------------------------------------------- redraw
        window.request_redraw();
    }

    // --------------------------------------------------------------------------------------------- handle window events
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let renderer = self.renderer.as_mut().unwrap();

        match event {
            // --------------------------------------------------------------------------------------------- close window
            WindowEvent::CloseRequested => {
                println!("close requested; stopping");
                event_loop.exit();
            }
            // --------------------------------------------------------------------------------------------------- redraw
            WindowEvent::RedrawRequested => {
                renderer.render();
            }

            // --------------------------------------------------------------------------------------------------- resize
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                renderer.resize(size);
            }
            // ----------------------------------------------------------------------------------------- handle key input
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    self.key_table[code as usize] = event.state.is_pressed();
                }
            }
            _ => (),
        }
    }

    #[allow(unused_variables)]
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let renderer = self.renderer.as_mut().unwrap();

        let mut direction: Direction = Direction::Still;

        if self.key_table[KeyCode::ArrowUp as usize] {
            direction = Direction::Up
        }

        if self.key_table[KeyCode::ArrowDown as usize] {
            direction = Direction::Down
        }

        self.brightness += direction_to_brightness(direction);
        renderer.update(self.brightness);
        renderer.get_window().request_redraw()
    }
}

fn direction_to_brightness(direction: Direction) -> f32 {
    match direction {
        crate::Direction::Up => 0.01,
        crate::Direction::Down => -0.01,
        crate::Direction::Still => 0.0,
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
