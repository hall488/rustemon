use super::moves::Move;
use crate::renderer::sprite::Sprite;
use crate::renderer::Renderer;

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub level: u32,
    pub current_hp: u32,
    pub stats: Stats,
    pub base_stats: Stats,
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
            "Caterpie" => (
                10,
                Stats::new(45, 30, 35, 45, 20, 20),
                255,
                39,
                vec!["Tackle"]
            ),
            "Metapod" => (
                11,
                Stats::new(50, 20, 55, 30, 25, 25),
                120,
                72,
                vec!["Harden"]
            ),
            "Butterfree" => (
                12,
                Stats::new(60, 45, 50, 70, 80, 80),
                45,
                178,
                vec!["Confusion", "Poison Powder", "Stun Spore", "Sleep Powder"]
            ),
            "Weedle" => (
                13,
                Stats::new(40, 35, 30, 50, 20, 20),
                255,
                39,
                vec!["Poison Sting"]
            ),
            "Kakuna" => (
                14,
                Stats::new(45, 25, 50, 35, 25, 25),
                120,
                72,
                vec!["Harden"]
            ),
            "Beedrill" => (
                15,
                Stats::new(65, 80, 40, 75, 45, 80),
                45,
                178,
                vec!["Twineedle", "Rage", "Focus Energy", "Pin Missile"]
            ),
            "Pidgey" => (
                16,
                Stats::new(40, 45, 40, 56, 35, 35),
                255,
                50,
                vec!["Gust"]
            ),
            "Pidgeotto" => (
                17,
                Stats::new(63, 60, 55, 71, 50, 50),
                120,
                122,
                vec!["Gust", "Quick Attack", "Whirlwind"]
            ),
            "Pidgeot" => (
                18,
                Stats::new(83, 80, 75, 91, 70, 70),
                45,
                216,
                vec!["Gust", "Quick Attack", "Whirlwind", "Wing Attack"]
            ),
            "Rattata" => (
                19,
                Stats::new(30, 56, 35, 72, 25, 35),
                255,
                51,
                vec!["Tackle", "Tail Whip"]
            ),
            "Raticate" => (
                20,
                Stats::new(55, 81, 60, 97, 50, 70),
                127,
                145,
                vec!["Tackle", "Tail Whip", "Quick Attack", "Hyper Fang"]
            ),
            "Spearow" => (
                21,
                Stats::new(40, 60, 30, 70, 31, 31),
                255,
                52,
                vec!["Peck"]
            ),
            "Fearow" => (
                22,
                Stats::new(65, 90, 65, 100, 61, 61),
                90,
                155,
                vec!["Peck", "Growl", "Leer", "Fury Attack"]
            ),
            "Ekans" => (
                23,
                Stats::new(35, 60, 44, 55, 40, 54),
                255,
                58,
                vec!["Wrap"]
            ),
            "Arbok" => (
                24,
                Stats::new(60, 85, 69, 80, 65, 79),
                90,
                153,
                vec!["Wrap", "Poison Sting", "Bite", "Glare"]
            ),
            "Pikachu" => (
                25,
                Stats::new(35, 55, 40, 90, 50, 50),
                190,
                112,
                vec!["Thundershock", "Growl"]
            ),
            "Raichu" => (
                26,
                Stats::new(60, 90, 55, 100, 90, 80),
                75,
                218,
                vec!["Thundershock", "Growl", "Thunderbolt", "Quick Attack"]
            ),
            "Sandshrew" => (
                27,
                Stats::new(50, 75, 85, 40, 20, 30),
                255,
                93,
                vec!["Scratch", "Defense Curl"]
            ),
            "Sandslash" => (
                28,
                Stats::new(75, 100, 110, 65, 45, 55),
                90,
                163,
                vec!["Scratch", "Defense Curl", "Sand Attack", "Slash"]
            ),
            "Nidoran♀" => (
                29,
                Stats::new(55, 47, 52, 41, 40, 40),
                235,
                59,
                vec!["Scratch", "Growl"]
            ),
            "Nidorina" => (
                30,
                Stats::new(70, 62, 67, 56, 55, 55),
                120,
                117,
                vec!["Scratch", "Growl", "Tackle", "Poison Sting"]
            ),
            "Nidoqueen" => (
                31,
                Stats::new(90, 82, 87, 76, 75, 85),
                45,
                194,
                vec!["Scratch", "Growl", "Tackle", "Poison Sting"]
            ),
            "Nidoran♂" => (
                32,
                Stats::new(46, 57, 40, 50, 40, 40),
                235,
                60,
                vec!["Peck"]
            ),
            "Nidorino" => (
                33,
                Stats::new(61, 72, 57, 65, 55, 55),
                120,
                118,
                vec!["Peck", "Leer", "Horn Attack", "Double Kick"]
            ),
            "Nidoking" => (
                34,
                Stats::new(81, 92, 77, 85, 85, 75),
                45,
                195,
                vec!["Peck", "Leer", "Horn Attack", "Double Kick"]
            ),
            "Clefairy" => (
                35,
                Stats::new(70, 45, 48, 35, 60, 65),
                150,
                113,
                vec!["Pound", "Growl"]
            ),
            "Clefable" => (
                36,
                Stats::new(95, 70, 73, 60, 85, 90),
                25,
                217,
                vec!["Pound", "Growl", "Sing", "Double Slap"]
            ),
            "Vulpix" => (
                37,
                Stats::new(38, 41, 40, 65, 50, 65),
                190,
                60,
                vec!["Ember"]
            ),
            "Ninetales" => (
                38,
                Stats::new(73, 76, 75, 100, 81, 100),
                75,
                177,
                vec!["Ember", "Tail Whip", "Quick Attack", "Confuse Ray"]
            ),
            "Jigglypuff" => (
                39,
                Stats::new(115, 45, 20, 20, 45, 25),
                170,
                76,
                vec!["Sing", "Pound"]
            ),
            "Wigglytuff" => (
                40,
                Stats::new(140, 70, 45, 45, 85, 50),
                50,
                109,
                vec!["Sing", "Pound", "Disable", "Defense Curl"]
            ),
            "Zubat" => (
                41,
                Stats::new(40, 45, 35, 55, 30, 40),
                255,
                54,
                vec!["Leech Life", "Supersonic"]
            ),
            "Golbat" => (
                42,
                Stats::new(75, 80, 70, 90, 65, 75),
                90,
                171,
                vec!["Leech Life", "Supersonic", "Wing Attack", "Confuse Ray"]
            ),
            "Oddish" => (
                43,
                Stats::new(45, 50, 55, 30, 75, 65),
                255,
                78,
                vec!["Absorb", "Sweet Scent"]
            ),
            "Gloom" => (
                44,
                Stats::new(60, 65, 70, 40, 85, 75),
                120,
                132,
                vec!["Absorb", "Sweet Scent", "Acid", "Poison Powder"]
            ),
            "Vileplume" => (
                45,
                Stats::new(75, 80, 85, 50, 110, 90),
                45,
                184,
                vec!["Absorb", "Sweet Scent", "Acid", "Poison Powder"]
            ),
            "Paras" => (
                46,
                Stats::new(35, 70, 55, 25, 45, 55),
                190,
                70,
                vec!["Scratch", "Stun Spore"]
            ),
            "Parasect" => (
                47,
                Stats::new(60, 95, 80, 30, 60, 80),
                75,
                128,
                vec!["Scratch", "Stun Spore", "Leech Life", "Spore"]
            ),
            "Venonat" => (
                48,
                Stats::new(60, 55, 50, 45, 40, 55),
                190,
                75,
                vec!["Tackle", "Disable"]
            ),
            "Venomoth" => (
                49,
                Stats::new(70, 65, 60, 90, 90, 75),
                75,
                138,
                vec!["Tackle", "Disable", "Supersonic", "Confusion"]
            ),
            "Diglett" => (
                50,
                Stats::new(10, 55, 25, 95, 35, 45),
                255,
                81,
                vec!["Scratch", "Growl"]
            ),
            "Dugtrio" => (
                51,
                Stats::new(35, 80, 50, 120, 50, 70),
                50,
                153,
                vec!["Scratch", "Growl", "Dig", "Sand Attack"]
            ),
            "Meowth" => (
                52,
                Stats::new(40, 45, 35, 90, 40, 40),
                255,
                69,
                vec!["Scratch", "Growl"]
            ),
            "Persian" => (
                53,
                Stats::new(65, 70, 60, 115, 65, 65),
                90,
                148,
                vec!["Scratch", "Growl", "Bite", "Pay Day"]
            ),
            "Psyduck" => (
                54,
                Stats::new(50, 52, 48, 55, 65, 50),
                190,
                80,
                vec!["Scratch", "Tail Whip"]
            ),
            "Golduck" => (
                55,
                Stats::new(80, 82, 78, 85, 95, 80),
                75,
                174,
                vec!["Scratch", "Tail Whip", "Disable", "Confusion"]
            ),
            "Mankey" => (
                56,
                Stats::new(40, 80, 35, 70, 35, 45),
                190,
                74,
                vec!["Scratch", "Growl"]
            ),
            "Primeape" => (
                57,
                Stats::new(65, 105, 60, 95, 60, 70),
                75,
                149,
                vec!["Scratch", "Growl", "Fury Swipes", "Karate Chop"]
            ),
            "Growlithe" => (
                58,
                Stats::new(55, 70, 45, 60, 70, 50),
                190,
                91,
                vec!["Bite", "Roar"]
            ),
            "Arcanine" => (
                59,
                Stats::new(90, 110, 80, 95, 100, 80),
                75,
                213,
                vec!["Bite", "Roar", "Ember", "Flamethrower"]
            ),
            "Poliwag" => (
                60,
                Stats::new(40, 50, 40, 90, 40, 40),
                255,
                77,
                vec!["Bubble"]
            ),
            "Poliwhirl" => (
                61,
                Stats::new(65, 65, 65, 90, 50, 50),
                120,
                131,
                vec!["Bubble", "Hypnosis", "Water Gun", "Double Slap"]
            ),
            "Poliwrath" => (
                62,
                Stats::new(90, 85, 95, 70, 70, 90),
                45,
                185,
                vec!["Bubble", "Hypnosis", "Water Gun", "Double Slap"]
            ),
            "Abra" => (
                63,
                Stats::new(25, 20, 15, 90, 105, 55),
                200,
                73,
                vec!["Teleport"]
            ),
            "Kadabra" => (
                64,
                Stats::new(40, 35, 30, 105, 120, 70),
                100,
                145,
                vec!["Teleport", "Kinesis", "Confusion", "Disable"]
            ),
            "Alakazam" => (
                65,
                Stats::new(55, 50, 45, 120, 135, 85),
                50,
                186,
                vec!["Teleport", "Kinesis", "Confusion", "Disable"]
            ),
            "Machop" => (
                66,
                Stats::new(70, 80, 50, 35, 35, 35),
                180,
                75,
                vec!["Karate Chop", "Low Kick"]
            ),
            "Machoke" => (
                67,
                Stats::new(80, 100, 70, 45, 50, 60),
                90,
                146,
                vec!["Karate Chop", "Low Kick", "Leer", "Focus Energy"]
            ),
            "Machamp" => (
                68,
                Stats::new(90, 130, 80, 55, 65, 85),
                45,
                193,
                vec!["Karate Chop", "Low Kick", "Leer", "Focus Energy"]
            ),
            "Bellsprout" => (
                69,
                Stats::new(50, 75, 35, 40, 70, 30),
                255,
                84,
                vec!["Vine Whip"]
            ),
            "Weepinbell" => (
                70,
                Stats::new(65, 90, 50, 55, 85, 45),
                120,
                151,
                vec!["Vine Whip", "Growth", "Wrap", "Poison Powder"]
            ),
            "Victreebel" => (
                71,
                Stats::new(80, 105, 65, 70, 100, 60),
                45,
                191,
                vec!["Vine Whip", "Growth", "Wrap", "Poison Powder"]
            ),
            "Tentacool" => (
                72,
                Stats::new(40, 40, 35, 70, 50, 100),
                190,
                105,
                vec!["Poison Sting", "Supersonic"]
            ),
            "Tentacruel" => (
                73,
                Stats::new(80, 70, 65, 100, 80, 120),
                60,
                205,
                vec!["Poison Sting", "Supersonic", "Constrict", "Acid"]
            ),
            "Geodude" => (
                74,
                Stats::new(40, 80, 100, 20, 30, 30),
                255,
                86,
                vec!["Tackle", "Defense Curl"]
            ),
            "Graveler" => (
                75,
                Stats::new(55, 95, 115, 35, 45, 45),
                120,
                134,
                vec!["Tackle", "Defense Curl", "Rock Throw", "Self-Destruct"]
            ),
            "Golem" => (
                76,
                Stats::new(80, 110, 130, 45, 55, 65),
                45,
                177,
                vec!["Tackle", "Defense Curl", "Rock Throw", "Self-Destruct"]
            ),
            "Ponyta" => (
                77,
                Stats::new(50, 85, 55, 90, 65, 65),
                190,
                152,
                vec!["Ember", "Tail Whip"]
            ),
            "Rapidash" => (
                78,
                Stats::new(65, 100, 70, 105, 80, 80),
                60,
                192,
                vec!["Ember", "Tail Whip", "Stomp", "Growl"]
            ),
            "Slowpoke" => (
                79,
                Stats::new(90, 65, 65, 15, 40, 40),
                190,
                99,
                vec!["Confusion", "Disable"]
            ),
            "Slowbro" => (
                80,
                Stats::new(95, 75, 110, 30, 100, 80),
                75,
                164,
                vec!["Confusion", "Disable", "Headbutt", "Growl"]
            ),
            "Magnemite" => (
                81,
                Stats::new(25, 35, 70, 45, 95, 55),
                190,
                89,
                vec!["Tackle", "Sonic Boom"]
            ),
            "Magneton" => (
                82,
                Stats::new(50, 60, 95, 70, 120, 70),
                60,
                161,
                vec!["Tackle", "Sonic Boom", "Supersonic", "Thunder Wave"]
            ),
            "Farfetch'd" => (
                83,
                Stats::new(52, 65, 55, 60, 58, 62),
                45,
                94,
                vec!["Peck", "Sand Attack"]
            ),
            "Doduo" => (
                84,
                Stats::new(35, 85, 45, 75, 35, 35),
                190,
                96,
                vec!["Peck"]
            ),
            "Dodrio" => (
                85,
                Stats::new(60, 110, 70, 100, 60, 60),
                45,
                158,
                vec!["Peck", "Growl", "Fury Attack", "Drill Peck"]
            ),
            "Seel" => (
                86,
                Stats::new(65, 45, 55, 45, 45, 70),
                190,
                100,
                vec!["Headbutt"]
            ),
            "Dewgong" => (
                87,
                Stats::new(90, 70, 80, 70, 70, 95),
                75,
                176,
                vec!["Headbutt", "Growl", "Aurora Beam", "Rest"]
            ),
            "Grimer" => (
                88,
                Stats::new(80, 80, 50, 25, 40, 50),
                190,
                90,
                vec!["Pound", "Disable"]
            ),
            "Muk" => (
                89,
                Stats::new(105, 105, 75, 50, 65, 100),
                75,
                157,
                vec!["Pound", "Disable", "Minimize", "Sludge"]
            ),
            "Shellder" => (
                90,
                Stats::new(30, 65, 100, 40, 45, 25),
                190,
                97,
                vec!["Tackle"]
            ),
            "Cloyster" => (
                91,
                Stats::new(50, 95, 180, 70, 85, 45),
                60,
                203,
                vec!["Tackle", "Withdraw", "Spike Cannon", "Clamp"]
            ),
            "Gastly" => (
                92,
                Stats::new(30, 35, 30, 80, 100, 35),
                190,
                95,
                vec!["Lick", "Confuse Ray"]
            ),
            "Haunter" => (
                93,
                Stats::new(45, 50, 45, 95, 115, 55),
                90,
                126,
                vec!["Lick", "Confuse Ray", "Night Shade", "Hypnosis"]
            ),
            "Gengar" => (
                94,
                Stats::new(60, 65, 60, 110, 130, 75),
                45,
                190,
                vec!["Lick", "Confuse Ray", "Night Shade", "Hypnosis"]
            ),
            "Onix" => (
                95,
                Stats::new(35, 45, 160, 70, 30, 45),
                45,
                108,
                vec!["Tackle", "Screech"]
            ),
            "Drowzee" => (
                96,
                Stats::new(60, 48, 45, 42, 43, 90),
                190,
                102,
                vec!["Pound", "Hypnosis"]
            ),
            "Hypno" => (
                97,
                Stats::new(85, 73, 70, 67, 73, 115),
                75,
                165,
                vec!["Pound", "Hypnosis", "Disable", "Confusion"]
            ),
            "Krabby" => (
                98,
                Stats::new(30, 105, 90, 50, 25, 25),
                225,
                115,
                vec!["Bubble"]
            ),
            "Kingler" => (
                99,
                Stats::new(55, 130, 115, 75, 50, 50),
                60,
                206,
                vec!["Bubble", "Leer", "Vice Grip", "Guillotine"]
            ),
            "Voltorb" => (
                100,
                Stats::new(40, 30, 50, 100, 55, 55),
                190,
                103,
                vec!["Tackle", "Screech"]
            ),
            "Electrode" => (
                101,
                Stats::new(60, 50, 70, 140, 80, 80),
                60,
                150,
                vec!["Tackle", "Screech", "Sonic Boom", "Self-Destruct"]
            ),
            "Exeggcute" => (
                102,
                Stats::new(60, 40, 80, 40, 60, 45),
                90,
                98,
                vec!["Barrage", "Hypnosis"]
            ),
            "Exeggutor" => (
                103,
                Stats::new(95, 95, 85, 55, 125, 65),
                45,
                212,
                vec!["Barrage", "Hypnosis", "Confusion", "Stomp"]
            ),
            "Cubone" => (
                104,
                Stats::new(50, 50, 95, 35, 40, 50),
                190,
                87,
                vec!["Growl", "Tail Whip"]
            ),
            "Marowak" => (
                105,
                Stats::new(60, 80, 110, 45, 50, 80),
                75,
                124,
                vec!["Growl", "Tail Whip", "Bone Club", "Leer"]
            ),
            "Hitmonlee" => (
                106,
                Stats::new(50, 120, 53, 87, 35, 110),
                45,
                139,
                vec!["Double Kick", "Meditate", "Rolling Kick", "Jump Kick"]
            ),
            "Hitmonchan" => (
                107,
                Stats::new(50, 105, 79, 76, 35, 110),
                45,
                140,
                vec!["Comet Punch", "Agility", "Fire Punch", "Ice Punch"]
            ),
            "Lickitung" => (
                108,
                Stats::new(90, 55, 75, 30, 60, 75),
                45,
                127,
                vec!["Wrap", "Supersonic"]
            ),
            "Koffing" => (
                109,
                Stats::new(40, 65, 95, 35, 60, 45),
                190,
                114,
                vec!["Tackle", "Smog"]
            ),
            "Weezing" => (
                110,
                Stats::new(65, 90, 120, 60, 85, 70),
                60,
                173,
                vec!["Tackle", "Smog", "Sludge", "Smokescreen"]
            ),
            "Rhyhorn" => (
                111,
                Stats::new(80, 85, 95, 25, 30, 30),
                120,
                135,
                vec!["Horn Attack", "Tail Whip"]
            ),
            "Rhydon" => (
                112,
                Stats::new(105, 130, 120, 40, 45, 45),
                60,
                204,
                vec!["Horn Attack", "Tail Whip", "Stomp", "Fury Attack"]
            ),
            "Chansey" => (
                113,
                Stats::new(250, 5, 5, 50, 35, 105),
                30,
                395,
                vec!["Pound", "Growl"]
            ),
            "Tangela" => (
                114,
                Stats::new(65, 55, 115, 60, 100, 40),
                45,
                166,
                vec!["Constrict", "Growth"]
            ),
            "Kangaskhan" => (
                115,
                Stats::new(105, 95, 80, 90, 40, 80),
                45,
                175,
                vec!["Comet Punch", "Rage", "Bite", "Tail Whip"]
            ),
            "Horsea" => (
                116,
                Stats::new(30, 40, 70, 60, 70, 25),
                225,
                83,
                vec!["Bubble"]
            ),
            "Seadra" => (
                117,
                Stats::new(55, 65, 95, 85, 95, 45),
                75,
                155,
                vec!["Bubble", "Smokescreen", "Leer", "Water Gun"]
            ),
            "Goldeen" => (
                118,
                Stats::new(45, 67, 60, 63, 35, 50),
                225,
                111,
                vec!["Peck", "Tail Whip"]
            ),
            "Seaking" => (
                119,
                Stats::new(80, 92, 65, 68, 65, 80),
                60,
                170,
                vec!["Peck", "Tail Whip", "Supersonic", "Horn Attack"]
            ),
            "Staryu" => (
                120,
                Stats::new(30, 45, 55, 85, 70, 55),
                225,
                106,
                vec!["Tackle", "Water Gun"]
            ),
            "Starmie" => (
                121,
                Stats::new(60, 75, 85, 115, 100, 85),
                60,
                207,
                vec!["Tackle", "Water Gun", "Harden", "Recover"]
            ),
            "Mr. Mime" => (
                122,
                Stats::new(40, 45, 65, 90, 100, 120),
                45,
                136,
                vec!["Confusion", "Barrier"]
            ),
            "Scyther" => (
                123,
                Stats::new(70, 110, 80, 105, 55, 80),
                45,
                187,
                vec!["Quick Attack", "Leer", "Focus Energy", "Double Team"]
            ),
            "Jynx" => (
                124,
                Stats::new(65, 50, 35, 95, 115, 95),
                45,
                137,
                vec!["Pound", "Lick", "Double Slap", "Ice Punch"]
            ),
            "Electabuzz" => (
                125,
                Stats::new(65, 83, 57, 105, 95, 85),
                45,
                156,
                vec!["Quick Attack", "Leer", "Thundershock", "Thunderpunch"]
            ),
            "Magmar" => (
                126,
                Stats::new(65, 95, 57, 93, 100, 85),
                45,
                167,
                vec!["Ember", "Leer", "Smog", "Fire Punch"]
            ),
            "Pinsir" => (
                127,
                Stats::new(65, 125, 100, 85, 55, 70),
                45,
                200,
                vec!["Vice Grip", "Seismic Toss", "Focus Energy", "Harden"]
            ),
            "Tauros" => (
                128,
                Stats::new(75, 100, 95, 110, 40, 70),
                45,
                211,
                vec!["Tackle", "Stomp", "Tail Whip", "Leer"]
            ),
            "Magikarp" => (
                129,
                Stats::new(20, 10, 55, 80, 15, 20),
                255,
                20,
                vec!["Splash"]
            ),
            "Gyarados" => (
                130,
                Stats::new(95, 125, 79, 81, 60, 100),
                45,
                214,
                vec!["Bite", "Dragon Rage", "Leer", "Hydro Pump"]
            ),
            "Lapras" => (
                131,
                Stats::new(130, 85, 80, 60, 85, 95),
                45,
                219,
                vec!["Water Gun", "Growl", "Sing", "Mist"]
            ),
            "Ditto" => (
                132,
                Stats::new(48, 48, 48, 48, 48, 48),
                35,
                61,
                vec!["Transform"]
            ),
            "Eevee" => (
                133,
                Stats::new(55, 55, 50, 55, 45, 65),
                45,
                92,
                vec!["Tackle", "Tail Whip"]
            ),
            "Vaporeon" => (
                134,
                Stats::new(130, 65, 60, 65, 110, 95),
                45,
                196,
                vec!["Tackle", "Tail Whip", "Water Gun", "Quick Attack"]
            ),
            "Jolteon" => (
                135,
                Stats::new(65, 65, 60, 130, 110, 95),
                45,
                197,
                vec!["Tackle", "Tail Whip", "Thunder Shock", "Quick Attack"]
            ),
            "Flareon" => (
                136,
                Stats::new(65, 130, 60, 65, 95, 110),
                45,
                198,
                vec!["Tackle", "Tail Whip", "Ember", "Quick Attack"]
            ),
            "Porygon" => (
                137,
                Stats::new(65, 60, 70, 40, 85, 75),
                45,
                130,
                vec!["Tackle", "Sharpen"]
            ),
            "Omanyte" => (
                138,
                Stats::new(35, 40, 100, 35, 90, 55),
                45,
                71,
                vec!["Water Gun", "Withdraw"]
            ),
            "Omastar" => (
                139,
                Stats::new(70, 60, 125, 55, 115, 70),
                45,
                173,
                vec!["Water Gun", "Withdraw", "Horn Attack", "Leer"]
            ),
            "Kabuto" => (
                140,
                Stats::new(30, 80, 90, 55, 55, 45),
                45,
                71,
                vec!["Scratch", "Harden"]
            ),
            "Kabutops" => (
                141,
                Stats::new(60, 115, 105, 80, 65, 70),
                45,
                173,
                vec!["Scratch", "Harden", "Absorb", "Slash"]
            ),
            "Aerodactyl" => (
                142,
                Stats::new(80, 105, 65, 130, 60, 75),
                45,
                202,
                vec!["Wing Attack", "Agility", "Supersonic", "Bite"]
            ),
            "Snorlax" => (
                143,
                Stats::new(160, 110, 65, 30, 65, 110),
                25,
                154,
                vec!["Headbutt", "Amnesia", "Rest", "Body Slam"]
            ),
            "Articuno" => (
                144,
                Stats::new(90, 85, 100, 85, 95, 125),
                3,
                261,
                vec!["Peck", "Ice Beam", "Blizzard", "Agility"]
            ),
            "Zapdos" => (
                145,
                Stats::new(90, 90, 85, 100, 125, 90),
                3,
                261,
                vec!["Peck", "Thunder Shock", "Thunderbolt", "Agility"]
            ),
            "Moltres" => (
                146,
                Stats::new(90, 100, 90, 90, 125, 85),
                3,
                261,
                vec!["Peck", "Ember", "Fire Spin", "Agility"]
            ),
            "Dratini" => (
                147,
                Stats::new(41, 64, 45, 50, 50, 50),
                45,
                67,
                vec!["Wrap"]
            ),
            "Dragonair" => (
                148,
                Stats::new(61, 84, 65, 70, 70, 70),
                45,
                144,
                vec!["Wrap", "Slam", "Agility", "Dragon Rage"]
            ),
            "Dragonite" => (
                149,
                Stats::new(91, 134, 95, 80, 100, 100),
                45,
                218,
                vec!["Wrap", "Slam", "Agility", "Dragon Rage"]
            ),
            "Mewtwo" => (
                150,
                Stats::new(106, 110, 90, 130, 154, 90),
                3,
                220,
                vec!["Confusion", "Disable", "Swift", "Psychic"]
            ),
            "Mew" => (
                151,
                Stats::new(100, 100, 100, 100, 100, 100),
                45,
                64,
                vec!["Pound"]
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
