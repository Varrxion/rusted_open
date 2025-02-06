use glfw::{Action, Key, WindowEvent};
use std::collections::HashMap;

// Define the states a key can be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeyState {
    Pressed,
    Held,
    Released,
}

pub struct KeyStates {
    keys: HashMap<Key, KeyState>, // Track the state of each key
}

impl KeyStates {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    // Handle window events to update the key states
    pub fn handle_key_event(&mut self, event: WindowEvent) {
        if let WindowEvent::Key(key, _, action, _) = event {
            let new_state = match action {
                Action::Press => KeyState::Pressed,
                Action::Repeat => KeyState::Held,
                Action::Release => KeyState::Released,
                _ => return,
            };

            // Update the state of the key in the hashmap
            self.keys.insert(key, new_state);
        }
    }

    // Returns true if the key was just pressed (not held from the previous tick)
    pub fn is_key_pressed(&self, key: Key) -> bool {
        match self.keys.get(&key) {
            Some(KeyState::Pressed) => true,
            _ => false,
        }
    }

    // Returns true if the key is pressed (either newly pressed or held)
    pub fn is_key_pressed_raw(&self, key: Key) -> bool {
        match self.keys.get(&key) {
            Some(KeyState::Pressed) | Some(KeyState::Held) => true,
            _ => false,
        }
    }
}
