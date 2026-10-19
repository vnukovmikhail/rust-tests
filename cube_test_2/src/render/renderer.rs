use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use glam::{Mat4, Vec3};
use glow::HasContext;

use crate::render::{Camera, Input};

use super::chunk::CS;
use super::mesh::Mesh;
use super::overlay::{FpsCounter, Overlay};
use super::shader::Program;
use super::texture::Texture;
use super::world::{ChunkPos, World};

pub struct Renderer {
    gl: Rc<glow::Context>,
    program: Program,
    meshes: HashMap<ChunkPos, Mesh>,
    overlay: Overlay,
    fps: FpsCounter,
    camera: Camera,
    last_frame: Instant,
    texture: Texture,
}

impl Renderer {
    pub fn new(gl: Rc<glow::Context>) -> Result<Self, String> {
        let program = Program::new(
            gl.clone(),
            include_str!("shaders/basic.vert"),
            include_str!("shaders/basic.frag"),
        )?;

        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::CULL_FACE);
        }

        let overlay = Overlay::new(gl.clone())?;

        let texture = Texture::from_png(gl.clone(), include_bytes!("../../assets/debug.png"))?;

        let mut world = World::default();
        world.generate(6, 4, 6);

        let mut meshes = HashMap::new();
        for pos in world.positions() {
            let (vertices, indices) = world.build_mesh(pos);
            if !indices.is_empty() {
                meshes.insert(pos, Mesh::new(gl.clone(), &vertices, &indices)?);
            }
        }

        let mut camera = Camera::new(Vec3::new(32.0, 14.0, 76.0));
        camera.speed = 10.0;

        Ok(Self {
            gl,
            program,
            meshes,
            overlay,
            fps: FpsCounter::new(),
            camera,
            last_frame: Instant::now(),
            texture,
        })
    }

    pub fn look(&mut self, dx: f32, dy: f32) {
        self.camera.rotate(dx, dy);
    }

    pub fn render(&mut self, width: u32, height: u32, input: &Input) {
        if width == 0 || height == 0 {
            return;
        }

        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.camera.update(input, dt);

        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
            self.gl.clear_color(0.1, 0.1, 0.1, 1.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        let view = self.camera.view();
        let aspect = width as f32 / height as f32;
        let projection =
            glam::camera::rh::proj::opengl::perspective(45.0_f32.to_radians(), aspect, 0.1, 200.0);

        self.program.bind();

        self.texture.bind(0);
        self.program.set_i32("u_texture", 0);

        self.program.set_mat4("u_view", &view);
        self.program.set_mat4("u_projection", &projection);

        for (&(cx, cy, cz), mesh) in &self.meshes {
            let model = Mat4::from_translation(Vec3::new(
                (cx * CS as i32) as f32,
                (cy * CS as i32) as f32,
                (cz * CS as i32) as f32,
            ));
            self.program.set_mat4("u_model", &model);
            mesh.draw();
        }

        self.fps.tick();
        self.overlay.draw_number(self.fps.fps, width, height);
    }
}
