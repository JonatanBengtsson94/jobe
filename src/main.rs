use app::App;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod context;
mod ecs;
mod game_manager;
mod renderer_backend;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
