use crate::renderer::instance::Instance;
use crate::game::map_loader::Rectangle;
use cgmath::{Vector3, Matrix4};
use std::time::{Duration, Instant};
use winit::keyboard::KeyCode;
use crate::game::pokemon::Pokemon;
use crate::game::entity::Entity;
use crate::game::npc::NPC;
use crate::game::animation_player::{AnimationPlayer, Animation, AnimationSheet};
use crate::renderer::Renderer;
use crate::renderer::texture_manager::Atlas;
use std::collections::HashMap;

use super::input_manager::{self, InputManager};

const GRID_SIZE: f32 = 1.0; // Define the grid size
const MOVEMENT_DURATION_WALKING: Duration = Duration::from_millis(250); // Duration to move from one grid cell to another
const MOVEMENT_DURATION_RUNNING: Duration = Duration::from_millis(125); // Duration to move from one grid cell to another
const MOVEMENT_THRESHOLD: Duration = Duration::from_millis(100); // Threshold to check for movement input
const ANIMATION_DURATION_WALKING: Duration = Duration::from_millis(125); // Duration to switch animation frames
const ANIMATION_DURATION_RUNNING: Duration = Duration::from_millis(67); // Duration to switch animation frames

//TODO FIX ANIMATION PLAYER IT CALLS PLAY ANIMATION EVERY FRAME ON COLLISIONS
//I TOOK OUT THE PRINT STATEMENT CUZ IT GOT ANNOYING

pub struct Player {
    pub instances: Vec<Instance>,
    pub position: Vector3<f32>,
    pub target_position: Vector3<f32>,
    pub movement_timer: Duration, // Timer to track movement duration
    pub animation_player: AnimationPlayer,
    pub input_provided: bool, // Flag to track if input is being provided
    pub time_of_input: Instant, // Time when the input was provided
    pub last_direction: Vector3<f32>, // Last key pressed
    pub facing_direction: Vector3<f32>, // Direction of the player
    pub spot_arrival: bool,
    running: bool,
}

impl Entity for Player {
    fn position(&self) -> Vector3<f32> {
        self.position // Assuming `Player` has a `position` field of type `Vector3<f32>`
    }

    fn instances(&self) -> &[Instance] {
        &self.animation_player.get_instances() // Assuming `Player` has an `instances` field of type `Vec<Instance>`
    }
}

impl Player {
    pub fn new(renderer: &Renderer) -> Self {
        let position = Vector3::new(16.0, -12.0, 0.0);

        let sheet = AnimationSheet {
            frame_width: 1,
            frame_height: 2,
            frame_order: vec![1, 2, 1, 0],
            frame_duration: ANIMATION_DURATION_WALKING,
            atlas: renderer.get_atlas("player").unwrap().clone(),
            looped: true,
        };

        let mut animations = HashMap::new();

        let down_animation = Animation::new(position, &sheet, 0, 0, 3, 2);
        let up_animation = Animation::new(position, &sheet, 0, 2, 3, 2);
        let left_animation = Animation::new(position, &sheet, 0, 4, 3, 2);
        let right_animation = Animation::new(position, &sheet, 0, 6, 3, 2);

        animations.insert("up".to_string(), up_animation);
        animations.insert("down".to_string(), down_animation);
        animations.insert("left".to_string(), left_animation);
        animations.insert("right".to_string(), right_animation);

        let animation_player = AnimationPlayer {
            playing: false,
            animations,
            current_animation: "down".to_string(),
        };

        Self {
            instances: vec![
                Instance {
                    model: Matrix4::from_translation(position).into(),
                    tex_index: 4,
                    atlas_index: 1,
                },
                Instance {
                    model: Matrix4::from_translation(position + Vector3::new(0.0, 1.0, 0.0)).into(),
                    tex_index: 1,
                    atlas_index: 1,
                }],

            position,
            target_position: position,
            movement_timer: Duration::new(0, 0),
            animation_player,
            input_provided: false, // Initialize the input_provided flag
            time_of_input: Instant::now(), // Initialize the time_of_input variable
            last_direction: Vector3::new(0.0,0.0,0.0), // Initialize the last_key variable
            facing_direction: Vector3::new(0.0, -1.0, 0.0), // Initialize the direction variable
            spot_arrival: false,
            running: false,
        }
    }

