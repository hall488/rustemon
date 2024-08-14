mod slot;
mod cancel;

use cancel::Cancel;
use slot::Slot;
use crate::renderer::Renderer;
use crate::game::pokemon::Pokemon;
use crate::game::input_manager::InputManager;
use std::time::Duration;
use crate::renderer::sprite::Sprite;
use std::time::Instant;
use winit::keyboard::KeyCode;

const HOLD_DURATION: Duration = Duration::from_millis(500);
const HOLD_TIME_PER_SLOT: Duration = Duration::from_millis(100);

pub struct Party {
    background: Sprite,
    pub slots: Vec<Slot>,
    selected_slot: i32,
    cancel: Cancel,
    pub last_key: Option<KeyCode>,
    time_of_input: Instant,
    time_of_hold: Instant,
    processed_initial_press: bool,
    in_battle: bool,
}

impl Party {
    pub fn new(pokemon: &mut Vec<Pokemon>, in_battle: bool, renderer: &mut Renderer) -> Self {

        let background = renderer.create_sprite(0.0, 0.0, 0, 0, 15, 10, "party", 1.0, 1.0).expect("");

        let mut slots = Vec::new();

        for (i, p) in pokemon.iter().enumerate() {
            slots.push(Slot::new(p, i as u32, renderer));
        }

        let cancel = Cancel::new(renderer);

        let last_key = None;
        let time_of_input = Instant::now();
        let time_of_hold = Instant::now();
        let processed_initial_press = false;

        Self {
            background,
            slots,
            selected_slot: 0,
            cancel,
            last_key,
            time_of_input,
            time_of_hold,
            processed_initial_press,
            in_battle,
        }
    }

    pub fn update(&mut self, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> u32 {

        let input_key = input_manager.get_last_key();
        let single_press_key = input_manager.get_key_on_press();

        if input_key != self.last_key {
            self.last_key = input_key;
            self.time_of_input = Instant::now();
            self.processed_initial_press = false;
        }

        let time_held = Instant::now().duration_since(self.time_of_input);

        match self.last_key {
            Some(KeyCode::KeyW) => {
                self.handle_key_press(time_held, -1);
            }
            Some(KeyCode::KeyS) => {
                self.handle_key_press(time_held, 1);
            }
            Some(KeyCode::KeyX) => {
                self.selected_slot = self.slots.len() as i32;
            }
            _ => {}
        }

        if single_press_key == Some(KeyCode::KeyZ) {
            if self.selected_slot == self.slots.len() as i32 {
                return 6;
            } else {
                return self.selected_slot as u32;
            }
        }

        self.slots.iter_mut().for_each(|slot| {
            slot.selected = false;
        });

        self.cancel.selected = false;

        if self.selected_slot < self.slots.len() as i32 {
            self.slots[self.selected_slot as usize].selected = true;
        } else {
            self.cancel.selected = true;
        }

        return 7;
    }

    fn handle_key_press(&mut self, time_held: Duration, direction: i32) {
        if self.processed_initial_press {
            if time_held > HOLD_DURATION {
                if self.time_of_hold.elapsed() > HOLD_TIME_PER_SLOT {
                    self.selected_slot += direction;
                    self.time_of_hold = Instant::now();
                }
            }
        } else {
            self.selected_slot += direction;
            self.processed_initial_press = true;
        }

        if self.selected_slot < 0 {
            self.selected_slot = self.slots.len() as i32;
        } else if self.selected_slot > self.slots.len() as i32 {
            self.selected_slot = 0;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let mut instances = Vec::new();

        instances.extend_from_slice(&self.background.texture);

        for slot in &self.slots {
            slot.draw(&mut instances);
        }

        self.cancel.draw(&mut instances);

        let _ = renderer.render(&instances, false);
    }
}
