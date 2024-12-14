use crate::context::Context;
use crate::game_manager::GameManager;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default)]
pub struct App<'a> {
    window: Option<Arc<Window>>,
    game_manager: Option<GameManager<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Pong")
            .with_inner_size(LogicalSize::new(800, 600))
            .with_resizable(false);
        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window"),
        );
        self.window = Some(window.clone());

        let context = pollster::block_on(Context::new(window.clone()));
        self.game_manager = Some(GameManager::new(context));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                if let Some(game_manager) = &self.game_manager {
                    game_manager.handle_input(event)
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(game_manager) = &mut self.game_manager {
                    game_manager.update();
                    game_manager.render();
                }
            }

            _ => (),
        }
    }
}
