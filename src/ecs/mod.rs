pub mod component;
pub mod components;
pub mod entity;
pub mod manager;
pub mod systems;

pub use manager::Manager;

pub type Signature = u8;
pub type Entity = u16;
pub const MAX_ENTITIES: usize = u16::MAX as usize;
