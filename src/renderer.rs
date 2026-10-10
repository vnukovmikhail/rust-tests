use std::rc::Rc;

use glow::HasContext;

pub struct Renderer {
    gl: Rc<glow::Context>,
}

impl Renderer {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
        }
        Self { gl }
    }

    pub fn render(&self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
            self.gl.clear_color(0.1, 0.1, 0.1, 1.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }
}
