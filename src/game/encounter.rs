use crate::game::input_manager::InputManager;
use std::time::Duration;
use crate::renderer::Renderer;
use crate::game::pokemon::Pokemon;
use crate::renderer::image_loader::ImageData;
use crate::renderer::instance::Instance;
use crate::renderer::sprite::Sprite;
use crate::game::stationary_loader::Stationary;
use tiled::Loader;
use crate::game::font::Font;
use winit::keyboard::KeyCode;
use std::collections::HashMap;
use crate::game::gamestate::GameState;
use super::party::Party;

//order of turns
//player chooses fight, bag, pokemon, or run
//players decision is locked in
//enemy chooses move
//based on both inputs, the moves are executed
//output text for each move is made
//repeat until one pokemon faints

pub enum BattleState {
    PlayerTurn,
    EnemyTurn,
    PlayerMove,
    EnemyMove,
    MoveText,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MenuState {
    Main,
    Fight,
    Bag,
    Pokemon,
    Run,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FightState {
    Move1,
    Move2,
    Move3,
    Move4,
}

pub struct Encounter {
    pokemon_back: Pokemon,
    pokemon_front: Pokemon,
    scene: Stationary,
    front_bg: Sprite,
    back_bg: Sprite,
    front_name: Font,
    front_level: Font,
    back_name: Font,
    back_level: Font,
    back_hp_current: Font,
    back_hp_max: Font,
    battle_state: BattleState,
    menu_state: Option<MenuState>,
    fight_selection: Option<FightState>,
    menu_selection: Option<MenuState>,
    selector: Sprite,
    main_menu: Sprite,
    fight_menu: Sprite,
    move_fonts: Vec<Font>,
    valid_moves: [bool; 4],
    party: Option<Party>,
}

impl Encounter {
    pub fn new(player_pokemon: &Vec<Pokemon>, pokemon_front: Pokemon, renderer: &mut Renderer) -> Self {

        println!("You encountered a level {} {}", pokemon_front.level, pokemon_front.name);

        //scale model matrix
        let mut loader = Loader::new();
        let map_loader = loader.load_tmx_map("/home/chris/games/SirSquare/assets/battle.tmx").unwrap();
        let scene = Stationary::new(&map_loader, 3);

        let pokemon_back = player_pokemon[0].clone();

        let front_bg = create_front_bg();
        let back_bg = create_back_bg();
        let front_name = Font::new(42.0/480.0*2.0, 38.0/320.0*2.0, &pokemon_front.name.to_uppercase(), true);
        //let level = "Lv".to_string() + &pokemon_front.level.to_string();
        let level = "Lv".to_string() + &pokemon_front.level.to_string();
        let front_level = Font::new(188.0/480.0*2.0, 38.0/320.0*2.0, &level, false);

        let back_name = Font::new(282.0/480.0*2.0, 154.0/320.0*2.0, &pokemon_back.name.to_uppercase(), true);
        let level = "Lv".to_string() + &pokemon_back.level.to_string();
        let back_level = Font::new(430.0/480.0*2.0, 154.0/320.0*2.0, &level, false);

        let hp_current = pokemon_back.current_hp.to_string() + "/";
        let back_hp_current = Font::new(400.0/480.0*2.0, 190.0/320.0*2.0, &hp_current, false);
        let back_hp_max = Font::new(430.0/480.0*2.0, 190.0/320.0*2.0, &pokemon_back.stats.hp.to_string(), false);

        let battle_state = BattleState::PlayerTurn;
        let menu_state = Some(MenuState::Main);
        let menu_selection = Some(MenuState::Fight);
        let fight_selection = Some(FightState::Move1);

        let selector = Sprite::new(
            7.6875 * 2.0 / 15.0, 7.5 * 2.0 / 10.0, 13, 10, 1, 1,
            3, 15, 10, 2.0, 2.0,
        );

        let main_menu = Sprite::new(
            7.0 * 2.0 / 15.0, 7.0 * 2.0 / 10.0, 7, 18, 8, 3,
            3, 15, 10, 2.0, 2.0,
        );

        let fight_menu = Sprite::new(
            0.0 * 2.0 / 15.0, 7.0 * 2.0 / 10.0, 0, 15, 15, 3,
            3, 15, 10, 2.0, 2.0,
        );

        let mut move_fonts = Vec::new();

        // Function to get the position based on the index
        fn get_position(index: usize) -> (f32, f32) {
            match index {
                0 => (31.0 / 480.0 * 2.0, 248.0 / 320.0 * 2.0),
                1 => (175.0 / 480.0 * 2.0, 248.0 / 320.0 * 2.0),
                2 => (31.0 / 480.0 * 2.0, 280.0 / 320.0 * 2.0),
                3 => (175.0 / 480.0 * 2.0, 280.0 / 320.0 * 2.0),
                _ => (0.0, 0.0), // This should not occur since we iterate only over 0 to 3
            }
        }

        let mut valid_moves = [false; 4]; // Initialize all moves as invalid
        // Iterate over a fixed range of 0 to 3 to ensure exactly four move fonts are created
        for i in 0..4 {
            let (x, y) = get_position(i);
            let move_name = if let Some(a_move) = pokemon_back.moves.get(i) {
                valid_moves[i] = true;
                a_move.name.to_uppercase()
            } else {
                "-".to_string()
            };
            let move_text = Font::new(x, y, &move_name, true);
            move_fonts.push(move_text);
        }

        let party = None;

        Self {
            // Initialize fields
            pokemon_back,
            pokemon_front,
            scene,
            front_bg,
            back_bg,
            front_name,
            front_level,
            back_name,
            back_level,
            back_hp_current,
            back_hp_max,
            battle_state,
            menu_state,
            fight_selection,
            menu_selection,
            selector,
            main_menu,
            fight_menu,
            move_fonts,
            valid_moves,
            party,
        }
    }

    pub fn update(&mut self, pokemon: &mut Vec<Pokemon>, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> bool {
        // Handle encounter updates and input
        renderer.camera_controller.update_camera(&mut renderer.camera, cgmath::Vector3::new(0.0, 0.0, 0.0));

        match self.battle_state {
            BattleState::PlayerTurn => {
                //check party is some
                if let Some(party) = &mut self.party {
                    if party.update(input_manager, dt, renderer) {
                        println!("Party update returned true");
                        self.party = None;
                        self.menu_state = Some(MenuState::Main);
                    }
                    return false;
                }

                if self.handle_player_turn(pokemon, input_manager, renderer) {
                    return true;
                }
            },
            BattleState::PlayerMove => {
                // Handle PlayerMove state
            },
            BattleState::EnemyTurn => {
                // Handle EnemyTurn state
            },
            BattleState::EnemyMove => {
                // Handle EnemyMove state
            },
            BattleState::MoveText => {
                // Handle MoveText state
            },
        }

        return false;
    }

    fn handle_player_turn(&mut self, pokemon: &mut Vec<Pokemon>, input_manager: &mut InputManager, renderer: &mut Renderer) -> bool {
        match self.menu_state {
            Some(MenuState::Main) => {
                if let Some(key) = input_manager.get_key_on_press() {
                    let transitions: HashMap<(MenuState, KeyCode), MenuState> = [
                        ((MenuState::Pokemon, KeyCode::KeyW), MenuState::Fight),
                        ((MenuState::Run, KeyCode::KeyW), MenuState::Bag),
                        ((MenuState::Fight, KeyCode::KeyS), MenuState::Pokemon),
                        ((MenuState::Bag, KeyCode::KeyS), MenuState::Run),
                        ((MenuState::Bag, KeyCode::KeyA), MenuState::Fight),
                        ((MenuState::Run, KeyCode::KeyA), MenuState::Pokemon),
                        ((MenuState::Fight, KeyCode::KeyD), MenuState::Bag),
                        ((MenuState::Pokemon, KeyCode::KeyD), MenuState::Run),
                    ].iter().cloned().collect();

                    if let Some(current_selection) = self.menu_selection {

                        if let Some(&new_selection) = transitions.get(&(current_selection, key)) {
                            println!("{:?}", new_selection);
                            self.menu_selection = Some(new_selection);
                            let (x_pos, y_pos) = match new_selection {
                                MenuState::Fight => (7.6875, 7.5),
                                MenuState::Bag => (11.1875, 7.5),
                                MenuState::Pokemon => (7.6875, 8.5),
                                MenuState::Run => (11.1875, 8.5),
                                _ => (0.0, 0.0),
                            };

                            self.selector.update_position(x_pos * 2.0 / 15.0, y_pos * 2.0 / 10.0);
                        }
                    }
                    if key == KeyCode::KeyZ {
                        self.menu_state = self.menu_selection;
                        if self.menu_state == Some(MenuState::Fight) {
                            self.selector.update_position(0.25 * 2.0 / 15.0, 7.5 * 2.0 / 10.0);
                        }
                    }
                }
            },
            Some(MenuState::Fight) => {
                if let Some(key) = input_manager.get_key_on_press() {
                    let transitions: HashMap<(FightState, KeyCode), FightState> = [
                        ((FightState::Move3, KeyCode::KeyW), FightState::Move1),
                        ((FightState::Move4, KeyCode::KeyW), FightState::Move2),
                        ((FightState::Move1, KeyCode::KeyS), FightState::Move3),
                        ((FightState::Move2, KeyCode::KeyS), FightState::Move4),
                        ((FightState::Move2, KeyCode::KeyA), FightState::Move1),
                        ((FightState::Move4, KeyCode::KeyA), FightState::Move3),
                        ((FightState::Move1, KeyCode::KeyD), FightState::Move2),
                        ((FightState::Move3, KeyCode::KeyD), FightState::Move4),
                    ].iter().cloned().collect();

                    if let Some(current_selection) = self.fight_selection {

                        if let Some(&new_selection) = transitions.get(&(current_selection, key)) {
                            if self.valid_moves[new_selection as usize] {
                                println!("{:?}", new_selection);
                                self.fight_selection = Some(new_selection);
                                let (x_pos, y_pos) = match new_selection {
                                    FightState::Move1 => (0.25, 7.5),
                                    FightState::Move2 => (4.625, 7.5),
                                    FightState::Move3 => (0.25, 8.5),
                                    FightState::Move4 => (4.625, 8.5),
                                };

                                self.selector.update_position(x_pos * 2.0 / 15.0, y_pos * 2.0 / 10.0);
                            }
                        }
                    }
                    if key == KeyCode::KeyX {
                        self.menu_state = Some(MenuState::Main);
                        self.selector.update_position(7.6875 * 2.0 / 15.0, 7.5 * 2.0 / 10.0);
                    }
                }
            },
            Some(MenuState::Bag) => {
                //throw a pokeball
                //if pokemon is caught return true and add pokemon to party

            },
            Some(MenuState::Pokemon) => {
                self.party = Some(Party::new(pokemon, true, renderer));
            },
            Some(MenuState::Run) => {
                return true;
            },
            None => {},
        }

        return false;
    }

    fn begin_player_turn(&mut self) {
        self.battle_state = BattleState::PlayerTurn;
    }

    fn end_player_turn(&mut self) {
        Self::begin_enemy_turn(self);
    }

    fn begin_enemy_turn(&mut self) {
        self.battle_state = BattleState::EnemyTurn;
    }

    fn end_enemy_turn(&mut self) {
        Self::begin_player_turn(self);
    }

    fn begin_player_move(&mut self) {
        self.battle_state = BattleState::PlayerMove;
    }

    fn end_player_move(&mut self) {
        Self::begin_enemy_move(self);
    }

    fn begin_enemy_move(&mut self) {
        self.battle_state = BattleState::EnemyMove;
    }

    fn end_enemy_move(&mut self) {
        Self::begin_player_move(self);
    }

    fn begin_move_text(&mut self) {
        self.battle_state = BattleState::MoveText;
    }

    fn end_move_text(&mut self) {
        Self::begin_player_move(self);
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        // Handle encounter drawing

        if self.menu_state == Some(MenuState::Pokemon) {
            //check if party is Some
            if let Some(party) = &self.party {
                party.draw(renderer);
            }

            return;
        }

        let mut instances = Vec::new();
        instances.extend_from_slice(&self.scene.background);
        instances.extend_from_slice(&self.pokemon_front.front_instances);
        instances.extend_from_slice(&self.pokemon_back.back_instances);
        instances.extend_from_slice(&self.scene.ui);
        instances.extend_from_slice(&self.front_bg.texture);
        instances.extend_from_slice(&self.back_bg.texture);
        for sprite in &self.front_name.sprites {
            instances.extend_from_slice(&sprite.texture);
        }
        for sprite in &self.front_level.sprites {
            instances.extend_from_slice(&sprite.texture);
        }
        for sprite in &self.back_name.sprites {
            instances.extend_from_slice(&sprite.texture);
        }
        for sprite in &self.back_level.sprites {
            instances.extend_from_slice(&sprite.texture);
        }
        for sprite in &self.back_hp_current.sprites {
            instances.extend_from_slice(&sprite.texture);
        }
        for sprite in &self.back_hp_max.sprites {
            instances.extend_from_slice(&sprite.texture);
        }
        match self.menu_state {
            Some(MenuState::Main) => {
                instances.extend_from_slice(&self.main_menu.texture);
                instances.push(self.selector.texture[0]);
            },
            Some(MenuState::Fight) => {
                instances.extend_from_slice(&self.fight_menu.texture);
                for move_font in &self.move_fonts {
                    for sprite in &move_font.sprites {
                        instances.extend_from_slice(&sprite.texture);
                    }
                }
                instances.push(self.selector.texture[0]);
            },
            Some(MenuState::Bag) => {
                instances.extend_from_slice(&self.main_menu.texture);
                instances.push(self.selector.texture[0]);
            },
            Some(MenuState::Pokemon) => {
                instances.extend_from_slice(&self.main_menu.texture);
                instances.push(self.selector.texture[0]);
            },
            Some(MenuState::Run) => {
                instances.extend_from_slice(&self.main_menu.texture);
                instances.push(self.selector.texture[0]);
            },
            None => {},
        }

        let _ = renderer.render(&instances, false);
    }
}

fn create_front_bg() -> Sprite {

    let pos_x = 0.0;
    let pos_y = 2.0/10.0;
    let tex_x = 0;
    let tex_y = 10;
    let tex_w = 7;
    let tex_h = 2;
    let atlas_index = 3;
    let atlas_w = 15;
    let atlas_h = 10;
    let scale_x = 2.0;
    let scale_y = 2.0;

    Sprite::new(pos_x, pos_y, tex_x, tex_y, tex_w, tex_h, atlas_index, atlas_w, atlas_h, scale_x, scale_y)
}

fn create_back_bg() -> Sprite {

    let pos_x = 14.0/15.0;
    let pos_y = 8.0/10.0;
    let tex_x = 0;
    let tex_y = 12;
    let tex_w = 8;
    let tex_h = 3;
    let atlas_index = 3;
    let atlas_w = 15;
    let atlas_h = 10;
    let scale_x = 2.0;
    let scale_y = 2.0;

    Sprite::new(pos_x, pos_y, tex_x, tex_y, tex_w, tex_h, atlas_index, atlas_w, atlas_h, scale_x, scale_y)
}
