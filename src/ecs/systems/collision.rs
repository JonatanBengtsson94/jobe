use crate::ecs::{
    components::{
        signatures::{COLLIDER, TRANSFORM},
        Collider, Transform,
    },
    Signature, MAX_ENTITIES,
};

pub struct Collision;

impl Collision {
    pub const SIGNATURE: Signature = TRANSFORM | COLLIDER;

    pub fn detect_collisions(
        transforms: &Vec<Option<Transform>>,
        colliders: &Vec<Option<Collider>>,
        entity_signatures: &[Signature; MAX_ENTITIES],
    ) {
        for (i, signature) in entity_signatures.iter().enumerate() {
            if (*signature & Collision::SIGNATURE) == Collision::SIGNATURE {
                if let Some(collider_a) = &colliders[i] {
                    println!("{:?}", collider_a.layer);
                    for (j, collider_b) in colliders.iter().enumerate().skip(i + 1) {
                        if let Some(collider_b) = collider_b {
                            println!("{:?}", collider_b.layer);
                            if Self::can_collide(collider_a, collider_b) {
                                if let Some(transform_a) = &transforms[i] {
                                    println!("{:?}", transform_a.position);
                                    if let Some(transform_b) = &transforms[j] {
                                        println!("{:?}", transform_b.position);
                                        if Self::is_colliding(transform_a, transform_b) {
                                            println!("Colliding");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn can_collide(collider_a: &Collider, collider_b: &Collider) -> bool {
        (collider_a.collision_layers & collider_b.layer) != 0
            && (collider_b.collision_layers & collider_a.layer) != 0
    }

    fn is_colliding(transform_a: &Transform, transform_b: &Transform) -> bool {
        let x_overlap = (transform_a.position[0] - transform_b.position[0]).abs()
            <= (transform_a.scale[0] + transform_b.scale[0] / 2.0);
        let y_overlap = (transform_a.position[1] - transform_b.position[1]).abs()
            <= (transform_a.scale[1] + transform_b.scale[1] / 2.0);

        x_overlap && y_overlap
    }
}
