pub mod sprite;
pub mod transform;
pub mod velocity;

pub use sprite::Sprite;
pub use transform::Transform;
pub use velocity::Velocity;

pub mod signatures {
    pub const TRANSFORM: u8 = 0b0000_0001;
    pub const SPRITE: u8 = 0b0000_0010;
    pub const VELOCITY: u8 = 0b0000_0100;
}
