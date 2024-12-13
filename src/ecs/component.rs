use wgpu::naga::FastHashMap;

use crate::renderer_backend::{material::Material, mesh::Quad};
use std::{any::TypeId, usize};

use super::{entity::Entity, ComponentId};

pub struct Transform {
    position: [f32; 2],
    scale: [f32; 2],
}

pub struct Sprite {
    pub material: Material,
    pub quad: Quad,
}

pub enum Component {
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
    component_type_ids: FastHashMap<TypeId, ComponentId>,
}

impl ComponentManager {
    pub fn new() -> Self {
        let mut component_type_ids = FastHashMap::default();
        component_type_ids.insert(TypeId::of::<Transform>(), 0);
        component_type_ids.insert(TypeId::of::<Sprite>(), 1);
        ComponentManager {
            component_arrays: FastHashMap::default(),
            component_type_ids,
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

    pub fn get_component_type_id<Component: 'static>(&self) -> ComponentId {
        let type_id = TypeId::of::<Component>();
        self.component_type_ids[&type_id]
    }
}
