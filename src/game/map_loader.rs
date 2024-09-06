use crate::renderer::instance::Instance;
use cgmath::Vector3;
use tiled::PropertyValue::IntValue;
use tiled;

#[derive(Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

#[derive(Clone)]
pub struct Spawn {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub direction: Vector3<f32>,
    pub location: u32,
}

#[derive(Clone)]
pub struct Npc {
    pub name: String,
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub direction: Vector3<f32>,
    pub interaction: String,
    pub los: u32,
    pub path_id: Option<u32>,
}

#[derive(Clone)]
pub struct Interaction {
    pub name: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Door {
    pub rectangle: Rectangle,
    pub name: String,
    pub location: u32,
}

#[derive(Clone)]
pub struct Grass {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Animated {
    pub x: f32,
    pub y: f32,
    pub frames: Vec<u32>,
}

#[derive(Clone)]
pub struct Path {
    pub id: u32,
    pub points: Vec<Vector3<f32>>,
    pub direction: String,
}

pub struct Map {
    pub background: Vec<Instance>,
    pub ground: Vec<Instance>,
    pub foreground: Vec<Instance>,
    pub aboveground: Vec<Instance>,
    pub collisions: Vec<Rectangle>,
    pub doors: Vec<Door>,
    pub spawns: Vec<Spawn>,
    pub grasses: Vec<Grass>,
    pub npcs: Vec<Npc>,
    pub interactions: Vec<Interaction>,
    pub animated: Vec<Animated>,
    pub paths: Vec<Path>,
}

impl Map {
    pub fn new(map: &tiled::Map, atlas_index: u32) -> Self {
        let mut background = Vec::new();
        let mut ground = Vec::new();
        let mut foreground = Vec::new();
        let mut aboveground = Vec::new();
        let mut collisions = Vec::new();
        let mut doors = Vec::new();
        let mut spawns = Vec::new();
        let mut grasses = Vec::new();
        let mut npcs = Vec::new();
        let mut interactions = Vec::new();
        let mut animated = Vec::new();
        let mut paths = Vec::new();

        println!("Atlas index for new map is: {}", atlas_index);

        for layer in map.layers() {
            if let tiled::LayerType::Tiles(tile_layer) = layer.layer_type() {
                println!("Layer name: {}", layer.name);

                match layer.name.as_str() {
                    "Background" => {
                        Self::push_instances(&mut background, &tile_layer, atlas_index);
                    }
                    "Ground" => {
                        Self::push_instances(&mut ground, &tile_layer, atlas_index);
                    }
                    "Foreground" => {
                        Self::push_instances(&mut foreground, &tile_layer, atlas_index);
                    }
                    "Aboveground" => {
                        Self::push_instances(&mut aboveground, &tile_layer, atlas_index);
                    }
                    "Collision" => {
                        Self::push_collisions(&mut collisions, &tile_layer);
                    }
                    _ => {}
                }
            }

            if let tiled::LayerType::Objects(object_layer) = layer.layer_type() {
                match layer.name.as_str() {
                    "Spawns" => {
                        Self::push_spawns(&mut spawns, &object_layer);
                    }
                    "Doors" => {
                        Self::push_doors(&mut doors, &object_layer);
                    }
                    "Grasses" => {
                        Self::push_grasses(&mut grasses, &object_layer);
                    }
                    "Npcs" => {
                        Self::push_npcs(&mut npcs, &object_layer);
                    }
                    "Interactions" => {
                        Self::push_interactions(&mut interactions, &object_layer);
                    }
                    "Animated" => {
                        Self::push_animated(&mut animated, &object_layer);
                    }
                    "Paths" => {
                        Self::push_paths(&mut paths, &object_layer);
                    }
                    _ => {}
                }

            }

        }

        Self {
            background,
            ground,
            foreground,
            aboveground,
            collisions,
            doors,
            spawns,
            grasses,
            npcs,
            interactions,
            animated,
            paths,
        }
    }

    fn push_instances(instances: &mut Vec<Instance>, tile_layer: &tiled::TileLayer, atlas_index: u32) {
        for i in 0..tile_layer.width().unwrap() {
            for j in 0..tile_layer.height().unwrap() {
                if let Some(tile) = tile_layer.get_tile(i as i32, j as i32) {
                    instances.push(Instance {
                        model: cgmath::Matrix4::from_translation(cgmath::Vector3::new(i as f32, -1.0 * j as f32, 0.0)).into(),
                        tex_index: tile.id() as u32,
                        atlas_index,
                    });
                }
            }
        }
    }

    fn push_collisions(collisions: &mut Vec<Rectangle>, tile_layer: &tiled::TileLayer) {
        for i in 0..tile_layer.width().unwrap() {
            for j in 0..tile_layer.height().unwrap() {
                if let Some(_tile) = tile_layer.get_tile(i as i32, j as i32) {
                    collisions.push(Rectangle::new(i as f32, -1.0 * j as f32, 1.0, 1.0));
                }
            }
        }
    }

