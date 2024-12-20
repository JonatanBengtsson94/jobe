use winit::event::KeyEvent;

use crate::{context::Context, game::InputState};

use super::{
    components::{
        signatures::{COLLIDER, SPRITE, TRANSFORM, VELOCITY},
        Collider, Sprite, Transform, Velocity,
    },
    systems::{Collision, Input, Movement, Physics, Render},
    ComponentArray, Entity, EventQueue, Signature, MAX_ENTITIES,
};

pub struct Manager {
    active_entities_count: u16,
    available_entities: u16,
    entity_signatures: [Signature; MAX_ENTITIES],
    event_queue: EventQueue,
    transform_components: ComponentArray<Transform>,
    sprite_components: ComponentArray<Sprite>,
    velocity_components: ComponentArray<Velocity>,
    collider_components: ComponentArray<Collider>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            active_entities_count: 0,
            available_entities: u16::MAX,
            entity_signatures: [Signature::default(); MAX_ENTITIES],
            event_queue: EventQueue::new(),
            transform_components: ComponentArray::new(),
            sprite_components: ComponentArray::new(),
            velocity_components: ComponentArray::new(),
            collider_components: ComponentArray::new(),
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
        self.entity_signatures[entity as usize] |= SPRITE;
    }

    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.transform_components.insert(entity, transform);
        self.entity_signatures[entity as usize] |= TRANSFORM;
    }

    pub fn add_velocity(&mut self, entity: Entity, velocity: Velocity) {
        self.velocity_components.insert(entity, velocity);
        self.entity_signatures[entity as usize] |= VELOCITY;
    }

    pub fn add_collider(&mut self, entity: Entity, collider: Collider) {
        self.collider_components.insert(entity, collider);
        self.entity_signatures[entity as usize] |= COLLIDER;
    }

    pub fn handle_input(&mut self, key_event: KeyEvent, input_state: &mut InputState) {
        if let Some(player_velocity) = &mut self.velocity_components.components[0] {
            Input::handle_key_event(key_event, player_velocity, input_state);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        Physics::update(
            &mut self.velocity_components.components,
            &self.entity_signatures,
            &mut self.event_queue,
        );

        Movement::update(
            delta_time,
            &mut self.transform_components.components,
            &self.velocity_components.components,
            &self.entity_signatures,
        );

        Collision::update(
            &self.transform_components.components,
            &self.collider_components.components,
            &self.entity_signatures,
            &mut self.event_queue,
        );
    }

    pub fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        return Render::render(
            context,
            &self.sprite_components.components,
            &self.transform_components.components,
            &self.entity_signatures,
        );
    }
}
