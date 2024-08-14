use std::time::Duration;
use crate::game::{Game, Animation};
use crate::renderer::Renderer;
use winit::keyboard::KeyCode;
use tiled::Loader;
use crate::game::gamestate::GameState;
use rand::Rng;
use crate::game::pokemon::Pokemon;
use crate::game::map_loader::{Door, Map, Grass};
use crate::renderer::instance::Instance;

impl Game {
    pub fn running(&mut self, renderer: &mut Renderer, dt: Duration) {
        renderer.update(self.player.position);
        // Handle game updates and input

        let last_key = self.input_manager.get_last_key();
        let release_key = self.input_manager.get_release_key();

        if let Some(release_key) = release_key {
            if release_key == KeyCode::Enter {
                self.state = GameState::Paused;
                return;
            }
        }

        self.player.input(&last_key, &self.map.collisions);
        self.player.update(dt);

        let mut animations_to_remove = Vec::new();
        for (i, animation) in self.animations.iter_mut().enumerate() {
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
            self.animations.remove(index);
        }

        //for each door check if player collides with door
        //if so load new map and texture
        let mut door_detected: Option<Door> = None;
        for door in &self.map.doors {
            let rect = &door.rectangle;
            let player_left = self.player.position.x - 0.5;
            let player_right = self.player.position.x + 0.5;
            let player_top = self.player.position.y + 0.5;
            let player_bottom = self.player.position.y - 0.5;

            let rect_left = rect.x - rect.width / 4.0;
            let rect_right = rect.x + rect.width / 4.0;
            let rect_top = rect.y + rect.height / 4.0;
            let rect_bottom = rect.y - rect.height / 4.0;

            if player_left < rect_right && player_right > rect_left &&
               player_top > rect_bottom && player_bottom < rect_top {
                door_detected = Some(door.clone());
                break;
            }
        }

        if let Some(door) = door_detected {
            let mut loader = Loader::new();
            let map_path = format!("/home/chris/games/SirSquare/assets/{}.tmx", door.name);
            let map_loader = loader.load_tmx_map(map_path).unwrap();

            self.map = Map::new(&map_loader, 0);

            let image = self.images.get(&door.name);
            if let Some(image) = image {
                let _ = renderer.update_texture(&image, 0);
            }

            //search map for player spawn that matches door.location
            //the spawn name must be player also
            let player_spawn = self.map.spawns.iter().find(|spawn| spawn.name == "player" && spawn.location == door.location);

            self.player.position = cgmath::Vector3::new(player_spawn.unwrap().x, player_spawn.unwrap().y, 0.0);
            self.player.target_position = self.player.position;
        }

        //check if player collides with grass
        let mut grass_detected: Option<Grass> = None;

        for grass in &self.map.grasses {
            if self.player.position.x == grass.x && self.player.position.y == grass.y && self.player.spot_arrival {
                grass_detected = Some(grass.clone());
                break;
            }
        }

        if let Some(grass) = grass_detected {
            let animation = Animation {
                frames: vec![5, 6, 7],
                frame_duration: Duration::from_millis(100),
                looped: false,
                current_frame: 0,
                instance: Instance {
                    model: cgmath::Matrix4::from_translation(cgmath::Vector3::new(grass.x, grass.y, 0.0)).into(),
                    tex_index: 5,
                    atlas_index: 0,
                },
                time_accumulator: Duration::from_millis(0),
            };

            self.animations.push(animation);

            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0..8);

            if random_number == 0 {
                println!("Wild Pokemon appeared!");
                let random_pokemon = rng.gen_range(0..3);

                let pokemon = match random_pokemon {
                    0 => Pokemon::new("Bulbasaur".to_string(), 5, renderer),
                    1 => Pokemon::new("Charmander".to_string(), 5, renderer),
                    2 => Pokemon::new("Squirtle".to_string(), 5, renderer),
                    _ => Pokemon::new("MissingNo".to_string(), 5, renderer),
                };


                self.start_encounter(pokemon, renderer);
            }
        }
    }
}

