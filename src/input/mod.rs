use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::collections::HashMap;

#[derive(Default)]
pub struct Input {
    held_keys: HashMap<Scancode, bool>,
    pressed_keys: HashMap<Scancode, bool>,
    release_keys: HashMap<Scancode, bool>,
}

impl Input {
    /// This function gets called at the beginning of each new frame
    /// to reset the keys that are no longer relevant
    pub fn begin_new_frame(&mut self) {
        self.pressed_keys.clear();
        self.release_keys.clear();
    }

    pub fn key_up_event(&mut self, event: &Event) {
        if let Event::KeyUp {
            scancode: Some(s), ..
        } = event
        {
            self.release_keys.insert(*s, true);
            self.held_keys.insert(*s, false);
        }
    }

    /// this gets called when a key has been pressed
    pub fn key_down_event(&mut self, event: &Event) {
        if let Event::KeyDown {
            scancode: Some(s), ..
        } = event
        {
            self.pressed_keys.insert(*s, true);
            self.held_keys.insert(*s, true);
        }
    }

    /// check if a certain key was pressed during the current frame
    pub fn was_key_pressed(&self, key: Scancode) -> bool {
        *self.pressed_keys.get(&key).unwrap_or(&false)
    }

    /// check if a certain key was released during the current frame
    pub fn was_key_release(&self, key: Scancode) -> bool {
        *self.release_keys.get(&key).unwrap_or(&false)
    }

    /// check if a certain key was held
    pub fn is_key_held(&self, key: Scancode) -> bool {
        *self.held_keys.get(&key).unwrap_or(&false)
    }
}
