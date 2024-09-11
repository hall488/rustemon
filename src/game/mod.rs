mod player;
mod input_manager;
mod map_loader;
mod menu;
mod pokemon;
mod gamestate;
mod encounter;
mod running;
mod paused;
mod stationary_loader;
mod party;
mod font;
mod moves;
mod battle;
mod npc;
mod entity;
mod animation_player;

use std::time::{Instant, Duration};
use player::Player;
use input_manager::InputManager;
use crate::renderer::Renderer;
use tiled::Loader;
use std::collections::HashMap;
use crate::renderer::image_loader;
use crate::renderer::instance::Instance;
use crate::game::gamestate::GameState;
use crate::game::menu::Menu;
//use crate::game::encounter::Encounter;
use crate::game::battle::{Battle, BattleType};
use pokemon::Pokemon;
use party::Party;
use crate::renderer::sprite::Sprite;
use cgmath::{Matrix4, vec4, Vector4, Vector3};
use npc::NPC;
use entity::Entity;


pub struct GAnimation {
    pub frames: Vec<u32>,
    pub frame_duration: Duration,
    pub current_frame: u32,
    pub instance: Instance,
    pub time_accumulator: Duration,
    pub looped: bool,
}

pub enum Interaction {
    Heal,
    Battle(bool, Vec<Vector3<f32>>),
    Talk,
    None,
}

pub struct Game {
    input_manager: InputManager,
    player: Player,
    map: map_loader::Map,
    animations: Vec<GAnimation>,
    state: GameState,
    time_of_last_update: Instant,
    pub menu: Menu,
    encounter: Option<Battle>,
    party: Option<Party>,
    player_pokemon: Vec<Pokemon>,
    debug_background: Sprite,
    npcs: Vec<NPC>,
    ground_animations: Vec<GAnimation>,
    queue_battle: (bool, (String, u32)),
    finished_battles: Vec<(String, u32)>, // (map_name, npc_id)
    trainer: Option<(String, u32)>,
}

