#[derive(Clone)]
pub enum AttackType {
    Physical,
    Special,
    Status,
}

#[derive(Clone)]
pub enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
}

#[derive(Clone)]
pub struct Move {
    pub name: String,
    pub move_type: Type,
    pub power: u32,
    pub accuracy: u32,
    pub pp: u32,
    pub attack_type: AttackType,
}

impl Move {
    pub fn new(name: &str) -> Self {
        let (move_type, power, accuracy, pp, attack_type) = match name {
            "Tackle" => (Type::Normal, 40, 100, 35, AttackType::Physical),
            "Growl" => (Type::Normal, 0, 100, 40, AttackType::Status),
            "Vine Whip" => (Type::Grass, 45, 100, 25, AttackType::Physical),
            "Razor Leaf" => (Type::Grass, 55, 95, 25, AttackType::Physical),
            "Scratch" => (Type::Normal, 40, 100, 35, AttackType::Physical),
            "Ember" => (Type::Fire, 40, 100, 25, AttackType::Special),
            "Flamethrower" => (Type::Fire, 90, 100, 15, AttackType::Special),
            "Tail Whip" => (Type::Normal, 0, 100, 40, AttackType::Status),
            "Water Gun" => (Type::Water, 40, 100, 25, AttackType::Special),
            "Hydro Pump" => (Type::Water, 110, 80, 5, AttackType::Special),
            _ => (Type::Normal, 0, 0, 0, AttackType::Status), // Default case for unknown moves
        };

        Self {
            name: name.to_string(),
            move_type,
            power,
            accuracy,
            pp,
            attack_type,
        }
    }
}
