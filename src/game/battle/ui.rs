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
        let selector = renderer.create_sprite(123.0, 120.0, 13, 10, 1, 1, "battle", 1.0, 1.0).expect("");
        let main_menu = renderer.create_sprite(7.0 * 16.0, 7.0 * 16.0, 7, 18, 8, 3, "battle", 1.0, 1.0).expect("");
        let fight_menu = renderer.create_sprite(0.0, 7.0 * 16.0, 0, 15, 15, 3, "battle", 1.0, 1.0).expect("");

        let (moves, valid_moves) = UI::initialize_moves(player_pokemon, renderer);

        Self {
            background,
            selector,
            main_menu,
            fight_menu,
            moves,
            valid_moves,
            menu_state: Some(MenuState::Main),
            menu_selection: Some(MenuState::Fight),
            fight_selection: Some(FightState::Move1),
            party: None,
        }
    }

    fn initialize_moves(player_pokemon: &Pokemon, renderer: &mut Renderer) -> (Vec<Font>, [bool; 4]) {
        let mut moves = Vec::new();
        let mut valid_moves = [false; 4];

        for i in 0..4 {
            let (x, y) = match i {
                0 => (14.0, 123.0),
                1 => (86.0, 123.0),
                2 => (14.0, 138.0),
                3 => (86.0, 138.0),
                _ => (0.0, 0.0),
            };

            let move_name = if let Some(a_move) = player_pokemon.moves.get(i) {
                valid_moves[i] = true;
                a_move.name.to_uppercase()
            } else {
                "-".to_string()
            };

            let move_text = Font::new(x, y, &move_name, true, "black_font", renderer);
            moves.push(move_text);
        }

        (moves, valid_moves)
    }

    fn get_move_position(&self, index: usize) -> (f32, f32) {
        match index {
            0 => (14.0, 123.0),
            1 => (86.0, 123.0),
            2 => (14.0, 138.0),
            3 => (86.0, 138.0),
            _ => (0.0, 0.0),
        }
    }

    fn get_menu_position(&self, state: MenuState) -> (f32, f32) {
        match state {
            MenuState::Main => (123.0, 120.0),
            MenuState::Fight => (123.0, 120.0),
            MenuState::Bag => (179.0, 120.0),
            MenuState::Pokemon => (123.0, 136.0),
            MenuState::Run => (179.0, 136.0),
        }
    }

    fn get_fight_position(&self, state: FightState) -> (f32, f32) {
        match state {
            FightState::Move1 => (3.0, 119.0),
            FightState::Move2 => (74.0, 119.0),
            FightState::Move3 => (3.0, 135.0),
            FightState::Move4 => (74.0, 135.0),
        }
    }

    fn update_selector_position(&mut self) {
        if let Some(menu_state) = self.menu_state {
            match menu_state {
                MenuState::Main => {
                    if let Some(selection) = self.menu_selection {
                        self.selector.update_position(self.get_menu_position(selection).0, self.get_menu_position(selection).1);
                    }
                }
                MenuState::Fight => {
                    if let Some(selection) = self.fight_selection {
                        self.selector.update_position(self.get_fight_position(selection).0, self.get_fight_position(selection).1);
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_menu_input(&mut self, key: KeyCode) {
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
                self.menu_selection = Some(new_selection);
            }
        }
    }

    fn handle_fight_input(&mut self, key: KeyCode) {
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
                    self.fight_selection = Some(new_selection);
                }
            }
        }
    }

    pub fn update(&mut self, pokemon: &mut Vec<Pokemon>, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> Option<UIMessage> {
        match self.menu_state {
            Some(MenuState::Main) => {
                if let Some(key) = input_manager.get_key_on_press() {
                    self.handle_menu_input(key);
                    self.update_selector_position();

                    if key == KeyCode::KeyZ {
                        self.menu_state = self.menu_selection;
                        self.update_selector_position();

                        if self.menu_state == Some(MenuState::Pokemon) {
                            self.party = Some(Party::new(pokemon, false, true, renderer));
                        }
                    }
                }
            },
            Some(MenuState::Fight) => {
                if let Some(key) = input_manager.get_key_on_press() {
                    self.handle_fight_input(key);
                    self.update_selector_position();

                    if key == KeyCode::KeyX {
                        self.menu_state = Some(MenuState::Main);
                        self.update_selector_position();
                    }
                    if key == KeyCode::KeyZ {
                        if let Some(fight_selection) = self.fight_selection {
                            return Some(UIMessage::Move { fight_state: fight_selection });
                        }
                    }
                }
            },
            Some(MenuState::Bag) => {
                return Some(UIMessage::Catch);
            }
            Some(MenuState::Pokemon) => {
                if let Some(party) = &mut self.party {
                    let selected_slot = party.update(input_manager, dt, renderer);
                    //only send message if pokemon isnt fainted
                    if selected_slot < party.slots.len() as u32 {
                        if !party.slots[selected_slot as usize].fainted {
                            self.party = None;

                            println!("Selected slot: {}", selected_slot);
                            self.return_to_main();
                            return Some(UIMessage::Swap { slot: selected_slot });
                        } else {
                            println!("Can't swap to a fainted Pokémon");
                        }
                    }

                    if selected_slot == 6 {
                        self.party = None;
                        self.menu_state = Some(MenuState::Main);
                    }
                }
            }
            Some(MenuState::Run) => {
                let message = UIMessage::Run;
                self.menu_state = Some(MenuState::Main);

                return Some(message);
            }
            None => {}
        }

        None
    }

    pub fn open_swap_menu(&mut self, pokemon: &mut Vec<Pokemon>, renderer: &mut Renderer) {
        self.menu_state = Some(MenuState::Pokemon);
        self.party = Some(Party::new(pokemon, true, true, renderer));
    }

    pub fn return_to_main(&mut self) {
        self.menu_state = Some(MenuState::Main);
        self.update_selector_position();
    }

    pub fn update_moves(&mut self, player_pokemon: &Pokemon, renderer: &mut Renderer) {
        let mut moves = Vec::new();
        let mut valid_moves = [false; 4];

        for i in 0..4 {
            let move_name = if let Some(a_move) = player_pokemon.moves.get(i) {
                valid_moves[i] = true;
                a_move.name.to_uppercase()
            } else {
                "-".to_string()
            };
            let move_text = Font::new(self.get_move_position(i).0, self.get_move_position(i).1, &move_name, true, "black_font", renderer);
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
            }
            Some(MenuState::Fight) => {
                instances.extend_from_slice(&self.fight_menu.texture);
                for move_font in &self.moves {
                    for sprite in &move_font.sprites {
                        instances.extend_from_slice(&sprite.texture);
                    }
                }
                instances.push(self.selector.texture[0]);
            }
            Some(MenuState::Bag) => {
                // Bag UI drawing logic can go here
            }
            Some(MenuState::Pokemon) => {
                // Pokemon UI drawing logic can go here
            }
            Some(MenuState::Run) => {
                // Run UI drawing logic can go here
            }
            None => {},
        }
    }
}
