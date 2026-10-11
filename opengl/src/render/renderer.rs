use std::rc::Rc;
use std::time::Instant;

use glam::{Mat4, Vec3};
use glow::HasContext;

use super::mesh::Mesh;
use super::shader::Program;

#[rustfmt::skip]
const CUBE_VERTICES: [f32; 48] = [
    -0.5, -0.5,  0.5,   1.0, 0.0, 0.0,
     0.5, -0.5,  0.5,   0.0, 1.0, 0.0,
     0.5,  0.5,  0.5,   0.0, 0.0, 1.0,
    -0.5,  0.5,  0.5,   1.0, 1.0, 0.0,
    -0.5, -0.5, -0.5,   1.0, 0.0, 1.0,
     0.5, -0.5, -0.5,   0.0, 1.0, 1.0,
     0.5,  0.5, -0.5,   1.0, 1.0, 1.0,
    -0.5,  0.5, -0.5,   0.5, 0.5, 0.5,
];

#[rustfmt::skip]
const CUBE_INDICES: [u32; 36] = [
    0, 1, 2,  2, 3, 0,
    5, 4, 7,  7, 6, 5,
    1, 5, 6,  6, 2, 1,
    4, 0, 3,  3, 7, 4,
    3, 2, 6,  6, 7, 3,
    4, 5, 1,  1, 0, 4,
];

pub struct Renderer {
    gl: Rc<glow::Context>,
    program: Program,
    cube: Mesh,
    start: Instant,
}

impl Renderer {
    pub fn new(gl: Rc<glow::Context>) -> Result<Self, String> {
        let program = Program::new(
            gl.clone(),
            include_str!("shaders/basic.vert"),
            include_str!("shaders/basic.frag"),
        )?;
        let cube = Mesh::new(gl.clone(), &CUBE_VERTICES, &CUBE_INDICES)?;

        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::CULL_FACE);
        }

        Ok(Self {
            gl,
            program,
            cube,
            start: Instant::now(),
        })
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

        let time = self.start.elapsed().as_secs_f32();

        let model = Mat4::from_rotation_y(time) * Mat4::from_rotation_x(time * 0.5);
        let view =
            glam::camera::rh::view::look_at_mat4(Vec3::new(0.0, 0.0, 6.0), Vec3::ZERO, Vec3::Y);
        let aspect = width as f32 / height as f32;
        let projection =
            glam::camera::rh::proj::opengl::perspective(45.0_f32.to_radians(), aspect, 0.1, 100.0);

        self.program.bind();
        self.program.set_mat4("u_model", &model);
        self.program.set_mat4("u_view", &view);
        self.program.set_mat4("u_projection", &projection);

        self.cube.draw();
    }
}
