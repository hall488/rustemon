use crate::game::{Game, GameState, GameStateTrait};
use crate::renderer::Renderer;
use super::running::Running;
use std::time::Duration;
use winit::event::KeyEvent;

pub struct Paused;

impl GameStateTrait for Paused {
    fn input(&mut self, game: &mut Game, event: &KeyEvent) {
        game.input_manager.handle_input(event);
        let last_key = game.input_manager.get_last_key();
        if let Some(last_key) = last_key {
            if last_key == winit::keyboard::KeyCode::Enter {
                game.change_state(GameState::Running(Running));
            }
        }
    }

    fn update(&mut self, game: &mut Game, renderer: &mut Renderer, dt: Duration) {
        game.state = game.menu.update(&mut game.input_manager, &game.player);
    }

    fn draw(&mut self, game: &mut Game, renderer: &mut Renderer) {
        let mut instances = Vec::new();
        instances.extend_from_slice(&game.map.background);
        instances.extend_from_slice(&game.map.ground);
        instances.extend_from_slice(&game.map.foreground);
        instances.extend_from_slice(&game.player.instances);
        instances.extend_from_slice(&game.animations.iter().map(|animation| animation.instance).collect::<Vec<_>>());
        instances.extend_from_slice(&game.map.aboveground);
        instances.extend_from_slice(&game.menu.instances);
        instances.push(game.menu.pointer);

        let _ = renderer.render(&instances);
    }
}
