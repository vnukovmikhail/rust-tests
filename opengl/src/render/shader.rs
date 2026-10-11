use std::rc::Rc;

use glam::Mat4;
use glow::HasContext;

pub struct Program {
    gl: Rc<glow::Context>,
    raw: glow::Program,
}

impl Program {
    pub fn new(
        gl: Rc<glow::Context>,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<Self, String> {
        unsafe {
            let program = gl.create_program()?;
            let mut shaders: Vec<glow::Shader> = Vec::new();

            for (kind, src) in [
                (glow::VERTEX_SHADER, vertex_src),
                (glow::FRAGMENT_SHADER, fragment_src),
            ] {
                let shader = gl.create_shader(kind)?;
                gl.shader_source(shader, src);
                gl.compile_shader(shader);

                if !gl.get_shader_compile_status(shader) {
                    let log = gl.get_shader_info_log(shader);
                    gl.delete_shader(shader);
                    for s in shaders {
                        gl.delete_shader(s);
                    }
                    gl.delete_program(program);
                    let name = if kind == glow::VERTEX_SHADER {
                        "vertex"
                    } else {
                        "fragment"
                    };
                    return Err(format!("shader {name} compilation error:\n{log}"));
                }

                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            gl.link_program(program);
            let linked = gl.get_program_link_status(program);

            for s in shaders {
                gl.detach_shader(program, s);
                gl.delete_shader(s);
            }

            if !linked {
                let log = gl.get_program_info_log(program);
                gl.delete_program(program);
                return Err(format!("linking program error:\n{log}"));
            }

            Ok(Self { gl, raw: program })
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.use_program(Some(self.raw)) };
    }

    pub fn set_mat4(&self, name: &str, value: &Mat4) {
        unsafe {
            let location = self.gl.get_uniform_location(self.raw, name);
            self.gl
                .uniform_matrix_4_f32_slice(location.as_ref(), false, &value.to_cols_array());
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.delete_program(self.raw) };
    }
}
