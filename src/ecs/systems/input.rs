use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::ecs::components::Velocity;

pub struct Input;

impl Input {
    pub fn handle_key_event(key_event: KeyEvent, player_velocity: &mut Velocity) {
        match key_event.state {
            ElementState::Pressed => match key_event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    player_velocity.velocity[1] = 0.1;
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    player_velocity.velocity[1] = -0.1;
                }
                _ => {}
            },
            ElementState::Released => match key_event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    if player_velocity.velocity[1] > 0.0 {
                        player_velocity.velocity[1] = 0.0;
                    }
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    if player_velocity.velocity[1] < 0.0 {
                        player_velocity.velocity[1] = 0.0;
                    }
                }
                _ => {}
            },
        }
    }
}
