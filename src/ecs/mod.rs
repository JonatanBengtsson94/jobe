pub mod component;
pub mod components;
pub mod entity;
pub mod manager;
pub mod system;
pub mod systems;

pub use manager::Manager;

pub type Signature = u8;
pub type ComponentId = u8;
