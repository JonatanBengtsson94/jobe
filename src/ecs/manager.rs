use winit::event::KeyEvent;

use crate::{
    context::Context,
    ecs::{
        component::{Component, ComponentManager, Sprite},
        entity::{Entity, EntityManager},
        systems::Render,
    },
    renderer_backend::{material::Material, mesh::Quad},
};

pub struct Manager {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    render: Render,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            render: Render::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity().unwrap()
    }

    pub fn add_component(&mut self, entity: Entity, component: Component) {
        self.component_manager.add_component(entity, component);
        let signature = self.entity_manager.get_signature(entity);
        // Set signature bits based on componentid
        // entitymanager.setsignature
    }

    pub fn render(&self, context: &Context) {
        self.render.render(context);
    }
}
