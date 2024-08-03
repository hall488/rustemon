use crate::game::{Game, GameState, GameStateTrait};
use crate::renderer::Renderer;
use std::time::Duration;
use winit::event::KeyEvent;
use super::paused::Paused;

pub struct Running;

impl GameStateTrait for Running {
    fn input(&mut self, game: &mut Game, event: &KeyEvent) {
        game.input_manager.handle_input(event);
        if let Some(last_key) = game.input_manager.get_last_key() {
            if last_key == winit::keyboard::KeyCode::Enter {
                game.change_state(GameState::Paused(Paused));
            }
        }
    }

    fn update(&mut self, game: &mut Game, renderer: &mut Renderer, dt: Duration) {
        // Handle game updates and input
        game.player.input(&game.input_manager.get_last_key(), &game.map.collisions);
        game.player.update(dt);

        // Update animations
        let mut animations_to_remove = Vec::new();
        for (i, animation) in game.animations.iter_mut().enumerate() {
            animation.time_accumulator += dt;
            if animation.time_accumulator >= animation.frame_duration {
                animation.time_accumulator -= animation.frame_duration;
                animation.current_frame += 1;
                if animation.current_frame >= animation.frames.len() as u32 {
                    if animation.looped {
                        animation.current_frame = 0;
                    } else {
                        animation.current_frame = animation.frames.len() as u32 - 1;
                        animations_to_remove.push(i); // Mark this animation for removal
                        continue;
                    }
                }
                let index = animation.frames[animation.current_frame as usize];
                animation.instance.tex_index = index;
            }
        }

        // Remove finished animations
        for &index in animations_to_remove.iter().rev() {
            game.animations.remove(index);
        }

        // Handle door collisions
        if let Some(door) = game.check_door_collision() {
            game.load_new_map(&door, renderer);
        }

        // Handle grass encounters
        if let Some(grass) = game.check_grass_collision() {
            game.trigger_grass_encounter(&grass);
        }
    }

    fn draw(&mut self, game: &mut Game, renderer: &mut Renderer) {
        let mut instances = Vec::new();
        instances.extend_from_slice(&game.map.background);
        instances.extend_from_slice(&game.map.ground);
        instances.extend_from_slice(&game.map.foreground);
        instances.extend_from_slice(&game.player.instances);
        instances.extend_from_slice(&game.animations.iter().map(|animation| animation.instance).collect::<Vec<_>>());
        instances.extend_from_slice(&game.map.aboveground);

        let _ = renderer.render(&instances);
    }
}
