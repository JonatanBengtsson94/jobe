use std::u16;

pub type Entity = u16;
pub const MAX_ENTITIES: usize = u16::MAX as usize;

pub const MAX_COMPONENTS: u8 = 16;
pub type Signature = u16;
