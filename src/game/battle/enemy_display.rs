use crate::game::font::Font;
use crate::game::pokemon::Pokemon;
use crate::renderer::Renderer;
use crate::renderer::sprite::Sprite;
use crate::renderer::instance::Instance;
use super::Display;

pub struct EnemyDisplay {
    background: Sprite,
    level: Font,
    name: Font,
    health_bar: Sprite,
}

impl Display for EnemyDisplay {
    fn update_health_bar(&mut self, pokemon: &Pokemon, renderer: &mut Renderer) {

        let percent_hp = pokemon.current_hp as f32 / pokemon.stats.hp as f32;
        let y_offset = match percent_hp {
            x if x > 0.5 => 0,
            x if x > 0.25 => 1,
            _ => 2,
        };
        self.health_bar = renderer.create_sprite(54.0, 33.0, 0, 18 + y_offset, 3, 1, "battle", 1.0 * percent_hp, 1.0).expect("");
    }

    fn draw(&self, instances: &mut Vec<Instance>) {
        instances.extend_from_slice(&self.background.texture);
        instances.extend_from_slice(&self.level.instanced());
        instances.extend_from_slice(&self.name.instanced());
        instances.extend_from_slice(&self.health_bar.texture);
    }
}

impl EnemyDisplay {
    pub fn new(pokemon: &Pokemon, renderer: &mut Renderer) -> Self {

        let background = renderer.create_sprite(0.0, 16.0, 0, 10, 7, 2, "battle", 1.0, 1.0).expect("");

        let name = Font::new(21.0, 19.0, &pokemon.name.to_uppercase(), true, "black_font", renderer);
        let level = "Lv".to_string() + &pokemon.level.to_string();
        let level = Font::new(95.0, 19.0, &level, false, "black_font",  renderer);

        let percent_hp = pokemon.current_hp as f32 / pokemon.stats.hp as f32;
        let y_offset = match percent_hp {
            x if x > 0.5 => 0,
            x if x > 0.25 => 1,
            _ => 2,
        };
        let health_bar = renderer.create_sprite(54.0, 33.0, 0, 18 + y_offset, 3, 1, "battle", 1.0 * percent_hp, 1.0).expect("");

        Self {
            background,
            level,
            name,
            health_bar,
        }
    }


}
