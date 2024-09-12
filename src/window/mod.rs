pub mod config;

use std::sync::Arc;
use config::{WIDTH, HEIGHT};
use crate::game::Game;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::renderer::Renderer;

use std::time::{Duration, Instant};

pub struct App {
    window: Option<Arc<Window>>,
    game: Option<Game>,
    renderer: Option<Renderer>,
    last_update: Instant,
    frame_count: u32,
    fps_interval: Duration,
    surface_configured: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            game: None,
            renderer: None,
            last_update: Instant::now(),
            frame_count: 0,
            fps_interval: Duration::new(1, 0),
            surface_configured: false,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Fantastic window number one!")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT));
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());


        let renderer_future = Renderer::new(Arc::clone(&window));
        let mut renderer  = futures::executor::block_on(renderer_future);

        let game_future = Game::new(&mut renderer);
        let game = futures::executor::block_on(game_future);


        self.window = Some(window);
        self.game = Some(game);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { event , .. } => {
                if let Some(game) = &mut self.game {
                    game.input(&event);
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(game), Some(renderer)) = (&mut self.game, &mut self.renderer) {
                    game.update(renderer);
                    game.draw(renderer);
                }

                // Update frame count
                self.frame_count += 1;

                // Calculate and print FPS
                let now = Instant::now();
                if now.duration_since(self.last_update) >= self.fps_interval {
                    let fps = self.frame_count as f64 / self.fps_interval.as_secs_f64();
                    //println!("FPS: {}", fps);
                    self.frame_count = 0;
                    self.last_update = now;
                }

                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(physical_size) => {
                self.surface_configured = true;
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(physical_size);
                }
            }
            _ => (),
        }
    }
}