impl Game {
    pub async fn new(renderer: &mut Renderer) -> Self {

        let mut loader = Loader::new();
        let map_loader = loader.load_tmx_map("/home/chris/games/SirSquare/assets/landing.tmx").unwrap();
        let map = map_loader::Map::new(&map_loader, 0);

        let mut ground_animations = Vec::new();

        for animated in &map.animated {
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

            //push 4 frames starting from object.id
        let menu = Menu::new(&mut loader, cgmath::Vector3::new(0.0, 0.0, 0.0));

        let mut player_pokemon = Vec::new();

        let bulbasaur = Pokemon::new("Bulbasaur".to_string(), 100, renderer);
        let charmander = Pokemon::new("Charmander".to_string(), 1, renderer);

        player_pokemon.push(bulbasaur);
        player_pokemon.push(charmander);

        let debug_background = renderer.create_sprite(0.5 + 6.0, 6.0, 0, 0, 15, 10, "debug", 1.0/240.0*15.0, 1.0/160.0*10.0).expect("");

        let mut npcs = Vec::new();

        for npc in &map.npcs {
            let position = cgmath::Vector3::new(npc.x, npc.y, 0.0);
            let path = npc.path_id
                .and_then(|path_id| map.paths.iter().find(|path| path.id == path_id).map(|p| p.points.clone()));

            let interaction = match npc.interaction.as_str() {
                "Heal" => Interaction::Heal,
                "Battle" => {
                    let mut battle_squares = Vec::new();
                    for i in 0..npc.los {
                        let position = position + (npc.direction * i as f32);
                        battle_squares.push(position);
                    }
                    Interaction::Battle(false, battle_squares)
                },
                "Talk" => Interaction::Talk,
                _ => Interaction::None,
            };

            let new_npc = NPC::new(("landing".to_string(), npc.id), position, npc.direction, &npc.name, interaction, npc.los, path, renderer);

            npcs.push(new_npc);
        }

        Self {
            input_manager: InputManager::new(),
            player: Player::new(renderer),
            map,
            animations: Vec::new(),
            state: GameState::Running,
            time_of_last_update: Instant::now(),
            menu,
            encounter: None,
            party: None,
            player_pokemon,
            debug_background,
            npcs,
            ground_animations,
            queue_battle: (false, ("".to_string(), 0)),
            finished_battles: Vec::new(),
            trainer: None,
        }
    }

    pub fn input(&mut self, event: &winit::event::KeyEvent) {
        // Handle input
        self.input_manager.handle_input(&event);
    }

    pub fn update(&mut self, renderer: &mut Renderer) {
        let now = Instant::now();
        let dt = now.duration_since(self.time_of_last_update);
        self.time_of_last_update = now;

        //if self.encounter.is_none() {
        //    let pokemon = Pokemon::new("Charizard".to_string(), 5, renderer);
        //    self.start_encounter(pokemon, renderer);
        //}

        match self.state {
            GameState::Running => self.running(renderer, dt),
            GameState::Paused => self.paused(renderer),
            GameState::Encounter => {
                if let Some(encounter) = &mut self.encounter {

                    if let Some(player_won) = encounter.update(&mut self.player_pokemon, &mut self.input_manager, dt, renderer) {
                        if player_won {
                            if encounter.battle_type == BattleType::Trainer {
                                self.npc_defeated();
                            }

                        } else {
                            //send to pokemon center
                            self.load_map("pokecenter", 1, renderer);
                            self.heal_pokemon();
                        }

                        self.encounter = None;
                        self.state = GameState::Running;
                    }
                }
            },
            GameState::Party => {
                if let Some(party) = &mut self.party {
                    //6 is just the value returned when cancel is selected
                    if party.update(&mut self.input_manager, dt, renderer) == 6 {
                        self.state = GameState::Paused;
                    }
                }
            },
            GameState::Debug => {
                // Add debug state

            },
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        match self.state {
            GameState::Running | GameState::Paused => {
                // Add instances for running and paused states
                let mut instances = Vec::new();
                instances.extend_from_slice(&self.map.background);
                instances.extend_from_slice(&self.map.ground);
                instances.extend_from_slice(&self.ground_animations.iter().map(|animation| animation.instance).collect::<Vec<Instance>>());
                instances.extend_from_slice(&self.map.foreground);

                //push animation player from player
                //instances.extend_from_slice(&self.player.get_instances());

                let mut entities: Vec<&dyn Entity> = Vec::new();
                entities.push(&self.player);
                entities.extend(self.npcs.iter().map(|npc| npc as &dyn Entity));

                entities.sort_by(|a, b| b.position().y.partial_cmp(&a.position().y).unwrap_or(std::cmp::Ordering::Equal));

                for entity in entities {
                    instances.extend_from_slice(entity.instances());
                }

                instances.extend_from_slice(&self.animations.iter().map(|animation| animation.instance).collect::<Vec<Instance>>());
                instances.extend_from_slice(&self.map.aboveground);

                if self.state == GameState::Paused {
                    instances.extend_from_slice(&self.menu.instances);
                    instances.push(self.menu.pointer);
                }

                let _ = renderer.render(&instances, true);
            },
            GameState::Encounter => {
                if let Some(encounter) = &self.encounter {
                    encounter.draw(&mut self.player_pokemon, renderer);
                }
            },
            GameState::Party => {
                if let Some(party) = &self.party {
                    party.draw(renderer);
                }
            },
            GameState::Debug => {
                // Add debug state
                let mut instances = Vec::new();

                instances.extend_from_slice(&self.debug_background.texture);

                let _ = renderer.render(&instances, false);
            },
        }
    }

    pub fn start_encounter(&mut self, pokemon: Pokemon, renderer: &mut Renderer) {
        let enemy_pokemon = vec![pokemon.clone()];  // Clone the enemy Pokémon

        // Pass the player's Pokémon by reference and the cloned enemy Pokémon by reference
        self.encounter = Some(Battle::new(BattleType::Wild, &mut self.player_pokemon, enemy_pokemon, renderer));
        self.state = GameState::Encounter;
    }

    pub fn start_battle(&mut self, npc_id: (String, u32), pokemon: Vec<Pokemon>, renderer: &mut Renderer) {
        self.encounter = Some(Battle::new(BattleType::Trainer, &mut self.player_pokemon, pokemon, renderer));
        self.state = GameState::Encounter;
        self.trainer = Some(npc_id);
    }

    pub fn npc_defeated(&mut self) {
        println!("NPC defeated");
        let npc_id = self.trainer.clone().unwrap();
        println!("{:?}", npc_id);

        // Make NPC defeated
        if let Some(npc) = self.npcs.iter_mut().find(|npc| npc.id == npc_id) {
            if let Interaction::Battle(ref mut battled, _) = npc.interaction {
                *battled = true; // Mutate the boolean to mark battle as true
            }
        }
        self.finished_battles.push(npc_id);
        self.trainer = None;
    }

    pub fn enter_party(&mut self, renderer: &mut Renderer) {
        self.party = Some(Party::new(&mut self.player_pokemon, false, false, renderer));
        self.state = GameState::Party;
    }
}
