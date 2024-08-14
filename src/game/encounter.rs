//use crate::game::input_manager::InputManager;
//use std::time::Duration;
//use crate::renderer::Renderer;
//use crate::game::pokemon::Pokemon;
//use crate::renderer::sprite::Sprite;
//use crate::game::stationary_loader::Stationary;
//use tiled::Loader;
//use crate::game::font::Font;
//use winit::keyboard::KeyCode;
//use std::collections::HashMap;
//use super::party::Party;
//
////order of turns
////player chooses fight, bag, pokemon, or run
////players decision is locked in
////enemy chooses move
////based on both inputs, the moves are executed
////output text for each move is made
////repeat until one pokemon faints
//
//
//pub struct Encounter {
//    pokemon_back: Pokemon,
//    pokemon_front: Pokemon,
//    background: Sprite,
//    ui: Sprite,
//    front_bg: Sprite,
//    back_bg: Sprite,
//    front_name: Font,
//    front_level: Font,
//    back_name: Font,
//    back_level: Font,
//    back_hp_current: Font,
//    back_hp_max: Font,
//    battle_state: BattleState,
//    menu_state: Option<MenuState>,
//    fight_selection: Option<FightState>,
//    menu_selection: Option<MenuState>,
//    selector: Sprite,
//    main_menu: Sprite,
//    fight_menu: Sprite,
//    move_fonts: Vec<Font>,
//    valid_moves: [bool; 4],
//    party: Option<Party>,
//}
//
//impl Encounter {
//    pub fn new(player_pokemon: &Vec<Pokemon>, pokemon_front: Pokemon, renderer: &mut Renderer) -> Self {
//
//        println!("You encountered a level {} {}", pokemon_front.level, pokemon_front.name);
//
//        let background = renderer.create_sprite(0.0, 0.0, 0, 0, 15, 7, "battle", 1.0, 1.0).expect("");
//        let ui = renderer.create_sprite(0.0, 7.0*16.0, 0, 7, 15, 3, "battle", 1.0, 1.0).expect("");
//
//        let pokemon_back = player_pokemon[0].clone();
//
//        let front_bg = renderer.create_sprite(0.0, 16.0, 0, 10, 7, 2, "battle", 1.0, 1.0).expect("");
//        let back_bg = renderer.create_sprite(7.0 * 16.0, 4.0 * 16.0, 0, 12, 8, 3, "battle", 1.0, 1.0).expect("");
//
//        let front_name = Font::new(21.0, 19.0, &pokemon_front.name.to_uppercase(), true, renderer);
//        let level = "Lv".to_string() + &pokemon_front.level.to_string();
//        let front_level = Font::new(95.0, 19.0, &level, false, renderer);
//
//        let back_name = Font::new(141.0, 77.0, &pokemon_back.name.to_uppercase(), true, renderer);
//        let level = "Lv".to_string() + &pokemon_back.level.to_string();
//        let back_level = Font::new(215.0, 77.0, &level, false, renderer);
//
//        let hp_current = pokemon_back.current_hp.to_string() + "/";
//        let back_hp_current = Font::new(200.0, 95.0, &hp_current, false, renderer);
//        let back_hp_max = Font::new(215.0, 95.0, &pokemon_back.stats.hp.to_string(), false, renderer);
//
//        let battle_state = BattleState::PlayerTurn;
//        let menu_state = Some(MenuState::Main);
//        let menu_selection = Some(MenuState::Fight);
//        let fight_selection = Some(FightState::Move1);
//
//        let selector = renderer.create_sprite(123.0, 120.0, 13, 10, 1, 1, "battle", 1.0, 1.0).expect("");
//        let main_menu = renderer.create_sprite(7.0 * 16.0, 7.0 * 16.0, 7, 18, 8, 3, "battle", 1.0, 1.0).expect("");
//
//        let fight_menu = renderer.create_sprite(0.0, 7.0 * 16.0, 0, 15, 15, 3, "battle", 1.0, 1.0).expect("");
//
//        let mut move_fonts = Vec::new();
//
//        fn get_position(index: usize) -> (f32, f32) {
//            match index {
//                0 => (14.0, 123.0),
//                1 => (86.0, 123.0),
//                2 => (14.0, 138.0),
//                3 => (86.0, 138.0),
//                _ => (0.0, 0.0), // This should not occur since we iterate only over 0 to 3
//            }
//        }
//
//        let mut valid_moves = [false; 4]; // Initialize all moves as invalid
//        for i in 0..4 {
//            let (x, y) = get_position(i);
//            let move_name = if let Some(a_move) = pokemon_back.moves.get(i) {
//                valid_moves[i] = true;
//                a_move.name.to_uppercase()
//            } else {
//                "-".to_string()
//            };
//            let move_text = Font::new(x, y, &move_name, true, renderer);
//            move_fonts.push(move_text);
//        }
//
//        let party = None;
//
//        Self {
//            // Initialize fields
//            pokemon_back,
//            pokemon_front,
//            background,
//            ui,
//            front_bg,
//            back_bg,
//            front_name,
//            front_level,
//            back_name,
//            back_level,
//            back_hp_current,
//            back_hp_max,
//            battle_state,
//            menu_state,
//            fight_selection,
//            menu_selection,
//            selector,
//            main_menu,
//            fight_menu,
//            move_fonts,
//            valid_moves,
//            party,
//        }
//    }
//
//    pub fn update(&mut self, pokemon: &mut Vec<Pokemon>, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> bool {
//        // Handle encounter updates and input
//        renderer.camera_controller.update_camera(&mut renderer.camera, cgmath::Vector3::new(0.0, 0.0, 0.0));
//
//        match self.battle_state {
//            BattleState::PlayerTurn => {
//                //check party is some
//                if let Some(party) = &mut self.party {
//                    if party.update(input_manager, dt, renderer) {
//                        println!("Party update returned true");
//                        self.party = None;
//                        self.menu_state = Some(MenuState::Main);
//                    }
//                    return false;
//                }
//
//                if self.handle_player_turn(pokemon, input_manager, renderer) {
//                    return true;
//                }
//            },
//            BattleState::PlayerMove => {
//                // Handle PlayerMove state
//            },
//            BattleState::EnemyTurn => {
//                // Handle EnemyTurn state
//            },
//            BattleState::EnemyMove => {
//                // Handle EnemyMove state
//            },
//            BattleState::MoveText => {
//                // Handle MoveText state
//            },
//        }
//
//        return false;
//    }
//
//    fn handle_player_turn(&mut self, pokemon: &mut Vec<Pokemon>, input_manager: &mut InputManager, renderer: &mut Renderer) -> bool {
//        match self.menu_state {
//            Some(MenuState::Main) => {
//                if let Some(key) = input_manager.get_key_on_press() {
//                    let transitions: HashMap<(MenuState, KeyCode), MenuState> = [
//                        ((MenuState::Pokemon, KeyCode::KeyW), MenuState::Fight),
//                        ((MenuState::Run, KeyCode::KeyW), MenuState::Bag),
//                        ((MenuState::Fight, KeyCode::KeyS), MenuState::Pokemon),
//                        ((MenuState::Bag, KeyCode::KeyS), MenuState::Run),
//                        ((MenuState::Bag, KeyCode::KeyA), MenuState::Fight),
//                        ((MenuState::Run, KeyCode::KeyA), MenuState::Pokemon),
//                        ((MenuState::Fight, KeyCode::KeyD), MenuState::Bag),
//                        ((MenuState::Pokemon, KeyCode::KeyD), MenuState::Run),
//                    ].iter().cloned().collect();
//
//                    if let Some(current_selection) = self.menu_selection {
//
//                        if let Some(&new_selection) = transitions.get(&(current_selection, key)) {
//                            println!("{:?}", new_selection);
//                            self.menu_selection = Some(new_selection);
//                            let (x_pos, y_pos) = match new_selection {
//                                MenuState::Fight => (123.0, 120.0),
//                                MenuState::Bag => (179.0, 120.0),
//                                MenuState::Pokemon => (123.0, 136.0),
//                                MenuState::Run => (179.0, 136.0),
//                                _ => (0.0, 0.0),
//                            };
//
//                            self.selector.update_position(x_pos, y_pos);
//                        }
//                    }
//                    if key == KeyCode::KeyZ {
//                        self.menu_state = self.menu_selection;
//                        if self.menu_state == Some(MenuState::Fight) {
//                            self.selector.update_position(3.0, 119.0);
//                        }
//                    }
//                }
//            },
//            Some(MenuState::Fight) => {
//                if let Some(key) = input_manager.get_key_on_press() {
//                    let transitions: HashMap<(FightState, KeyCode), FightState> = [
//                        ((FightState::Move3, KeyCode::KeyW), FightState::Move1),
//                        ((FightState::Move4, KeyCode::KeyW), FightState::Move2),
//                        ((FightState::Move1, KeyCode::KeyS), FightState::Move3),
//                        ((FightState::Move2, KeyCode::KeyS), FightState::Move4),
//                        ((FightState::Move2, KeyCode::KeyA), FightState::Move1),
//                        ((FightState::Move4, KeyCode::KeyA), FightState::Move3),
//                        ((FightState::Move1, KeyCode::KeyD), FightState::Move2),
//                        ((FightState::Move3, KeyCode::KeyD), FightState::Move4),
//                    ].iter().cloned().collect();
//
//                    if let Some(current_selection) = self.fight_selection {
//
//                        if let Some(&new_selection) = transitions.get(&(current_selection, key)) {
//                            if self.valid_moves[new_selection as usize] {
//                                println!("{:?}", new_selection);
//                                self.fight_selection = Some(new_selection);
//                                let (x_pos, y_pos) = match new_selection {
//                                    FightState::Move1 => (3.0, 119.0),
//                                    FightState::Move2 => (74.0, 119.0),
//                                    FightState::Move3 => (3.0, 135.0),
//                                    FightState::Move4 => (74.0, 135.0),
//                                };
//
//                                self.selector.update_position(x_pos, y_pos);
//                            }
//                        }
//                    }
//                    if key == KeyCode::KeyX {
//                        self.menu_state = Some(MenuState::Main);
//                        self.selector.update_position(123.0, 120.0);
//                    }
//                }
//            },
//            Some(MenuState::Bag) => {
//                pokemon.push(self.pokemon_front.clone());
//                return true;
//            },
//            Some(MenuState::Pokemon) => {
//                self.party = Some(Party::new(pokemon, true, renderer));
//            },
//            Some(MenuState::Run) => {
//                return true;
//            },
//            None => {},
//        }
//
//        return false;
//    }
//
//    fn begin_player_turn(&mut self) {
//        self.battle_state = BattleState::PlayerTurn;
//    }
//
//    fn end_player_turn(&mut self) {
//        Self::begin_enemy_turn(self);
//    }
//
//    fn begin_enemy_turn(&mut self) {
//        self.battle_state = BattleState::EnemyTurn;
//    }
//
//    fn end_enemy_turn(&mut self) {
//        Self::begin_player_turn(self);
//    }
//
//    fn begin_player_move(&mut self) {
//        self.battle_state = BattleState::PlayerMove;
//    }
//
//    fn end_player_move(&mut self) {
//        Self::begin_enemy_move(self);
//    }
//
//    fn begin_enemy_move(&mut self) {
//        self.battle_state = BattleState::EnemyMove;
//    }
//
//    fn end_enemy_move(&mut self) {
//        Self::begin_player_move(self);
//    }
//
//    fn begin_move_text(&mut self) {
//        self.battle_state = BattleState::MoveText;
//    }
//
//    fn end_move_text(&mut self) {
//        Self::begin_player_move(self);
//    }
//
//    pub fn draw(&self, renderer: &mut Renderer) {
//        // Handle encounter drawing
//
//        if self.menu_state == Some(MenuState::Pokemon) {
//            //check if party is Some
//            if let Some(party) = &self.party {
//                party.draw(renderer);
//            }
//
//            return;
//        }
//
//        let mut instances = Vec::new();
//        instances.extend_from_slice(&self.background.texture);
//        instances.extend_from_slice(&self.pokemon_front.front_sprite.texture);
//        instances.extend_from_slice(&self.pokemon_back.back_sprite.texture);
//        instances.extend_from_slice(&self.ui.texture);
//        instances.extend_from_slice(&self.front_bg.texture);
//        instances.extend_from_slice(&self.back_bg.texture);
//        for sprite in &self.front_name.sprites {
//            instances.extend_from_slice(&sprite.texture);
//        }
//        for sprite in &self.front_level.sprites {
//            instances.extend_from_slice(&sprite.texture);
//        }
//        for sprite in &self.back_name.sprites {
//            instances.extend_from_slice(&sprite.texture);
//        }
//        for sprite in &self.back_level.sprites {
//            instances.extend_from_slice(&sprite.texture);
//        }
//        for sprite in &self.back_hp_current.sprites {
//            instances.extend_from_slice(&sprite.texture);
//        }
//        for sprite in &self.back_hp_max.sprites {
//            instances.extend_from_slice(&sprite.texture);
//        }
//        match self.menu_state {
//            Some(MenuState::Main) => {
//                instances.extend_from_slice(&self.main_menu.texture);
//                instances.push(self.selector.texture[0]);
//            },
//            Some(MenuState::Fight) => {
//                instances.extend_from_slice(&self.fight_menu.texture);
//                for move_font in &self.move_fonts {
//                    for sprite in &move_font.sprites {
//                        instances.extend_from_slice(&sprite.texture);
//                    }
//                }
//                instances.push(self.selector.texture[0]);
//            },
//            Some(MenuState::Bag) => {
//                //instances.extend_from_slice(&self.main_menu.texture);
//                //instances.push(self.selector.texture[0]);
//            },
//            Some(MenuState::Pokemon) => {
//                //instances.extend_from_slice(&self.main_menu.texture);
//                //instances.push(self.selector.texture[0]);
//            },
//            Some(MenuState::Run) => {
//                //instances.extend_from_slice(&self.main_menu.texture);
//                //instances.push(self.selector.texture[0]);
//            },
//            None => {},
//        }
//
//        let _ = renderer.render(&instances, false);
//    }
//}

