use crate::ecs::{
    component::signatures::{TRANSFORM, VELOCITY},
    components::{Transform, Velocity},
    entity::MAX_ENTITIES,
    Signature,
};

pub struct Movement {}

impl Movement {
    pub const SIGNATURE: Signature = TRANSFORM | VELOCITY;

    pub fn update(
        delta_time: f32,
        transforms: &mut Vec<Option<Transform>>,
        velocities: &Vec<Option<Velocity>>,
        entity_signatures: &[Signature; MAX_ENTITIES],
    ) {
        for (index, signature) in entity_signatures.iter().enumerate() {
            if (*signature & Movement::SIGNATURE) == Movement::SIGNATURE {
                if let Some(transform) = &mut transforms[index] {
                    if let Some(velocity) = &velocities[index] {
                        transform.position[0] =
                            transform.position[0] + velocity.velocity[0] * delta_time;
                        transform.position[1] =
                            transform.position[1] + velocity.velocity[1] * delta_time;
                    }
                }
            }
        }
    }
}
