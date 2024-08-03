use crate::game::input_manager::InputManager;
use std::time::{Duration, Instant};
use crate::renderer::Renderer;
use crate::renderer::instance::Instance;
use crate::game::pokemon::Pokemon;
use crate::renderer::sprite::Sprite;
use crate::game::stationary_loader::Stationary;
use tiled::Loader;
use crate::game::gamestate::GameState;
use winit::keyboard::KeyCode;

const HOLD_DURATION: Duration = Duration::from_millis(500);
const HOLD_TIME_PER_SLOT: Duration = Duration::from_millis(100);

pub struct Slot {
    pub selected_background: Sprite,
    pub background: Sprite,
    pub pokemon_sprite: Sprite,
    pub name: Text,
    pub level: Text,
    pub max_hp: Text,
    pub current_hp: Text,
    pub health_bar: Sprite,
}

pub struct Party {
    pub scene: Stationary,
    pub slots: Vec<Slot>,
    pub selected_slot: i32,
    pub cancel_button: Sprite,
    pub selected_cancel_button: Sprite,
    pub last_key: Option<KeyCode>,
    pub time_of_input: Instant,
    time_of_hold: Instant,
    processed_initial_press: bool,
    pub in_battle: bool,
}

pub struct Text {
    pub textures: Vec<Sprite>,
}

impl Text {
    pub fn new(text: &str, x: f32, y: f32, left: bool) -> Self {
        let mut textures = Vec::new();

        for (i, c) in text.char_indices() {
            let c = match left {
                true => c,
                false => text.chars().rev().nth(i).unwrap(),
            };

            let tex_index = match c {
                'A' | 'a' => 0,
                'B' | 'b' => 1,
                'C' | 'c' => 2,
                'D' | 'd' => 3,
                'E' | 'e' => 4,
                'F' | 'f' => 5,
                'G' | 'g' => 6,
                'H' | 'h' => 7,
                'I' | 'i' => 8,
                'J' | 'j' => 9,
                'K' | 'k' => 10,
                'L' | 'l' => 11,
                'M' | 'm' => 12,
                'N' | 'n' => 13,
                'O' | 'o' => 14,
                'P' | 'p' => 15,
                'Q' | 'q' => 16,
                'R' | 'r' => 17,
                'S' | 's' => 18,
                'T' | 't' => 19,
                'U' | 'u' => 20,
                'V' | 'v' => 21,
                'W' | 'w' => 22,
                'X' | 'x' => 23,
                'Y' | 'y' => 24,
                'Z' | 'z' => 25,
                '1' => 26,
                '2' => 27,
                '3' => 28,
                '4' => 29,
                '5' => 30,
                '6' => 31,
                '7' => 32,
                '8' => 33,
                '9' => 34,
                '0' => 35,
                _ => 0,
            };

            let tex_coord_x1 = tex_index % 13;
            let tex_coord_y1 = tex_index / 13;

            let tex_width = 1;
            let tex_height = 1;

            let pos_x = if left {
                x + i as f32 * 14.0 / 480.0 * 2.0 * 10.0 / 14.0
            } else {
                x - i as f32 * 14.0 / 480.0 * 2.0 * 10.0 / 14.0
            };

            let sprite = Sprite::new(
                pos_x, y, tex_coord_x1, tex_coord_y1, tex_width, tex_height,
                8, 13, 3, 0.7565, 0.3759,
            );

            textures.push(sprite);
        }

        Self { textures }
    }
}

