use crate::program::{self, Shader};
use crate::Transformation;
use iced_graphics::layer;
use iced_native::Rectangle;

const MAX_INSTANCES: usize = 100_000;

#[derive(Debug)]
pub struct Pipeline {
    program: u32,
    vertex_array: u32,
    instances: u32,
    transform_location: u32,
    scale_location: u32,
    screen_height_location: u32,
    current_transform: Transformation,
    current_scale: f32,
    current_target_height: u32,
}

impl Pipeline {
    pub fn new(gl: &elysium_gl::Context, shader_version: &program::Version) -> Pipeline {
        let program = {
            let vertex_shader =
                Shader::vertex(gl, shader_version, include_str!("../shader/core/quad.vert"));

            let fragment_shader =
                Shader::fragment(gl, shader_version, include_str!("../shader/core/quad.frag"));

            unsafe {
                program::create(
                    gl,
                    &[vertex_shader, fragment_shader],
                    &[
                        (0, "i_Pos"),
                        (1, "i_Scale"),
                        (2, "i_Color"),
                        (3, "i_BorderColor"),
                        (4, "i_BorderRadius"),
                        (5, "i_BorderWidth"),
                    ],
                )
            }
        };

        let transform_location = gl
            .get_uniform_location(program, "u_Transform")
            .expect("Get transform location");

        let scale_location = gl
            .get_uniform_location(program, "u_Scale")
            .expect("Get scale location");

        let screen_height_location = gl
            .get_uniform_location(program, "u_ScreenHeight")
            .expect("Get target height location");

        gl.use_program(program);

        let matrix: [f32; 16] = Transformation::identity().into();

        gl.uniform_matrix_4_f32_slice(transform_location, false, &matrix);

        gl.uniform_1_f32(scale_location, 1.0);
        gl.uniform_1_f32(screen_height_location, 0.0);

        gl.use_program(0);

        let (vertex_array, instances) = unsafe { create_instance_buffer(gl, MAX_INSTANCES) };

        Pipeline {
            program,
            vertex_array,
            instances,
            transform_location,
            scale_location,
            screen_height_location,
            current_transform: Transformation::identity(),
            current_scale: 1.0,
            current_target_height: 0,
        }
    }

    pub fn draw(
        &mut self,
        gl: &elysium_gl::Context,
        target_height: u32,
        instances: &[layer::Quad],
        transformation: Transformation,
        scale: f32,
        bounds: Rectangle<u32>,
    ) {
        gl.enable(elysium_gl::SCISSOR_TEST);
        gl.scissor(
            bounds.x as i32,
            (target_height - (bounds.y + bounds.height)) as i32,
            bounds.width as i32,
            bounds.height as i32,
        );

        gl.use_program(self.program);
        gl.bind_vertex_array(self.vertex_array);
        gl.bind_buffer(elysium_gl::ARRAY_BUFFER, self.instances);

        if transformation != self.current_transform {
            let matrix: [f32; 16] = transformation.into();

            gl.uniform_matrix_4_f32_slice(self.transform_location, false, &matrix);

            self.current_transform = transformation;
        }

        if scale != self.current_scale {
            gl.uniform_1_f32(self.scale_location, scale);

            self.current_scale = scale;
        }

        if target_height != self.current_target_height {
            gl.uniform_1_f32(self.screen_height_location, target_height as f32);

            self.current_target_height = target_height;
        }

        for instances in instances.chunks(MAX_INSTANCES) {
            gl.buffer_sub_data_u8_slice(
                elysium_gl::ARRAY_BUFFER,
                0,
                bytemuck::cast_slice(&instances),
            );

            gl.draw_arrays_instanced(elysium_gl::TRIANGLE_STRIP, 0, 4, instances.len() as i32);
        }

        gl.bind_vertex_array(0);
        gl.use_program(0);
        gl.disable(elysium_gl::SCISSOR_TEST);
    }
}

unsafe fn create_instance_buffer(gl: &elysium_gl::Context, size: usize) -> (u32, u32) {
    let vertex_array = gl.create_vertex_array();
    let buffer = gl.create_buffer();

    gl.bind_vertex_array(vertex_array);
    gl.bind_buffer(elysium_gl::ARRAY_BUFFER, buffer);
    gl.buffer_data_size(
        elysium_gl::ARRAY_BUFFER,
        (size * std::mem::size_of::<layer::Quad>()) as i32,
        elysium_gl::DYNAMIC_DRAW,
    );

    let stride = std::mem::size_of::<layer::Quad>() as i32;

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, elysium_gl::FLOAT, false, stride, 0);
    gl.vertex_attrib_divisor(0, 1);

    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_f32(1, 2, elysium_gl::FLOAT, false, stride, 4 * 2);
    gl.vertex_attrib_divisor(1, 1);

    gl.enable_vertex_attrib_array(2);
    gl.vertex_attrib_pointer_f32(2, 4, elysium_gl::FLOAT, false, stride, 4 * (2 + 2));
    gl.vertex_attrib_divisor(2, 1);

    gl.enable_vertex_attrib_array(3);
    gl.vertex_attrib_pointer_f32(3, 4, elysium_gl::FLOAT, false, stride, 4 * (2 + 2 + 4));
    gl.vertex_attrib_divisor(3, 1);

    gl.enable_vertex_attrib_array(4);
    gl.vertex_attrib_pointer_f32(4, 1, elysium_gl::FLOAT, false, stride, 4 * (2 + 2 + 4 + 4));
    gl.vertex_attrib_divisor(4, 1);

    gl.enable_vertex_attrib_array(5);
    gl.vertex_attrib_pointer_f32(
        5,
        1,
        elysium_gl::FLOAT,
        false,
        stride,
        4 * (2 + 2 + 4 + 4 + 1),
    );
    gl.vertex_attrib_divisor(5, 1);

    gl.bind_vertex_array(0);
    gl.bind_buffer(elysium_gl::ARRAY_BUFFER, 0);

    (vertex_array, buffer)
}
