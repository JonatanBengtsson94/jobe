pub mod component;
pub mod components;
pub mod entity;
pub mod manager;
pub mod system;
pub mod systems;

pub use component::ComponentManager;
pub use entity::Entity;
pub use entity::EntityManager;
pub use manager::Manager;

pub type Signature = u8;
pub type ComponentId = u8;
