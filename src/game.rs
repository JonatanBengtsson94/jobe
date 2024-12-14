use winit::event::KeyEvent;

use crate::context::Context;
use crate::ecs::component::{Component, Sprite};
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
        let racket_sprite = Sprite {
            material: Material::new(
                "racket.png",
                &context.device,
                &context.queue,
                &context.material_bind_group_layout,
            ),
            quad: Quad::new(&context.device),
        };
        manager.add_component(racket, Component::Sprite(racket_sprite));

        Game { context, manager }
    }

    pub fn handle_input(&self, key_event: KeyEvent) {}

    pub fn update(&self) {}

    pub fn render(&self) {
        self.manager.render(&self.context)
    }
}
