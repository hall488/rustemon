use crate::renderer::Renderer;
use crate::renderer::sprite::Sprite;
use crate::renderer::instance::Instance;
use crate::game::input_manager::InputManager;
use crate::game::font::Font;
use crate::game::pokemon::Pokemon;
use winit::keyboard::KeyCode;
use std::collections::HashMap;
use crate::game::party::Party;
use std::time::Duration;
use super::{Action, ActionType};

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

pub enum UIMessage {
    Move {
       fight_state: FightState,
    },
    Catch,
    Swap {
        slot: u32,
    },
    Run,
}

pub struct UI {
    background: Sprite,
    selector: Sprite,
    main_menu: Sprite,
    fight_menu: Sprite,
    moves: Vec<Font>,
    valid_moves: [bool; 4],
    pub menu_state: Option<MenuState>,
    menu_selection: Option<MenuState>,
    fight_selection: Option<FightState>,
    pub party: Option<Party>,
}

impl UI {
    pub fn new(player_pokemon: &Pokemon, renderer: &mut Renderer) -> Self {
        let background = renderer.create_sprite(0.0, 7.0*16.0, 0, 7, 15, 3, "battle", 1.0, 1.0).expect("");

        let menu_state = Some(MenuState::Main);
        let menu_selection = Some(MenuState::Fight);
        let fight_selection = Some(FightState::Move1);

        let selector = renderer.create_sprite(123.0, 120.0, 13, 10, 1, 1, "battle", 1.0, 1.0).expect("");
        let main_menu = renderer.create_sprite(7.0 * 16.0, 7.0 * 16.0, 7, 18, 8, 3, "battle", 1.0, 1.0).expect("");

        let fight_menu = renderer.create_sprite(0.0, 7.0 * 16.0, 0, 15, 15, 3, "battle", 1.0, 1.0).expect("");

        let mut moves = Vec::new();

        fn get_position(index: usize) -> (f32, f32) {
            match index {
                0 => (14.0, 123.0),
                1 => (86.0, 123.0),
                2 => (14.0, 138.0),
                3 => (86.0, 138.0),
                _ => (0.0, 0.0), // This should not occur since we iterate only over 0 to 3
            }
        }

        let mut valid_moves = [false; 4]; // Initialize all moves as invalid
        for i in 0..4 {
            let (x, y) = get_position(i);
            let move_name = if let Some(a_move) = player_pokemon.moves.get(i) {
                valid_moves[i] = true;
                a_move.name.to_uppercase()
            } else {
                "-".to_string()
            };
            let move_text = Font::new(x, y, &move_name, true, "black_font", renderer);
            moves.push(move_text);
        }

        Self {
            background,
            selector,
            main_menu,
            fight_menu,
            moves,
            valid_moves,
            menu_state,
            menu_selection,
            fight_selection,
            party: None,
        }
    }

    pub fn update(&mut self, pokemon: &mut Vec<Pokemon>, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> Option<UIMessage> {
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
                                MenuState::Fight => (123.0, 120.0),
                                MenuState::Bag => (179.0, 120.0),
                                MenuState::Pokemon => (123.0, 136.0),
                                MenuState::Run => (179.0, 136.0),
                                _ => (0.0, 0.0),
                            };

                            self.selector.update_position(x_pos, y_pos);
                        }
                    }
                    if key == KeyCode::KeyZ {
                        self.menu_state = self.menu_selection;

                        if self.menu_state == Some(MenuState::Pokemon) {
                            self.party = Some(Party::new(pokemon, true, renderer));
                        }

                        if self.menu_state == Some(MenuState::Fight) {
                            self.selector.update_position(3.0, 119.0);
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
                                    FightState::Move1 => (3.0, 119.0),
                                    FightState::Move2 => (74.0, 119.0),
                                    FightState::Move3 => (3.0, 135.0),
                                    FightState::Move4 => (74.0, 135.0),
                                };

                                self.selector.update_position(x_pos, y_pos);
                            }
                        }
                    }
                    if key == KeyCode::KeyX {
                        self.menu_state = Some(MenuState::Main);
                        self.selector.update_position(123.0, 120.0);
                    }
                    if key == KeyCode::KeyZ {
                        if let Some(fight_selection) = self.fight_selection {
                            return Some(UIMessage::Move { fight_state: fight_selection });
                        }
                    }
                }
            },
            Some(MenuState::Bag) => {
                let message = UIMessage::Catch;
                self.menu_state = Some(MenuState::Main);

                return Some(message);
            },
            Some(MenuState::Pokemon) => {
                if let Some(party) = &mut self.party {
                    let selected_slot = party.update(input_manager, dt, renderer);

                    if selected_slot < party.slots.len() as u32 {
                        self.party = None;
                        self.menu_state = Some(MenuState::Main);
                        return Some(UIMessage::Swap { slot: selected_slot });
                    }

                    if selected_slot == 6 {
                        println!("Canceling party selection");
                        self.party = None;
                        self.menu_state = Some(MenuState::Main);
                    }
                }
            },
            Some(MenuState::Run) => {
                let message = UIMessage::Run;
                self.menu_state = Some(MenuState::Main);

                return Some(message);
            },
            None => {},
        }

        return None;
    }

    pub fn update_moves(&mut self, player_pokemon: &Pokemon, renderer: &mut Renderer) {
        let mut moves = Vec::new();

        fn get_position(index: usize) -> (f32, f32) {
            match index {
                0 => (14.0, 123.0),
                1 => (86.0, 123.0),
                2 => (14.0, 138.0),
                3 => (86.0, 138.0),
                _ => (0.0, 0.0), // This should not occur since we iterate only over 0 to 3
            }
        }

        let mut valid_moves = [false; 4]; // Initialize all moves as invalid
        for i in 0..4 {
            let (x, y) = get_position(i);
            let move_name = if let Some(a_move) = player_pokemon.moves.get(i) {
                valid_moves[i] = true;
                a_move.name.to_uppercase()
            } else {
                "-".to_string()
            };
            let move_text = Font::new(x, y, &move_name, true, "black_font", renderer);
            moves.push(move_text);
        }

        self.moves = moves;
        self.valid_moves = valid_moves;
    }

    pub fn draw(&self, instances: &mut Vec<Instance>) {
        instances.extend_from_slice(&self.background.texture);

        match self.menu_state {
            Some(MenuState::Main) => {
                instances.extend_from_slice(&self.main_menu.texture);
                instances.push(self.selector.texture[0]);
            },
            Some(MenuState::Fight) => {
                instances.extend_from_slice(&self.fight_menu.texture);
                for move_font in &self.moves {
                    for sprite in &move_font.sprites {
                        instances.extend_from_slice(&sprite.texture);
                    }
                }
                instances.push(self.selector.texture[0]);
            },
            Some(MenuState::Bag) => {
                //instances.extend_from_slice(&self.main_menu.texture);
                //instances.push(self.selector.texture[0]);
            },
            Some(MenuState::Pokemon) => {
                //instances.extend_from_slice(&self.main_menu.texture);
                //instances.push(self.selector.texture[0]);
            },
            Some(MenuState::Run) => {
                //instances.extend_from_slice(&self.main_menu.texture);
                //instances.push(self.selector.texture[0]);
            },
            None => {},
        }
    }
}
