use super::types::Entity;
use std::any::Any;
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

    pub fn insert(&mut self, entity: Entity, component: T) {
        if entity as usize > self.components.len() {
            self.components.resize_with(entity as usize + 1, || None);
        }
        self.components[entity as usize] = Some(component);
    }
}

pub struct ComponentManager {
    component_arrays: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager {
            component_arrays: HashMap::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let array = self
            .component_arrays
            .entry(type_id)
            .or_insert_with(|| Box::new(ComponentArray::<T>::new()) as Box<dyn Any>);

        if let Some(array) = array.downcast_mut::<ComponentArray<T>>() {
            array.insert(entity, component);
        } else {
            panic!("Failed to downcast to correct component array type.");
        }
    }
}
