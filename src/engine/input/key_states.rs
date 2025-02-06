use glfw::{Action, Key, WindowEvent};
use std::collections::HashSet;

pub struct KeyStates {
    active_keys: HashSet<Key>, // Track active keys
}

impl KeyStates {
    pub fn new() -> Self {
        Self {
            active_keys: HashSet::new(),
        }
    }

    // Handle window events to update active keys
    pub fn handle_key_event(&mut self, event: WindowEvent) {
        if let WindowEvent::Key(key, _, action, _) = event {
            match action {
                Action::Press | Action::Repeat => {
                    self.active_keys.insert(key);
                }
                Action::Release => {
                    self.active_keys.remove(&key);
                }
                _ => {}
            }
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.active_keys.contains(&key)
    }
}
