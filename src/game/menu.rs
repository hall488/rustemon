use cgmath;
use crate::renderer::instance::Instance;
use super::input_manager::InputManager;
use winit::keyboard::KeyCode;
use crate::game::Player;
use crate::game::GameState;

pub enum MenuAction {
    Pokemon,
    Bag,
    Save,
    Exit,
    // Add other actions here
}

impl MenuAction {
    pub fn execute(&self, player: &Player) -> GameState {
        match self {
            MenuAction::Pokemon => {
                println!("Pokemon action");
                GameState::Party
            },
            MenuAction::Bag => {
                println!("Bag action");
                GameState::Paused
            },
            MenuAction::Save => {
                println!("Save action");
                GameState::Paused
            },
            MenuAction::Exit => {
                println!("Exit action");
                GameState::Running
            },
            // Implement other actions here
        }
    }
}

pub struct Menu {
    pub instances: Vec<Instance>,
    pub offsets: Vec<cgmath::Vector3<f32>>,
    pub pointer: Instance,
    pub option: u32,
}

impl Menu {
    pub fn new(loader: &mut tiled::Loader, player_position: cgmath::Vector3<f32>) -> Self {
        let menu_loader = loader.load_tmx_map("/home/chris/games/SirSquare/assets/menu.tmx").unwrap();
        let mut instances = Vec::new();
        let mut offsets = Vec::new();

        for layer in menu_loader.layers() {
            if let tiled::LayerType::Tiles(tile_layer) = layer.layer_type() {
                for i in 0..tile_layer.width().unwrap() {
                    for j in 0..tile_layer.height().unwrap() {
                        if let Some(tile) = tile_layer.get_tile(i as i32, j as i32) {
                            instances.push(Instance {
                                model: cgmath::Matrix4::from_translation(cgmath::Vector3::new(i as f32, -1.0 * j as f32, 0.0)).into(),
                                tex_index: tile.id() as u32,
                                atlas_index: 2,
                            });
                            offsets.push(cgmath::Vector3::new(i as f32, -1.0 * j as f32, 0.0));
                        }
                    }
                }
            }
        }

        let pointer = Instance {
            model: cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 0.0)).into(),
            tex_index: 55,
            atlas_index: 2,
        };

        Self {
            instances,
            offsets,
            pointer,
            option: 0,
        }
    }

    fn get_action_for_option(option: u32) -> MenuAction {
        match option {
            0 => MenuAction::Pokemon,
            1 => MenuAction::Bag,
            2 => MenuAction::Save,
            3 => MenuAction::Exit,
            _ => MenuAction::Exit,
        }
    }

    pub fn update(&mut self, input_manager: &mut InputManager, player: &Player ) -> GameState {
        let release_key = input_manager.get_key_on_press();

        // Update menu instances positions
        for (i, offset) in self.offsets.iter().enumerate() {
            let model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(
                player.position.x + offset.x + 5.5,
                player.position.y + offset.y + 10.0,
                offset.z)).into();
            self.instances[i].model = model;
        }

        // Update menu pointer position
        self.pointer.model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(
            player.position.x + 5.5,
            player.position.y + 9.0 - self.option as f32,
            0.0)).into();

        // Handle input
        if let Some(release_key) = release_key {
            match release_key {
                KeyCode::Enter | KeyCode::KeyX => {
                    return GameState::Running;
                },
                KeyCode::KeyZ => {
                    let action = Menu::get_action_for_option(self.option);
                    return action.execute(player);
                },
                KeyCode::KeyW => {
                    if self.option > 0 {
                        self.option -= 1;
                    }
                },
                KeyCode::KeyS => {
                    if self.option < 3 { // Adjust this value based on the number of menu options
                        self.option += 1;
                    }
                },
                _ => {},
            }
        }

        GameState::Paused
    }
}
