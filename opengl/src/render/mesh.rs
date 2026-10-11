use std::mem::size_of;
use std::rc::Rc;

use glow::HasContext;

const FLOATS_PER_VERTEX: usize = 6;

pub struct Mesh {
    gl: Rc<glow::Context>,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ebo: glow::Buffer,
    index_count: i32,
}

impl Mesh {
    pub fn new(gl: Rc<glow::Context>, vertices: &[f32], indices: &[u32]) -> Result<Self, String> {
        unsafe {
            let vao = gl.create_vertex_array()?;
            let vbo = gl.create_buffer()?;
            let ebo = gl.create_buffer()?;

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(indices),
                glow::STATIC_DRAW,
            );

            let stride = (FLOATS_PER_VERTEX * size_of::<f32>()) as i32;

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                stride,
                (3 * size_of::<f32>()) as i32,
            );

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            Ok(Self {
                gl,
                vao,
                vbo,
                ebo,
                index_count: indices.len() as i32,
            })
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl
                .draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);
            self.gl.bind_vertex_array(None);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_buffer(self.ebo);
        }
    }
}
