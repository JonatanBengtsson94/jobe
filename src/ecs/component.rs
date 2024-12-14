use std::usize;

use super::{
    components::{Sprite, Transform, Velocity},
    entity::Entity,
};

pub mod signatures {
    pub const TRANSFORM: u8 = 0b0000_0001;
    pub const SPRITE: u8 = 0b0000_0010;
    pub const VELOCITY: u8 = 0b0000_0100;
}

pub struct ComponentArray<T> {
    pub components: Vec<Option<T>>,
}

impl<T> ComponentArray<T> {
    pub fn new() -> Self {
        ComponentArray {
            components: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        if entity as usize >= self.components.len() {
            self.components.resize_with(entity as usize + 1, || None);
        }
        self.components[entity as usize] = Some(component);
    }
}

pub struct ComponentManager {
    pub transform_components: ComponentArray<Transform>,
    pub sprite_components: ComponentArray<Sprite>,
    pub velocity_components: ComponentArray<Velocity>,
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager {
            transform_components: ComponentArray::new(),
            sprite_components: ComponentArray::new(),
            velocity_components: ComponentArray::new(),
        }
    }

    pub fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.sprite_components.insert(entity, sprite);
    }

    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.transform_components.insert(entity, transform);
    }

    pub fn add_velocity(&mut self, entity: Entity, velocity: Velocity) {
        self.velocity_components.insert(entity, velocity);
    }
}
