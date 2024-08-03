use crate::renderer::instance::Instance;
use super::moves::{get_moves, Move};

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub level: u32,
    pub current_hp: u32,
    pub stats: Stats,
    pub front_instances: Vec<Instance>,
    pub back_instances: Vec<Instance>,
    pub id: u32,
    pub moves: Vec<Move>,
}

impl Pokemon {
    pub fn new(name: String, level: u32) -> Self {

        let id = match name.as_str() {
            "Bulbasaur" => 1,
            "Ivysaur" => 2,
            "Venusaur" => 3,
            "Charmander" => 4,
            "Charmeleon" => 5,
            "Charizard" => 6,
            "Squirtle" => 7,
            "Wartortle" => 8,
            "Blastoise" => 9,
            _ => 0,
        };

        let stats = match name.as_str() {
            "Bulbasaur" => Stats::new(45, 49, 49, 45, 65, 65),
            "Ivysaur" => Stats::new(60, 62, 63, 60, 80, 80),
            "Venusaur" => Stats::new(80, 82, 83, 80, 100, 100),
            "Charmander" => Stats::new(39, 52, 43, 65, 60, 50),
            "Charmeleon" => Stats::new(58, 64, 58, 80, 80, 65),
            "Charizard" => Stats::new(78, 84, 78, 100, 109, 85),
            "Squirtle" => Stats::new(44, 48, 65, 43, 50, 64),
            "Wartortle" => Stats::new(59, 63, 80, 58, 65, 80),
            "Blastoise" => Stats::new(79, 83, 100, 78, 85, 105),
            _ => Stats::new(0, 0, 0, 0, 0, 0),
        };

        let sprite_coords = match name.as_str() {
            "Bulbasaur" => (0, 0, 1, 1),
            "Ivysaur" => (2, 0, 3, 1),
            "Venusaur" => (4, 0, 5, 1),
            "Charmander" => (6, 0, 7, 1),
            "Charmeleon" => (8, 0, 9, 1),
            "Charizard" => (10, 0, 11, 1),
            "Squirtle" => (12, 0, 13, 1),
            "Wartortle" => (14, 0, 15, 1),
            "Blastoise" => (16, 0, 17, 1),
            _ => (0, 0, 0, 0),
        };

        let moves_dict = get_moves();
        let mut moves = Vec::new();

        match name.as_str() {
            "Bulbasaur" => {
                moves.push(moves_dict["Tackle"].clone());
                moves.push(moves_dict["Growl"].clone());
            },
            "Ivysaur" => {
                moves.push(moves_dict["Tackle"].clone());
                moves.push(moves_dict["Growl"].clone());
                moves.push(moves_dict["Vine Whip"].clone());
            },
            "Venusaur" => {
                moves.push(moves_dict["Tackle"].clone());
                moves.push(moves_dict["Growl"].clone());
                moves.push(moves_dict["Vine Whip"].clone());
                moves.push(moves_dict["Razor Leaf"].clone());
            },
            "Charmander" => {
                moves.push(moves_dict["Scratch"].clone());
                moves.push(moves_dict["Growl"].clone());
            },
            "Charmeleon" => {
                moves.push(moves_dict["Scratch"].clone());
                moves.push(moves_dict["Growl"].clone());
                moves.push(moves_dict["Ember"].clone());
            },
            "Charizard" => {
                moves.push(moves_dict["Scratch"].clone());
                moves.push(moves_dict["Growl"].clone());
                moves.push(moves_dict["Ember"].clone());
                moves.push(moves_dict["Flamethrower"].clone());
            },
            "Squirtle" => {
                moves.push(moves_dict["Tackle"].clone());
                moves.push(moves_dict["Tail Whip"].clone());
            },
            "Wartortle" => {
                moves.push(moves_dict["Tackle"].clone());
                moves.push(moves_dict["Tail Whip"].clone());
                moves.push(moves_dict["Water Gun"].clone());
            },
            "Blastoise" => {
                moves.push(moves_dict["Tackle"].clone());
                moves.push(moves_dict["Tail Whip"].clone());
                moves.push(moves_dict["Water Gun"].clone());
                moves.push(moves_dict["Hydro Pump"].clone());
            },
            _ => (),
        }

        let x = 3 as f32 * 2.0/15.0 - 1.0 + 1.0/15.0;
        let y = 4 as f32 * 2.0/10.0;

        let scale = cgmath::Matrix4::from_nonuniform_scale(4.0/15.0, 4.0/10.0, 1.0);
        let translation0 = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x, 1.0 - y, 0.0));
        let translation1 = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x + 4.0/15.0, 1.0 - y, 0.0));
        let translation2 = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x, 1.0 - y - 4.0/10.0, 0.0));
        let translation3 = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x + 4.0/15.0, 1.0 - y - 4.0/10.0, 0.0));

        let back_instance0 = Instance {
            model: (translation0*scale).into(),
            tex_index: sprite_coords.0 as u32 + sprite_coords.1 as u32 * 32,
            atlas_index: 4,
        };

        let back_instance1 = Instance {
            model: (translation1*scale).into(),
            tex_index: sprite_coords.2 as u32 + sprite_coords.1 as u32 * 32,
            atlas_index: 4,
        };

        let back_instance2 = Instance {
            model: (translation2*scale).into(),
            tex_index: sprite_coords.0 as u32 + sprite_coords.3 as u32 * 32,
            atlas_index: 4,
        };

        let back_instance3 = Instance {
            model: (translation3*scale).into(),
            tex_index: sprite_coords.2 as u32 + sprite_coords.3 as u32 * 32,
            atlas_index: 4,
        };

        let back_instances = vec![back_instance0, back_instance1, back_instance2, back_instance3];

        let x = 9.5 as f32 * 2.0/15.0 - 1.0 + 1.0/15.0;
        let y = 1.5 as f32 * 2.0/10.0;

        let translation0 = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x, 1.0 - y, 0.0));
        let translation1 = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x + 4.0/15.0, 1.0 - y, 0.0));
        let translation2 = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x, 1.0 - y - 4.0/10.0, 0.0));
        let translation3 = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x + 4.0/15.0, 1.0 - y - 4.0/10.0, 0.0));

        let front_instance0 = Instance {
            model: (translation0*scale).into(),
            tex_index: sprite_coords.0 as u32 + sprite_coords.1 as u32 * 32,
            atlas_index: 5,
        };

        let front_instance1 = Instance {
            model: (translation1*scale).into(),
            tex_index: sprite_coords.2 as u32 + sprite_coords.1 as u32 * 32,
            atlas_index: 5,
        };

        let front_instance2 = Instance {
            model: (translation2*scale).into(),
            tex_index: sprite_coords.0 as u32 + sprite_coords.3 as u32 * 32,
            atlas_index: 5,
        };

        let front_instance3 = Instance {
            model: (translation3*scale).into(),
            tex_index: sprite_coords.2 as u32 + sprite_coords.3 as u32 * 32,
            atlas_index: 5,
        };

        let front_instances = vec![front_instance0, front_instance1, front_instance2, front_instance3];

        Self {
            name,
            level,
            current_hp: stats.hp,
            stats,
            back_instances,
            front_instances,
            id,
            moves,
        }
    }
}

#[derive(Clone)]
pub struct Stats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub special: u32,
    pub special_defense: u32,
}

impl Stats {
    pub fn new(hp: u32, attack: u32, defense: u32, speed: u32, special: u32, special_defense: u32) -> Self {
        Self {
            hp,
            attack,
            defense,
            speed,
            special,
            special_defense,
        }
    }
}
