use winit::event::KeyEvent;

use crate::context::Context;

use super::{
    components::{
        signatures::{SPRITE, TRANSFORM, VELOCITY},
        Sprite, Transform, Velocity,
    },
    systems::{Input, Movement, Render},
    ComponentArray, Entity, Signature, MAX_ENTITIES,
};

pub struct Manager {
    active_entities_count: u16,
    available_entities: u16,
    entity_signatures: [Signature; MAX_ENTITIES],
    transform_components: ComponentArray<Transform>,
    sprite_components: ComponentArray<Sprite>,
    velocity_components: ComponentArray<Velocity>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            active_entities_count: 0,
            available_entities: u16::MAX,
            entity_signatures: [Signature::default(); MAX_ENTITIES],
            transform_components: ComponentArray::new(),
            sprite_components: ComponentArray::new(),
            velocity_components: ComponentArray::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        if self.active_entities_count as usize >= MAX_ENTITIES {
            panic!("Maximum amount of entities reached.")
        }

        let entity = self.available_entities.trailing_zeros() as Entity;
        self.available_entities &= !(1 << entity);
        self.active_entities_count += 1;
        entity
    }

    pub fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.sprite_components.insert(entity, sprite);
        self.entity_signatures[entity as usize] |= 1 << SPRITE;
    }

    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.transform_components.insert(entity, transform);
        self.entity_signatures[entity as usize] |= 1 << TRANSFORM;
    }

    pub fn add_velocity(&mut self, entity: Entity, velocity: Velocity) {
        self.velocity_components.insert(entity, velocity);
        self.entity_signatures[entity as usize] |= 1 << VELOCITY;
    }

    pub fn handle_input(&self, key_event: KeyEvent) {
        Input::handle_key_event(key_event);
    }

    pub fn update(&self, delta_time: f32) {
        Movement::update(
            delta_time,
            &mut self.transform_components.components,
            &self.velocity_components.components,
            &self.entity_signatures,
        );
    }

    pub fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        return Render::render(
            context,
            &self.sprite_components.components,
            &self.entity_signatures,
        );
    }
}
