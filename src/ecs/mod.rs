pub mod components;
pub mod manager;
pub mod systems;

pub use manager::Manager;

pub type Signature = u8;
pub type Entity = u16;
pub const MAX_ENTITIES: usize = u16::MAX as usize;

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
        if entity as usize >= self.components.len() {
            self.components.resize_with(entity as usize + 1, || None);
        }
        self.components[entity as usize] = Some(component);
    }
}

pub enum Event {
    Collision(CollisionEvent),
}

pub enum CollisionEvent {
    PlayerOnEnemy(Entity),
}

pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue { events: Vec::new() }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }
}

pub mod layers {
    pub const PLAYER: u8 = 0b0000_0001;
    pub const ENEMY: u8 = 0b0000_0010;
}
