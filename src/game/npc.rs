use crate::renderer::instance::Instance;
use cgmath::{Matrix4, Vector3, InnerSpace};
use crate::renderer::Renderer;
use crate::game::entity::Entity;
use crate::game::animation_player::{Animation, AnimationSheet, AnimationPlayer};
use std::collections::HashMap;
use std::time::Duration;
use crate::game::Interaction;
use crate::game::pokemon::Pokemon;

const ANIMATION_DURATION_WALKING: Duration = Duration::from_millis(125); // Duration to switch animation frames
const MOVEMENT_DURATION_WALKING: Duration = Duration::from_millis(250); // Duration to move from one grid cell to another

pub struct NPC {
    pub id: (String, u32),
    pub animation_player: AnimationPlayer,
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub interaction: Interaction,
    pub los: u32,
    pub movement_timer: Duration,
    pub target_position: Vector3<f32>,
    pub path: Option<Vec<Vector3<f32>>>,
    pub next_point: Vector3<f32>,
}

impl Entity for NPC {
    fn position(&self) -> Vector3<f32> {
        self.position // Assuming `NPC` has a `position` field of type `Vector3<f32>`
    }

    fn instances(&self) -> &[Instance] {
        &self.animation_player.get_instances() // Assuming `Player` has an `instances` field of type `Vec<Instance>`
    }
}

impl NPC {
    pub fn new(id: (String, u32), position: Vector3<f32>, direction: Vector3<f32>, sprite_name: &str, interaction: Interaction, los: u32, mut path: Option<Vec<Vector3<f32>>>, renderer: &mut Renderer) -> Self {

        println!("Creating NPC: {}", sprite_name);

        let (x_idx, y_idx, atlas_name) = match sprite_name {
            "Girl" => (0, 0, "npcs"),
            "Swimmer" => (3, 0, "npcs"),
            "Suit" => (6, 0, "npcs"),
            "Misty" => (0, 8, "npcs"),
            "Chubs" => (3, 8, "npcs"),
            "Nurse" => (6, 8, "npcs"),
            _ => (0, 0, ""),
        };

        let index_offset = match direction {
            Vector3 { x: 0.0, y: -1.0, z: 0.0 } => 0,
            Vector3 { x: 0.0, y: 1.0, z: 0.0 } => 1,
            Vector3 { x: -1.0, y: 0.0, z: 0.0 } => 2,
            Vector3 { x: 1.0, y: 0.0, z: 0.0 } => 3,
            _ => 0,
        };

        println!("x: {}, y: {}, z: {}", direction.x, direction.y, direction.z);
        println!("{}",index_offset);

        let atlas = renderer.get_atlas(atlas_name).unwrap();

        let sheet = AnimationSheet {
            frame_width: 1,
            frame_height: 2,
            frame_order: vec![1, 2, 1, 0],
            frame_duration: ANIMATION_DURATION_WALKING,
            atlas: atlas.clone(),
            looped: true,
        };

        let mut animations = HashMap::new();

        let down_animation = Animation::new(position, &sheet, x_idx, y_idx, 3, 2);
        let up_animation = Animation::new(position, &sheet, x_idx, y_idx + 2, 3, 2);
        let left_animation = Animation::new(position, &sheet, x_idx, y_idx + 4, 3, 2);
        let right_animation = Animation::new(position, &sheet, x_idx, y_idx + 6, 3, 2);

        animations.insert("up".to_string(), up_animation);
        animations.insert("down".to_string(), down_animation);
        animations.insert("left".to_string(), left_animation);
        animations.insert("right".to_string(), right_animation);

        let current_animation = match direction {
            Vector3 { x: 0.0, y: -1.0, z: 0.0 } => "down",
            Vector3 { x: 0.0, y: 1.0, z: 0.0 } => "up",
            Vector3 { x: -1.0, y: 0.0, z: 0.0 } => "left",
            Vector3 { x: 1.0, y: 0.0, z: 0.0 } => "right",
            _ => "down",
        };

        let animation_player = AnimationPlayer {
            playing: true,
            animations,
            current_animation: current_animation.to_string(),
        };

        let movement_timer = Duration::new(0, 0);
        let target_position = position;

        if let Some(ref mut path) = path {
            //move first point to the back
            let first_point = path.remove(0);
            path.push(first_point);
        }

        //only set next point if path is not empty
        //otherwise it will be set to the current position

        let next_point = path.as_ref().map_or(position, |p| p[0]);

        Self {
            id,
            animation_player,
            position,
            direction,
            interaction,
            los,
            movement_timer,
            target_position,
            path,
            next_point,
        }
    }


    pub fn update(&mut self, player_target: Vector3<f32>, dt: Duration) {

        //return early if NPC has no path
        if self.next_point == self.position {
            return
        }

        if self.movement_timer > Duration::new(0, 0) {
            if dt >= self.movement_timer {
                // NPC has reached the target position
                self.position = self.target_position;
                self.movement_timer = Duration::new(0, 0);
            } else {
                // Smoothly interpolate the NPC's position towards the target
                let t = dt.as_secs_f32() / self.movement_timer.as_secs_f32();
                self.position = self.position * (1.0 - t) + self.target_position * t;
                self.movement_timer -= dt;
            }
        }

        if self.position == self.next_point {
            if let Some(ref mut path) = self.path {
                let reached_point = path.remove(0);
                path.push(reached_point);

                self.next_point = path[0];

                let direction_vector = self.next_point - self.position;

                self.direction = direction_vector.normalize();
            }
        }

        let potential_target_position = self.position + self.direction;
        let aligned_target_position = Vector3::new(
            potential_target_position.x.round(),
            potential_target_position.y.round(),
            0.0,
        );

        if self.target_position == self.position {

            if aligned_target_position != player_target {
                self.target_position = self.position + self.direction;
            }

            // Adjust the movement timer based on the grid distance
            self.movement_timer = MOVEMENT_DURATION_WALKING;

            // Set appropriate animation based on direction
            let direction_name = match self.direction {
                Vector3 { x: 0.0, y: -1.0, z: 0.0 } => "down",
                Vector3 { x: 0.0, y: 1.0, z: 0.0 } => "up",
                Vector3 { x: -1.0, y: 0.0, z: 0.0 } => "left",
                Vector3 { x: 1.0, y: 0.0, z: 0.0 } => "right",
                _ => "down", // Default or no movement
            };
            self.animation_player.current_animation = direction_name.to_string();
        }

        self.animation_player.update(self.position, dt);
    }

    pub fn walk_to(&mut self, next: Vector3<f32>) {
        self.next_point = next;
    }

}

pub fn generate_pokemon(name: String, id: u32, renderer: &mut Renderer) -> Vec<Pokemon> {

    let pokemon = match (name.as_str(), id) {
        ("gym", 5) => vec![Pokemon::new("Charizard".to_string(), 5, renderer)],
        ("gym", 4) => vec![Pokemon::new("Blastoise".to_string(), 5, renderer)],
        ("gym", 3) => vec![
            Pokemon::new("Venusaur".to_string(), 1, renderer),
            Pokemon::new("Charizard".to_string(), 1, renderer)
        ],
        _ => vec![Pokemon::new("Charizard".to_string(), 5, renderer)],
    };

    pokemon

}

