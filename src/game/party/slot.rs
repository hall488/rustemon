use crate::renderer::Renderer;
use crate::game::pokemon::Pokemon;
use crate::renderer::sprite::Sprite;
use crate::game::font::Font;
use crate::renderer::instance::Instance;

pub struct Slot {
    position: u32,
    selected_background: Sprite,
    background: Sprite,
    fainted_background: Sprite,
    selected_fainted_background: Sprite,
    pokemon_sprite: Sprite,
    name: Font,
    level: Font,
    max_hp: Font,
    current_hp: Font,
    health_bar: Sprite,
    pub selected: bool,
    pub fainted: bool,
}

impl Slot {

    pub fn new(pokemon: &Pokemon, position: u32, renderer: &mut Renderer) -> Self {
        if position == 0 {
            Self::generate_first_slot(pokemon, renderer)
        } else {
            Self::generate_other_slot(pokemon, position, renderer)
        }
    }

    fn generate_first_slot(pokemon: &Pokemon, renderer: &mut Renderer) -> Self {

        let pos_x = 0.0;
        let pos_y = 16.0;

        let background = renderer.create_sprite(pos_x, pos_y, 0, 18, 6, 4, "party", 1.0, 1.0).expect("");
        let selected_background = renderer.create_sprite(pos_x, pos_y, 6, 18, 6, 4, "party", 1.0, 1.0).expect("");
        let fainted_background = renderer.create_sprite(pos_x, pos_y, 0, 22, 6, 4, "party", 1.0, 1.0).expect("");
        let selected_fainted_background = renderer.create_sprite(pos_x, pos_y, 6, 22, 6, 4, "party", 1.0, 1.0).expect("");

        let (tx ,ty) = ((pokemon.id - 1) % 16, (pokemon.id - 1) / 16);
        let pokemon_sprite = renderer.create_sprite(pos_x, pos_y, tx, ty, 1, 1, "pokemon_party", 1.0, 1.0).expect("");

        let name = Font::new(31.0, 37.0, &pokemon.name.to_uppercase(), true, "white_font", renderer);
        let level = Font::new(47.0, 46.0, &pokemon.level.to_string(), true, "white_font", renderer);
        let max_hp = Font::new(70.0, 62.0, &pokemon.stats.hp.to_string(), true, "white_font", renderer);
        let current_hp = Font::new(50.0, 62.0, &pokemon.current_hp.to_string(), true, "white_font", renderer);

        let percent_hp = pokemon.current_hp as f32 / pokemon.stats.hp as f32;
        let y_offset = match percent_hp {
            x if x > 0.5 => 0,
            x if x > 0.25 => 1,
            _ => 2,
        };
        let health_bar = renderer.create_sprite(32.0, 59.0, 0, 14 + y_offset, 3, 1, "party", 1.0 * percent_hp, 1.0).expect("");

        let fainted = pokemon.current_hp == 0;


        println!("{} {}", pokemon.name, fainted);

        Self {
            position: 0,
            selected_background,
            background,
            fainted_background,
            selected_fainted_background,
            pokemon_sprite,
            name,
            level,
            max_hp,
            current_hp,
            health_bar,
            selected: false,
            fainted,
        }
    }

    fn generate_other_slot(pokemon: &Pokemon, position: u32, renderer: &mut Renderer) -> Self {

        let pos_y = 5.0 + ((position - 1) as f32 * 24.0);

        let background = renderer.create_sprite(82.0, pos_y, 4, 10, 10, 2, "party", 1.0, 1.0).expect("");
        let selected_background = renderer.create_sprite(82.0, pos_y, 4, 12, 10, 2, "party", 1.0, 1.0).expect("");
        let fainted_background = renderer.create_sprite(82.0, pos_y, 4, 14, 10, 2, "party", 1.0, 1.0).expect("");
        let selected_fainted_background = renderer.create_sprite(82.0, pos_y, 4, 16, 10, 2, "party", 1.0, 1.0).expect("");

        let (tx ,ty) = ((pokemon.id - 1) % 16, (pokemon.id - 1) / 16);
        let pokemon_sprite = renderer.create_sprite(83.0, pos_y - 2.0, tx, ty, 1, 1, "pokemon_party", 1.0, 1.0).expect("");

        let name = Font::new(116.0, pos_y + 7.0, &pokemon.name.to_uppercase(), true, "white_font", renderer);
        let level = Font::new(135.0, pos_y + 17.0, &pokemon.level.to_string(), true, "white_font", renderer);
        let max_hp = Font::new(222.0, pos_y + 17.0, &pokemon.stats.hp.to_string(), true, "white_font", renderer);
        let current_hp = Font::new(202.0, pos_y + 17.0, &pokemon.current_hp.to_string(), true, "white_font", renderer);

        let percent_hp = pokemon.current_hp as f32 / pokemon.stats.hp as f32;
        let y_offset = match percent_hp {
            x if x > 0.5 => 0,
            x if x > 0.25 => 1,
            _ => 2,
        };
        let health_bar = renderer.create_sprite(184.0, pos_y + 13.0, 0, 14 + y_offset, 3, 1, "party", 1.0 * percent_hp, 1.0).expect("");

        let fainted = pokemon.current_hp == 0;

        println!("{} {}", pokemon.name, fainted);

        Self {
            position,
            selected_background,
            background,
            fainted_background,
            selected_fainted_background,
            pokemon_sprite,
            name,
            level,
            max_hp,
            current_hp,
            health_bar,
            selected: false,
            fainted,
        }
    }

    pub fn draw(&self, instances: &mut Vec<Instance>) {
        //draw all backgrounds based on fainted bool and selected bool

        let background = match (self.fainted, self.selected) {
            (true, true) => &self.selected_fainted_background.texture,
            (true, false) => &self.fainted_background.texture,
            (false, true) => &self.selected_background.texture,
            (false, false) => &self.background.texture,
        };

        instances.extend_from_slice(background);

        instances.extend_from_slice(&self.pokemon_sprite.texture);
        instances.extend_from_slice(&self.name.instanced());
        if !self.fainted {
            instances.extend_from_slice(&self.level.instanced());
        }
        instances.extend_from_slice(&self.max_hp.instanced());
        instances.extend_from_slice(&self.current_hp.instanced());
        instances.extend_from_slice(&self.health_bar.texture);
    }

}
