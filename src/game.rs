use std::time::{Duration, Instant};

use winit::event::KeyEvent;

use crate::context::Context;
use crate::ecs::components::{Sprite, Transform, Velocity};
use crate::ecs::Manager;
use crate::renderer_backend::material::Material;

#[derive(Default)]
pub struct InputState {
    pub up_pressed: bool,
    pub down_pressed: bool,
}

pub struct Game<'a> {
    context: Context<'a>,
    manager: Manager,
    last_update: Instant,
    delta_time: Duration,
    input_state: InputState,
}

impl<'a> Game<'a> {
    pub fn new(context: Context<'a>) -> Self {
        let mut manager = Manager::new();

        let racket = manager.create_entity();
        let racket_transform = Transform {
            position: [-0.8, 0.0],
            scale: [0.1, 0.1],
        };
        let racket_sprite = Sprite {
            material: Material::new(
                "assets/racket.png",
                &context.device,
                &context.queue,
                &context.material_bind_group_layout,
            ),
        };
        let racket_velocity = Velocity::default();
        manager.add_sprite(racket, racket_sprite);
        manager.add_transform(racket, racket_transform);
        manager.add_velocity(racket, racket_velocity);

        Game {
            context,
            manager,
            last_update: Instant::now(),
            delta_time: Duration::default(),
            input_state: InputState::default(),
        }
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        self.manager.handle_input(key_event, &mut self.input_state);
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_update;
        self.last_update = now;
        self.manager.update(self.delta_time.as_secs_f32());
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.manager.render(&self.context)
    }
}
