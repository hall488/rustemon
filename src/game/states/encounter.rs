use crate::game::{Game, GameStateTrait};
use crate::renderer::Renderer;
use std::time::Duration;
use winit::event::KeyEvent;

pub struct Encounter;

impl GameStateTrait for Encounter {
    fn input(&mut self, game: &mut Game, event: &KeyEvent) {
        // Encounter state input logic
    }

    fn update(&mut self, game: &mut Game, renderer: &mut Renderer, dt: Duration) {
        // Encounter state update logic
    }

    fn draw(&mut self, game: &mut Game, renderer: &mut Renderer) {
        // Encounter state draw logic
    }
}
