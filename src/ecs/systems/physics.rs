use crate::ecs::{
    components::Velocity, CollisionEvent, Event, EventQueue, Signature, MAX_ENTITIES,
};

pub struct Physics;

impl Physics {
    pub fn update(
        velocities: &mut Vec<Option<Velocity>>,
        entity_signatures: &[Signature; MAX_ENTITIES],
        event_queue: &mut EventQueue,
    ) {
        let events = event_queue.drain();
        for event in events {
            match event {
                Event::Collision(collision_event) => match collision_event {
                    CollisionEvent::PlayerOnEnemy(enemy) => {
                        if let Some(velocity) = &mut velocities[enemy as usize] {
                            velocity.velocity[0] = velocity.velocity[0] * -1.0;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
