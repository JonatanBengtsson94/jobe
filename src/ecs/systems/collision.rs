use crate::ecs::{
    components::{
        signatures::{COLLIDER, TRANSFORM},
        Collider, Transform,
    },
    CollisionEvent, Event, EventQueue, Signature, MAX_ENTITIES,
};

pub struct Collision;

impl Collision {
    pub const SIGNATURE: Signature = TRANSFORM | COLLIDER;

    pub fn update(
        transforms: &Vec<Option<Transform>>,
        colliders: &Vec<Option<Collider>>,
        entity_signatures: &[Signature; MAX_ENTITIES],
        event_queue: &mut EventQueue,
    ) {
        for (i, signature) in entity_signatures.iter().enumerate() {
            if (*signature & Collision::SIGNATURE) == Collision::SIGNATURE {
                if let Some(collider_a) = &colliders[i] {
                    for (j, collider_b) in colliders.iter().enumerate().skip(i + 1) {
                        if let Some(collider_b) = collider_b {
                            if Self::can_collide(collider_a, collider_b) {
                                if let Some(transform_a) = &transforms[i] {
                                    if let Some(transform_b) = &transforms[j] {
                                        if Self::is_colliding(transform_a, transform_b) {
                                            event_queue.push(Event::Collision(
                                                CollisionEvent::PlayerOnEnemy(j as u16),
                                            ));
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