impl Party {
    pub fn new(pokemon: &Vec<Pokemon>, in_battle: bool, renderer: &mut Renderer) -> Self {
        let mut loader = Loader::new();
        let map_loader = loader.load_tmx_map("/home/chris/games/SirSquare/assets/party.tmx").unwrap();
        let scene = Stationary::new(&map_loader, 6);

        let tex_coord_x1 = 0;
        let tex_coord_y1 = 14;
        let tex_coord_x2 = 6;
        let tex_coord_y2 = 18;
        let pos_x = 0.0;
        let pos_y = 2.0 / 10.0;

        let mut slots = Vec::new();

        let first_background = Sprite::new(
            pos_x, pos_y, tex_coord_x1, tex_coord_y1,
            tex_coord_x2 - tex_coord_x1, tex_coord_y2 - tex_coord_y1,
            6, 15, 10, 2.0, 2.0,
        );

        let first_selected_background = Sprite::new(
            pos_x, pos_y, tex_coord_x1 + 6, tex_coord_y1,
            tex_coord_x2 - tex_coord_x1, tex_coord_y2 - tex_coord_y1,
            6, 15, 10, 2.0, 2.0,
        );

        let first_pokemon = create_pokemon_sprite(&pokemon[0], pos_x, pos_y);

        let first_name = Text::new(&pokemon[0].name, pos_x + 4.0 / 15.0, pos_y + 4.0 / 15.0, true);
        let first_level = Text::new(&pokemon[0].level.to_string(), pos_x + 5.875 / 15.0, pos_y + 3.875 / 10.0, true);
        let first_current_hp = Text::new(&pokemon[0].current_hp.to_string(), pos_x + 6.875 / 15.0, pos_y + 5.875 / 10.0, false);
        let first_max_hp = Text::new(&pokemon[0].stats.hp.to_string(), pos_x + 9.375 / 15.0, pos_y + 5.875 / 10.0, false);

        let health_bar = create_health_bar(&pokemon[0], pos_x, pos_y, true);

        let first_slot = Slot {
            selected_background: first_selected_background,
            background: first_background,
            pokemon_sprite: first_pokemon,
            name: first_name,
            level: first_level,
            max_hp: first_max_hp,
            current_hp: first_current_hp,
            health_bar,
        };

        slots.push(first_slot);

        for i in 1..pokemon.len() {
            let pos_x = 10.0 / 15.0;
            let pos_y = (i - 1) as f32 * 96.0 / 320.0;

            let background = create_background_sprite(pos_x + 2.0/15.0 * 3.0 / 32.0, pos_y + 2.0/10.0 * 10.0 / 32.0);
            let selected_background = create_selected_background_sprite(pos_x + 2.0/15.0 * 3.0 / 32.0, pos_y + 2.0/10.0 * 10.0 / 32.0);
            let pokemon_sprite = create_pokemon_sprite(&pokemon[i], pos_x, pos_y);

            let name = Text::new(&pokemon[i].name, pos_x + 4.0 / 15.0, pos_y + 2.5 / 15.0, true);
            let level = Text::new(&pokemon[i].level.to_string(), pos_x + 6.8 / 15.0, pos_y + 2.875 / 10.0, true);
            let current_hp = Text::new(&pokemon[i].current_hp.to_string(), pos_x + 15.82 / 15.0, pos_y + 2.875 / 10.0, false);
            let max_hp = Text::new(&pokemon[i].stats.hp.to_string(), pos_x + 18.32 / 15.0, pos_y + 2.875 / 10.0, false);

            let health_bar = create_health_bar(&pokemon[i], pos_x, pos_y, false);

            let slot = Slot {
                selected_background,
                background,
                pokemon_sprite,
                name,
                level,
                max_hp,
                current_hp,
                health_bar,
            };

            slots.push(slot);
        }

        let pos_x = 22.0/15.0;
        let pos_y = 16.0/10.0;
        let tx1 = 0;
        let ty1 = 10;
        let width = 4;
        let height = 2;
        let atlas_index = 6;
        let atlas_width = 15;
        let atlas_height = 10;
        let scale_x = 2.0;
        let scale_y = 2.0;

        let cancel_button = Sprite::new(
            pos_x, pos_y, tx1, ty1, width, height,
            atlas_index, atlas_width, atlas_height, scale_x, scale_y,
        );

        let ty1 = 12;

        let selected_cancel_button = Sprite::new(
            pos_x, pos_y, tx1, ty1, width, height,
            atlas_index, atlas_width, atlas_height, scale_x, scale_y,
        );

        let selected_slot = 0;

        let last_key = None;
        let time_of_input = Instant::now();
        let time_of_hold = Instant::now();

        let processed_initial_press = false;

        Self {
            scene,
            slots,
            selected_slot,
            cancel_button,
            selected_cancel_button,
            last_key,
            time_of_input,
            processed_initial_press,
            time_of_hold,
            in_battle,
        }
    }

    pub fn update(&mut self, input_manager: &mut InputManager, dt: Duration, renderer: &mut Renderer) -> bool {
        renderer.camera_controller.update_camera(&mut renderer.camera, cgmath::Vector3::new(0.0, 0.0, 0.0));

        let input_key = input_manager.get_last_key();
        let single_press_key = input_manager.get_key_on_press();

        if input_key != self.last_key {
            self.last_key = input_key;
            self.time_of_input = Instant::now();
            self.processed_initial_press = false;
        }

        let time_held = Instant::now().duration_since(self.time_of_input);

        match self.last_key {
            Some(KeyCode::KeyW) => {
                self.handle_key_press(time_held, -1);
            }
            Some(KeyCode::KeyS) => {
                self.handle_key_press(time_held, 1);
            }
            Some(KeyCode::KeyX) => {
                self.selected_slot = self.slots.len() as i32;
            }
            _ => {}
        }

        if single_press_key == Some(KeyCode::KeyZ) {
            if self.selected_slot == self.slots.len() as i32 {
                return true;
            }
        }

        return false;
    }

