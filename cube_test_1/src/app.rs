use std::error::Error;

use crate::render::{Input, Renderer};
use winit::application::ApplicationHandler;
use winit::event::ElementState;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowId;

use crate::platform::Platform;
use winit::event::{DeviceEvent, DeviceId, MouseButton};

struct State {
    renderer: Renderer,
    platform: Platform,
    input: Input,
    captured: bool,
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
        self.state = Some(State {
            renderer,
            platform,
            input: Input::default(),
            captured: false,
        });
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
        let Some(state) = &mut self.state else { return };
        if !state.captured {
            return;
        }
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            state.renderer.look(dx as f32, dy as f32);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => state.platform.resize(size.width, size.height),

            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                if let PhysicalKey::Code(code) = event.physical_key {
                    match code {
                        KeyCode::KeyW => state.input.forward = pressed,
                        KeyCode::KeyS => state.input.back = pressed,
                        KeyCode::KeyA => state.input.left = pressed,
                        KeyCode::KeyD => state.input.right = pressed,
                        KeyCode::Space => state.input.up = pressed,
                        KeyCode::ControlLeft | KeyCode::ControlRight => state.input.down = pressed,
                        KeyCode::Escape if pressed && state.captured => {
                            state.platform.set_cursor_captured(false);
                            state.captured = false;
                        }
                        _ => {}
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                let (width, height) = state.platform.size();
                state.renderer.render(width, height, &state.input);
                state.platform.swap();
                state.platform.window.request_redraw();
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if !state.captured {
                    state.platform.set_cursor_captured(true);
                    state.captured = true;
                }
            }

            WindowEvent::Focused(false) => {
                state.input = Input::default();
                if state.captured {
                    state.platform.set_cursor_captured(false);
                    state.captured = false;
                }
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
