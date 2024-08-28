use std::time::Duration;
use crate::game::{Game, GAnimation};
use crate::renderer::Renderer;
use winit::keyboard::KeyCode;
use tiled::Loader;
use crate::game::gamestate::GameState;
use rand::Rng;
use crate::game::pokemon::Pokemon;
use crate::game::map_loader::{Door, Map, Grass};
use crate::renderer::instance::Instance;
use super::npc::NPC;

impl Game {
    pub fn running(&mut self, renderer: &mut Renderer, dt: Duration) {
        renderer.update(self.player.position);
        // Handle game updates and input

        let last_key = self.input_manager.get_last_key();
        let single_press_key = self.input_manager.get_key_on_press();

        if let Some(key) = single_press_key {
            match key {
                KeyCode::Enter => {
                    self.state = GameState::Paused;
                    return;
                }
                KeyCode::KeyZ => {
                    //check if player is if front of interaction
                    let mut interaction_detected: Option<&str> = None;

                    for interaction in &self.map.interactions {
                        //check if player is facing interaction
                        if self.player.facing_direction + self.player.position == cgmath::Vector3::new(interaction.x, interaction.y, 0.0) {
                            interaction_detected = Some(&interaction.name);
                            break;
                        }

                    }

                    if let Some(interaction) = interaction_detected {
                        match interaction {
                            "Heal" => {
                                for pokemon in &mut self.player_pokemon {
                                    pokemon.current_hp = pokemon.stats.hp;
                                }
                                println!("Your Pokémon have been healed!");
                            }
                            _ => {}
                        }
                    }
                }

                _ => {}
            }
        }


        self.player.input(&last_key, &mut self.input_manager, &self.map.collisions, &self.npcs);
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

        let mut ground_animations_to_remove = Vec::new();
        for (i, animation) in self.ground_animations.iter_mut().enumerate() {
            animation.time_accumulator += dt;
            if animation.time_accumulator >= animation.frame_duration {
                animation.time_accumulator -= animation.frame_duration;
                animation.current_frame += 1;
                if animation.current_frame >= animation.frames.len() as u32 {
                    if animation.looped {
                        animation.current_frame = 0;
                    } else {
                        animation.current_frame = animation.frames.len() as u32 - 1;
                        ground_animations_to_remove.push(i); // Mark this animation for removal
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

            let _ = renderer.update_texture(0, &door.name, 16, 16);

            let mut ground_animations = Vec::new();

            for animated in &self.map.animated {
                ground_animations.push(GAnimation {
                    frames: animated.frames.clone(),
                    frame_duration: Duration::from_millis(250),
                    current_frame: 0,
                    instance: Instance {
                        model: cgmath::Matrix4::from_translation(cgmath::Vector3::new(animated.x, animated.y, 0.0)).into(),
                        tex_index: animated.frames[0] as u32,
                        atlas_index: 0,
                    },
                    time_accumulator: Duration::from_millis(0),
                    looped: true,
                });
            }

            self.ground_animations = ground_animations;

            //search map for player spawn that matches door.location
            //the spawn name must be player also
            let player_spawn = self.map.spawns.iter().find(|spawn| spawn.name == "player" && spawn.location == door.location);

            self.player.position = cgmath::Vector3::new(player_spawn.unwrap().x, player_spawn.unwrap().y, 0.0);
            self.player.target_position = self.player.position;

            //load npcs
            self.npcs = Vec::new();

            for npc in &self.map.npcs {
                //search self.map.paths for path id
                let path = self.map.paths.iter().find(|path| path.id == npc.path_id).unwrap().points.clone();
                let _npc = NPC::new(cgmath::Vector3::new(npc.x, npc.y, 0.0), npc.direction, &npc.name, &npc.interaction, npc.los, path, renderer);
                self.npcs.push(_npc);
            }
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
            let animation = GAnimation {
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

