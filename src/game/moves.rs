use std::collections::HashMap;

#[derive(Clone)]
pub struct Move {
    pub name: String,
    pub power: u32,
    pub accuracy: u32,
    pub pp: u32,
}

impl Move {
    pub fn new(name: &str, power: u32, accuracy: u32, pp: u32) -> Self {
        Self {
            name: name.to_string(),
            power,
            accuracy,
            pp,
        }
    }
}

pub fn get_moves() -> HashMap<&'static str, Move> {
    let mut moves = HashMap::new();

    moves.insert("Tackle", Move::new("Tackle", 40, 100, 35));
    moves.insert("Growl", Move::new("Growl", 0, 100, 40));
    moves.insert("Vine Whip", Move::new("Vine Whip", 45, 100, 25));
    moves.insert("Razor Leaf", Move::new("Razor Leaf", 55, 95, 25));
    moves.insert("Scratch", Move::new("Scratch", 40, 100, 35));
    moves.insert("Ember", Move::new("Ember", 40, 100, 25));
    moves.insert("Flamethrower", Move::new("Flamethrower", 90, 100, 15));
    moves.insert("Tail Whip", Move::new("Tail Whip", 0, 100, 40));
    moves.insert("Water Gun", Move::new("Water Gun", 40, 100, 25));
    moves.insert("Hydro Pump", Move::new("Hydro Pump", 110, 80, 5));

    moves
}
