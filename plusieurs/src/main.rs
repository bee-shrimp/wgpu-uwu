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

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos_x: f32,
    pub pos_y: f32,
    pub speed: f32,
    pub last_pos: (f32, f32),
}

impl Rect {
    fn new() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            speed: 0.01,
            last_pos: (0.0, 0.0),
        }
    }
    fn update(&mut self, dir: (Direction, Direction)) -> (f32, f32) {
        self.last_pos = (self.pos_x, self.pos_y);

        let dir_x = match dir.0 {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
            _ => 0.0,
        };
        let dir_y = match dir.1 {
            Direction::Down => -1.0,
            Direction::Up => 1.0,
            _ => 0.0,
        };
        self.pos_x += dir_x * self.speed;
        self.pos_y += dir_y * self.speed;
        (self.pos_x, self.pos_y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    #[default]
    Still,
}

#[derive(Default)]
struct App {
    renderer: Option<Renderer>,
    key_table: Box<[bool]>,
    rect: Option<Rect>,
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
        self.rect = Some(Rect::new());

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
        let rect = self.rect.as_mut().unwrap();

        let mut direction_x: Direction = Direction::Still;
        let mut direction_y: Direction = Direction::Still;

        if self.key_table[KeyCode::ArrowLeft as usize] {
            direction_x = Direction::Left
        }

        if self.key_table[KeyCode::ArrowRight as usize] {
            direction_x = Direction::Right
        }

        if self.key_table[KeyCode::ArrowUp as usize] {
            direction_y = Direction::Up
        }

        if self.key_table[KeyCode::ArrowDown as usize] {
            direction_y = Direction::Down
        }

        let direction = (direction_x, direction_y);
        let rect_pos = rect.update(direction);

        if rect_pos != rect.last_pos {
            renderer.update(rect_pos);
            renderer.get_window().request_redraw()
        };
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
