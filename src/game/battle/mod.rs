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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BattleType {
    Wild,
    Trainer,
}

pub enum BattleState {
    PlayerTurn,
    EnemyTurn,
    HandleMoves,
    MoveText,
    PlayerFaint,
    PlayerForceSwap,
    EnemyFaint,
}

pub trait Display {
    fn update_health_bar(&mut self, pokemon: &Pokemon, renderer: &mut Renderer);
    fn draw(&self, instances: &mut Vec<Instance>);
}

pub struct Battle {
    pub battle_type: BattleType,
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
        ball_bonus: f32,
    },
    Swap {
        slot: u32,
    },
    Run,
}

impl Battle {
    pub fn new(battle_type: BattleType, player_pokemon: &mut Vec<Pokemon>, enemy_pokemon: Vec<Pokemon>, renderer: &mut Renderer) -> Self {
        println!("You encountered a level {} {}", enemy_pokemon[0].level, enemy_pokemon[0].name);
        let background = renderer.create_sprite(0.0, 0.0, 0, 0, 15, 7, "battle", 1.0, 1.0).expect("");

        //first non fainted pokemon
        let first_index = player_pokemon.iter().position(|p| p.current_hp > 0).unwrap();
        let first_pokemon = &player_pokemon[first_index];

        let ui = UI::new(first_pokemon, renderer);
        let player_display = PlayerDisplay::new(first_pokemon, renderer);
        let enemy_display = EnemyDisplay::new(&enemy_pokemon[0], renderer);

        let battle_state = BattleState::PlayerTurn;

        Battle {
            battle_type,
            battle_state,
            ui,
            player_pokemon_index: first_index,
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


    pub fn update(&mut self, player_party: &mut Vec<Pokemon>, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> Option<bool> {
        // Handle encounter updates and input
        renderer.camera.update_camera(cgmath::Vector3::new(0.0, 0.0, 0.0));

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
                    return Some(true);
                }
            },
            BattleState::PlayerFaint => {
                // Handle PlayerFaint state
                // if player has no pokemon they lose
                if player_party.iter().all(|p| p.current_hp == 0) {
                    println!("Player has no more pokemon. Player loses.");
                    return Some(false);
                }

                //switch to next pokemon
                self.ui.open_swap_menu(player_party, renderer);
                self.battle_state = BattleState::PlayerForceSwap;
            },
            BattleState::PlayerForceSwap => {
                // Handle PlayerForceSwap state
                if let Some(ui_message) = self.ui.update(player_party, input_manager, dt, renderer) {
                    match ui_message {
                        UIMessage::Swap { slot } => {
                            self.player_pokemon_index = slot as usize;
                            self.player_display.swap(&player_party[self.player_pokemon_index], renderer);
                            self.ui.update_moves(&player_party[self.player_pokemon_index], renderer);
                            self.battle_state = BattleState::PlayerTurn;
                        },
                        _ => {},
                    }
                }
            },
            BattleState::EnemyFaint => {
                // Handle EnemyFaint state
                //gain exp
                let exp = self.enemy_party[self.enemy_pokemon_index].experience_yield;
                player_party[self.player_pokemon_index].gain_experience(exp);

                if self.enemy_party.iter().all(|p| p.current_hp == 0) {
                    println!("All enemy pokemon fainted. Player wins.");
                    return Some(true);
                }

                self.enemy_pokemon_index += 1;
                self.enemy_display.swap(&self.enemy_party[self.enemy_pokemon_index], renderer);
                self.battle_state = BattleState::PlayerTurn;
            },
            BattleState::MoveText => {
                // Handle MoveText state
            },
        }

        None
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
                        action_type: ActionType::Catch { ball_bonus: 1.0},
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

            if self.execute_action(&first_action, player_party, renderer) {

                if let ActionType::Run = first_action.action_type {
                    return true;
                }

                if let ActionType::Catch { .. } = first_action.action_type {
                    //add pokemon to party
                    player_party.push(self.enemy_party.remove(self.enemy_pokemon_index));
                    return true;
                }

                if first_action.is_player {
                    self.battle_state = BattleState::EnemyFaint;
                } else {
                    self.battle_state = BattleState::PlayerFaint;
                }

                return false;
            }

            if self.execute_action(&second_action, player_party, renderer) {
                if second_action.is_player {
                    self.battle_state = BattleState::EnemyFaint;
                } else {
                    self.battle_state = BattleState::PlayerFaint;
                }
                return false;
            }


            // If the battle continues, reset the state to PlayerTurn
            self.battle_state = BattleState::PlayerTurn;
            self.ui.return_to_main();
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
                    let user = if !action.is_player { "Player" } else { "Enemy" };
                    println!("{}'s {} fainted.", user, defender.name);

                    return true;
                }
            },
            ActionType::Catch { ball_bonus } => {
                //if catch succeeds add to party and return true
                //print max hp and current
                let modifier = (3.0 * defender.stats.hp as f32 - 2.0 * defender.current_hp as f32) * defender.catch_rate as f32 / (3.0 * defender.stats.hp as f32);

                println!("Modifier: {}", modifier);
                let catch_rate = ball_bonus * modifier;
                println!("Catch rate: {}", catch_rate);
                let mut rng = rand::thread_rng();
                let val = rng.gen_range(0..256);
                println!("Random value: {}", val);
                if val < catch_rate as u32 {
                    println!("Player caught {}.", defender.name);
                    return true;
                }
            },
            ActionType::Swap { slot } => {
                //only switch if not fainted
                println!("Player swapped to {}.", player_party[*slot as usize].name);
                self.player_pokemon_index = *slot as usize;
                self.player_display.swap(&player_party[self.player_pokemon_index], renderer);
                self.ui.update_moves(&player_party[self.player_pokemon_index], renderer);
            },
            ActionType::Run => {

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


