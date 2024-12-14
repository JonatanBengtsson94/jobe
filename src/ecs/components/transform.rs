use crate::ecs::ComponentId;

pub struct Transform {
    pub position: [f32; 2],
    pub scale: [f32; 2],
}

impl Transform {
    pub const ID: ComponentId = 0;
}
