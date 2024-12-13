use wgpu::naga::FastHashMap;

use crate::renderer_backend::{material::Material, mesh::Quad};

use super::entity::Entity;
use std::{any::TypeId, usize};

pub struct Transform {
    position: [f32; 2],
    scale: [f32; 2],
}

pub struct Sprite {
    material: Material,
    quad: Quad,
}

enum Component {
    Transform(Transform),
    Sprite(Sprite),
}

pub struct ComponentArray<Component> {
    components: Vec<Option<Component>>,
}

impl<Component> ComponentArray<Component> {
    pub fn new() -> Self {
        ComponentArray {
            components: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: Component) {
        if entity as usize > self.components.len() {
            self.components.resize_with(entity as usize + 1, || None);
        }
        self.components[entity as usize] = Some(component);
    }
}

pub struct ComponentManager {
    component_arrays: FastHashMap<TypeId, ComponentArray<Component>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager {
            component_arrays: FastHashMap::default(),
        }
    }

    pub fn add_component(&mut self, entity: Entity, component: Component) {
        let type_id = TypeId::of::<Component>();
        let array = self
            .component_arrays
            .entry(type_id)
            .or_insert(ComponentArray::new());

        array.insert(entity, component);
    }
}
