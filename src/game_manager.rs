use crate::ecs::{component::ComponentManager, entity::EntityManager};

pub struct GameManager {
    entitymanager: EntityManager,
    componentmanager: ComponentManager,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            entitymanager: EntityManager::new(),
            componentmanager: ComponentManager::new(),
        }
    }
}
