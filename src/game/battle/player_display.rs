use crate::game::font::Font;
use crate::game::pokemon::Pokemon;
use crate::renderer::Renderer;
use crate::renderer::sprite::Sprite;
use crate::renderer::instance::Instance;
use super::Display;

pub struct PlayerDisplay {
    background: Sprite,
    level: Font,
    max_hp: Font,
    current_hp: Font,
    name: Font,
    health_bar: Sprite,
}

impl Display for PlayerDisplay {

    fn update_health_bar(&mut self, pokemon: &Pokemon, renderer: &mut Renderer) {
        let hp_string = pokemon.current_hp.to_string() + "/";
        let current_hp = Font::new(200.0, 95.0, &hp_string, false, "black_font", renderer);

        let percent_hp = pokemon.current_hp as f32 / pokemon.stats.hp as f32;
        let y_offset = match percent_hp {
            x if x > 0.5 => 0,
            x if x > 0.25 => 1,
            _ => 2,
        };
        self.health_bar = renderer.create_sprite(174.0, 91.0, 0, 18 + y_offset, 3, 1, "battle", 1.0 * percent_hp, 1.0).expect("");
        self.current_hp = current_hp;
    }

    fn draw(&self, instances: &mut Vec<Instance>) {
        instances.extend_from_slice(&self.background.texture);
        instances.extend_from_slice(&self.level.instanced());
        instances.extend_from_slice(&self.max_hp.instanced());
        instances.extend_from_slice(&self.current_hp.instanced());
        instances.extend_from_slice(&self.name.instanced());
        instances.extend_from_slice(&self.health_bar.texture);
    }
}

impl PlayerDisplay {
    pub fn new(pokemon: &Pokemon, renderer: &mut Renderer) -> Self {

        let background = renderer.create_sprite(7.0 * 16.0, 4.0 * 16.0, 0, 12, 8, 3, "battle", 1.0, 1.0).expect("");

        let name = Font::new(141.0, 77.0, &pokemon.name.to_uppercase(), true, "black_font", renderer);
        let level = "Lv".to_string() + &pokemon.level.to_string();
        let level = Font::new(215.0, 77.0, &level, false, "black_font", renderer);

        let hp_string = pokemon.current_hp.to_string() + "/";
        let current_hp = Font::new(200.0, 95.0, &hp_string, false, "black_font", renderer);
        let max_hp = Font::new(215.0, 95.0, &pokemon.stats.hp.to_string(), false, "black_font", renderer);

        let percent_hp = pokemon.current_hp as f32 / pokemon.stats.hp as f32;
        let y_offset = match percent_hp {
            x if x > 0.5 => 0,
            x if x > 0.25 => 1,
            _ => 2,
        };
        let health_bar = renderer.create_sprite(174.0, 91.0, 0, 18 + y_offset, 3, 1, "battle", 1.0 * percent_hp, 1.0).expect("");

        Self {
            background,
            level,
            current_hp,
            max_hp,
            name,
            health_bar,
        }
    }

    pub fn swap(&mut self, pokemon: &Pokemon, renderer: &mut Renderer) {
        self.name = Font::new(141.0, 77.0, &pokemon.name.to_uppercase(), true, "black_font", renderer);
        let level = "Lv".to_string() + &pokemon.level.to_string();
        self.level = Font::new(215.0, 77.0, &level, false, "black_font", renderer);

        let hp_string = pokemon.current_hp.to_string() + "/";
        self.current_hp = Font::new(200.0, 95.0, &hp_string, false, "black_font", renderer);
        self.max_hp = Font::new(215.0, 95.0, &pokemon.stats.hp.to_string(), false, "black_font", renderer);

        let percent_hp = pokemon.current_hp as f32 / pokemon.stats.hp as f32;
        let y_offset = match percent_hp {
            x if x > 0.5 => 0,
            x if x > 0.25 => 1,
            _ => 2,
        };
        self.health_bar = renderer.create_sprite(174.0, 91.0, 0, 18 + y_offset, 3, 1, "battle", 1.0 * percent_hp, 1.0).expect("");
    }

}
