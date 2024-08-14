mod ui;
mod player_display;
mod enemy_display;

use winit::keyboard::KeyCode;
use std::collections::HashMap;
use std::time::Duration;
use crate::game::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::game::party::Party;
use crate::renderer::sprite::Sprite;
use crate::game::pokemon::Pokemon;
use ui::{UI, MenuState, FightState, UIMessage};
use player_display::PlayerDisplay;
use enemy_display::EnemyDisplay;
use super::moves::Move;
use rand::Rng;
use crate::renderer::instance::Instance;

pub enum BattleType {
    Wild,
    Trainer,
}

pub enum BattleState {
    PlayerTurn,
    EnemyTurn,
    HandleMoves,
    MoveText,
}

pub trait Display {
    fn update_health_bar(&mut self, pokemon: &Pokemon, renderer: &mut Renderer);
    fn draw(&self, instances: &mut Vec<Instance>);
}

pub struct Battle {
    battle_type: BattleType,
    battle_state: BattleState,
    ui: UI,
    player_pokemon_index: usize,
    enemy_pokemon_index: usize,
    player_display: PlayerDisplay,
    enemy_display: EnemyDisplay,
    background: Sprite,
    player_action: Option<Action>,
    enemy_action: Option<Action>,
    enemy_party: Vec<Pokemon>,
    escape_attempts: u32,
}

pub struct Action {
    action_type: ActionType,
    priority: u32,
    is_player: bool,
}

pub enum ActionType {
    Move {
        _move: Move,
    },
    Catch {
        rate: u32,
    },
    Swap {
        slot: u32,
    },
    Run,
}

impl Battle {
    pub fn new(player_pokemon: &mut Vec<Pokemon>, enemy_pokemon: Vec<Pokemon>, renderer: &mut Renderer) -> Self {
        println!("You encountered a level {} {}", enemy_pokemon[0].level, enemy_pokemon[0].name);
        let background = renderer.create_sprite(0.0, 0.0, 0, 0, 15, 7, "battle", 1.0, 1.0).expect("");

        let ui = UI::new(&player_pokemon[0], renderer);
        let player_display = PlayerDisplay::new(&player_pokemon[0], renderer);
        let enemy_display = EnemyDisplay::new(&enemy_pokemon[0], renderer);

        let battle_state = BattleState::PlayerTurn;

        Battle {
            battle_type: BattleType::Wild,
            battle_state,
            ui,
            player_pokemon_index: 0,
            enemy_pokemon_index: 0,
            player_display,
            enemy_display,
            background,
            player_action: None,
            enemy_action: None,
            enemy_party: enemy_pokemon,
            escape_attempts: 0,
        }
    }


