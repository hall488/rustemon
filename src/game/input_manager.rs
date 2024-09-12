use std::collections::{HashSet, VecDeque};
use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{PhysicalKey, KeyCode};

pub struct InputManager {
    pub pressed_keys: HashSet<KeyCode>,
    pub key_order: VecDeque<KeyCode>,
    pub release_key: Option<KeyCode>,
    pub triggered_keys: HashSet<KeyCode>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            key_order: VecDeque::new(),
            release_key: None,
            triggered_keys: HashSet::new(),
        }
    }

    pub fn handle_input(&mut self, input: &KeyEvent) -> HashSet<KeyCode> {
        match input.physical_key {
            PhysicalKey::Code(keycode) => {
                match input.state {
                    ElementState::Pressed => {
                        if !self.pressed_keys.contains(&keycode) {
                            self.pressed_keys.insert(keycode);
                            self.key_order.push_back(keycode);
                            self.triggered_keys.insert(keycode);
                        }
                    }
                    ElementState::Released => {
                        self.pressed_keys.remove(&keycode);
                        self.key_order.retain(|&k| k != keycode);
                        self.release_key = Some(keycode);
                        self.triggered_keys.remove(&keycode);
                    }
                }
            }
            PhysicalKey::Unidentified(_) => {
            }
        }

        self.pressed_keys.clone()
    }

    pub fn get_last_key(&self) -> Option<KeyCode> {
        self.key_order.back().cloned()
    }

    pub fn get_release_key(&mut self) -> Option<KeyCode> {
        if let Some(keycode) = self.release_key {
            self.release_key = None;
            Some(keycode)
        } else {
            None
        }
    }

   pub fn get_key_on_press(&mut self) -> Option<KeyCode> {
        if let Some(&keycode) = self.triggered_keys.iter().next() {
            self.triggered_keys.remove(&keycode);
            Some(keycode)
        } else {
            None
        }
    }
}
