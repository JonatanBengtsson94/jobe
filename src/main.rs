use app::App;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod context;
mod input;
mod renderer_backend;

// TODO::
// Bindgroup layout builder - Describe the shape of data going into shader
// Make sure pieline builder can take bindgroup layout
// Construct bindgroup layout - Use it in pipeline
// Bindgroup builder
// Engine has texture
// Shader uses texture

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
