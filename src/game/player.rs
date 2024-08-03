use crate::renderer::instance::Instance;
use crate::game::map_loader::Rectangle;
use cgmath::{Vector3, Matrix4};
use std::time::{Duration, Instant};
use winit::keyboard::KeyCode;
use crate::game::pokemon::Pokemon;

const GRID_SIZE: f32 = 1.0; // Define the grid size
const MOVEMENT_DURATION: Duration = Duration::from_millis(250); // Duration to move from one grid cell to another
const MOVEMENT_THRESHOLD: Duration = Duration::from_millis(100); // Threshold to check for movement input

//TODO FIX ANIMATION PLAYER IT CALLS PLAY ANIMATION EVERY FRAME ON COLLISIONS
//I TOOK OUT THE PRINT STATEMENT CUZ IT GOT ANNOYING

pub struct AnimationPlayer {
    pub playing: bool,
    pub frame_id: u32,
    pub current_frame: u32,
    pub max_frame: u32,
    pub frame_duration: Duration,
    pub frame_time_accumulator: Duration,
}

pub struct Player {
    pub instances: Vec<Instance>,
    pub position: Vector3<f32>,
    pub target_position: Vector3<f32>,
    pub movement_timer: Duration, // Timer to track movement duration
    pub animation_player: AnimationPlayer,
    pub input_provided: bool, // Flag to track if input is being provided
    pub time_of_input: Instant, // Time when the input was provided
    pub last_key: Option<KeyCode>, // Last key pressed
    pub facing_direction: Vector3<f32>, // Direction of the player
    pub spot_arrival: bool,
}

impl Player {
    pub fn new() -> Self {
        let position = Vector3::new(16.0, -12.0, 0.0);

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
            animation_player: AnimationPlayer {
                playing: false,
                frame_id: 4,
                current_frame: 0,
                max_frame: 3,
                frame_duration: Duration::from_millis(125),
                frame_time_accumulator: Duration::new(0, 0),
            },
            input_provided: false, // Initialize the input_provided flag
            time_of_input: Instant::now(), // Initialize the time_of_input variable
            last_key: None, // Initialize the last_key variable
            facing_direction: Vector3::new(0.0, -1.0, 0.0), // Initialize the direction variable
            spot_arrival: false,
        }
    }

    pub fn input(&mut self, key: &Option<KeyCode>, collisions: &Vec<Rectangle>) {
        let mut direction = Vector3::new(0.0, 0.0, 0.0);
        let mut input_key = None;
        self.input_provided = match key {
            Some(key) => {
                input_key = Some(*key);
                match key {
                    KeyCode::KeyW => {
                        direction.y += 1.0;
                        true
                    }
                    KeyCode::KeyA => {
                        direction.x -= 1.0;
                        true
                    }
                    KeyCode::KeyS => {
                        direction.y -= 1.0;
                        true
                    }
                    KeyCode::KeyD => {
                        direction.x += 1.0;
                        true
                    }
                    _ => false,
                }
            }
            None => false,
        };

        // Check if the input key has changed
        if input_key != self.last_key {
            self.last_key = input_key;
            self.time_of_input = Instant::now();
        }

        // if player target position is the same as the current position, update the direction

        let time_held = Instant::now().duration_since(self.time_of_input);

        if self.target_position == self.position && self.input_provided && self.facing_direction != direction {
            let new_frame_id = match direction {
                Vector3 { x: 0.0, y: 1.0, z: 0.0 } => 10,
                Vector3 { x: -1.0, y: 0.0, z: 0.0 } => 16,
                Vector3 { x: 0.0, y: -1.0, z: 0.0 } => 4,
                Vector3 { x: 1.0, y: 0.0, z: 0.0 } => 22,
                _ => self.animation_player.frame_id,
            };

            self.facing_direction = direction;
            self.animation_player.frame_id = new_frame_id;
            self.animation_player.current_frame = 0; // Reset to first frame of the new direction
            self.instances[0].tex_index = self.animation_player.frame_id;
            self.instances[1].tex_index = self.animation_player.frame_id - 3;
        }


        if self.input_provided && time_held > MOVEMENT_THRESHOLD {
            self.set_direction(direction, collisions);
        }
    }

    pub fn update(&mut self, dt: Duration) {
        // Calculate the elapsed time since the last update

        self.spot_arrival = false;

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

            // Update the instance model matrix
            self.instances[0].model = Matrix4::from_translation(self.position).into();
            self.instances[1].model = Matrix4::from_translation(self.position + Vector3::new(0.0, 1.0, 0.0)).into();
        }

        // Always update animation if playing
        if self.animation_player.playing {
            self.animation_player.frame_time_accumulator += dt;
            if self.animation_player.frame_time_accumulator >= self.animation_player.frame_duration {
                self.animation_player.current_frame += 1;
                self.animation_player.frame_time_accumulator = Duration::new(0, 0);
                if self.animation_player.current_frame > self.animation_player.max_frame {
                    self.animation_player.current_frame = 0;
                }
                match self.animation_player.current_frame {
                    0 => {
                        self.instances[0].tex_index = self.animation_player.frame_id;
                        self.instances[1].tex_index = self.animation_player.frame_id - 3;
                    }
                    1 => {
                        self.instances[0].tex_index = self.animation_player.frame_id + 1;
                        self.instances[1].tex_index = self.animation_player.frame_id - 2;
                    }
                    2 => {
                        self.instances[0].tex_index = self.animation_player.frame_id;
                        self.instances[1].tex_index = self.animation_player.frame_id - 3;
                    }
                    3 => {
                        self.instances[0].tex_index = self.animation_player.frame_id - 1;
                        self.instances[1].tex_index = self.animation_player.frame_id - 4;
                    }
                    _ => {}
                }
            }
        }

        // Stop animation if no input is provided and the player has reached the target position
        if !self.input_provided && self.target_position == self.position && self.animation_player.playing{
            self.stop_animation();
        }
    }

    pub fn set_direction(&mut self, new_direction: Vector3<f32>, collisions: &Vec<Rectangle>) {
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

        if !collision_detected {
            // Only update direction and target position if no collision detected
            self.target_position = aligned_target_position;
            self.movement_timer = MOVEMENT_DURATION; // Reset the movement timer
        }


        // Set animation based on the target direction
        self.start_animation(new_direction);
    }

    fn start_animation(&mut self, new_direction: Vector3<f32>) {
        self.animation_player.playing = true;

        let new_frame_id = match new_direction {
            Vector3 { x: 0.0, y: 1.0, z: 0.0 } => 10,
            Vector3 { x: -1.0, y: 0.0, z: 0.0 } => 16,
            Vector3 { x: 0.0, y: -1.0, z: 0.0 } => 4,
            Vector3 { x: 1.0, y: 0.0, z: 0.0 } => 22,
            _ => self.animation_player.frame_id,
        };

        if new_frame_id != self.animation_player.frame_id {
            self.animation_player.frame_id = new_frame_id;
            self.animation_player.current_frame = 0; // Reset to first frame of the new direction
            self.animation_player.frame_time_accumulator = Duration::new(0, 0);
            self.instances[0].tex_index = self.animation_player.frame_id;
            self.instances[1].tex_index = self.animation_player.frame_id - 3;

        }
    }

    fn stop_animation(&mut self) {
        self.animation_player.playing = false;
        self.animation_player.current_frame = 0;
        self.instances[0].tex_index = self.animation_player.frame_id;
        self.instances[1].tex_index = self.animation_player.frame_id - 3;
    }
}
