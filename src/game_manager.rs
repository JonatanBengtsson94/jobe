use winit::event::KeyEvent;

use crate::{
    context::Context,
    ecs::{
        component::{Component, ComponentManager},
        entity::{Entity, EntityManager},
    },
};

pub struct GameManager<'a> {
    context: Context<'a>,
    entity_manager: EntityManager,
    component_manager: ComponentManager,
}

impl<'a> GameManager<'a> {
    pub fn new(context: Context<'a>) -> Self {
        GameManager {
            context,
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
        }
    }

    pub fn add_component(&mut self, entity: Entity, component: Component) {
        self.component_manager.add_component(entity, component);
        let signature = self.entity_manager.get_signature(entity);
        // Set signature bits based on componentid
        // entitymanager.setsignature
    }

    pub fn handle_input(&self, key_event: KeyEvent) {}

    pub fn update(&mut self) {}

    pub fn render(&mut self) {}
}
