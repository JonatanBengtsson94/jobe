use super::types::Entity;
use std::{any::TypeId, collections::HashMap, usize};

pub struct ComponentArray<T> {
    components: Vec<Option<T>>,
}

impl<T> ComponentArray<T> {
    pub fn new() -> Self {
        ComponentArray {
            components: Vec::new(),
        }
    }
}

impl<T> ComponentArray<T> {
    pub fn insert(&mut self, entity: Entity, component: T) {
        if entity as usize > self.components.len() {
            self.components.resize_with(entity as usize + 1, || None);
        }
        self.components[entity as usize] = Some(component);
    }
}

pub struct ComponentManager {
    component_arrays: HashMap<TypeId, Box<dyn std::any::Any>>,
}
