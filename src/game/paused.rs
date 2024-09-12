use crate::game::{Game, GameState};
use crate::renderer::Renderer;

impl Game {
    pub fn paused(&mut self, renderer: &mut Renderer) {
        self.state = self.menu.update(&mut self.input_manager, &self.player);

        if self.state == GameState::Party {
            self.enter_party(renderer);
        }
    }
}
