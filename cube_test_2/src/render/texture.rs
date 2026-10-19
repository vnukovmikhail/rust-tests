use std::rc::Rc;

use glow::HasContext;

pub struct Texture {
    gl: Rc<glow::Context>,
    raw: glow::Texture,
}

impl Texture {
    pub fn from_png(gl: Rc<glow::Context>, bytes: &[u8]) -> Result<Self, String> {
        let img = image::load_from_memory(bytes)
            .map_err(|e| format!("texture decode error: {e}"))?
            .flipv()
            .to_rgba8();
        let (w, h) = img.dimensions();

        unsafe {
            let raw = gl.create_texture()?;
            gl.bind_texture(glow::TEXTURE_2D, Some(raw));

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );

            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                w as i32,
                h as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(img.as_raw())),
            );

            gl.bind_texture(glow::TEXTURE_2D, None);
            Ok(Self { gl, raw })
        }
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            self.gl.active_texture(glow::TEXTURE0 + unit);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.raw));
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.delete_texture(self.raw) };
    }
}
