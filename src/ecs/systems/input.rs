use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::ecs::components::Velocity;
use crate::game::InputState;

pub struct Input;

impl Input {
    pub fn handle_key_event(
        key_event: KeyEvent,
        player_velocity: &mut Velocity,
        input_state: &mut InputState,
    ) {
        match key_event.state {
            ElementState::Pressed => match key_event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    player_velocity.velocity[1] = 0.1;
                    input_state.up_pressed = true;
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    player_velocity.velocity[1] = -0.1;
                    input_state.down_pressed = true;
                }
                _ => {}
            },
            ElementState::Released => match key_event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    input_state.up_pressed = false;
                    if !input_state.down_pressed {
                        player_velocity.velocity[1] = 0.0;
                    } else {
                        player_velocity.velocity[1] = -0.1;
                    }
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    input_state.down_pressed = false;
                    if !input_state.up_pressed {
                        player_velocity.velocity[1] = 0.0;
                    } else {
                        player_velocity.velocity[1] = 0.1;
                    }
                }
                _ => {}
            },
        }
    }
}
