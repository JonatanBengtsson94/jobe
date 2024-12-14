use std::usize;

use super::entity::Entity;

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
