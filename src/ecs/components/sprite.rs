use crate::{
    ecs::ComponentId,
    renderer_backend::{material::Material, mesh::Quad},
};

pub struct Sprite {
    pub material: Material,
    pub quad: Quad,
}

impl Sprite {
    pub const ID: ComponentId = 1;
}
