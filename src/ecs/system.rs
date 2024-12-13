use std::any::TypeId;

use wgpu::naga::FastHashMap;

use super::systems::Render;

pub enum System {
    Render(Render),
}

pub struct SystemManager {
    systems: FastHashMap<TypeId, System>,
}

impl SystemManager {
    pub fn new() -> Self {
        SystemManager {
            systems: FastHashMap::default(),
        }
    }
}
