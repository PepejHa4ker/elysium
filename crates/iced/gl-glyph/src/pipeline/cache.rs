pub struct Cache {
    pub(crate) texture: u32,
    format: u32,
}

impl Cache {
    pub unsafe fn new(
        gl: &elysium_gl::Context,
        width: u32,
        height: u32,
    ) -> Cache {
        let (internal_format, format) =
            (elysium_gl::R8 as i32, elysium_gl::RED);

        gl.pixel_store_i32(elysium_gl::UNPACK_ALIGNMENT, 1);

        let texture = {
            let handle = gl.create_texture();

            gl.bind_texture(elysium_gl::TEXTURE_2D, handle);

            gl.tex_parameter_i32(
                elysium_gl::TEXTURE_2D,
                elysium_gl::TEXTURE_WRAP_S,
                elysium_gl::CLAMP_TO_EDGE as i32,
            );

            gl.tex_parameter_i32(
                elysium_gl::TEXTURE_2D,
                elysium_gl::TEXTURE_WRAP_T,
                elysium_gl::CLAMP_TO_EDGE as i32,
            );

            gl.tex_parameter_i32(
                elysium_gl::TEXTURE_2D,
                elysium_gl::TEXTURE_MIN_FILTER,
                elysium_gl::LINEAR as i32,
            );

            gl.tex_parameter_i32(
                elysium_gl::TEXTURE_2D,
                elysium_gl::TEXTURE_MAG_FILTER,
                elysium_gl::LINEAR as i32,
            );

            gl.tex_parameter_i32_slice(
                elysium_gl::TEXTURE_2D,
                elysium_gl::TEXTURE_SWIZZLE_RGBA,
                &[
                    elysium_gl::ZERO as i32,
                    elysium_gl::ZERO as i32,
                    elysium_gl::ZERO as i32,
                    elysium_gl::RED as i32,
                ],
            );

            gl.tex_image_2d(
                elysium_gl::TEXTURE_2D,
                0,
                internal_format,
                width as i32,
                height as i32,
                0,
                format,
                elysium_gl::UNSIGNED_BYTE,
                None,
            );

            gl.bind_texture(elysium_gl::TEXTURE_2D, 0);

            handle
        };

        Cache { texture, format }
    }

    pub unsafe fn update(
        &self,
        gl: &elysium_gl::Context,
        offset: [u16; 2],
        size: [u16; 2],
        data: &[u8],
    ) {
        let [offset_x, offset_y] = offset;
        let [width, height] = size;

        gl.bind_texture(elysium_gl::TEXTURE_2D, self.texture);

        gl.tex_sub_image_2d(
            elysium_gl::TEXTURE_2D,
            0,
            i32::from(offset_x),
            i32::from(offset_y),
            i32::from(width),
            i32::from(height),
            self.format,
            elysium_gl::UNSIGNED_BYTE,
            elysium_gl::PixelUnpackData::Slice(data),
        );

        gl.bind_texture(elysium_gl::TEXTURE_2D, 0);
    }

    pub unsafe fn destroy(&self, gl: &elysium_gl::Context) {
        gl.delete_texture(self.texture);
    }
}
