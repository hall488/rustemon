use crate::renderer::instance::Instance;
use cgmath::{Matrix4, Vector3};
use crate::renderer::Renderer;
use crate::game::entity::Entity;

pub struct NPC {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub instances: Vec<Instance>,
    pub interaction: Interaction,
    pub los: u32,
}

pub enum Interaction {
    Heal,
    Battle,
    Talk,
    None,
}

impl Entity for NPC {
    fn position(&self) -> Vector3<f32> {
        self.position // Assuming `NPC` has a `position` field of type `Vector3<f32>`
    }

    fn instances(&self) -> &[Instance] {
        &self.instances // Assuming `NPC` has an `instances` field of type `Vec<Instance>`
    }
}

impl NPC {
    pub fn new(position: Vector3<f32>, direction: Vector3<f32>, sprite_name: &str, _interaction: &str, los: u32, path: Vec<Vector3<f32>>, renderer: &mut Renderer) -> Self {

        println!("Creating NPC: {}", sprite_name);

        let (index, atlas_name) = match sprite_name {
            "Nurse" => (179, "pokecenter"),
            "Girl" => (10, "npcs"),
            "Swimmer" => (13, "npcs"),
            "Suit" => (16, "npcs"),
            "Misty" => (82, "npcs"),
            "Chubs" => (85, "npcs"),
            _ => (0, ""),
        };

        let interaction = match _interaction {
            "Heal" => Interaction::Heal,
            "Battle" => Interaction::Battle,
            "Talk" => Interaction::Talk,
            _ => Interaction::None,
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

        let instances = vec![
            Instance {
                model: Matrix4::from_translation(position).into(),
                tex_index: index + index_offset*atlas.cols*2 as u32,
                atlas_index: atlas.index,
            },
            Instance {
                model: Matrix4::from_translation(position + Vector3::new(0.0, 1.0, 0.0)).into(),
                tex_index: index - atlas.cols as u32 + index_offset*atlas.cols*2 as u32,
                atlas_index: atlas.index,
            }
        ];

        Self {
            position,
            direction,
            instances,
            interaction,
            los,
        }
    }

    pub fn update(&mut self, _delta_time: f32) {
        // Update NPC
    }
}
