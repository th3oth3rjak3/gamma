use winit::keyboard::KeyCode;

use crate::gamma::Gamma;

impl<S> Gamma<S> {
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
}
