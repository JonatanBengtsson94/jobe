pub struct Transform {
    position: [f32; 2],
    scale: [f32; 2],
}

impl Transform {
    pub const ID: ComponentId = 0;
}
