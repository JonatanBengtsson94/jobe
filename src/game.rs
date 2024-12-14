use winit::event::KeyEvent;

use crate::context::Context;
use crate::ecs::components::{Sprite, Transform};
use crate::ecs::Manager;
use crate::renderer_backend::material::Material;
use crate::renderer_backend::mesh::Quad;

pub struct Game<'a> {
    context: Context<'a>,
    manager: Manager,
}

impl<'a> Game<'a> {
    pub fn new(context: Context<'a>) -> Self {
        let mut manager = Manager::new();

        let racket = manager.create_entity();
        let racket_transform = Transform {
            position: [0.0, 0.0],
            scale: [0.5, 0.5],
        };
        let racket_sprite = Sprite {
            material: Material::new(
                "assets/racket.png",
                &context.device,
                &context.queue,
                &context.material_bind_group_layout,
            ),
            quad: Quad::new(&context.device, &racket_transform),
        };
        manager.add_sprite(racket, racket_sprite);
        manager.add_transform(racket, racket_transform);

        Game { context, manager }
    }

    pub fn handle_input(&self, key_event: KeyEvent) {
        self.manager.handle_input(key_event);
    }

    pub fn update(&self) {}

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.manager.render(&self.context)
    }
}
