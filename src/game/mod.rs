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
use crate::game::encounter::Encounter;
use pokemon::Pokemon;
use party::Party;

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
    images: HashMap<String, image_loader::ImageData>,
    animations: Vec<Animation>,
    state: GameState,
    time_of_last_update: Instant,
    pub menu: Menu,
    encounter: Option<Encounter>,
    party: Option<Party>,
    player_pokemon: Vec<Pokemon>,
}

impl Game {
    pub async fn new() -> Self {
        let paths = vec![
            "landing",
            "pokecenter",
            "pokemart",
            "menu",
            "house_1",
            "battle",
        ];
        let images = image_loader::load_images(&paths, "/home/chris/games/SirSquare/assets").unwrap();

        let mut loader = Loader::new();
        let map_loader = loader.load_tmx_map("/home/chris/games/SirSquare/assets/landing.tmx").unwrap();
        let map = map_loader::Map::new(&map_loader, 0);

        let menu = Menu::new(&mut loader, cgmath::Vector3::new(0.0, 0.0, 0.0));

        let mut player_pokemon = Vec::new();

        let bulbasaur = Pokemon::new("Bulbasaur".to_string(), 5);
        let charmander = Pokemon::new("Charmander".to_string(), 6);
        let squirtle = Pokemon::new("Squirtle".to_string(), 7);

        player_pokemon.push(bulbasaur);
        player_pokemon.push(charmander);
        player_pokemon.push(squirtle);

        Self {
            input_manager: InputManager::new(),
            player: Player::new(),
            map,
            images,
            animations: Vec::new(),
            state: GameState::Running,
            time_of_last_update: Instant::now(),
            menu,
            encounter: None,
            party: None,
            player_pokemon,
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
        //    let pokemon = Pokemon::new("Charizard".to_string(), 5);
        //    self.start_encounter(self.player.pokemon.clone(), pokemon, renderer);
        //}

        match self.state {
            GameState::Running => self.running(renderer, dt),
            GameState::Paused => self.paused(renderer),
            GameState::Encounter => {
                if let Some(encounter) = &mut self.encounter {
                    if encounter.update(&mut self.input_manager, dt, renderer) {
                        self.state = GameState::Running;
                        self.encounter = None;
                    }
                }
            },
            GameState::Party => {
                if let Some(party) = &mut self.party {
                    if party.update(&mut self.input_manager, dt, renderer) {
                        self.state = GameState::Paused;
                    }
                }
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
                    encounter.draw(renderer);
                }
            },
            GameState::Party => {
                if let Some(party) = &self.party {
                    party.draw(renderer);
                }
            },
        }
    }

    pub fn start_encounter(&mut self, pokemon: Pokemon, renderer: &mut Renderer) {
        self.encounter = Some(Encounter::new(&mut self.player_pokemon, pokemon, renderer));
        self.state = GameState::Encounter;
    }

    pub fn enter_party(&mut self, renderer: &mut Renderer) {
        self.party = Some(Party::new(&mut self.player_pokemon, false, renderer));
        self.state = GameState::Party;
    }
}
