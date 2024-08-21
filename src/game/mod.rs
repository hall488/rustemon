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
use crate::game::battle::Battle;
use pokemon::Pokemon;
use party::Party;
use crate::renderer::sprite::Sprite;
use cgmath::{Matrix4, vec4, Vector4};
use npc::NPC;

pub struct Animation {
    pub frames: Vec<u32>,
    pub frame_duration: Duration,
    pub current_frame: u32,
    pub instance: Instance,
    pub time_accumulator: Duration,
    pub looped: bool,
}

pub struct Game {
    input_manager: InputManager,
    player: Player,
    map: map_loader::Map,
    animations: Vec<Animation>,
    state: GameState,
    time_of_last_update: Instant,
    pub menu: Menu,
    encounter: Option<Battle>,
    party: Option<Party>,
    player_pokemon: Vec<Pokemon>,
    debug_background: Sprite,
    npcs: Vec<NPC>,
}

impl Game {
    pub async fn new(renderer: &mut Renderer) -> Self {

        let mut loader = Loader::new();
        let map_loader = loader.load_tmx_map("/home/chris/games/SirSquare/assets/landing.tmx").unwrap();
        let map = map_loader::Map::new(&map_loader, 0);

        let menu = Menu::new(&mut loader, cgmath::Vector3::new(0.0, 0.0, 0.0));

        let mut player_pokemon = Vec::new();

        let bulbasaur = Pokemon::new("Bulbasaur".to_string(), 5, renderer);
        let charmander = Pokemon::new("Charmander".to_string(), 6, renderer);
        let squirtle = Pokemon::new("Squirtle".to_string(), 7, renderer);

        player_pokemon.push(bulbasaur);
        player_pokemon.push(charmander);
        player_pokemon.push(squirtle);

        let debug_background = renderer.create_sprite(0.5 + 6.0, 6.0, 0, 0, 15, 10, "debug", 1.0/240.0*15.0, 1.0/160.0*10.0).expect("");

        Self {
            input_manager: InputManager::new(),
            player: Player::new(),
            map,
            animations: Vec::new(),
            state: GameState::Running,
            time_of_last_update: Instant::now(),
            menu,
            encounter: None,
            party: None,
            player_pokemon,
            debug_background,
            npcs: Vec::new(),
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
                    if encounter.update(&mut self.player_pokemon, &mut self.input_manager, dt, renderer) {
                        self.state = GameState::Running;
                        self.encounter = None;
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
                instances.extend_from_slice(&self.map.foreground);
                instances.extend_from_slice(&self.player.instances);
                for npc in &self.npcs {
                    instances.extend_from_slice(&npc.instances);
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
        self.encounter = Some(Battle::new(&mut self.player_pokemon, enemy_pokemon, renderer));
        self.state = GameState::Encounter;
    }


    pub fn enter_party(&mut self, renderer: &mut Renderer) {
        self.party = Some(Party::new(&mut self.player_pokemon, false, false, renderer));
        self.state = GameState::Party;
    }
}
