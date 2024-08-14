use crate::renderer::sprite::Sprite;
use crate::renderer::Renderer;
use crate::renderer::instance::Instance;

pub struct Cancel {
    background: Sprite,
    selected_background: Sprite,
    pub selected: bool,
}

impl Cancel {
    pub fn new(renderer: &mut Renderer) -> Self {

        let background = renderer.create_sprite(11.0 * 16.0, 8.0 * 16.0, 0, 10, 4, 2, "party", 1.0, 1.0).expect("");
        let selected_background = renderer.create_sprite(11.0 * 16.0, 8.0 * 16.0, 0, 12, 4, 2, "party", 1.0, 1.0).expect("");

        Self {
            background,
            selected_background,
            selected: false,
        }
    }

    pub fn draw(&self, instances: &mut Vec<Instance>) {
        if self.selected {
            instances.extend_from_slice(&self.selected_background.texture);
        } else {
            instances.extend_from_slice(&self.background.texture);
        }
    }
}
