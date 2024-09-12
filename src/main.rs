mod window;
mod game;
mod renderer;
mod audio;

use window::App;
use winit::event_loop::EventLoop;

#[tokio::main]
async fn main() {

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App::new();

    let _ = event_loop.run_app(&mut app);
}


