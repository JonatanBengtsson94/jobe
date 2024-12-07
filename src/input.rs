use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub fn handle_key_event(key_event: KeyEvent) {
    match key_event.state {
        ElementState::Pressed => match key_event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => {
                println!("Move up");
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                println!("Move down");
            }
            _ => {}
        },
        ElementState::Released => {
            println!("Stop moving");
        }
    }
}
