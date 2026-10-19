use std::mem::size_of;
use std::rc::Rc;
use std::time::Instant;

use glow::HasContext;

const VERT: &str = r#"#version 330 core
layout(location = 0) in vec2 a_pos;
uniform vec2 u_screen;
void main() {
    vec2 ndc = vec2(a_pos.x / u_screen.x * 2.0 - 1.0, 1.0 - a_pos.y / u_screen.y * 2.0);
    gl_Position = vec4(ndc, 0.0, 1.0);
}
"#;

const FRAG: &str = r#"#version 330 core
out vec4 frag_color;
void main() { frag_color = vec4(0.2, 1.0, 0.2, 1.0); }
"#;

const DIGITS: [u8; 10] = [
    0b0111111, 0b0000110, 0b1011011, 0b1001111, 0b1100110, 0b1101101, 0b1111101, 0b0000111,
    0b1111111, 0b1101111,
];

pub struct FpsCounter {
    last: Instant,
    frames: u32,
    pub fps: u32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
            frames: 0,
            fps: 0,
        }
    }

    pub fn tick(&mut self) {
        self.frames += 1;
        let elapsed = self.last.elapsed().as_secs_f32();
        if elapsed >= 0.5 {
            self.fps = (self.frames as f32 / elapsed).round() as u32;
            self.frames = 0;
            self.last = Instant::now();
        }
    }
}

pub struct Overlay {
    gl: Rc<glow::Context>,
    program: glow::Program,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    u_screen: Option<glow::UniformLocation>,
}

impl Overlay {
    pub fn new(gl: Rc<glow::Context>) -> Result<Self, String> {
        unsafe {
            let program = gl.create_program()?;
            for (kind, src) in [(glow::VERTEX_SHADER, VERT), (glow::FRAGMENT_SHADER, FRAG)] {
                let sh = gl.create_shader(kind)?;
                gl.shader_source(sh, src);
                gl.compile_shader(sh);
                if !gl.get_shader_compile_status(sh) {
                    return Err(format!(
                        "overlay shader error:\n{}",
                        gl.get_shader_info_log(sh)
                    ));
                }
                gl.attach_shader(program, sh);
                gl.delete_shader(sh);
            }
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(format!(
                    "overlay link error:\n{}",
                    gl.get_program_info_log(program)
                ));
            }
            let u_screen = gl.get_uniform_location(program, "u_screen");

            let vao = gl.create_vertex_array()?;
            let vbo = gl.create_buffer()?;
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                (2 * size_of::<f32>()) as i32,
                0,
            );
            gl.bind_vertex_array(None);

            Ok(Self {
                gl,
                program,
                vao,
                vbo,
                u_screen,
            })
        }
    }

    fn rect(v: &mut Vec<f32>, x: f32, y: f32, w: f32, h: f32) {
        v.extend_from_slice(&[x, y, x + w, y, x + w, y + h, x + w, y + h, x, y + h, x, y]);
    }

    fn build_number(mut n: u32, x0: f32, y0: f32, t: f32) -> Vec<f32> {
        let mut digits = Vec::new();
        loop {
            digits.push((n % 10) as usize);
            n /= 10;
            if n == 0 {
                break;
            }
        }
        digits.reverse();

        let mut v = Vec::new();
        for (i, d) in digits.iter().enumerate() {
            let x = x0 + i as f32 * 7.0 * t;
            let y = y0;
            let m = DIGITS[*d];
            if m & 1 << 0 != 0 {
                Self::rect(&mut v, x, y, 5.0 * t, t);
            }
            if m & 1 << 1 != 0 {
                Self::rect(&mut v, x + 4.0 * t, y, t, 5.0 * t);
            }
            if m & 1 << 2 != 0 {
                Self::rect(&mut v, x + 4.0 * t, y + 4.0 * t, t, 5.0 * t);
            }
            if m & 1 << 3 != 0 {
                Self::rect(&mut v, x, y + 8.0 * t, 5.0 * t, t);
            }
            if m & 1 << 4 != 0 {
                Self::rect(&mut v, x, y + 4.0 * t, t, 5.0 * t);
            }
            if m & 1 << 5 != 0 {
                Self::rect(&mut v, x, y, t, 5.0 * t);
            }
            if m & 1 << 6 != 0 {
                Self::rect(&mut v, x, y + 4.0 * t, 5.0 * t, t);
            }
        }
        v
    }

    pub fn draw_number(&self, n: u32, width: u32, height: u32) {
        let verts = Self::build_number(n, 10.0, 10.0, 3.0);
        unsafe {
            self.gl.disable(glow::DEPTH_TEST);
            self.gl.disable(glow::CULL_FACE);

            self.gl.use_program(Some(self.program));
            self.gl
                .uniform_2_f32(self.u_screen.as_ref(), width as f32, height as f32);
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&verts),
                glow::DYNAMIC_DRAW,
            );
            self.gl
                .draw_arrays(glow::TRIANGLES, 0, (verts.len() / 2) as i32);
            self.gl.bind_vertex_array(None);

            self.gl.enable(glow::DEPTH_TEST);
            self.gl.enable(glow::CULL_FACE);
        }
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.vbo);
        }
    }
}
