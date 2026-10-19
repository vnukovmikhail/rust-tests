use std::error::Error;
use std::num::NonZeroU32;
use std::rc::Rc;

use glow::HasContext;
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentGlContext, PossiblyCurrentContext,
    Version,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::event_loop::ActiveEventLoop;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::CursorGrabMode;
use winit::window::Window;

pub struct Platform {
    pub gl: Rc<glow::Context>,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    pub window: Window,
}

impl Platform {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>> {
        let window_attributes = Window::default_attributes()
            .with_title("OpenGL 3.3 Core")
            .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0));

        let template = ConfigTemplateBuilder::new()
            .with_depth_size(24)
            .prefer_hardware_accelerated(Some(true));

        let (window, gl_config) = DisplayBuilder::new()
            .with_window_attributes(Some(window_attributes))
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|a, b| {
                        if b.num_samples() > a.num_samples() {
                            b
                        } else {
                            a
                        }
                    })
                    .expect("no suitable GL configuration found")
            })?;
        let window = window.ok_or("failed to create window")?;

        let gl_display = gl_config.display();
        let raw_handle = window.window_handle()?.as_raw();
        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_debug(cfg!(debug_assertions))
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_handle));
        let not_current = unsafe { gl_display.create_context(&gl_config, &context_attributes)? };

        let surface_attributes = window.build_surface_attributes(Default::default())?;
        let surface = unsafe { gl_display.create_window_surface(&gl_config, &surface_attributes)? };
        let context = not_current.make_current(&surface)?;

        let mut gl =
            unsafe { glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s)) };

        if let Err(e) =
            surface.set_swap_interval(&context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("failed to enable vsync: {e}");
        }
        /*if let Err(e) = surface.set_swap_interval(&context, SwapInterval::DontWait) {
            eprintln!("failed to enable vsync: {e}");
        }*/

        unsafe {
            println!("GL version:  {}", gl.get_parameter_string(glow::VERSION));
            println!("GL renderer: {}", gl.get_parameter_string(glow::RENDERER));

            if cfg!(debug_assertions) && gl.supported_extensions().contains("GL_KHR_debug") {
                gl.enable(glow::DEBUG_OUTPUT);
                gl.enable(glow::DEBUG_OUTPUT_SYNCHRONOUS);
                gl.debug_message_callback(|_source, _type, id, severity, msg| {
                    eprintln!("GL [{id}] severity={severity:#x}: {msg}");
                });
            }
        }

        Ok(Self {
            gl: Rc::new(gl),
            surface,
            context,
            window,
        })
    }

    pub fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    pub fn resize(&self, width: u32, height: u32) {
        if let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            self.surface.resize(&self.context, w, h);
        }
    }

    pub fn swap(&self) {
        self.window.pre_present_notify();
        if let Err(e) = self.surface.swap_buffers(&self.context) {
            eprintln!("swap_buffers: {e}");
        }
    }

    pub fn set_cursor_captured(&self, captured: bool) {
        if captured {
            if self.window.set_cursor_grab(CursorGrabMode::Locked).is_err() {
                if let Err(e) = self.window.set_cursor_grab(CursorGrabMode::Confined) {
                    eprintln!("cursor grab: {e}");
                }
            }
            self.window.set_cursor_visible(false);
        } else {
            let _ = self.window.set_cursor_grab(CursorGrabMode::None);
            self.window.set_cursor_visible(true);
        }
    }
}
