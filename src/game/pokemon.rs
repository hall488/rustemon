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
    pub base_stats: Stats,  // Added to store the base stats for calculation
    pub back_sprite: Sprite,
    pub front_sprite: Sprite,
    pub id: u32,
    pub moves: Vec<Move>,
    pub catch_rate: u32,
    pub experience: u32,
    pub experience_threshold: u32,
    pub experience_yield: u32,
}

impl Pokemon {
     pub fn new(name: String, level: u32, renderer: &mut Renderer) -> Self {
        let (id, base_stats, catch_rate, experience_yield, move_names) = match name.as_str() {
            "Bulbasaur" => (
                1,
                Stats::new(45, 49, 49, 45, 65, 65),
                45,
                64,
                vec!["Tackle", "Growl"]
            ),
            "Ivysaur" => (
                2,
                Stats::new(60, 62, 63, 60, 80, 80),
                45,
                142,
                vec!["Tackle", "Growl", "Vine Whip"]
            ),
            "Venusaur" => (
                3,
                Stats::new(80, 82, 83, 80, 100, 100),
                45,
                236,
                vec!["Tackle", "Growl", "Vine Whip", "Razor Leaf"]
            ),
            "Charmander" => (
                4,
                Stats::new(39, 52, 43, 65, 60, 50),
                45,
                62,
                vec!["Scratch", "Growl"]
            ),
            "Charmeleon" => (
                5,
                Stats::new(58, 64, 58, 80, 80, 65),
                45,
                142,
                vec!["Scratch", "Growl", "Ember"]
            ),
            "Charizard" => (
                6,
                Stats::new(78, 84, 78, 100, 109, 85),
                45,
                240,
                vec!["Scratch", "Growl", "Ember", "Flamethrower"]
            ),
            "Squirtle" => (
                7,
                Stats::new(44, 48, 65, 43, 50, 64),
                45,
                63,
                vec!["Tackle", "Tail Whip"]
            ),
            "Wartortle" => (
                8,
                Stats::new(59, 63, 80, 58, 65, 80),
                45,
                142,
                vec!["Tackle", "Tail Whip", "Water Gun"]
            ),
            "Blastoise" => (
                9,
                Stats::new(79, 83, 100, 78, 85, 105),
                45,
                239,
                vec!["Tackle", "Tail Whip", "Water Gun", "Hydro Pump"]
            ),
            _ => (
                0,
                Stats::new(0, 0, 0, 0, 0, 0),
                0,
                0,
                vec![]
            ),
        };

        // Calculate stats based on base stats and level
        let stats = Stats::calculate(&base_stats, level);

        // Create moves vector from move names
        let moves: Vec<Move> = move_names.into_iter().map(Move::new).collect();

        let sprite_coords = ((id - 1) % 16 * 2, (id - 1) / 16 * 2);

        let x = 2.5 * 16.0;
        let y = 3.0 * 16.0;
        let back_sprite = renderer
            .create_sprite(x, y, sprite_coords.0, sprite_coords.1, 2, 2, "pokemon_back", 1.0, 1.0)
            .expect("");

        let x = 9.0 * 16.0;
        let y = 0.5 * 16.0;
        let front_sprite = renderer
            .create_sprite(x, y, sprite_coords.0, sprite_coords.1, 2, 2, "pokemon_front", 1.0, 1.0)
            .expect("");

        let experience_threshold = level.pow(3);

        Self {
            name,
            level,
            current_hp: stats.hp,
            stats,
            base_stats,  // Store base stats for future calculations
            back_sprite,
            front_sprite,
            id,
            moves,
            catch_rate,
            experience: 0,
            experience_threshold,
            experience_yield
        }
    }

    pub fn gain_experience(&mut self, experience: u32) {
        self.experience += experience;

        if self.experience >= self.experience_threshold {
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.experience = self.experience - self.experience_threshold;
        self.experience_threshold = self.level.pow(3);

        // Store the old HP before recalculating stats
        let old_hp = self.stats.hp;

        // Recalculate stats based on the new level
        self.stats = Stats::calculate(&self.base_stats, self.level);

        // Calculate HP gain and add to current HP
        let hp_gain = self.stats.hp - old_hp;
        self.current_hp += hp_gain;
    }

}

#[derive(Clone)]
pub struct Stats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub special_attack: u32,
    pub special_defense: u32,
}

impl Stats {
    pub fn new(hp: u32, attack: u32, defense: u32, speed: u32, special_attack: u32, special_defense: u32) -> Self {
        Self {
            hp,
            attack,
            defense,
            speed,
            special_attack,
            special_defense,
        }
    }

    // Calculate stats based on base stats and level
    pub fn calculate(base_stats: &Stats, level: u32) -> Self {
        let hp = ((base_stats.hp * 2 * level) / 100) + level + 10;
        let attack = ((base_stats.attack * 2 * level) / 100) + 5;
        let defense = ((base_stats.defense * 2 * level) / 100) + 5;
        let speed = ((base_stats.speed * 2 * level) / 100) + 5;
        let special_attack = ((base_stats.special_attack * 2 * level) / 100) + 5;
        let special_defense = ((base_stats.special_defense * 2 * level) / 100) + 5;

        Self {
            hp,
            attack,
            defense,
            speed,
            special_attack,
            special_defense,
        }
    }
}
