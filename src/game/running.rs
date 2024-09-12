use std::time::Duration;
use crate::game::Game;
use crate::renderer::Renderer;
use winit::keyboard::KeyCode;
use tiled::Loader;
use crate::game::gamestate::GameState;
use rand::Rng;
use crate::game::pokemon::Pokemon;
use crate::game::map_loader::{Door, Map, Grass};
use super::npc::{NPC, generate_pokemon};
use crate::game::Interaction;
use cgmath::Vector3;
use crate::game::animation_player::{Animation, AnimationSheet};

impl Game {
    pub fn running(&mut self, renderer: &mut Renderer, dt: Duration) {
        renderer.update(self.player.position);

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
                                self.heal_pokemon();
                            }
                            _ => {}
                        }
                    }

                    //if player is in front of battle npc then start battle
                    let npc = self.npcs.iter().find(|npc| {
                        if let Interaction::Battle(battled, _) = npc.interaction {
                            !battled && npc.position == self.player.facing_direction + self.player.position
                        } else {
                            false
                        }
                    });

                    if let Some(npc) = npc {
                        self.queue_battle = (true, npc.id.clone());
                    }

                }

                _ => {}
            }
        }

        if self.queue_battle.0 {
            println!("Starting battle with {:?}", self.queue_battle.1);

            let npc = self.npcs.iter_mut().find(|npc| npc.id == self.queue_battle.1).unwrap();
            npc.update(self.player.target_position, dt);

            if npc.position == npc.next_point {
                let id = npc.id.clone();

                self.start_battle(id.clone(), generate_pokemon(id.0.clone(), id.1, renderer), renderer);

                self.queue_battle = (false, ("".to_string(), 0));
            }
        } else {
            self.player.input(&last_key, &mut self.input_manager, &self.map.collisions, &self.npcs);
            self.player.update(dt);
            for npc in &mut self.npcs {
                npc.update(self.player.target_position, dt);


                if let Interaction::Battle(false, ref battle_squares) = npc.interaction {

                    if battle_squares.contains(&self.player.position) {
                        self.queue_battle = (true, npc.id.clone());
                        npc.walk_to(self.player.position - npc.direction);
                    }
                }
            }
        }

        //update animations
        for animation in &mut self.ground_animations {
            animation.update(animation.position, dt);
        }

        let mut foreground_animations_to_remove = Vec::new();
        for (i, animation) in self.foreground_animations.iter_mut().enumerate() {
            if animation.update(animation.position, dt) {
                foreground_animations_to_remove.push(i);
            }
        }

        // Remove finished animations
        for &index in foreground_animations_to_remove.iter().rev() {
            self.foreground_animations.remove(index);
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
            self.load_map(&door.name, door.location, renderer);
        }

        let mut grass_detected: Option<Grass> = None;

        for grass in &self.map.grasses {
            if self.player.position.x == grass.x && self.player.position.y == grass.y && self.player.spot_arrival {
                grass_detected = Some(grass.clone());
                break;
            }
        }

        if let Some(grass) = grass_detected {
            let sheet = AnimationSheet {
                frame_width: 1,
                frame_height: 1,
                frame_order: vec![0, 1, 2],
                frame_duration: Duration::from_millis(100),
                atlas: renderer.get_atlas("landing").unwrap().clone(),
                looped: false,
            };

            let position = cgmath::Vector3::new(grass.x, grass.y, 0.0);

            let animation = Animation::new(position, &sheet, 5, 0, 3, 1);

            self.foreground_animations.push(animation);

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

    pub fn heal_pokemon(&mut self) {
        for pokemon in &mut self.player_pokemon {
            pokemon.current_hp = pokemon.stats.hp;
        }

        println!("Your Pokémon have been healed!");
    }

    pub fn load_map(&mut self, map_name: &str, door_location: u32 , renderer: &mut Renderer) {
        let mut loader = Loader::new();
        let map_path = format!("/home/chris/games/SirSquare/assets/{}.tmx", map_name);
        let map_loader = loader.load_tmx_map(map_path).unwrap();

        self.map = Map::new(&map_loader, 0, map_name.to_string());

        let _ = renderer.update_texture(0, map_name, 16, 16);

        let mut ground_animations = Vec::new();

        for animated in &self.map.animated {

            let sheet = AnimationSheet {
                frame_width: 1,
                frame_height: 1,
                frame_order: vec![0, 1, 2, 3],
                frame_duration: Duration::from_millis(250),
                atlas: renderer.get_atlas("landing").unwrap().clone(),
                looped: true,
            };

            let position = Vector3::new(animated.x, animated.y, 0.0);
            let animation = Animation::new(position, &sheet, 0, 9, 4, 1);

            ground_animations.push(animation);
        }

        self.ground_animations = ground_animations;

        //search map for player spawn that matches door.location
        //the spawn name must be player also
        let player_spawn = self.map.spawns.iter().find(|spawn| spawn.name == "player" && spawn.location == door_location);

        self.player.orient(player_spawn.unwrap().direction);
        self.player.position = cgmath::Vector3::new(player_spawn.unwrap().x, player_spawn.unwrap().y, 0.0);
        self.player.target_position = self.player.position;

        self.npcs = Vec::new();

        for npc in &self.map.npcs {
            let position = cgmath::Vector3::new(npc.x, npc.y, 0.0);
            let path = npc.path_id
                .and_then(|path_id| self.map.paths.iter().find(|path| path.id == path_id).map(|p| p.points.clone()));

            let interaction = match npc.interaction.as_str() {
                "Heal" => Interaction::Heal,
                "Battle" => {
                    let battled = self.finished_battles.contains(&(map_name.to_string(), npc.id));
                    println!("npc id: {:?} battled: {:?}", npc.id, battled);
                    println!("finished_battles: {:?}", self.finished_battles);
                    let mut battle_squares = Vec::new();
                    for i in 0..npc.los {
                        let position = position + (npc.direction * (i+1) as f32);
                        battle_squares.push(position);
                    }
                    Interaction::Battle(battled, battle_squares)
                },
                "Talk" => Interaction::Talk,
                _ => Interaction::None,
            };

            let new_npc = NPC::new((map_name.to_string(), npc.id), position, npc.direction, &npc.name, interaction, npc.los, path, renderer);

            self.npcs.push(new_npc);
        }

        match map_name {
            "pokecenter" => self.audio_player.play("/home/chris/games/SirSquare/assets/Pokemon Center.mp3"),
            "gym" => self.audio_player.play("/home/chris/games/SirSquare/assets/Pokemon Gym.mp3"),
            _ => self.audio_player.play("/home/chris/games/SirSquare/assets/Pallet Town.mp3"),
        }

    }

}

