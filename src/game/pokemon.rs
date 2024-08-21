use crate::renderer::instance::Instance;
use super::moves::Move;
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
    pub catch_rate: u32,
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


        let catch_rate = match name.as_str() {
            "Bulbasaur" => 45,
            "Ivysaur" => 45,
            "Venusaur" => 45,
            "Charmander" => 45,
            "Charmeleon" => 45,
            "Charizard" => 45,
            "Squirtle" => 45,
            "Wartortle" => 45,
            "Blastoise" => 45,
            _ => 0, // Default to 0 if the Pokémon's name doesn't match any known Pokémon
        };

        //make sprite coords a function of id
        let sprite_coords = ((id - 1) % 16 * 2, (id - 1) / 16 * 2);

        let mut moves = Vec::new();

        match name.as_str() {
            "Bulbasaur" => {
                moves.push(Move::new("Tackle"));
                moves.push(Move::new("Growl"));
            },
            "Ivysaur" => {
                moves.push(Move::new("Tackle"));
                moves.push(Move::new("Growl"));
                moves.push(Move::new("Vine Whip"));
            },
            "Venusaur" => {
                moves.push(Move::new("Tackle"));
                moves.push(Move::new("Growl"));
                moves.push(Move::new("Vine Whip"));
                moves.push(Move::new("Razor Leaf"));
            },
            "Charmander" => {
                moves.push(Move::new("Scratch"));
                moves.push(Move::new("Growl"));
            },
            "Charmeleon" => {
                moves.push(Move::new("Scratch"));
                moves.push(Move::new("Growl"));
                moves.push(Move::new("Ember"));
            },
            "Charizard" => {
                moves.push(Move::new("Scratch"));
                moves.push(Move::new("Growl"));
                moves.push(Move::new("Ember"));
                moves.push(Move::new("Flamethrower"));
            },
            "Squirtle" => {
                moves.push(Move::new("Tackle"));
                moves.push(Move::new("Tail Whip"));
            },
            "Wartortle" => {
                moves.push(Move::new("Tackle"));
                moves.push(Move::new("Tail Whip"));
                moves.push(Move::new("Water Gun"));
            },
            "Blastoise" => {
                moves.push(Move::new("Tackle"));
                moves.push(Move::new("Tail Whip"));
                moves.push(Move::new("Water Gun"));
                moves.push(Move::new("Hydro Pump"));
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
            catch_rate,
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