    fn handle_key_press(&mut self, time_held: Duration, direction: i32) {
        println!("Selected slot: {}, Time_of_Hold: {}, Initial Press: {} ", self.selected_slot, self.time_of_hold.elapsed().as_millis(), self.processed_initial_press);
        if self.processed_initial_press {
            if time_held > HOLD_DURATION {
                if self.time_of_hold.elapsed() > HOLD_TIME_PER_SLOT {
                    self.selected_slot += direction;
                    self.time_of_hold = Instant::now();
                }
            }
        } else {
            self.selected_slot += direction;
            self.processed_initial_press = true;
        }

        if self.selected_slot < 0 {
            self.selected_slot = self.slots.len() as i32;
        } else if self.selected_slot > self.slots.len() as i32 {
            self.selected_slot = 0;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let mut instances = Vec::new();
        instances.extend_from_slice(&self.scene.background);
        for (i, slot) in self.slots.iter().enumerate() {
            if i as i32 == self.selected_slot {
                instances.extend_from_slice(&slot.selected_background.texture);
            } else {
                instances.extend_from_slice(&slot.background.texture);
            }

            if self.selected_slot == self.slots.len() as i32 {
                instances.extend_from_slice(&self.selected_cancel_button.texture);
            } else {
                instances.extend_from_slice(&self.cancel_button.texture);
            }

            instances.extend_from_slice(&slot.pokemon_sprite.texture);
            Self::extend_with_textures(&mut instances, &slot.name.textures);
            Self::extend_with_textures(&mut instances, &slot.level.textures);
            Self::extend_with_textures(&mut instances, &slot.max_hp.textures);
            Self::extend_with_textures(&mut instances, &slot.current_hp.textures);
            instances.extend_from_slice(&slot.health_bar.texture);
        }
        renderer.render(&instances, false).unwrap();
    }

    fn extend_with_textures(instances: &mut Vec<Instance>, sprites: &[Sprite]) {
        for sprite in sprites {
            instances.extend_from_slice(&sprite.texture);
        }
    }
}

fn create_pokemon_sprite(pokemon: &Pokemon, pos_x: f32, pos_y: f32) -> Sprite {
    let tex_coord_x1 = (pokemon.id - 1) % 16;
    let tex_coord_y1 = (pokemon.id - 1) / 16;

    Sprite::new(
        pos_x, pos_y, tex_coord_x1, tex_coord_y1,
        1, 1, 7, 16, 10, 4.0, 4.0,
    )
}

fn create_background_sprite(pos_x: f32, pos_y: f32) -> Sprite {
    let tex_coord_x1 = 4;
    let tex_coord_y1 = 10;
    let tex_width = 10;
    let tex_height = 2;

    Sprite::new(
        pos_x, pos_y, tex_coord_x1, tex_coord_y1,
        tex_width, tex_height, 6, 15, 10, 2.0, 2.0,
    )
}

fn create_selected_background_sprite(pos_x: f32, pos_y: f32) -> Sprite {
    let tex_coord_x1 = 4;
    let tex_coord_y1 = 12;
    let tex_width = 10;
    let tex_height = 2;

    Sprite::new(
        pos_x, pos_y, tex_coord_x1, tex_coord_y1,
        tex_width, tex_height, 6, 15, 10, 2.0, 2.0,
    )
}

fn create_health_bar(pokemon: &Pokemon, pos_x: f32, pos_y: f32, first: bool) -> Sprite {
    let percent_hp = pokemon.current_hp as f32 / pokemon.stats.hp as f32;
    let y_offset = match percent_hp {
        x if x > 0.5 => 0,
        x if x > 0.25 => 1,
        _ => 2,
    };

    let new_x = if first {
        pos_x + 4.0 / 15.0
    } else {
        pos_x + 12.9325 / 15.0
    };

    let new_y = if first {
        pos_y + 5.375 / 10.0
    } else {
        pos_y + 2.25 / 10.0
    };

    Sprite::new(
        new_x, new_y,
        1, 18 + y_offset, 3, 1,
        6, 15, 10, 2.0 * percent_hp, 2.0,
    )
}
