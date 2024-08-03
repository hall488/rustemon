use crate::renderer::instance::Instance;
use cgmath::Vector3;
use tiled::PropertyValue::IntValue;

pub struct Stationary {
    pub background: Vec<Instance>,
    pub ui: Vec<Instance>,
}

impl Stationary {
    pub fn new(map: &tiled::Map, atlas_index: u32) -> Self {
        let mut background = Vec::new();
        let mut ui = Vec::new();

        for layer in map.layers() {
            if let tiled::LayerType::Tiles(tile_layer) = layer.layer_type() {
                println!("Layer name: {}", layer.name);

                match layer.name.as_str() {
                    "Background" => {
                        Self::push_instances(&mut background, &tile_layer, atlas_index);
                    }
                    "UI" => {
                        Self::push_instances(&mut ui, &tile_layer, atlas_index);
                    }
                    _ => {}
                }
            }

        }

        Self {
            background,
            ui,
        }
    }

    fn push_instances(instances: &mut Vec<Instance>, tile_layer: &tiled::TileLayer, atlas_index: u32) {
        for i in 0..tile_layer.width().unwrap() {
            for j in 0..tile_layer.height().unwrap() {
                if let Some(tile) = tile_layer.get_tile(i as i32, j as i32) {
                    let scale = cgmath::Matrix4::from_nonuniform_scale(2.0/15.0, 2.0/10.0, 1.0);
                    let x = i as f32 * 2.0/15.0 - 1.0 + 1.0/15.0;
                    let y = j as f32 * 2.0/10.0 + 1.0/10.0;
                    let translation = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x, 1.0 - y, 0.0));
                    let model = translation * scale;
                    instances.push(Instance {
                        model: model.into(),
                        tex_index: tile.id() as u32,
                        atlas_index,
                    });
                }
            }
        }
    }

}
