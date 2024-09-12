mod player;
mod input_manager;
mod map_loader;
mod menu;
mod pokemon;
mod gamestate;
mod running;
mod paused;
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
use crate::game::gamestate::GameState;
use crate::game::menu::Menu;
use crate::game::battle::{Battle, BattleType};
use pokemon::Pokemon;
use party::Party;
use crate::renderer::sprite::Sprite;
use cgmath:: Vector3;
use npc::NPC;
use entity::Entity;
use animation_player::{Animation, AnimationSheet};
use crate::audio::AudioPlayer;
use rodio::OutputStream;

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
    foreground_animations: Vec<Animation>,
    state: GameState,
    time_of_last_update: Instant,
    pub menu: Menu,
    encounter: Option<Battle>,
    party: Option<Party>,
    player_pokemon: Vec<Pokemon>,
    debug_background: Sprite,
    npcs: Vec<NPC>,
    ground_animations: Vec<Animation>,
    queue_battle: (bool, (String, u32)),
    finished_battles: Vec<(String, u32)>, // (map_name, npc_id)
    trainer: Option<(String, u32)>,
    audio_player: AudioPlayer,
    //required to keep audio player alive
    #[allow(dead_code)]
    stream: OutputStream,
}

impl Game {
    pub async fn new(renderer: &mut Renderer) -> Self {

        let mut loader = Loader::new();
        let map_loader = loader.load_tmx_map("/home/chris/games/SirSquare/assets/landing.tmx").unwrap();
        let map = map_loader::Map::new(&map_loader, 0, "landing".to_string());

        let mut ground_animations = Vec::new();

        for animated in &map.animated {

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

        let menu = Menu::new(&mut loader, cgmath::Vector3::new(0.0, 0.0, 0.0));

        let mut player_pokemon = Vec::new();

        let pikachu = Pokemon::new("Pikachu".to_string(), 5, renderer);

        player_pokemon.push(pikachu);

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

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut audio_player = AudioPlayer::new(stream_handle);
        audio_player.play("/home/chris/games/SirSquare/assets/Pallet Town.mp3");

        Self {
            input_manager: InputManager::new(),
            player: Player::new(renderer),
            map,
            foreground_animations: Vec::new(),
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
            audio_player,
            stream: _stream,
        }
    }

    pub fn input(&mut self, event: &winit::event::KeyEvent) {
        self.input_manager.handle_input(&event);
    }

    pub fn update(&mut self, renderer: &mut Renderer) {
        let now = Instant::now();
        let dt = now.duration_since(self.time_of_last_update);
        self.time_of_last_update = now;

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
                            self.load_map("pokecenter", 1, renderer);
                            self.heal_pokemon();
                        }

                        self.encounter = None;
                        self.state = GameState::Running;

                        match self.map.name.as_str() {
                            "pokecenter" => self.audio_player.play("/home/chris/games/SirSquare/assets/Pokemon Center.mp3"),
                            "gym" => self.audio_player.play("/home/chris/games/SirSquare/assets/Pokemon Gym.mp3"),
                            _ => self.audio_player.play("/home/chris/games/SirSquare/assets/Pallet Town.mp3"),
                        }
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

            },
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        match self.state {
            GameState::Running | GameState::Paused => {
                let mut instances = Vec::new();
                instances.extend_from_slice(&self.map.background);
                instances.extend_from_slice(&self.map.ground);

                instances.extend(self.ground_animations.iter().flat_map(|animation| &animation.instances));

                instances.extend_from_slice(&self.map.foreground);

                let mut entities: Vec<&dyn Entity> = Vec::new();
                entities.push(&self.player);
                entities.extend(self.npcs.iter().map(|npc| npc as &dyn Entity));
                entities.extend(self.foreground_animations.iter().map(|animation| animation as &dyn Entity));

                entities.sort_by(|a, b| b.position().y.partial_cmp(&a.position().y).unwrap_or(std::cmp::Ordering::Equal));

                for entity in entities {
                    instances.extend_from_slice(entity.instances());
                }

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
                let mut instances = Vec::new();

                instances.extend_from_slice(&self.debug_background.texture);

                let _ = renderer.render(&instances, false);
            },
        }
    }

    pub fn start_encounter(&mut self, pokemon: Pokemon, renderer: &mut Renderer) {
        let enemy_pokemon = vec![pokemon.clone()];

        self.encounter = Some(Battle::new(BattleType::Wild, &mut self.player_pokemon, enemy_pokemon, renderer));
        self.state = GameState::Encounter;
        self.audio_player.play("/home/chris/games/SirSquare/assets/Wild Battle.mp3");
    }

    pub fn start_battle(&mut self, npc_id: (String, u32), pokemon: Vec<Pokemon>, renderer: &mut Renderer) {
        self.encounter = Some(Battle::new(BattleType::Trainer, &mut self.player_pokemon, pokemon, renderer));
        self.state = GameState::Encounter;

        if npc_id == ("gym".to_string(), 5) {
            self.audio_player.play("/home/chris/games/SirSquare/assets/Gym Battle.mp3");
        } else {
            self.audio_player.play("/home/chris/games/SirSquare/assets/Trainer Battle.mp3");
        }

        self.trainer = Some(npc_id);
    }

    pub fn npc_defeated(&mut self) {
        println!("NPC defeated");
        let npc_id = self.trainer.clone().unwrap();
        println!("{:?}", npc_id);

        if let Some(npc) = self.npcs.iter_mut().find(|npc| npc.id == npc_id) {
            if let Interaction::Battle(ref mut battled, _) = npc.interaction {
                *battled = true;
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
