use crate::renderer::instance::Instance;
use cgmath::{Matrix4, Vector3};
use crate::renderer::Renderer;

pub struct NPC {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub instances: Vec<Instance>,
}

impl NPC {
    pub fn new(position: Vector3<f32>, direction: Vector3<f32>, sprite_name: &str, renderer: &mut Renderer) -> Self {

        let (index, atlas_name) = match sprite_name {
            "Nurse" => (164, "pokecenter"),
            _ => (0, ""),
        };

        let atlas = renderer.get_atlas(atlas_name).unwrap();

        let instances = vec![
            Instance {
                model: Matrix4::from_translation(position).into(),
                tex_index: index + atlas.cols as u32,
                atlas_index: atlas.index,
            },
            Instance {
                model: Matrix4::from_translation(position + Vector3::new(0.0, 1.0, 0.0)).into(),
                tex_index: index,
                atlas_index: atlas.index,
            }
        ];

        Self {
            position,
            direction,
            instances,
        }
    }
}
