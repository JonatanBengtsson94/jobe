use winit::event::KeyEvent;

use crate::{
    context::Context,
    ecs::{components::Sprite, systems::Render, ComponentManager, Entity, EntityManager},
};

use super::systems::Input;

pub struct Manager {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity().unwrap()
    }

    pub fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.component_manager.add_sprite(entity, sprite);
        let mut signature = self.entity_manager.get_signature(entity);
        signature |= 1 << Sprite::ID;
        self.entity_manager.set_signature(entity, signature);
    }

    pub fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        return Render::render(
            context,
            &self.component_manager.sprite_components.components,
            &self.entity_manager.entity_signatures,
        );
    }

    pub fn handle_input(&self, key_event: KeyEvent) {
        Input::handle_key_event(key_event);
    }
}
