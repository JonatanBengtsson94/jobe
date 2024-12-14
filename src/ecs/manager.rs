use winit::event::KeyEvent;

use crate::context::Context;

use super::{
    component::{
        signatures::{SPRITE, TRANSFORM, VELOCITY},
        ComponentArray,
    },
    components::{Sprite, Transform, Velocity},
    systems::{movement::Movement, Input, Render},
    Entity, EntityManager,
};

pub struct Manager {
    entity_manager: EntityManager,
    transform_components: ComponentArray<Transform>,
    sprite_components: ComponentArray<Sprite>,
    velocity_components: ComponentArray<Velocity>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            entity_manager: EntityManager::new(),
            transform_components: ComponentArray::new(),
            sprite_components: ComponentArray::new(),
            velocity_components: ComponentArray::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity().unwrap()
    }

    pub fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.sprite_components.insert(entity, sprite);
        let mut signature = self.entity_manager.get_signature(entity);
        signature |= 1 << SPRITE;
        self.entity_manager.set_signature(entity, signature);
    }

    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.transform_components.insert(entity, transform);
        let mut signature = self.entity_manager.get_signature(entity);
        signature |= 1 << TRANSFORM;
        self.entity_manager.set_signature(entity, signature);
    }

    pub fn add_velocity(&mut self, entity: Entity, velocity: Velocity) {
        self.velocity_components.insert(entity, velocity);
        let mut signature = self.entity_manager.get_signature(entity);
        signature |= 1 << VELOCITY;
        self.entity_manager.set_signature(entity, signature);
    }

    pub fn handle_input(&self, key_event: KeyEvent) {
        Input::handle_key_event(key_event);
    }

    pub fn update(&self, delta_time: f32) {
        Movement::update(
            delta_time,
            &mut self.component_manager.transform_components.components,
            &self.component_manager.velocity_components.components,
            &self.entity_manager.entity_signatures,
        );
    }

    pub fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        return Render::render(
            context,
            &self.component_manager.sprite_components.components,
            &self.entity_manager.entity_signatures,
        );
    }
}
