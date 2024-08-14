use crate::renderer::instance::Instance;
use super::moves::{get_moves, Move};
use crate::renderer::sprite::Sprite;
use crate::renderer::Renderer;

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub level: u32,
    pub current_hp: u32,
    pub stats: Stats,
    pub back_sprite: Sprite,
    pub front_sprite: Sprite,
    pub id: u32,
    pub moves: Vec<Move>,
}

impl Pokemon {
    pub fn new(name: String, level: u32, renderer: &mut Renderer) -> Self {

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

        //make sprite coords a function of id
        let sprite_coords = ((id - 1) % 16 * 2, (id - 1) / 16 * 2);

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

        let x = 2.5 * 16.0;
        let y = 3.0 * 16.0;
        let back_sprite = renderer.create_sprite(x, y, sprite_coords.0, sprite_coords.1, 2, 2, "pokemon_back", 1.0, 1.0).expect("");

        let x = 9.0 * 16.0;
        let y = 0.5 * 16.0;
        let front_sprite = renderer.create_sprite(x, y, sprite_coords.0, sprite_coords.1, 2, 2, "pokemon_front", 1.0, 1.0).expect("");

        Self {
            name,
            level,
            current_hp: stats.hp,
            stats,
            back_sprite,
            front_sprite,
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
