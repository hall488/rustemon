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
            "Withdraw" => (Type::Water, 0, 100, 40, AttackType::Status),
            "Thundershock" => (Type::Electric, 40, 100, 30, AttackType::Special),
            "Thunderbolt" => (Type::Electric, 90, 100, 15, AttackType::Special),
            "Quick Attack" => (Type::Normal, 40, 100, 30, AttackType::Physical),
            "Thunder Wave" => (Type::Electric, 0, 90, 20, AttackType::Status),
            "Body Slam" => (Type::Normal, 85, 100, 15, AttackType::Physical),
            "Ice Beam" => (Type::Ice, 90, 100, 10, AttackType::Special),
            "Blizzard" => (Type::Ice, 110, 70, 5, AttackType::Special),
            "Confusion" => (Type::Psychic, 50, 100, 25, AttackType::Special),
            "Psychic" => (Type::Psychic, 90, 100, 10, AttackType::Special),
            "Hyper Beam" => (Type::Normal, 150, 90, 5, AttackType::Special),
            "Fire Spin" => (Type::Fire, 35, 85, 15, AttackType::Special),
            "Seismic Toss" => (Type::Fighting, 0, 100, 20, AttackType::Physical),
            "Double Kick" => (Type::Fighting, 30, 100, 30, AttackType::Physical),
            "Karate Chop" => (Type::Normal, 50, 100, 25, AttackType::Physical),
            "Surf" => (Type::Water, 90, 100, 15, AttackType::Special),
            "Leech Seed" => (Type::Grass, 0, 90, 10, AttackType::Status),
            "Stun Spore" => (Type::Grass, 0, 75, 30, AttackType::Status),
            "Poison Powder" => (Type::Poison, 0, 75, 35, AttackType::Status),
            "Sleep Powder" => (Type::Grass, 0, 75, 15, AttackType::Status),
            "Slam" => (Type::Normal, 80, 75, 20, AttackType::Physical),
            "Mega Punch" => (Type::Normal, 80, 85, 20, AttackType::Physical),
            "Solar Beam" => (Type::Grass, 120, 100, 10, AttackType::Special),
            "Earthquake" => (Type::Ground, 100, 100, 10, AttackType::Physical),
            "Rock Slide" => (Type::Rock, 75, 90, 10, AttackType::Physical),
            "Strength" => (Type::Normal, 80, 100, 15, AttackType::Physical),
            "Double-Edge" => (Type::Normal, 120, 100, 15, AttackType::Physical),
            "Wrap" => (Type::Normal, 15, 90, 20, AttackType::Physical),
            "Dragon Rage" => (Type::Dragon, 0, 100, 10, AttackType::Special),
            "Agility" => (Type::Psychic, 0, 100, 30, AttackType::Status),
            "Bite" => (Type::Normal, 60, 100, 25, AttackType::Physical),
            "Sing" => (Type::Normal, 0, 55, 15, AttackType::Status),
            "Bubble" => (Type::Water, 40, 100, 30, AttackType::Special),
            "Leech Life" => (Type::Bug, 20, 100, 15, AttackType::Physical),
            "Rage" => (Type::Normal, 20, 100, 20, AttackType::Physical),
            "Harden" => (Type::Normal, 0, 100, 30, AttackType::Status),
            "Minimize" => (Type::Normal, 0, 100, 20, AttackType::Status),
            "Flash" => (Type::Normal, 0, 70, 20, AttackType::Status),
            "Thunder" => (Type::Electric, 110, 70, 10, AttackType::Special),

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