    fn push_grasses(grasses: &mut Vec<Grass>, object_layer: &tiled::ObjectLayer) {
        for object in object_layer.objects() {
            let x = object.x as f32 / 16.0;
            let y = -1.0 * object.y as f32 / 16.0 + 1.0;
            grasses.push(Grass { x, y });
        }
    }

    fn push_spawns(spawns: &mut Vec<Spawn>, object_layer: &tiled::ObjectLayer) {
        for object in object_layer.objects() {

            let name = object.name.clone();
            let direction = match object.properties.get("direction") {
                Some(tiled::PropertyValue::IntValue(val)) => match val {
                    0 => Vector3::new(0.0, 1.0, 0.0),
                    1 => Vector3::new(1.0, 0.0, 0.0),
                    2 => Vector3::new(0.0, -1.0, 0.0),
                    3 => Vector3::new(-1.0, 0.0, 0.0),
                    _ => Vector3::new(0.0, 1.0, 0.0),
                },
                _ => Vector3::new(0.0, 1.0, 0.0),
            };

            let x = object.x as f32 / 16.0;
            let y = -1.0 * object.y as f32 / 16.0 + 1.0;

            let location = match object.properties.get("location") {
                Some(tiled::PropertyValue::IntValue(val)) => *val,
                _ => 0, // Default direction
            } as u32;

            spawns.push(Spawn {name, x, y, direction, location});
        }
    }

    fn push_paths(paths: &mut Vec<Path>, object_layer: &tiled::ObjectLayer) {
        for object in object_layer.objects() {

            let id = object.id();
            let mut points = Vec::new();
            let direction = match object.properties.get("direction") {
                Some(tiled::PropertyValue::StringValue(val)) => val.clone(),
                _ => "".to_string(),
            };

            //get world map points
            if let tiled::ObjectShape::Polygon { points: polygon_points } = &object.shape {
                for point in polygon_points {
                    points.push(Vector3::new((point.0 + object.x) as f32 / 16.0, -1.0 * (point.1 + object.y) as f32 / 16.0 + 1.0, 0.0));
                }

            }

            //print points
            for point in &points {
                println!("path id: {},  x: {}, y: {}", id, point.x, point.y);
            }


            paths.push(Path {id, points, direction});
        }
    }

    fn push_npcs(npcs: &mut Vec<Npc>, object_layer: &tiled::ObjectLayer) {
        for object in object_layer.objects() {
            let name = object.name.clone();
            let x = object.x as f32 / 16.0;
            let y = -1.0 * object.y as f32 / 16.0 + 1.0;

            let direction = match object.properties.get("direction") {
                Some(tiled::PropertyValue::IntValue(val)) => match val {
                    0 => Vector3::new(0.0, 1.0, 0.0),
                    1 => Vector3::new(1.0, 0.0, 0.0),
                    2 => Vector3::new(0.0, -1.0, 0.0),
                    3 => Vector3::new(-1.0, 0.0, 0.0),
                    _ => Vector3::new(0.0, 1.0, 0.0),
                },
                _ => Vector3::new(0.0, 1.0, 0.0),
            };

            let interaction = match object.properties.get("interaction") {
                Some(tiled::PropertyValue::StringValue(val)) => val.clone(),
                _ => "".to_string(),
            };

            let los = match object.properties.get("los") {
                Some(tiled::PropertyValue::IntValue(val)) => *val as u32,
                _ => 0,
            };

            let path_id = match object.properties.get("path") {
                Some(tiled::PropertyValue::ObjectValue(val)) => Some(val.clone()),
                _ => None,
            };

            let id = object.id();

            //get path from path_id

            npcs.push(Npc {name, id, x, y, direction, interaction, los, path_id});
        }
    }

    fn push_interactions(interactions: &mut Vec<Interaction>, object_layer: &tiled::ObjectLayer) {
        for object in object_layer.objects() {
            let name = object.name.clone();
            let x = object.x as f32 / 16.0;
            let y = -1.0 * object.y as f32 / 16.0 + 1.0;
            interactions.push(Interaction {name, x, y});
        }
    }

    fn push_doors(doors: &mut Vec<Door>, object_layer: &tiled::ObjectLayer) {
        for object in object_layer.objects() {
            let name = object.name.clone();
            let x = object.x as f32 / 16.0;
            let y = -1.0 * object.y as f32 / 16.0 + 1.0;
            let rectangle = Rectangle::new(
                x,
                y,
                1.0 as f32,
                1.0 as f32,
            );

            let location = match object.properties.get("location") {
                Some(tiled::PropertyValue::IntValue(val)) => *val,
                _ => 0, // Default direction
            } as u32;

            doors.push(Door { rectangle, name, location });
        }
    }

    fn push_animated(animated: &mut Vec<Animated>, object_layer: &tiled::ObjectLayer) {
        for object in object_layer.objects() {
            let x = object.x as f32 / 16.0;
            let y = -1.0 * object.y as f32 / 16.0 + 1.0;

            let mut frames = Vec::new();
            for i in 0..4 {
                frames.push(object.tile_data().expect("").id() + i);
            }

            animated.push(Animated { x, y, frames });
        }
    }
}