    pub fn input(&mut self, key: &Option<KeyCode>, input_manager: &mut InputManager, collisions: &Vec<Rectangle>, npcs: &Vec<NPC>) {

        // Check if the player is running
        if input_manager.pressed_keys.contains(&KeyCode::KeyX) {
            self.running = true;
            self.animation_player.set_duration(ANIMATION_DURATION_RUNNING);
        } else {
            self.running = false;
            self.animation_player.set_duration(ANIMATION_DURATION_WALKING);
        }

        let mut direction = Vector3::new(0.0, 0.0, 0.0);
        let mut input_direction = None;

        //get last pressed wasd key

        for &key in input_manager.key_order.iter().rev() {
            match key {
                KeyCode::KeyW => {
                    direction.y += 1.0;
                    break;
                }
                KeyCode::KeyA => {
                    direction.x -= 1.0;
                    break;
                }
                KeyCode::KeyS => {
                    direction.y -= 1.0;
                    break;
                }
                KeyCode::KeyD => {
                    direction.x += 1.0;
                    break;
                }
                _ => {}
            }
        }

        if direction != Vector3::new(0.0, 0.0, 0.0) {
            input_direction = Some(direction);
        }

        // Check if the input key has changed
        if let Some(input_direction) = input_direction {
            self.input_provided = true;
            if input_direction != self.last_direction {
                self.last_direction = input_direction;
                self.time_of_input = Instant::now();
            }
        } else {
            self.input_provided = false;
        }

        // if player target position is the same as the current position, update the direction

        let time_held = Instant::now().duration_since(self.time_of_input);

        if self.target_position == self.position && self.input_provided && self.facing_direction != direction {
            let next_animation = match direction {
                Vector3 { x: 0.0, y: 1.0, z: 0.0 } => "up",
                Vector3 { x: -1.0, y: 0.0, z: 0.0 } => "left",
                Vector3 { x: 0.0, y: -1.0, z: 0.0 } => "down",
                Vector3 { x: 1.0, y: 0.0, z: 0.0 } => "right",
                _ => self.animation_player.current_animation.as_str(),
            };

            self.animation_player.current_animation = next_animation.to_string();
            self.facing_direction = direction;

            if !self.animation_player.playing {
                self.animation_player.start();
            }
        }


        if self.input_provided && time_held > MOVEMENT_THRESHOLD {
            self.set_direction(direction, collisions, npcs);
        }
    }

    pub fn get_instances(&self) -> Vec<Instance> {
        //get animation player instances
        self.animation_player.get_instances().to_vec()
    }


    pub fn update(&mut self, dt: Duration) {
        // Calculate the elapsed time since the last update

        self.spot_arrival = false;

        self.animation_player.update(self.position, dt);

        if self.movement_timer > Duration::new(0, 0) {
            if dt >= self.movement_timer {
                // Player has reached the target position
                self.position = self.target_position;
                self.movement_timer = Duration::new(0, 0);
                self.spot_arrival = true;
            } else {
                // Move the player towards the target position based on the timer
                let t = dt.as_secs_f32() / self.movement_timer.as_secs_f32();
                self.position = self.position * (1.0 - t) + self.target_position * t;
                self.movement_timer -= dt;
            }

        }

        // Always update animation if playing
        // Stop animation if no input is provided and the player has reached the target position
        if !self.input_provided && self.target_position == self.position && self.animation_player.playing{
            self.animation_player.stop();
        }
    }

    pub fn orient(&mut self, direction: Vector3<f32>) {
        self.facing_direction = direction;
        self.last_direction = direction;
        self.animation_player.current_animation = match direction {
            Vector3 { x: 0.0, y: 1.0, z: 0.0 } => "up",
            Vector3 { x: -1.0, y: 0.0, z: 0.0 } => "left",
            Vector3 { x: 0.0, y: -1.0, z: 0.0 } => "down",
            Vector3 { x: 1.0, y: 0.0, z: 0.0 } => "right",
            _ => "down",
        }.to_string();
        self.animation_player.start();
    }

    pub fn set_direction(&mut self, new_direction: Vector3<f32>, collisions: &Vec<Rectangle>, npcs: &Vec<NPC>) {
        if self.movement_timer > Duration::new(0, 0) {
            // Prevent direction changes while the player is moving
            return;
        }

        // Calculate the potential new target position
        let potential_target_position = self.position + new_direction * GRID_SIZE;

        // Ensure the target position aligns with the grid
        let aligned_target_position = Vector3::new(
            potential_target_position.x.round(),
            potential_target_position.y.round(),
            0.0,
        );

        // Calculate the bounding box of the player at the aligned target position
        let player_left = aligned_target_position.x - 0.5;
        let player_right = aligned_target_position.x + 0.5;
        let player_top = aligned_target_position.y + 0.5;
        let player_bottom = aligned_target_position.y - 0.5;

        // Check for collisions at the aligned target position
        let mut collision_detected = false;
        for rect in collisions {
            let rect_left = rect.x - rect.width / 2.0;
            let rect_right = rect.x + rect.width / 2.0;
            let rect_top = rect.y + rect.height / 2.0;
            let rect_bottom = rect.y - rect.height / 2.0;

            if player_left < rect_right && player_right > rect_left &&
                player_top > rect_bottom && player_bottom < rect_top {
                // Collision detected, do not update the target position
                collision_detected = true;
                break;
            }
        }

        for npc in npcs {
            let npc_left = npc.position.x - 0.5;
            let npc_right = npc.position.x + 0.5;
            let npc_top = npc.position.y + 0.5;
            let npc_bottom = npc.position.y - 0.5;

            if player_left < npc_right && player_right > npc_left &&
                player_top > npc_bottom && player_bottom < npc_top {
                // Collision detected, do not update the target position
                collision_detected = true;
                break;
            }

            if aligned_target_position == npc.target_position {
                collision_detected = true;
                break;
            }
        }

        if !collision_detected {
            // Only update direction and target position if no collision detected
            self.target_position = aligned_target_position;
            if self.running {
                self.movement_timer = MOVEMENT_DURATION_RUNNING; // Reset the movement timer
            } else {
                self.movement_timer = MOVEMENT_DURATION_WALKING; // Reset the movement timer
            }
        }


        // Set animation based on the target direction
        self.animation_player.start();
    }


}
