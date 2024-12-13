use crate::ecs::{
    component::{Component, ComponentManager},
    entity::{Entity, EntityManager},
    system::SystemManager,
};

pub struct GameManager {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: SystemManager::new(),
        }
    }

    pub fn add_component(&mut self, entity: Entity, component: Component) {
        self.component_manager.add_component(entity, component);
        let signature = self.entity_manager.get_signature(entity);
        // Set signature bits based on componentid
        // entitymanager.setsignature
        // Tell system manager?
    }
}