    pub fn update(&mut self, player_party: &mut Vec<Pokemon>, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> bool {
        // Handle encounter updates and input
        renderer.camera_controller.update_camera(&mut renderer.camera, cgmath::Vector3::new(0.0, 0.0, 0.0));

        match self.battle_state {
            BattleState::PlayerTurn => {
                if let Some(action) = self.handle_player_turn(player_party, input_manager, dt, renderer) {
                    self.player_action = Some(action);
                    self.battle_state = BattleState::EnemyTurn;
                }
            },
            BattleState::EnemyTurn => {
                if let Some(action) = self.handle_enemy_turn(input_manager, dt, renderer) {
                    self.enemy_action = Some(action);
                    self.battle_state = BattleState::HandleMoves;
                }
            },
            BattleState::HandleMoves => {
                if self.resolve_moves(player_party, input_manager, dt, renderer) {
                    return true;
                }
            },
            BattleState::MoveText => {
                // Handle MoveText state
            },
        }

        false
    }

    fn handle_player_turn(&mut self, player_party: &mut Vec<Pokemon>, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> Option<Action> {

        if let Some(ui_message) = self.ui.update(player_party, input_manager, dt, renderer) {

            let action = match ui_message {
                UIMessage::Move { fight_state } => {
                    //map fight_state to a move
                    let _move = match fight_state {
                        FightState::Move1 => player_party[self.player_pokemon_index].moves[0].clone(),
                        FightState::Move2 => player_party[self.player_pokemon_index].moves[1].clone(),
                        FightState::Move3 => player_party[self.player_pokemon_index].moves[2].clone(),
                        FightState::Move4 => player_party[self.player_pokemon_index].moves[3].clone(),
                    };

                    Action {
                        action_type: ActionType::Move { _move },
                        priority: player_party[self.player_pokemon_index].stats.speed,
                        is_player: true,
                    }
                },
                UIMessage::Catch => {
                    Action {
                        action_type: ActionType::Catch { rate: 0 },
                        priority: 910,
                        is_player: true,
                    }
                },
                UIMessage::Swap { slot } => {
                    Action {
                        action_type: ActionType::Swap { slot },
                        priority: 900,
                        is_player: true,
                    }
                },
                UIMessage::Run => {
                    Action {
                        action_type: ActionType::Run,
                        priority: 999,
                        is_player: true,
                    }
                },
            };

            return Some(action);
        }

        return None;
    }

    fn handle_enemy_turn(&mut self, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> Option<Action> {
        //select a random move from the enemy pokemon
        //should be from 0 to length of moves
        let move_index = rand::thread_rng().gen_range(0..self.enemy_party[self.enemy_pokemon_index].moves.len());
        let _move = self.enemy_party[self.enemy_pokemon_index].moves[move_index].clone();

        let action = Action {
            action_type: ActionType::Move { _move },
            priority: self.enemy_party[self.enemy_pokemon_index].stats.speed,
            is_player: false,
        };

        return Some(action);
    }

    fn resolve_moves(&mut self, player_party: &mut Vec<Pokemon>, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> bool {
        let player_action = self.player_action.take();
        let enemy_action = self.enemy_action.take();

        if let (Some(player_action), Some(enemy_action)) = (player_action, enemy_action) {
            // Determine the order of actions based on priority
            let (first_action, second_action) = if player_action.priority >= enemy_action.priority {
                (player_action, enemy_action)
            } else {
                (enemy_action, player_action)
            };

            // Execute the first action
            if self.execute_action(&first_action, player_party, renderer) {
                return true;
            }

            // Execute the second action if the battle hasn't ended
            if self.execute_action(&second_action, player_party, renderer) {
                return true;
            }

            // If the battle continues, reset the state to PlayerTurn
            self.battle_state = BattleState::PlayerTurn;
        }

        false
    }

    fn execute_action(&mut self, action: &Action, player_party: &mut Vec<Pokemon>, renderer: &mut Renderer) -> bool {
        let (attacker, defender, defender_display) = if action.is_player {
            (&player_party[self.player_pokemon_index], &mut self.enemy_party[self.enemy_pokemon_index], &mut self.enemy_display as &mut dyn Display)
        } else {
            (&self.enemy_party[self.enemy_pokemon_index], &mut player_party[self.player_pokemon_index], &mut self.player_display as &mut dyn Display)
        };

        match &action.action_type {
            ActionType::Move { _move } => {

                if _move.power != 0 {
                    let damage = ((2 * attacker.level / 5 + 2) * _move.power * attacker.stats.attack / defender.stats.defense) / 50 + 2;
                    let user = if action.is_player { "Player" } else { "Enemy" };
                    println!("{}'s {} used {}. Dealt {} damage.", user, attacker.name, _move.name, damage);

                    defender.current_hp = defender.current_hp.saturating_sub(damage);
                    defender_display.update_health_bar(defender, renderer);
                } else {
                    let user = if action.is_player { "Player" } else { "Enemy" };
                    println!("{}'s {} used {}.", user, attacker.name, _move.name);
                }

                if defender.current_hp == 0 {
                    let user = if action.is_player { "Player" } else { "Enemy" };
                    println!("{}'s {} fainted.", user, defender.name);
                    return true;
                }
            },
            ActionType::Catch { rate } => {
                println!("Player tried to catch with rate {}.", rate);
            },
            ActionType::Swap { slot } => {
                println!("Player swapped to {}.", player_party[*slot as usize].name);
                self.player_pokemon_index = *slot as usize;
                self.player_display.swap(&player_party[self.player_pokemon_index], renderer);
                self.ui.update_moves(&player_party[self.player_pokemon_index], renderer);
            },
            ActionType::Run => {
                if attacker.stats.speed >= defender.stats.speed {
                    println!("Player ran away.");
                    return true;
                }

                let escape_chance = (attacker.stats.speed * 128 / defender.stats.speed + 30*self.escape_attempts).min(255); // Ensures the value is within 0 to 255.
                let chance_percentage = escape_chance as f64 / 256.0 * 100.0;

                if rand::thread_rng().gen_range(0..256) < escape_chance {
                    println!("Player escaped with {:.2}% chance.", chance_percentage);
                    return true;
                } else {
                    println!("Player failed to escape with {:.2}% chance.", chance_percentage);
                    self.escape_attempts += 1;
                }

            },
        }

        false
    }

    pub fn draw(&self, player_party: &mut Vec<Pokemon>, renderer: &mut Renderer) {
        // Handle encounter drawing

        if self.ui.menu_state == Some(MenuState::Pokemon) {
            //check if party is Some
            if let Some(party) = &self.ui.party {
                party.draw(renderer);
            }

            return;
        }

        let mut instances = Vec::new();
        instances.extend_from_slice(&self.background.texture);
        instances.extend_from_slice(&self.enemy_party[self.enemy_pokemon_index].front_sprite.texture);
        instances.extend_from_slice(&player_party[self.player_pokemon_index].back_sprite.texture);
        self.ui.draw(&mut instances);
        self.player_display.draw(&mut instances);
        self.enemy_display.draw(&mut instances);

        let _ = renderer.render(&instances, false);
    }

}


