use std::error::Error;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

use crate::platform::Platform;
use crate::render::Renderer;

struct State {
    renderer: Renderer,
    platform: Platform,
}

struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let platform = match Platform::new(event_loop) {
            Ok(platform) => platform,
            Err(e) => {
                eprintln!("Window initialization error: {e}");
                event_loop.exit();
                return;
            }
        };

        let renderer = match Renderer::new(platform.gl.clone()) {
            Ok(renderer) => renderer,
            Err(e) => {
                eprintln!("Render initialization error: {e}");
                event_loop.exit();
                return;
            }
        };

        platform.window.request_redraw();
        self.state = Some(State { renderer, platform });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &self.state else { return };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => state.platform.resize(size.width, size.height),

            WindowEvent::RedrawRequested => {
                let (width, height) = state.platform.size();
                state.renderer.render(width, height);
                state.platform.swap();
                state.platform.window.request_redraw();
            }

            _ => {}
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App { state: None };
    event_loop.run_app(&mut app)?;
    Ok(())
}
