use std::usize;

use super::{
    components::{Sprite, Transform},
    entity::Entity,
};

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
        if entity as usize > self.components.len() {
            self.components.resize_with(entity as usize + 1, || None);
        }
        self.components[entity as usize] = Some(component);
    }
}

pub struct ComponentManager {
    pub transform_components: ComponentArray<Transform>,
    pub sprite_components: ComponentArray<Sprite>,
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager {
            transform_components: ComponentArray::new(),
            sprite_components: ComponentArray::new(),
        }
    }

    pub fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.sprite_components.insert(entity, sprite);
    }
}
